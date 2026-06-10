use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const TOOLKIT_WINMD: &str = "metadata/XamlToolkit.WinUI.winmd";
const DEFAULT_DEPS_DIR: &str = "metadata/deps";

fn main() {
    println!("cargo:rerun-if-changed={TOOLKIT_WINMD}");
    println!("cargo:rerun-if-changed={DEFAULT_DEPS_DIR}");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_WINMD");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_METADATA_DEPS");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_FILTERS");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let toolkit_winmd = env::var_os("XAMLTOOLKIT_WINUI_WINMD")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join(TOOLKIT_WINMD));

    require_file(
        &toolkit_winmd,
        "XamlToolkit.WinUI metadata is missing. Build or generate XamlToolkit.WinUI.winmd first, then copy it to xamltoolkit-rs/metadata/.",
    );

    let deps_dir = env::var_os("XAMLTOOLKIT_WINUI_METADATA_DEPS")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join(DEFAULT_DEPS_DIR));
    let deps = collect_winmd_files(&deps_dir);

    let filters = env::var("XAMLTOOLKIT_WINUI_FILTERS")
        .map(|value| {
            value
                .split(';')
                .map(str::trim)
                .filter(|filter| !filter.is_empty())
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|_| {
            vec![
                "XamlToolkit.WinUI.HslColor".to_string(),
                "XamlToolkit.WinUI.HsvColor".to_string(),
            ]
        });

    if filters.is_empty() {
        panic!("XAMLTOOLKIT_WINUI_FILTERS did not contain any filters.");
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_file = out_dir.join("bindings.rs");

    let mut args = vec![
        "--in".to_string(),
        "default".to_string(),
        toolkit_winmd.display().to_string(),
    ];
    args.extend(deps.iter().map(|path| path.display().to_string()));
    args.extend([
        "--out".to_string(),
        out_file.display().to_string(),
        "--no-allow".to_string(),
        "--filter".to_string(),
    ]);
    args.extend(filters);

    windows_bindgen::bindgen(args).unwrap();

    if !out_file.exists() {
        panic!(
            "windows-bindgen completed but did not create {}",
            out_file.display()
        );
    }
}

fn require_file(path: &Path, message: &str) {
    if !path.is_file() {
        let parent = path.parent().unwrap_or_else(|| Path::new("."));
        let _ = fs::create_dir_all(parent);
        panic!("{message}\nExpected: {}", path.display());
    }
}

fn collect_winmd_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_winmd_files_inner(dir, &mut files);
    files.sort();
    files
}

fn collect_winmd_files_inner(dir: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_winmd_files_inner(&path, files);
        } else if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("winmd"))
        {
            println!("cargo:rerun-if-changed={}", path.display());
            files.push(path);
        }
    }
}
