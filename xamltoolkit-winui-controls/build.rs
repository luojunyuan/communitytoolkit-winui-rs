use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use windows_metadata::reader;

const CONTROLS_WINMD: &str = "metadata/XamlToolkit.WinUI.Controls.winmd";
const EXTRA_FILTERS: &str = include_str!("extra.filters");
const WASDK_EXTRA_FILTERS: &str = include_str!("../wasdk/extra.filters");
const TOOLKIT_EXTRA_FILTERS: &str = include_str!("../xamltoolkit-winui/extra.filters");
const DEFAULT_DEPS_DIR: &str = "metadata/deps";
const DEFAULT_WASDK_DEPS_DIR: &str = "../wasdk/metadata/deps";
const BINDGEN_WARNINGS_ENV: &str = "XAMLTOOLKIT_WINUI_CONTROLS_BINDGEN_WARNINGS";

fn main() {
    println!("cargo:rerun-if-changed={CONTROLS_WINMD}");
    println!("cargo:rerun-if-changed=extra.filters");
    println!("cargo:rerun-if-changed={DEFAULT_DEPS_DIR}");
    println!("cargo:rerun-if-changed={DEFAULT_WASDK_DEPS_DIR}");
    println!("cargo:rerun-if-env-changed=WASDK_METADATA_DEPS");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_CONTROLS_WINMD");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_CONTROLS_METADATA_DEPS");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_CONTROLS_FILTERS");
    println!("cargo:rerun-if-env-changed={BINDGEN_WARNINGS_ENV}");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let controls_winmd = env::var_os("XAMLTOOLKIT_WINUI_CONTROLS_WINMD")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join(CONTROLS_WINMD));
    require_file(
        &controls_winmd,
        "XamlToolkit.WinUI.Controls metadata is missing. Build XamlToolkit.WinUI.Controls or copy XamlToolkit.WinUI.Controls.winmd to xamltoolkit-winui-controls/metadata/.",
    );

    let deps = dependency_winmd_files(
        &manifest_dir,
        "XAMLTOOLKIT_WINUI_CONTROLS_METADATA_DEPS",
        DEFAULT_DEPS_DIR,
    );

    let filters_overridden = env::var_os("XAMLTOOLKIT_WINUI_CONTROLS_FILTERS").is_some();
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
                "Microsoft.UI.Xaml.RoutedEventArgs".to_string(),
                "Microsoft.UI.Xaml.RoutedEventHandler".to_string(),
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
                "Microsoft.UI.Xaml.Controls.ColorChangedEventArgs".to_string(),
                "Microsoft.UI.Xaml.Controls.ColorSpectrumComponents".to_string(),
                "Microsoft.UI.Xaml.Controls.ColorSpectrumShape".to_string(),
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
                "Microsoft.UI.Xaml.Controls.Primitives.DragCompletedEventArgs".to_string(),
                "Microsoft.UI.Xaml.Controls.Primitives.DragCompletedEventHandler".to_string(),
                "Microsoft.UI.Xaml.Controls.Primitives.DragStartedEventArgs".to_string(),
                "Microsoft.UI.Xaml.Controls.Primitives.DragStartedEventHandler".to_string(),
                "Microsoft.UI.Xaml.Controls.Primitives.RangeBase".to_string(),
                "Microsoft.UI.Xaml.Controls.Primitives.Selector".to_string(),
                "Microsoft.UI.Xaml.Controls.Primitives.SelectorItem".to_string(),
                "Microsoft.UI.Xaml.Controls.Slider".to_string(),
                "Microsoft.UI.Xaml.Controls.TreeView".to_string(),
                "Microsoft.UI.Xaml.Controls.UIElementCollection".to_string(),
                "Microsoft.UI.Xaml.Controls.VirtualizingLayout".to_string(),
                "Microsoft.UI.Xaml.Input.ICommand".to_string(),
                "Microsoft.UI.Xaml.Media.Brush".to_string(),
                "Microsoft.UI.Xaml.Media.GeneralTransform".to_string(),
                "Microsoft.UI.Xaml.Media.ImageSource".to_string(),
                "Microsoft.UI.Xaml.Media.SolidColorBrush".to_string(),
                "Microsoft.UI.Xaml.Media.Transform".to_string(),
                "Microsoft.UI.Xaml.Media.Imaging.BitmapSource".to_string(),
                "Microsoft.UI.Xaml.Media.Imaging.WriteableBitmap".to_string(),
                "Microsoft.UI.Xaml.Data.INotifyPropertyChanged".to_string(),
                "Windows.UI.Xaml.Interop.TypeKind".to_string(),
                "Windows.UI.Xaml.Interop.TypeName".to_string(),
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
    let mut filters = without_wasdk_filters(filters);
    filters.extend(
        EXTRA_FILTERS
            .lines()
            .map(str::trim)
            .filter(|filter| !filter.is_empty() && !filter.starts_with('#'))
            .map(str::to_string),
    );
    filters.extend(
        WASDK_EXTRA_FILTERS
            .lines()
            .map(str::trim)
            .filter(|filter| !filter.is_empty() && !filter.starts_with('#'))
            .map(str::to_string),
    );
    filters.extend(
        TOOLKIT_EXTRA_FILTERS
            .lines()
            .map(str::trim)
            .filter(|filter| !filter.is_empty() && !filter.starts_with('#'))
            .map(str::to_string),
    );
    filters.sort();
    filters.dedup();

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
    patch_generated_bindings(&out_file);

    if !filters_overridden {
        assert_controls_surface_generated(&controls_winmd, &out_file);
    }
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

fn assert_controls_surface_generated(winmd: &Path, out_file: &Path) {
    let index = reader::Index::read(winmd)
        .unwrap_or_else(|| panic!("failed to read WinMD metadata from {}", winmd.display()));
    let generated = fs::read_to_string(out_file)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", out_file.display()));

    let mut missing = Vec::new();
    for ty in index.types() {
        if ty.name() == "<Module>" || !ty.namespace().starts_with("XamlToolkit.WinUI.Controls") {
            continue;
        }

        let full_name = format!("{}.{}", ty.namespace(), ty.name());
        if !generated.contains(&full_name) {
            missing.push(full_name);
        }
    }

    missing.sort();
    missing.dedup();
    if !missing.is_empty() {
        panic!(
            "toolkit-winui-controls generated bindings are missing WinMD Toolkit types:\n{}",
            missing.join("\n")
        );
    }
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
