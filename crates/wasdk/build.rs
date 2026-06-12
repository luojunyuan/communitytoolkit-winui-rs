use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_DEPS_DIR: &str = "../xamltoolkit-winui-controls/metadata/deps";
const DEFAULT_FILTERS: &str = include_str!("default.filters");
const BINDGEN_WARNINGS_ENV: &str = "WASDK_BINDGEN_WARNINGS";

fn main() {
    println!("cargo:rerun-if-changed={DEFAULT_DEPS_DIR}");
    println!("cargo:rerun-if-changed=default.filters");
    println!("cargo:rerun-if-env-changed=WASDK_METADATA_DEPS");
    println!("cargo:rerun-if-env-changed=WASDK_FILTERS");
    println!("cargo:rerun-if-env-changed={BINDGEN_WARNINGS_ENV}");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let deps_dir = env::var_os("WASDK_METADATA_DEPS")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join(DEFAULT_DEPS_DIR));
    let deps = collect_winmd_files(&deps_dir);
    if deps.is_empty() {
        panic!(
            "wasdk metadata dependencies are missing. Expected WinMD files under {}",
            deps_dir.display()
        );
    }

    let filters = env::var("WASDK_FILTERS")
        .map(|value| split_filters(&value))
        .unwrap_or_else(|_| default_filters());
    if filters.is_empty() {
        panic!("WASDK_FILTERS did not contain any filters.");
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_file = out_dir.join("bindings.rs");

    let mut args = vec!["--in".to_string(), "default".to_string()];
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
        "windows,skip-root,Windows.Graphics.DirectX".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Graphics.DirectX.Direct3D11".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Graphics.Imaging".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Media".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Media.Capture".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Media.Capture.Frames".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Media.MediaProperties".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Storage".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Storage.FileProperties".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Storage.Search".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Storage.Streams".to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.UI.Color".to_string(),
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
                "cargo:warning=wasdk bindgen skipped inherited or dependency members; see {}",
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
    let mut filters: Vec<String> = [
        "Microsoft.UI.Composition.CompositionBrush",
        "Microsoft.UI.Composition.CompositionClip",
        "Microsoft.UI.Composition.CompositionObject",
        "Microsoft.UI.Composition.CompositionShadow",
        "Microsoft.UI.Composition.Compositor",
        "Microsoft.UI.Composition.ContainerVisual",
        "Microsoft.UI.Composition.DropShadow",
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
        "Microsoft.UI.Dispatching.DispatcherExitDeferral",
        "Microsoft.UI.Dispatching.DispatcherQueue",
        "Microsoft.UI.Dispatching.DispatcherQueueHandler",
        "Microsoft.UI.Dispatching.DispatcherQueuePriority",
        "Microsoft.UI.Dispatching.DispatcherQueueShutdownStartingEventArgs",
        "Microsoft.UI.Dispatching.DispatcherQueueTimer",
        "Microsoft.UI.Dispatching.DispatcherRunOptions",
        "Microsoft.UI.Input.InputSystemCursorShape",
        "Microsoft.UI.Input.PointerPoint",
        "Microsoft.UI.Text.ITextCharacterFormat",
        "Microsoft.UI.Text.ITextRange",
        "Microsoft.UI.Text.RichEditTextDocument",
        "Microsoft.UI.Xaml.Application",
        "Microsoft.UI.Xaml.ApplicationTheme",
        "Microsoft.UI.Xaml.Automation.Peers.AutomationPeer",
        "Microsoft.UI.Xaml.Automation.Peers.ButtonBaseAutomationPeer",
        "Microsoft.UI.Xaml.Automation.Peers.FrameworkElementAutomationPeer",
        "Microsoft.UI.Xaml.Automation.Peers.ItemsControlAutomationPeer",
        "Microsoft.UI.Xaml.Automation.Peers.ListViewBaseAutomationPeer",
        "Microsoft.UI.Xaml.Automation.Peers.RangeBaseAutomationPeer",
        "Microsoft.UI.Xaml.Automation.Peers.SelectorAutomationPeer",
        "Microsoft.UI.Xaml.Automation.Provider.IRangeValueProvider",
        "Microsoft.UI.Xaml.Automation.Provider.IValueProvider",
        "Microsoft.UI.Xaml.Controls.AppBar",
        "Microsoft.UI.Xaml.Controls.AutoSuggestBox",
        "Microsoft.UI.Xaml.Controls.AutoSuggestBoxQuerySubmittedEventArgs",
        "Microsoft.UI.Xaml.Controls.AutoSuggestBoxSuggestionChosenEventArgs",
        "Microsoft.UI.Xaml.Controls.AutoSuggestBoxTextChangedEventArgs",
        "Microsoft.UI.Xaml.Controls.Border",
        "Microsoft.UI.Xaml.Controls.Button",
        "Microsoft.UI.Xaml.Controls.ColorChangedEventArgs",
        "Microsoft.UI.Xaml.Controls.ColorPicker",
        "Microsoft.UI.Xaml.Controls.ColorSpectrumComponents",
        "Microsoft.UI.Xaml.Controls.ColorSpectrumShape",
        "Microsoft.UI.Xaml.Controls.CommandBar",
        "Microsoft.UI.Xaml.Controls.ContentControl",
        "Microsoft.UI.Xaml.Controls.ContentPresenter",
        "Microsoft.UI.Xaml.Controls.Control",
        "Microsoft.UI.Xaml.Controls.DataTemplateSelector",
        "Microsoft.UI.Xaml.Controls.DisabledFormattingAccelerators",
        "Microsoft.UI.Xaml.Controls.DropDownButton",
        "Microsoft.UI.Xaml.Controls.FontIcon",
        "Microsoft.UI.Xaml.Controls.FontIconSource",
        "Microsoft.UI.Xaml.Controls.Grid",
        "Microsoft.UI.Xaml.Controls.IconElement",
        "Microsoft.UI.Xaml.Controls.IconSource",
        "Microsoft.UI.Xaml.Controls.ItemsControl",
        "Microsoft.UI.Xaml.Controls.Layout",
        "Microsoft.UI.Xaml.Controls.ListViewBase",
        "Microsoft.UI.Xaml.Controls.ListViewItem",
        "Microsoft.UI.Xaml.Controls.NavigationView",
        "Microsoft.UI.Xaml.Controls.Orientation",
        "Microsoft.UI.Xaml.Controls.Panel",
        "Microsoft.UI.Xaml.Controls.Primitives.ButtonBase",
        "Microsoft.UI.Xaml.Controls.Primitives.DragCompletedEventArgs",
        "Microsoft.UI.Xaml.Controls.Primitives.DragCompletedEventHandler",
        "Microsoft.UI.Xaml.Controls.Primitives.DragStartedEventArgs",
        "Microsoft.UI.Xaml.Controls.Primitives.DragStartedEventHandler",
        "Microsoft.UI.Xaml.Controls.Primitives.RangeBase",
        "Microsoft.UI.Xaml.Controls.Primitives.Selector",
        "Microsoft.UI.Xaml.Controls.Primitives.SelectorItem",
        "Microsoft.UI.Xaml.Controls.RichEditClipboardFormat",
        "Microsoft.UI.Xaml.Controls.Slider",
        "Microsoft.UI.Xaml.Controls.StyleSelector",
        "Microsoft.UI.Xaml.Controls.Symbol",
        "Microsoft.UI.Xaml.Controls.SymbolIcon",
        "Microsoft.UI.Xaml.Controls.SymbolIconSource",
        "Microsoft.UI.Xaml.Controls.TextBlock",
        "Microsoft.UI.Xaml.Controls.TextControlPasteEventArgs",
        "Microsoft.UI.Xaml.Controls.TreeView",
        "Microsoft.UI.Xaml.Controls.UIElementCollection",
        "Microsoft.UI.Xaml.Controls.VirtualizingLayout",
        "Microsoft.UI.Xaml.CornerRadius",
        "Microsoft.UI.Xaml.Data.INotifyPropertyChanged",
        "Microsoft.UI.Xaml.Data.IValueConverter",
        "Microsoft.UI.Xaml.DataTemplate",
        "Microsoft.UI.Xaml.DependencyObject",
        "Microsoft.UI.Xaml.DependencyObjectCollection",
        "Microsoft.UI.Xaml.DependencyProperty",
        "Microsoft.UI.Xaml.DependencyPropertyChangedCallback",
        "Microsoft.UI.Xaml.DependencyPropertyChangedEventArgs",
        "Microsoft.UI.Xaml.Documents.Hyperlink",
        "Microsoft.UI.Xaml.FrameworkElement",
        "Microsoft.UI.Xaml.FrameworkTemplate",
        "Microsoft.UI.Xaml.HorizontalAlignment",
        "Microsoft.UI.Xaml.IXamlServiceProvider",
        "Microsoft.UI.Xaml.Input.ICommand",
        "Microsoft.UI.Xaml.Markup.IXamlMetadataProvider",
        "Microsoft.UI.Xaml.Markup.IXamlType",
        "Microsoft.UI.Xaml.Markup.MarkupExtension",
        "Microsoft.UI.Xaml.Markup.XmlnsDefinition",
        "Microsoft.UI.Xaml.Media.Brush",
        "Microsoft.UI.Xaml.Media.FontFamily",
        "Microsoft.UI.Xaml.Media.GeneralTransform",
        "Microsoft.UI.Xaml.Media.ImageSource",
        "Microsoft.UI.Xaml.Media.Imaging.BitmapSource",
        "Microsoft.UI.Xaml.Media.Imaging.WriteableBitmap",
        "Microsoft.UI.Xaml.Media.Matrix",
        "Microsoft.UI.Xaml.Media.SolidColorBrush",
        "Microsoft.UI.Xaml.Media.Transform",
        "Microsoft.UI.Xaml.ResourceDictionary",
        "Microsoft.UI.Xaml.RoutedEventArgs",
        "Microsoft.UI.Xaml.RoutedEventHandler",
        "Microsoft.UI.Xaml.StateTriggerBase",
        "Microsoft.UI.Xaml.Style",
        "Microsoft.UI.Xaml.Thickness",
        "Microsoft.UI.Xaml.UIElement",
        "Microsoft.UI.Xaml.Visibility",
        "Windows.UI.Xaml.Interop.TypeKind",
        "Windows.UI.Xaml.Interop.TypeName",
    ]
    .into_iter()
    .map(str::to_string)
    .collect();

    filters.extend(
        DEFAULT_FILTERS
            .lines()
            .map(str::trim)
            .filter(|filter| !filter.is_empty() && !filter.starts_with('#'))
            .map(str::to_string),
    );
    filters.sort();
    filters.dedup();
    filters
}

fn split_filters(value: &str) -> Vec<String> {
    value
        .split(';')
        .map(str::trim)
        .filter(|filter| !filter.is_empty())
        .map(str::to_string)
        .collect()
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
            && should_include_metadata(&path)
        {
            println!("cargo:rerun-if-changed={}", path.display());
            files.push(path);
        }
    }
}

fn should_include_metadata(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };

    name.eq_ignore_ascii_case("Windows.winmd") || name.starts_with("Microsoft.")
}
