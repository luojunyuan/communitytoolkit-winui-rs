use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const TOOLKIT_WINMD: &str = "metadata/XamlToolkit.WinUI.winmd";
const DEFAULT_DEPS_DIR: &str = "metadata/deps";
const BINDGEN_WARNINGS_ENV: &str = "XAMLTOOLKIT_WINUI_BINDGEN_WARNINGS";

fn main() {
    println!("cargo:rerun-if-changed={TOOLKIT_WINMD}");
    println!("cargo:rerun-if-changed={DEFAULT_DEPS_DIR}");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_WINMD");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_METADATA_DEPS");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_FILTERS");
    println!("cargo:rerun-if-env-changed={BINDGEN_WARNINGS_ENV}");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let toolkit_winmd = env::var_os("XAMLTOOLKIT_WINUI_WINMD")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join(TOOLKIT_WINMD));

    require_file(
        &toolkit_winmd,
        "XamlToolkit.WinUI metadata is missing. Run tools/sync-metadata.ps1 -Project Root to refresh checked-in metadata.",
    );

    let deps_dir = env::var_os("XAMLTOOLKIT_WINUI_METADATA_DEPS")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join(DEFAULT_DEPS_DIR));
    let deps = collect_winmd_files(&deps_dir);

    let filters = env::var("XAMLTOOLKIT_WINUI_FILTERS")
        .map(|value| split_filters(&value))
        .unwrap_or_else(|_| default_filters());
    let filters = without_wasdk_filters(filters);

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
        "--reference".to_string(),
        "windows,skip-root,Windows.Foundation".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Foundation.Collections".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Foundation.Numerics".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Graphics".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.UI.Color".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.UI.Composition".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.UI.Core".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.UI.Text".to_string(),
    ]);
    append_wasdk_references(&mut args);
    args.push("--filter".to_string());
    args.extend(filters);

    let warnings = windows_bindgen::bindgen(args);
    if !warnings.is_empty() {
        let warnings_file = out_dir.join("bindgen-warnings.txt");
        let warnings_text = format!("{warnings}");
        fs::write(&warnings_file, warnings_text)
            .unwrap_or_else(|error| panic!("failed to write {}: {error}", warnings_file.display()));

        let warnings_text = fs::read_to_string(&warnings_file)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", warnings_file.display()));
        if has_toolkit_projection_warning(&warnings_text) {
            panic!(
                "windows-bindgen skipped Toolkit projection members for xamltoolkit-winui; see {}",
                warnings_file.display()
            );
        }

        if env::var_os(BINDGEN_WARNINGS_ENV).is_some() {
            println!(
                "cargo:warning=xamltoolkit-winui bindgen skipped inherited or dependency members; see {}",
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

    patch_generated_bindings(&out_file);
}

fn has_toolkit_projection_warning(warnings: &str) -> bool {
    warnings.contains("XamlToolkit.WinUI.")
}

fn default_filters() -> Vec<String> {
    [
        "Microsoft.UI.Composition.CompositionBrush",
        "Microsoft.UI.Composition.CompositionClip",
        "Microsoft.UI.Composition.CompositionObject",
        "Microsoft.UI.Composition.CompositionShadow",
        "Microsoft.UI.Composition.Compositor",
        "Microsoft.UI.Composition.ContainerVisual",
        "Microsoft.UI.Composition.DropShadow",
        "Microsoft.UI.Composition.ICompositor",
        "Microsoft.UI.Composition.ICompositor2",
        "Microsoft.UI.Composition.ICompositor4",
        "Microsoft.UI.Composition.ICompositor5",
        "Microsoft.UI.Composition.ICompositor6",
        "Microsoft.UI.Composition.ICompositor7",
        "Microsoft.UI.Composition.ICompositor8",
        "Microsoft.UI.Composition.ICompositorStatics",
        "Microsoft.UI.Composition.ICompositorWithProjectedShadow",
        "Microsoft.UI.Composition.ICompositorWithRadialGradient",
        "Microsoft.UI.Composition.ICompositorWithVisualSurface",
        "Microsoft.UI.Composition.ICompositionBrush",
        "Microsoft.UI.Composition.ICompositionBrushFactory",
        "Microsoft.UI.Composition.ICompositionClip",
        "Microsoft.UI.Composition.ICompositionClip2",
        "Microsoft.UI.Composition.ICompositionClipFactory",
        "Microsoft.UI.Composition.ICompositionObject",
        "Microsoft.UI.Composition.ICompositionObject2",
        "Microsoft.UI.Composition.ICompositionObject3",
        "Microsoft.UI.Composition.ICompositionObject4",
        "Microsoft.UI.Composition.ICompositionObject5",
        "Microsoft.UI.Composition.ICompositionObjectFactory",
        "Microsoft.UI.Composition.ICompositionObjectStatics",
        "Microsoft.UI.Composition.ICompositionShadow",
        "Microsoft.UI.Composition.ICompositionShadowFactory",
        "Microsoft.UI.Composition.IContainerVisual",
        "Microsoft.UI.Composition.IContainerVisualFactory",
        "Microsoft.UI.Composition.IDropShadow",
        "Microsoft.UI.Composition.IDropShadow2",
        "Microsoft.UI.Composition.ISpriteVisual",
        "Microsoft.UI.Composition.ISpriteVisual2",
        "Microsoft.UI.Composition.IVisual",
        "Microsoft.UI.Composition.IVisual2",
        "Microsoft.UI.Composition.IVisual3",
        "Microsoft.UI.Composition.IVisual4",
        "Microsoft.UI.Composition.IVisualFactory",
        "Microsoft.UI.Composition.SpriteVisual",
        "Microsoft.UI.Composition.Visual",
        "Microsoft.UI.Input.InputSystemCursorShape",
        "Microsoft.UI.Xaml.DependencyObject",
        "Microsoft.UI.Xaml.DependencyProperty",
        "Microsoft.UI.Xaml.DependencyPropertyChangedEventArgs",
        "Microsoft.UI.Xaml.FrameworkElement",
        "Microsoft.UI.Xaml.Markup.MarkupExtension",
        "Microsoft.UI.Xaml.StateTriggerBase",
        "Microsoft.UI.Xaml.UIElement",
        "Microsoft.UI.Xaml.Controls.ContentControl",
        "Microsoft.UI.Xaml.Controls.Control",
        "Microsoft.UI.Xaml.Controls.FontIcon",
        "Microsoft.UI.Xaml.Controls.FontIconSource",
        "Microsoft.UI.Xaml.Controls.IconElement",
        "Microsoft.UI.Xaml.Controls.IconSource",
        "Microsoft.UI.Xaml.Controls.ItemsControl",
        "Microsoft.UI.Xaml.Controls.ListViewBase",
        "Microsoft.UI.Xaml.Controls.Symbol",
        "Microsoft.UI.Xaml.Controls.SymbolIcon",
        "Microsoft.UI.Xaml.Controls.SymbolIconSource",
        "Microsoft.UI.Xaml.Controls.TextBlock",
        "Microsoft.UI.Xaml.DataTemplate",
        "Microsoft.UI.Xaml.Documents.Hyperlink",
        "Microsoft.UI.Xaml.Input.ICommand",
        "Microsoft.UI.Xaml.Media.Brush",
        "Microsoft.UI.Xaml.Media.FontFamily",
        "Microsoft.UI.Xaml.Media.GeneralTransform",
        "Microsoft.UI.Xaml.Media.Matrix",
        "Microsoft.UI.Xaml.Media.Transform",
        "Windows.UI.Xaml.Interop.TypeKind",
        "Windows.UI.Xaml.Interop.TypeName",
        "XamlToolkit.WinUI.AttachedDropShadow",
        "XamlToolkit.WinUI.AttachedShadowBase",
        "XamlToolkit.WinUI.AttachedShadowElementContext",
        "XamlToolkit.WinUI.ControlSizeTrigger",
        "XamlToolkit.WinUI.Effects",
        "XamlToolkit.WinUI.FontIconExtension",
        "XamlToolkit.WinUI.FontIconSourceExtension",
        "XamlToolkit.WinUI.FrameworkElementExtensions",
        "XamlToolkit.WinUI.HslColor",
        "XamlToolkit.WinUI.HsvColor",
        "XamlToolkit.WinUI.HyperlinkExtensions",
        "XamlToolkit.WinUI.IAlphaMaskProvider",
        "XamlToolkit.WinUI.IAttachedShadow",
        "XamlToolkit.WinUI.IsEqualStateTrigger",
        "XamlToolkit.WinUI.IsNullOrEmptyStateTrigger",
        "XamlToolkit.WinUI.ItemContainerStretchDirection",
        "XamlToolkit.WinUI.ListViewExtensions",
        "XamlToolkit.WinUI.MatrixExtensions",
        "XamlToolkit.WinUI.RectExtensions",
        "XamlToolkit.WinUI.ScrollItemPlacement",
        "XamlToolkit.WinUI.SymbolIconExtension",
        "XamlToolkit.WinUI.SymbolIconSourceExtension",
        "XamlToolkit.WinUI.TextIconExtension",
        "XamlToolkit.WinUI.TransformExtensions",
        "XamlToolkit.WinUI.UIElementExtensions",
        "XamlToolkit.WinUI.VisualExtensions",
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

fn is_wasdk_filter(filter: &str) -> bool {
    filter.starts_with("Microsoft.") || filter.starts_with("Windows.UI.Xaml.")
}

fn append_wasdk_references(args: &mut Vec<String>) {
    for namespace in [
        "Microsoft.UI",
        "Microsoft.UI.Composition",
        "Microsoft.UI.Dispatching",
        "Microsoft.UI.Input",
        "Microsoft.UI.Text",
        "Microsoft.UI.Xaml",
        "Microsoft.UI.Xaml.Automation",
        "Microsoft.UI.Xaml.Automation.Peers",
        "Microsoft.UI.Xaml.Automation.Provider",
        "Microsoft.UI.Xaml.Controls",
        "Microsoft.UI.Xaml.Controls.Primitives",
        "Microsoft.UI.Xaml.Data",
        "Microsoft.UI.Xaml.Documents",
        "Microsoft.UI.Xaml.Input",
        "Microsoft.UI.Xaml.Interop",
        "Microsoft.UI.Xaml.Markup",
        "Microsoft.UI.Xaml.Media",
        "Microsoft.UI.Xaml.Media.Animation",
        "Microsoft.UI.Xaml.Media.Imaging",
        "Microsoft.UI.Xaml.Media.Media3D",
        "Microsoft.UI.Xaml.Navigation",
        "Windows.UI.Xaml.Interop",
    ] {
        args.push("--reference".to_string());
        args.push(format!("wasdk,full,{namespace}"));
    }
}

fn patch_generated_bindings(out_file: &Path) {
    let mut generated = fs::read_to_string(out_file)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", out_file.display()));

    if generated.contains("pub struct IReference<T>")
        && !generated.contains("xamltoolkit_winui_ireference_from_bridge")
    {
        generated.push_str(
            r#"

#[allow(non_snake_case)]
mod xamltoolkit_winui_ireference_from_bridge {
    use super::Windows::Foundation::IReference;
    use windows_core::{Interface, RuntimeType};

    impl<T> From<T> for IReference<T>
    where
        T: RuntimeType + Clone + 'static,
    {
        fn from(value: T) -> Self {
            let reference = windows_reference::IReference::<T>::from(value);
            unsafe { Self::from_raw(reference.into_raw()) }
        }
    }
}
"#,
        );
    }

    fs::write(out_file, generated)
        .unwrap_or_else(|error| panic!("failed to write {}: {error}", out_file.display()));
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
