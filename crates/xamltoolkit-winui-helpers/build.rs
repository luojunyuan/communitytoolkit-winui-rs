use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const HELPERS_WINMD: &str = "metadata/XamlToolkit.WinUI.Helpers.winmd";
const DEFAULT_DEPS_DIR: &str = "metadata/deps";

fn main() {
    println!("cargo:rerun-if-changed={HELPERS_WINMD}");
    println!("cargo:rerun-if-changed={DEFAULT_DEPS_DIR}");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_HELPERS_WINMD");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_HELPERS_METADATA_DEPS");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_HELPERS_FILTERS");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let helpers_winmd = env::var_os("XAMLTOOLKIT_WINUI_HELPERS_WINMD")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join(HELPERS_WINMD));
    require_file(
        &helpers_winmd,
        "XamlToolkit.WinUI.Helpers metadata is missing. Build XamlToolkit.WinUI.Helpers or copy XamlToolkit.WinUI.Helpers.winmd to xamltoolkit-rs/crates/xamltoolkit-winui-helpers/metadata/.",
    );

    let deps_dir = env::var_os("XAMLTOOLKIT_WINUI_HELPERS_METADATA_DEPS")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join(DEFAULT_DEPS_DIR));
    let deps = collect_winmd_files(&deps_dir);

    let filters = env::var("XAMLTOOLKIT_WINUI_HELPERS_FILTERS")
        .map(|value| split_filters(&value))
        .unwrap_or_else(|_| {
            vec![
                "Windows.UI.Color".to_string(),
                "XamlToolkit.WinUI.HslColor".to_string(),
                "XamlToolkit.WinUI.HsvColor".to_string(),
                "XamlToolkit.WinUI.Helpers.ColorHelper".to_string(),
                "XamlToolkit.WinUI.Helpers.DesignTimeHelpers".to_string(),
            ]
        });

    generate_bindings(&helpers_winmd, &deps, filters, "xamltoolkit-winui-helpers");
}

fn generate_bindings(winmd: &Path, deps: &[PathBuf], filters: Vec<String>, crate_name: &str) {
    if filters.is_empty() {
        panic!("{crate_name} filters did not contain any entries.");
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_file = out_dir.join("bindings.rs");

    let mut args = vec![
        "--in".to_string(),
        "default".to_string(),
        winmd.display().to_string(),
    ];
    args.extend(deps.iter().map(|path| path.display().to_string()));
    args.extend([
        "--out".to_string(),
        out_file.display().to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Foundation".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Graphics".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Media".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Storage".to_string(),
        "--filter".to_string(),
    ]);
    args.extend(filters);

    let warnings = windows_bindgen::bindgen(args);
    if !warnings.is_empty() {
        println!(
            "cargo:warning={crate_name} generated with skipped members for the current projection:\n{warnings}"
        );
    }

    if !out_file.exists() {
        panic!(
            "windows-bindgen completed but did not create {}",
            out_file.display()
        );
    }
}

fn split_filters(value: &str) -> Vec<String> {
    value
        .split(';')
        .map(str::trim)
        .filter(|filter| !filter.is_empty())
        .map(str::to_string)
        .collect()
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
