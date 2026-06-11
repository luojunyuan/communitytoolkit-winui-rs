use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const CONTROLS_WINMD: &str = "metadata/XamlToolkit.WinUI.Controls.winmd";
const DEFAULT_DEPS_DIR: &str = "metadata/deps";

fn main() {
    println!("cargo:rerun-if-changed={CONTROLS_WINMD}");
    println!("cargo:rerun-if-changed={DEFAULT_DEPS_DIR}");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_CONTROLS_WINMD");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_CONTROLS_METADATA_DEPS");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_CONTROLS_FILTERS");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let controls_winmd = env::var_os("XAMLTOOLKIT_WINUI_CONTROLS_WINMD")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join(CONTROLS_WINMD));
    require_file(
        &controls_winmd,
        "XamlToolkit.WinUI.Controls metadata is missing. Build XamlToolkit.WinUI.Controls or copy XamlToolkit.WinUI.Controls.winmd to xamltoolkit-rs/crates/xamltoolkit-winui-controls/metadata/.",
    );

    let deps_dir = env::var_os("XAMLTOOLKIT_WINUI_CONTROLS_METADATA_DEPS")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join(DEFAULT_DEPS_DIR));
    let deps = collect_winmd_files(&deps_dir);

    let filters = env::var("XAMLTOOLKIT_WINUI_CONTROLS_FILTERS")
        .map(|value| split_filters(&value))
        .unwrap_or_else(|_| {
            vec![
                "Microsoft.UI.Xaml.DependencyObject".to_string(),
                "Microsoft.UI.Xaml.DependencyObjectCollection".to_string(),
                "Microsoft.UI.Xaml.DependencyProperty".to_string(),
                "Microsoft.UI.Xaml.FrameworkTemplate".to_string(),
                "Microsoft.UI.Xaml.Application".to_string(),
                "Microsoft.UI.Xaml.CornerRadius".to_string(),
                "Microsoft.UI.Xaml.Style".to_string(),
                "Microsoft.UI.Xaml.Thickness".to_string(),
                "Microsoft.UI.Xaml.UIElement".to_string(),
                "Microsoft.UI.Xaml.FrameworkElement".to_string(),
                "Microsoft.UI.Xaml.HorizontalAlignment".to_string(),
                "Microsoft.UI.Xaml.Markup.IXamlMetadataProvider".to_string(),
                "Microsoft.UI.Xaml.Markup.IXamlType".to_string(),
                "Microsoft.UI.Xaml.Markup.XmlnsDefinition".to_string(),
                "Microsoft.UI.Input.InputSystemCursorShape".to_string(),
                "Microsoft.UI.Input.PointerPoint".to_string(),
                "Microsoft.UI.Text.ITextCharacterFormat".to_string(),
                "Microsoft.UI.Text.ITextRange".to_string(),
                "Microsoft.UI.Text.RichEditTextDocument".to_string(),
                "Microsoft.UI.Xaml.Automation.Peers.AutomationPeer".to_string(),
                "Microsoft.UI.Xaml.Automation.Peers.ButtonBaseAutomationPeer".to_string(),
                "Microsoft.UI.Xaml.Automation.Peers.FrameworkElementAutomationPeer".to_string(),
                "Microsoft.UI.Xaml.Automation.Peers.ItemsControlAutomationPeer".to_string(),
                "Microsoft.UI.Xaml.Automation.Peers.ListViewBaseAutomationPeer".to_string(),
                "Microsoft.UI.Xaml.Automation.Peers.RangeBaseAutomationPeer".to_string(),
                "Microsoft.UI.Xaml.Automation.Peers.SelectorAutomationPeer".to_string(),
                "Microsoft.UI.Xaml.Automation.Provider.IRangeValueProvider".to_string(),
                "Microsoft.UI.Xaml.Automation.Provider.IValueProvider".to_string(),
                "Microsoft.UI.Xaml.DataTemplate".to_string(),
                "Microsoft.UI.Xaml.Data.IValueConverter".to_string(),
                "Microsoft.UI.Xaml.ResourceDictionary".to_string(),
                "Microsoft.UI.Xaml.Controls.AppBar".to_string(),
                "Microsoft.UI.Xaml.Controls.AutoSuggestBox".to_string(),
                "Microsoft.UI.Xaml.Controls.AutoSuggestBoxQuerySubmittedEventArgs".to_string(),
                "Microsoft.UI.Xaml.Controls.AutoSuggestBoxSuggestionChosenEventArgs".to_string(),
                "Microsoft.UI.Xaml.Controls.AutoSuggestBoxTextChangedEventArgs".to_string(),
                "Microsoft.UI.Xaml.Controls.Border".to_string(),
                "Microsoft.UI.Xaml.Controls.CommandBar".to_string(),
                "Microsoft.UI.Xaml.Controls.ContentPresenter".to_string(),
                "Microsoft.UI.Xaml.Controls.ContentControl".to_string(),
                "Microsoft.UI.Xaml.Controls.Control".to_string(),
                "Microsoft.UI.Xaml.Controls.Button".to_string(),
                "Microsoft.UI.Xaml.Controls.ColorPicker".to_string(),
                "Microsoft.UI.Xaml.Controls.DataTemplateSelector".to_string(),
                "Microsoft.UI.Xaml.Controls.DropDownButton".to_string(),
                "Microsoft.UI.Xaml.Controls.Grid".to_string(),
                "Microsoft.UI.Xaml.Controls.IconElement".to_string(),
                "Microsoft.UI.Xaml.Controls.IconSource".to_string(),
                "Microsoft.UI.Xaml.Controls.ItemsControl".to_string(),
                "Microsoft.UI.Xaml.Controls.Layout".to_string(),
                "Microsoft.UI.Xaml.Controls.ListViewBase".to_string(),
                "Microsoft.UI.Xaml.Controls.ListViewItem".to_string(),
                "Microsoft.UI.Xaml.Controls.NavigationView".to_string(),
                "Microsoft.UI.Xaml.Controls.Orientation".to_string(),
                "Microsoft.UI.Xaml.Controls.Panel".to_string(),
                "Microsoft.UI.Xaml.Controls.DisabledFormattingAccelerators".to_string(),
                "Microsoft.UI.Xaml.Controls.RichEditClipboardFormat".to_string(),
                "Microsoft.UI.Xaml.Controls.StyleSelector".to_string(),
                "Microsoft.UI.Xaml.Controls.TextControlPasteEventArgs".to_string(),
                "Microsoft.UI.Xaml.Controls.Primitives.ButtonBase".to_string(),
                "Microsoft.UI.Xaml.Controls.Primitives.RangeBase".to_string(),
                "Microsoft.UI.Xaml.Controls.Primitives.Selector".to_string(),
                "Microsoft.UI.Xaml.Controls.Primitives.SelectorItem".to_string(),
                "Microsoft.UI.Xaml.Controls.Slider".to_string(),
                "Microsoft.UI.Xaml.Controls.TreeView".to_string(),
                "Microsoft.UI.Xaml.Controls.UIElementCollection".to_string(),
                "Microsoft.UI.Xaml.Controls.VirtualizingLayout".to_string(),
                "Microsoft.UI.Xaml.Media.Brush".to_string(),
                "Microsoft.UI.Xaml.Media.ImageSource".to_string(),
                "Microsoft.UI.Xaml.Media.SolidColorBrush".to_string(),
                "Microsoft.UI.Xaml.Media.Imaging.BitmapSource".to_string(),
                "Microsoft.UI.Xaml.Media.Imaging.WriteableBitmap".to_string(),
                "Microsoft.UI.Xaml.Data.INotifyPropertyChanged".to_string(),
                "Windows.Foundation.Deferral".to_string(),
                "Windows.Foundation.IAsyncAction".to_string(),
                "Windows.Foundation.IReference".to_string(),
                "Windows.Foundation.Uri".to_string(),
                "Windows.Foundation.EventHandler".to_string(),
                "Windows.Foundation.Rect".to_string(),
                "Windows.Foundation.IStringable".to_string(),
                "Windows.Foundation.Collections.IObservableVector".to_string(),
                "Windows.Foundation.Collections.IVector".to_string(),
                "Windows.Storage.StorageFile".to_string(),
                "Windows.Storage.Streams.IRandomAccessStream".to_string(),
                "Windows.UI.Color".to_string(),
                "Windows.UI.Xaml.Interop.TypeKind".to_string(),
                "Windows.UI.Xaml.Interop.TypeName".to_string(),
                "XamlToolkit.WinUI.HsvColor".to_string(),
                "XamlToolkit.WinUI.Controls.BitmapFileFormat".to_string(),
                "XamlToolkit.WinUI.Controls.CameraPreview".to_string(),
                "XamlToolkit.WinUI.Controls.AccentColorConverter".to_string(),
                "XamlToolkit.WinUI.Controls.AspectRatio".to_string(),
                "XamlToolkit.WinUI.Controls.Case".to_string(),
                "XamlToolkit.WinUI.Controls.CaseCollection".to_string(),
                "XamlToolkit.WinUI.Controls.ColorChannel".to_string(),
                "XamlToolkit.WinUI.Controls.IColorPalette".to_string(),
                "XamlToolkit.WinUI.Controls.ColorPicker".to_string(),
                "XamlToolkit.WinUI.Controls.ColorPickerButton".to_string(),
                "XamlToolkit.WinUI.Controls.ColorRepresentation".to_string(),
                "XamlToolkit.WinUI.Controls.ColorToHexConverter".to_string(),
                "XamlToolkit.WinUI.Controls.CropShape".to_string(),
                "XamlToolkit.WinUI.Controls.Primitives.ColorPickerSlider".to_string(),
                "XamlToolkit.WinUI.Controls.Primitives.ColorPreviewer".to_string(),
                "XamlToolkit.WinUI.Controls.ConstrainedBox".to_string(),
                "XamlToolkit.WinUI.Controls.ContentSizer".to_string(),
                "XamlToolkit.WinUI.Controls.ContrastBrushConverter".to_string(),
                "XamlToolkit.WinUI.Controls.Dock".to_string(),
                "XamlToolkit.WinUI.Controls.DockPanel".to_string(),
                "XamlToolkit.WinUI.Controls.EqualPanel".to_string(),
                "XamlToolkit.WinUI.Controls.GridResizeBehavior".to_string(),
                "XamlToolkit.WinUI.Controls.GridResizeDirection".to_string(),
                "XamlToolkit.WinUI.Controls.GridSplitter".to_string(),
                "XamlToolkit.WinUI.Controls.HeaderedContentControl".to_string(),
                "XamlToolkit.WinUI.Controls.HeaderedItemsControl".to_string(),
                "XamlToolkit.WinUI.Controls.HeaderedTreeView".to_string(),
                "XamlToolkit.WinUI.Controls.ImageCropper".to_string(),
                "XamlToolkit.WinUI.Controls.ImageCropperThumb".to_string(),
                "XamlToolkit.WinUI.Controls.LayoutTransformControl".to_string(),
                "XamlToolkit.WinUI.Controls.MetadataControl".to_string(),
                "XamlToolkit.WinUI.Controls.MetadataItem".to_string(),
                "XamlToolkit.WinUI.Controls.NullToTransparentConverter".to_string(),
                "XamlToolkit.WinUI.Controls.PropertySizer".to_string(),
                "XamlToolkit.WinUI.Controls.PreviewFailedEventArgs".to_string(),
                "XamlToolkit.WinUI.Controls.RadialGauge".to_string(),
                "XamlToolkit.WinUI.Controls.RadialGaugeAutomationPeer".to_string(),
                "XamlToolkit.WinUI.Controls.RangeChangedEventArgs".to_string(),
                "XamlToolkit.WinUI.Controls.RangeSelector".to_string(),
                "XamlToolkit.WinUI.Controls.RangeSelectorProperty".to_string(),
                "XamlToolkit.WinUI.Controls.RichSuggestBox".to_string(),
                "XamlToolkit.WinUI.Controls.RichSuggestToken".to_string(),
                "XamlToolkit.WinUI.Controls.RichSuggestTokenPointerOverEventArgs".to_string(),
                "XamlToolkit.WinUI.Controls.RichSuggestTokenSelectedEventArgs".to_string(),
                "XamlToolkit.WinUI.Controls.Segmented".to_string(),
                "XamlToolkit.WinUI.Controls.SegmentedItem".to_string(),
                "XamlToolkit.WinUI.Controls.SegmentedMarginConverter".to_string(),
                "XamlToolkit.WinUI.Controls.ContentAlignment".to_string(),
                "XamlToolkit.WinUI.Controls.CornerRadiusConverter".to_string(),
                "XamlToolkit.WinUI.Controls.SettingsCard".to_string(),
                "XamlToolkit.WinUI.Controls.SettingsCardAutomationPeer".to_string(),
                "XamlToolkit.WinUI.Controls.SettingsExpander".to_string(),
                "XamlToolkit.WinUI.Controls.SettingsExpanderAutomationPeer".to_string(),
                "XamlToolkit.WinUI.Controls.SettingsExpanderItemStyleSelector".to_string(),
                "XamlToolkit.WinUI.Controls.SizerAutomationPeer".to_string(),
                "XamlToolkit.WinUI.Controls.SizerBase".to_string(),
                "XamlToolkit.WinUI.Controls.StaggeredLayout".to_string(),
                "XamlToolkit.WinUI.Controls.StaggeredLayoutItemsStretch".to_string(),
                "XamlToolkit.WinUI.Controls.StaggeredPanel".to_string(),
                "XamlToolkit.WinUI.Controls.StretchChild".to_string(),
                "XamlToolkit.WinUI.Controls.StyleExtensionResourceDictionary".to_string(),
                "XamlToolkit.WinUI.Controls.StyleExtensions".to_string(),
                "XamlToolkit.WinUI.Controls.SuggestionChosenEventArgs".to_string(),
                "XamlToolkit.WinUI.Controls.SuggestionPopupPlacementMode".to_string(),
                "XamlToolkit.WinUI.Controls.SuggestionRequestedEventArgs".to_string(),
                "XamlToolkit.WinUI.Controls.SwitchConverter".to_string(),
                "XamlToolkit.WinUI.Controls.SwitchPresenter".to_string(),
                "XamlToolkit.WinUI.Controls.TabbedCommandBar".to_string(),
                "XamlToolkit.WinUI.Controls.TabbedCommandBarItem".to_string(),
                "XamlToolkit.WinUI.Controls.TabbedCommandBarItemTemplateSelector".to_string(),
                "XamlToolkit.WinUI.Controls.ThumbPlacement".to_string(),
                "XamlToolkit.WinUI.Controls.ThumbPosition".to_string(),
                "XamlToolkit.WinUI.Controls.InterspersedObservableVector".to_string(),
                "XamlToolkit.WinUI.Controls.ITokenStringContainer".to_string(),
                "XamlToolkit.WinUI.Controls.PretokenStringContainer".to_string(),
                "XamlToolkit.WinUI.Controls.TokenItemAddingEventArgs".to_string(),
                "XamlToolkit.WinUI.Controls.TokenItemRemovingEventArgs".to_string(),
                "XamlToolkit.WinUI.Controls.TokenizingTextBox".to_string(),
                "XamlToolkit.WinUI.Controls.TokenizingTextBoxAutomationPeer".to_string(),
                "XamlToolkit.WinUI.Controls.TokenizingTextBoxItem".to_string(),
                "XamlToolkit.WinUI.Controls.TokenizingTextBoxStyleSelector".to_string(),
                "XamlToolkit.WinUI.Controls.UniformGrid".to_string(),
                "XamlToolkit.WinUI.Controls.WrapPanel".to_string(),
                "XamlToolkit.WinUI.Controls.XamlMetaDataProvider".to_string(),
            ]
        });

    if filters.is_empty() {
        panic!("XAMLTOOLKIT_WINUI_CONTROLS_FILTERS did not contain any filters.");
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_file = out_dir.join("bindings.rs");

    let mut args = vec![
        "--in".to_string(),
        "default".to_string(),
        controls_winmd.display().to_string(),
    ];
    args.extend(deps.iter().map(|path| path.display().to_string()));
    args.extend([
        "--out".to_string(),
        out_file.display().to_string(),
        "--reference".to_string(),
        "windows,skip-root,Windows.Graphics".to_string(),
        "--filter".to_string(),
    ]);
    args.extend(filters);

    let warnings = windows_bindgen::bindgen(args);
    if !warnings.is_empty() {
        println!("cargo:warning=xamltoolkit-winui-controls generated with skipped inherited WinUI members for the current minimal control projection:\n{warnings}");
    }

    if !out_file.exists() {
        panic!(
            "windows-bindgen completed but did not create {}",
            out_file.display()
        );
    }

    patch_generated_bindings(&out_file);
}

fn patch_generated_bindings(out_file: &Path) {
    let mut generated = fs::read_to_string(out_file)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", out_file.display()));

    if generated.contains("pub struct IReference<T>")
        && !generated.contains("xamltoolkit_controls_ireference_from_bridge")
    {
        generated.push_str(
            r#"

#[allow(non_snake_case)]
mod xamltoolkit_controls_ireference_from_bridge {
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
