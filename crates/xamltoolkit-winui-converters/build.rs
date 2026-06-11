use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const CONVERTERS_WINMD: &str = "metadata/XamlToolkit.WinUI.Converters.winmd";
const DEFAULT_DEPS_DIR: &str = "metadata/deps";
const BINDGEN_WARNINGS_ENV: &str = "XAMLTOOLKIT_WINUI_CONVERTERS_BINDGEN_WARNINGS";

fn main() {
    println!("cargo:rerun-if-changed={CONVERTERS_WINMD}");
    println!("cargo:rerun-if-changed={DEFAULT_DEPS_DIR}");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_CONVERTERS_WINMD");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_CONVERTERS_METADATA_DEPS");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_CONVERTERS_FILTERS");
    println!("cargo:rerun-if-env-changed={BINDGEN_WARNINGS_ENV}");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let converters_winmd = env::var_os("XAMLTOOLKIT_WINUI_CONVERTERS_WINMD")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join(CONVERTERS_WINMD));
    require_file(
        &converters_winmd,
        "XamlToolkit.WinUI.Converters metadata is missing. Run crates/xamltoolkit-winui-converters/sync-metadata.ps1 to refresh checked-in metadata.",
    );

    let deps_dir = env::var_os("XAMLTOOLKIT_WINUI_CONVERTERS_METADATA_DEPS")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join(DEFAULT_DEPS_DIR));
    let deps = collect_winmd_files(&deps_dir);

    let filters = env::var("XAMLTOOLKIT_WINUI_CONVERTERS_FILTERS")
        .map(|value| split_filters(&value))
        .unwrap_or_else(|_| default_filters());

    if filters.is_empty() {
        panic!("XAMLTOOLKIT_WINUI_CONVERTERS_FILTERS did not contain any filters.");
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_file = out_dir.join("bindings.rs");

    let mut args = vec![
        "--in".to_string(),
        "default".to_string(),
        converters_winmd.display().to_string(),
    ];
    args.extend(deps.iter().map(|path| path.display().to_string()));
    args.extend([
        "--out".to_string(),
        out_file.display().to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Foundation".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Foundation.Collections".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Foundation.Numerics".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Graphics".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.UI".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.UI.Composition".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.UI.Core".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.UI.Text".to_string(),
        "--filter".to_string(),
    ]);
    args.extend(filters);

    let warnings = windows_bindgen::bindgen(args);
    if !warnings.is_empty() {
        let warnings_file = out_dir.join("bindgen-warnings.txt");
        let warnings_text = format!("{warnings}");
        fs::write(&warnings_file, warnings_text)
            .unwrap_or_else(|error| panic!("failed to write {}: {error}", warnings_file.display()));

        if env::var_os(BINDGEN_WARNINGS_ENV).is_some() {
            println!(
                "cargo:warning=xamltoolkit-winui-converters bindgen skipped inherited or dependency members; see {}",
                warnings_file.display()
            );
        }
    }

    if !out_file.exists() {
        panic!(
            "windows-bindgen completed but did not create {}",
            out_file.display()
        );
    }
}

fn default_filters() -> Vec<String> {
    [
        "Microsoft.UI.Xaml.DependencyObject",
        "Microsoft.UI.Xaml.DependencyProperty",
        "Microsoft.UI.Xaml.Data.IValueConverter",
        "Microsoft.UI.Xaml.Visibility",
        "Windows.UI.Xaml.Interop.TypeKind",
        "Windows.UI.Xaml.Interop.TypeName",
        "XamlToolkit.WinUI.Converters.BoolNegationConverter",
        "XamlToolkit.WinUI.Converters.BoolToObjectConverter",
        "XamlToolkit.WinUI.Converters.BoolToVisibilityConverter",
        "XamlToolkit.WinUI.Converters.CollectionVisibilityConverter",
        "XamlToolkit.WinUI.Converters.ColorToDisplayNameConverter",
        "XamlToolkit.WinUI.Converters.DoubleToObjectConverter",
        "XamlToolkit.WinUI.Converters.DoubleToVisibilityConverter",
        "XamlToolkit.WinUI.Converters.EmptyCollectionToObjectConverter",
        "XamlToolkit.WinUI.Converters.EmptyObjectToObjectConverter",
        "XamlToolkit.WinUI.Converters.EmptyStringToObjectConverter",
        "XamlToolkit.WinUI.Converters.FileSizeToFriendlyStringConverter",
        "XamlToolkit.WinUI.Converters.ResourceNameToResourceStringConverter",
        "XamlToolkit.WinUI.Converters.StringFormatConverter",
        "XamlToolkit.WinUI.Converters.StringVisibilityConverter",
        "XamlToolkit.WinUI.Converters.TypeToObjectConverter",
        "XamlToolkit.WinUI.Converters.VisibilityToBoolConverter",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
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
