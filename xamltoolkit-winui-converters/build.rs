use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const CONVERTERS_WINMD: &str = "metadata/XamlToolkit.WinUI.Converters.winmd";
const EXTRA_FILTERS: &str = include_str!("extra.filters");
const DEFAULT_DEPS_DIR: &str = "metadata/deps";
const DEFAULT_WASDK_DEPS_DIR: &str = "../wasdk/metadata/deps";
const BINDGEN_WARNINGS_ENV: &str = "XAMLTOOLKIT_WINUI_CONVERTERS_BINDGEN_WARNINGS";

fn main() {
    println!("cargo:rerun-if-changed={CONVERTERS_WINMD}");
    println!("cargo:rerun-if-changed=extra.filters");
    println!("cargo:rerun-if-changed={DEFAULT_DEPS_DIR}");
    println!("cargo:rerun-if-changed={DEFAULT_WASDK_DEPS_DIR}");
    println!("cargo:rerun-if-env-changed=WASDK_METADATA_DEPS");
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
        "XamlToolkit.WinUI.Converters metadata is missing. Run tools/sync-metadata.ps1 -Project Converters to refresh checked-in metadata.",
    );

    let deps = dependency_winmd_files(
        &manifest_dir,
        "XAMLTOOLKIT_WINUI_CONVERTERS_METADATA_DEPS",
        DEFAULT_DEPS_DIR,
    );

    let filters = env::var("XAMLTOOLKIT_WINUI_CONVERTERS_FILTERS")
        .map(|value| split_filters(&value))
        .unwrap_or_else(|_| default_filters());
    let mut filters = without_wasdk_filters(filters);
    filters.extend(
        EXTRA_FILTERS
            .lines()
            .map(str::trim)
            .filter(|filter| !filter.is_empty() && !filter.starts_with('#'))
            .map(str::to_string),
    );
    filters.sort();
    filters.dedup();

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
    args.extend(["--out".to_string(), out_file.display().to_string()]);
    args.push("--filter".to_string());
    args.extend(filters);

    windows_bindgen::bindgen(args);

    if !out_file.exists() {
        panic!(
            "windows-bindgen completed but did not create {}",
            out_file.display()
        );
    }

    patch_collection_deref(&out_file);
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

fn without_wasdk_filters(filters: Vec<String>) -> Vec<String> {
    filters
        .into_iter()
        .filter(|filter| !is_wasdk_filter(filter))
        .collect()
}

fn patch_collection_deref(out_file: &Path) {
    let mut generated = fs::read_to_string(out_file)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", out_file.display()));

    let struct_names: Vec<String> = generated
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            let rest = line.strip_prefix("pub struct ")?;
            let (name, rest) = rest.split_once("(windows_core::IUnknown)")?;
            rest.trim_end_matches(';')
                .is_empty()
                .then(|| name.to_string())
        })
        .collect();

    for name in struct_names {
        let marker = format!("impl core::ops::Deref for {name} {{");
        if generated.contains(&marker) {
            continue;
        }

        let iterator_marker = format!("impl IntoIterator for &{name} {{");
        let Some(iterator_start) = generated.find(&iterator_marker) else {
            continue;
        };
        let Some(item_offset) = generated[iterator_start..].find("type Item =") else {
            continue;
        };
        let item_start = iterator_start + item_offset + "type Item =".len();
        let Some(item_len) = generated[item_start..].find(';') else {
            continue;
        };
        let item = generated[item_start..item_start + item_len]
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if item.is_empty() {
            continue;
        }

        let deref = format!(
            "impl core::ops::Deref for {name} {{\n\
    type Target = windows_collections::IIterable<{item}>;\n\
    fn deref(&self) -> &Self::Target {{\n\
        unsafe {{ core::mem::transmute(self) }}\n\
    }}\n\
}}\n"
        );
        generated.insert_str(iterator_start, &deref);
    }

    fs::write(out_file, generated)
        .unwrap_or_else(|error| panic!("failed to write {}: {error}", out_file.display()));
}

fn is_wasdk_filter(filter: &str) -> bool {
    filter.starts_with("Microsoft.") || filter.starts_with("Windows.UI.Xaml.")
}

fn dependency_winmd_files(
    manifest_dir: &Path,
    toolkit_deps_env: &str,
    default_toolkit_deps_dir: &str,
) -> Vec<PathBuf> {
    let wasdk_deps_dir = env::var_os("WASDK_METADATA_DEPS")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join(DEFAULT_WASDK_DEPS_DIR));
    let toolkit_deps_dir = env::var_os(toolkit_deps_env)
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join(default_toolkit_deps_dir));

    let mut files = collect_winmd_files(&wasdk_deps_dir);
    files.extend(collect_winmd_files(&toolkit_deps_dir));
    files.sort();
    files.dedup();
    files
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
