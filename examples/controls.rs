use std::any::Any;
use std::env;
use std::sync::Once;

use windows::core::{IInspectable, Interface, GUID, HSTRING};
use windows::Foundation::PropertyValue;
use windows_collections::IVector;
use windows_reactor::core::backend::{Backend, ControlId, ControlKind};
use windows_reactor::core::custom::{CustomElement, CustomElementHandle};
use windows_reactor::*;
use xamltoolkit_winui_controls::Microsoft::UI::Xaml::Controls::{
    Border as NativeBorder, Button as NativeButton, Orientation,
};
use xamltoolkit_winui_controls::Microsoft::UI::Xaml::{
    Application as NativeApplication, DataTemplate, HorizontalAlignment, ResourceDictionary, Style,
    Thickness as XamlThickness, UIElement,
};
use xamltoolkit_winui_controls::Windows::Foundation::{Rect, Uri};
use xamltoolkit_winui_controls::XamlToolkit::WinUI::Controls::Primitives::{
    ColorPickerSlider, ColorPreviewer,
};
use xamltoolkit_winui_controls::XamlToolkit::WinUI::Controls::{
    AccentColorConverter, CameraPreview, Case, CaseCollection, ColorChannel, ColorPicker,
    ColorPickerButton, ColorRepresentation, ColorToHexConverter, ConstrainedBox, ContentAlignment,
    ContentSizer, ContrastBrushConverter, CornerRadiusConverter, Dock, DockPanel, EqualPanel,
    GridResizeBehavior, GridResizeDirection, GridSplitter, HeaderedContentControl,
    HeaderedItemsControl, HeaderedTreeView, ImageCropper, ImageCropperThumb,
    LayoutTransformControl, MetadataControl, MetadataItem, NullToTransparentConverter,
    PreviewFailedEventArgs, PropertySizer, RadialGauge, RadialGaugeAutomationPeer, RangeSelector,
    RichSuggestBox, RichSuggestToken, Segmented, SegmentedItem, SegmentedMarginConverter,
    SettingsCard, SettingsCardAutomationPeer, SettingsExpander, SettingsExpanderAutomationPeer,
    SettingsExpanderItemStyleSelector, SizerAutomationPeer, StaggeredLayout,
    StaggeredLayoutItemsStretch, StaggeredPanel, StretchChild, StyleExtensionResourceDictionary,
    StyleExtensions, SuggestionChosenEventArgs, SuggestionRequestedEventArgs, SwitchConverter,
    SwitchPresenter, TabbedCommandBar, TabbedCommandBarItem, TabbedCommandBarItemTemplateSelector,
    ThumbPlacement, ThumbPosition, TokenItemAddingEventArgs, TokenItemRemovingEventArgs,
    TokenizingTextBox, TokenizingTextBoxAutomationPeer, TokenizingTextBoxItem,
    TokenizingTextBoxStyleSelector, UniformGrid, WrapPanel, XamlMetaDataProvider,
};
use xamltoolkit_winui_controls::XamlToolkit::WinUI::HsvColor;

fn main() {
    eprintln!("controls-example: starting");
    windows_reactor::register_xaml_metadata_provider_factory(|| {
        let provider = XamlMetaDataProvider::new()?;
        provider.cast::<IInspectable>()
    });
    if let Err(error) = App::new().title("XamlToolkit Controls").render(app) {
        eprintln!("controls-example failed: {error:?}");
        std::process::exit(1);
    }
}

fn app(_cx: &mut RenderCx) -> Element {
    eprintln!("controls-example: rendering app");
    install_toolkit_resources_once();

    let controls_status = verify_layout_controls();

    scroll_viewer(
        vstack((
            text_block("XamlToolkit Controls").font_size(22.0).bold(),
            text_block("Rust Controls example consuming the generated XamlToolkit.WinUI.Controls projection crate."),
            visual_samples(),
            text_block(controls_status),
        ))
        .spacing(12.0)
        .padding(24.0),
    )
    .into()
}

static INSTALL_TOOLKIT_RESOURCES: Once = Once::new();

fn install_toolkit_resources_once() {
    INSTALL_TOOLKIT_RESOURCES.call_once(|| match install_toolkit_resources() {
        Ok(()) => eprintln!("controls-example: installed selected Toolkit resources"),
        Err(error) => eprintln!("controls-example: Toolkit resource install failed: {error:?}"),
    });
}

fn install_toolkit_resources() -> windows::core::Result<()> {
    let selected = selected_visual_samples();
    let app = NativeApplication::Current()?;
    let resources = app.Resources()?;
    let merged = resources.MergedDictionaries()?;

    for source in toolkit_resource_dictionaries_for(&selected) {
        eprintln!("controls-example: loading Toolkit resource {source}");
        let dictionary = ResourceDictionary::new()?;
        let uri = Uri::CreateUri(&HSTRING::from(source))?;
        dictionary.SetSource(&uri)?;
        merged.Append(&dictionary)?;
    }

    Ok(())
}

fn toolkit_resource_dictionaries_for(samples: &[VisualSample]) -> Vec<&'static str> {
    let mut sources = Vec::new();
    for sample in samples {
        match sample.name {
            "SettingsCard" => {
                sources.push(
                    "ms-appx:///XamlToolkit.WinUI.Controls/SettingsControls/SettingsCard/SettingsCard.xaml",
                );
            }
            "SettingsExpander" => {
                sources.push(
                    "ms-appx:///XamlToolkit.WinUI.Controls/SettingsControls/SettingsExpander/SettingsExpander.xaml",
                );
            }
            "HeaderedContentControl" => {
                sources.push(
                    "ms-appx:///XamlToolkit.WinUI.Controls/HeaderedControls/HeaderedContentControl/HeaderedContentControl.xaml",
                );
            }
            "HeaderedItemsControl" => {
                sources.push(
                    "ms-appx:///XamlToolkit.WinUI.Controls/HeaderedControls/HeaderedItemsControl/HeaderedItemsControl.xaml",
                );
            }
            "HeaderedTreeView" => {
                sources.push(
                    "ms-appx:///XamlToolkit.WinUI.Controls/HeaderedControls/HeaderedTreeView/HeaderedTreeView.xaml",
                );
            }
            "MetadataControl" => {
                sources.push(
                    "ms-appx:///XamlToolkit.WinUI.Controls/MetadataControl/MetadataControl.xaml",
                );
            }
            "LayoutTransformControl" => {
                sources.push(
                    "ms-appx:///XamlToolkit.WinUI.Controls/LayoutTransformControl/LayoutTransformControl.xaml",
                );
            }
            "ImageCropper" => {
                sources
                    .push("ms-appx:///XamlToolkit.WinUI.Controls/ImageCropper/ImageCropper.xaml");
                sources.push(
                    "ms-appx:///XamlToolkit.WinUI.Controls/ImageCropper/ImageCropperThumb.xaml",
                );
            }
            "CameraPreview" => {
                sources
                    .push("ms-appx:///XamlToolkit.WinUI.Controls/CameraPreview/CameraPreview.xaml");
            }
            "TabbedCommandBar" => {
                sources.push(
                    "ms-appx:///XamlToolkit.WinUI.Controls/TabbedCommandBar/TabbedCommandBar.xaml",
                );
            }
            "ColorPreviewer" => {
                sources
                    .push("ms-appx:///XamlToolkit.WinUI.Controls/ColorPicker/ColorPreviewer.xaml");
            }
            "ColorPickerSlider" => {
                sources.push(
                    "ms-appx:///XamlToolkit.WinUI.Controls/ColorPicker/ColorPickerSlider.xaml",
                );
            }
            "Sizers" => {
                sources.push("ms-appx:///XamlToolkit.WinUI.Controls/Sizers/SizerBase.xaml");
            }
            "RadialGauge" => {
                sources.push("ms-appx:///XamlToolkit.WinUI.Controls/RadialGauge/RadialGauge.xaml");
            }
            "RangeSelector" => {
                sources
                    .push("ms-appx:///XamlToolkit.WinUI.Controls/RangeSelector/RangeSelector.xaml");
            }
            "Segmented" => {
                sources.push("ms-appx:///XamlToolkit.WinUI.Controls/Segmented/Segmented.xaml");
                sources.push("ms-appx:///XamlToolkit.WinUI.Controls/Segmented/SegmentedItem.xaml");
            }
            _ => {}
        }
    }
    sources.sort_unstable();
    sources.dedup();
    sources
}
fn visual_samples() -> Element {
    let samples = selected_visual_samples();
    let names = samples.iter().map(|sample| sample.name).collect::<Vec<_>>();
    eprintln!(
        "controls-example: visual samples selected: {}",
        names.join(",")
    );

    let mut cards = Vec::new();
    for (index, sample) in samples.iter().enumerate() {
        cards.push(
            sample_card(
                sample.name,
                toolkit_control_host(sample.name, sample.create),
            )
            .grid_row((index / 2) as i32)
            .grid_column((index % 2) as i32),
        );
    }

    let rows = ((cards.len().max(1) + 1) / 2).max(1);

    vstack((
        text_block("Mounted Controls samples")
            .font_size(18.0)
            .bold(),
        grid(cards)
            .rows((0..rows).map(|_| GridLength::Auto))
            .columns([GridLength::Star(1.0), GridLength::Star(1.0)])
            .row_spacing(12.0)
            .column_spacing(12.0),
    ))
    .spacing(8.0)
    .into()
}

#[derive(Clone, Copy)]
struct VisualSample {
    name: &'static str,
    create:
        fn() -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement>,
}

fn selected_visual_samples() -> Vec<VisualSample> {
    let requested =
        env::var("XAMLTOOLKIT_CONTROLS_VISUAL_SAMPLES").unwrap_or_else(|_| "WrapPanel".to_string());
    let requested = requested
        .split(',')
        .map(|name| name.trim())
        .filter(|name| !name.is_empty())
        .collect::<Vec<_>>();
    let include_all = requested
        .iter()
        .any(|name| name.eq_ignore_ascii_case("all"));

    let samples = all_visual_samples();
    let selected = samples
        .iter()
        .copied()
        .filter(|sample| {
            include_all
                || requested
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case(sample.name))
        })
        .collect::<Vec<_>>();

    if selected.is_empty() {
        eprintln!(
            "controls-example: no visual samples matched XAMLTOOLKIT_CONTROLS_VISUAL_SAMPLES={requested:?}; defaulting to WrapPanel"
        );
        samples
            .iter()
            .copied()
            .filter(|sample| sample.name == "WrapPanel")
            .collect()
    } else {
        selected
    }
}

fn all_visual_samples() -> [VisualSample; 23] {
    [
        VisualSample {
            name: "WrapPanel",
            create: create_wrap_panel_sample,
        },
        VisualSample {
            name: "DockPanel",
            create: create_dock_panel_sample,
        },
        VisualSample {
            name: "UniformGrid",
            create: create_uniform_grid_sample,
        },
        VisualSample {
            name: "EqualPanel",
            create: create_equal_panel_sample,
        },
        VisualSample {
            name: "StaggeredPanel",
            create: create_staggered_panel_sample,
        },
        VisualSample {
            name: "ConstrainedBox",
            create: create_constrained_box_sample,
        },
        VisualSample {
            name: "LayoutTransformControl",
            create: create_layout_transform_control_sample,
        },
        VisualSample {
            name: "ImageCropper",
            create: create_image_cropper_sample,
        },
        VisualSample {
            name: "CameraPreview",
            create: create_camera_preview_sample,
        },
        VisualSample {
            name: "TabbedCommandBar",
            create: create_tabbed_command_bar_sample,
        },
        VisualSample {
            name: "SwitchPresenter",
            create: create_switch_presenter_sample,
        },
        VisualSample {
            name: "MetadataControl",
            create: create_metadata_control_sample,
        },
        VisualSample {
            name: "ColorPreviewer",
            create: create_color_previewer_sample,
        },
        VisualSample {
            name: "ColorPickerSlider",
            create: create_color_picker_slider_sample,
        },
        VisualSample {
            name: "Sizers",
            create: create_sizers_sample,
        },
        VisualSample {
            name: "RangeSelector",
            create: create_range_selector_sample,
        },
        VisualSample {
            name: "Segmented",
            create: create_segmented_sample,
        },
        VisualSample {
            name: "RadialGauge",
            create: create_radial_gauge_sample,
        },
        VisualSample {
            name: "SettingsCard",
            create: create_settings_card_sample,
        },
        VisualSample {
            name: "SettingsExpander",
            create: create_settings_expander_sample,
        },
        VisualSample {
            name: "HeaderedContentControl",
            create: create_headered_content_control_sample,
        },
        VisualSample {
            name: "HeaderedItemsControl",
            create: create_headered_items_control_sample,
        },
        VisualSample {
            name: "HeaderedTreeView",
            create: create_headered_tree_view_sample,
        },
    ]
}

fn create_wrap_panel_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let panel = WrapPanel::new()?;
    panel.SetWidth(320.0)?;
    panel.SetHeight(96.0)?;
    panel.SetHorizontalSpacing(8.0)?;
    panel.SetVerticalSpacing(8.0)?;
    let children = panel.Children()?;
    for label in ["Alpha", "Beta", "Gamma", "Delta"] {
        children.Append(&sample_button(label, 74.0, 32.0)?)?;
    }
    Ok(panel.cast()?)
}

fn create_dock_panel_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let panel = DockPanel::new()?;
    panel.SetWidth(320.0)?;
    panel.SetHeight(132.0)?;
    panel.SetLastChildFill(true)?;
    panel.SetPadding(thickness(4.0, 4.0, 4.0, 4.0))?;

    let top = sample_button("Top", 300.0, 30.0)?;
    DockPanel::SetDock(&top, Dock::Top)?;
    panel.Children()?.Append(&top)?;

    let left = sample_button("Left", 84.0, 70.0)?;
    DockPanel::SetDock(&left, Dock::Left)?;
    panel.Children()?.Append(&left)?;

    panel
        .Children()?
        .Append(&sample_button("Fill", 180.0, 70.0)?)?;
    Ok(panel.cast()?)
}

fn create_uniform_grid_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let grid = UniformGrid::new()?;
    grid.SetWidth(320.0)?;
    grid.SetHeight(132.0)?;
    grid.SetRows(2)?;
    grid.SetColumns(3)?;
    for label in ["One", "Two", "Three", "Four", "Five", "Six"] {
        grid.Children()?
            .Append(&sample_button(label, 96.0, 44.0)?)?;
    }
    Ok(grid.cast()?)
}

fn create_equal_panel_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let panel = EqualPanel::new()?;
    panel.SetWidth(320.0)?;
    panel.SetHeight(96.0)?;
    panel.SetOrientation(Orientation::Horizontal)?;
    panel.SetSpacing(8.0)?;
    for label in ["One", "Two", "Three"] {
        panel
            .Children()?
            .Append(&sample_button(label, 88.0, 40.0)?)?;
    }
    Ok(panel.cast()?)
}

fn create_staggered_panel_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let panel = StaggeredPanel::new()?;
    panel.SetWidth(320.0)?;
    panel.SetHeight(140.0)?;
    panel.SetDesiredColumnWidth(92.0)?;
    panel.SetColumnSpacing(8.0)?;
    panel.SetRowSpacing(8.0)?;
    panel.SetPadding(thickness(4.0, 4.0, 4.0, 4.0))?;
    for (label, height) in [
        ("Short", 32.0),
        ("Medium", 54.0),
        ("Tall", 78.0),
        ("Wide", 44.0),
        ("Last", 60.0),
    ] {
        panel
            .Children()?
            .Append(&sample_button(label, 88.0, height)?)?;
    }
    Ok(panel.cast()?)
}

fn create_constrained_box_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let box_control = ConstrainedBox::new()?;
    box_control.SetWidth(320.0)?;
    box_control.SetHeight(80.0)?;
    box_control.SetScaleX(1.0)?;
    box_control.SetScaleY(1.0)?;
    box_control.SetMultipleX(2)?;
    box_control.SetMultipleY(1)?;
    box_control.SetContent(&sample_button("Constrained", 160.0, 44.0)?.cast::<IInspectable>()?)?;
    Ok(box_control.cast()?)
}

fn create_layout_transform_control_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let control = LayoutTransformControl::new()?;
    control.SetWidth(320.0)?;
    control.SetHeight(72.0)?;
    let child = sample_button("Layout child", 180.0, 44.0)?;
    control.SetChild(&child)?;
    Ok(control.cast()?)
}

fn create_image_cropper_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let cropper = ImageCropper::new()?;
    cropper.SetWidth(320.0)?;
    cropper.SetHeight(180.0)?;
    cropper.SetMinCroppedPixelLength(32.0)?;
    cropper.SetMinSelectedLength(24.0)?;
    cropper.SetCropShape(
        xamltoolkit_winui_controls::XamlToolkit::WinUI::Controls::CropShape::Rectangular,
    )?;
    cropper.SetThumbPlacement(ThumbPlacement::Corners)?;
    Ok(cropper.cast()?)
}

fn create_camera_preview_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let preview = CameraPreview::new()?;
    preview.SetWidth(320.0)?;
    preview.SetHeight(180.0)?;
    preview.SetIsFrameSourceGroupButtonVisible(true)?;
    Ok(preview.cast()?)
}

fn create_tabbed_command_bar_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let command_bar = TabbedCommandBar::new()?;
    command_bar.SetWidth(320.0)?;
    command_bar.SetHeight(96.0)?;
    Ok(command_bar.cast()?)
}

fn create_switch_presenter_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let presenter = SwitchPresenter::new()?;
    presenter.SetWidth(320.0)?;
    presenter.SetHeight(64.0)?;

    let selected = Case::new()?;
    selected.SetValue(&boxed_string("rust")?)?;
    selected.SetContent(&boxed_string("Rust case selected")?)?;

    let fallback = Case::new()?;
    fallback.SetIsDefault(true)?;
    fallback.SetContent(&boxed_string("Default case")?)?;

    let cases = CaseCollection::new()?;
    cases.Append(&selected)?;
    cases.Append(&fallback)?;

    presenter.SetSwitchCases(&cases)?;
    let value_property = SwitchPresenter::ValueProperty()?;
    presenter.SetValue(&value_property, &boxed_string("rust")?)?;
    Ok(presenter.cast()?)
}

fn create_metadata_control_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let metadata = MetadataControl::new()?;
    metadata.SetWidth(320.0)?;
    metadata.SetSeparator(&HSTRING::from(" | "))?;
    metadata.SetAccessibleSeparator(&HSTRING::from(", "))?;
    let items = IVector::<MetadataItem>::from(
        ["Rust", "WinRT", "Controls"]
            .into_iter()
            .map(metadata_item)
            .map(|item| item.map(Some))
            .collect::<windows::core::Result<Vec<_>>>()?,
    );
    let items = items
        .cast::<
            xamltoolkit_winui_controls::Windows::Foundation::Collections::IVector<MetadataItem>,
        >()?;
    metadata.SetItems(&items)?;
    Ok(metadata.cast()?)
}

fn create_color_previewer_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let previewer = ColorPreviewer::new()?;
    previewer.SetWidth(320.0)?;
    previewer.SetHeight(44.0)?;
    previewer.SetHsvColor(HsvColor {
        H: 214.0,
        S: 0.74,
        V: 0.82,
        A: 1.0,
    })?;
    previewer.SetShowAccentColors(true)?;
    Ok(previewer.cast()?)
}

fn create_color_picker_slider_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let slider = ColorPickerSlider::new()?;
    slider.SetWidth(320.0)?;
    slider.SetHeight(44.0)?;
    slider.SetMinimum(0.0)?;
    slider.SetMaximum(360.0)?;
    slider.SetValue2(214.0)?;
    slider.SetColorChannel(ColorChannel::Channel1)?;
    slider.SetColorRepresentation(ColorRepresentation::Hsva)?;
    slider.SetHsvColor(HsvColor {
        H: 214.0,
        S: 0.74,
        V: 0.82,
        A: 1.0,
    })?;
    slider.SetIsAutoUpdatingEnabled(true)?;
    slider.UpdateColors()?;
    Ok(slider.cast()?)
}

fn create_sizers_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let panel = WrapPanel::new()?;
    panel.SetWidth(320.0)?;
    panel.SetHeight(96.0)?;
    panel.SetHorizontalSpacing(10.0)?;
    panel.SetVerticalSpacing(8.0)?;

    let property = PropertySizer::new()?;
    property.SetWidth(88.0)?;
    property.SetHeight(32.0)?;
    property.SetOrientation(Orientation::Horizontal)?;
    property.SetBinding(120.0)?;
    property.SetMinimum(40.0)?;
    property.SetMaximum(360.0)?;
    property.SetDragIncrement(4.0)?;
    let property: UIElement = property.cast()?;
    panel.Children()?.Append(&property)?;

    let content = ContentSizer::new()?;
    content.SetWidth(88.0)?;
    content.SetHeight(32.0)?;
    content.SetOrientation(Orientation::Horizontal)?;
    content.SetKeyboardIncrement(8.0)?;
    content.SetIsThumbVisible(true)?;
    let content: UIElement = content.cast()?;
    panel.Children()?.Append(&content)?;

    let splitter = GridSplitter::new()?;
    splitter.SetWidth(88.0)?;
    splitter.SetHeight(32.0)?;
    splitter.SetResizeDirection(GridResizeDirection::Columns)?;
    splitter.SetResizeBehavior(GridResizeBehavior::CurrentAndNext)?;
    splitter.SetOrientation(Orientation::Vertical)?;
    let splitter: UIElement = splitter.cast()?;
    panel.Children()?.Append(&splitter)?;

    Ok(panel.cast()?)
}

fn create_range_selector_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let selector = RangeSelector::new()?;
    selector.SetWidth(320.0)?;
    selector.SetHeight(64.0)?;
    selector.SetMinimum(0.0)?;
    selector.SetMaximum(100.0)?;
    selector.SetRangeStart(20.0)?;
    selector.SetRangeEnd(80.0)?;
    selector.SetStepFrequency(5.0)?;
    Ok(selector.cast()?)
}

fn create_segmented_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let segmented = Segmented::new()?;
    segmented.SetWidth(320.0)?;
    segmented.SetHeight(48.0)?;
    segmented.SetOrientation(Orientation::Horizontal)?;
    let items = IVector::<IInspectable>::from(
        ["Daily", "Weekly", "Monthly"]
            .into_iter()
            .map(boxed_string)
            .map(|value| value.map(Some))
            .collect::<windows::core::Result<Vec<_>>>()?,
    );
    segmented.SetItemsSource(&items.cast::<IInspectable>()?)?;
    segmented.SetSelectedIndex(1)?;
    Ok(segmented.cast()?)
}

fn create_radial_gauge_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let gauge = RadialGauge::new()?;
    gauge.SetWidth(180.0)?;
    gauge.SetHeight(180.0)?;
    gauge.SetMinimum(0.0)?;
    gauge.SetMaximum(100.0)?;
    gauge.SetUnit(&HSTRING::from("%"))?;
    gauge.SetIsInteractive(false)?;
    Ok(gauge.cast()?)
}

fn create_settings_card_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let card = SettingsCard::new()?;
    card.SetWidth(320.0)?;
    card.SetHeader(&boxed_string("Rust projection")?)?;
    card.SetDescription(&boxed_string("Hosted from xamltoolkit-winui-controls")?)?;
    card.SetIsClickEnabled(false)?;
    Ok(card.cast()?)
}

fn create_settings_expander_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let expander = SettingsExpander::new()?;
    expander.SetWidth(320.0)?;
    expander.SetHeader(&boxed_string("Display")?)?;
    expander.SetDescription(&boxed_string(
        "Grouped settings from the generated Rust projection",
    )?)?;
    expander.SetContent(&boxed_string("2 options")?)?;
    expander.SetIsExpanded(true)?;

    let accent = SettingsCard::new()?;
    accent.SetHeader(&boxed_string("Accent color")?)?;
    accent.SetDescription(&boxed_string("Use system color")?)?;
    accent.SetContent(&boxed_string("On")?)?;
    accent.SetIsClickEnabled(false)?;
    expander.Items()?.Append(&accent.cast::<IInspectable>()?)?;

    let density = SettingsCard::new()?;
    density.SetHeader(&boxed_string("Layout density")?)?;
    density.SetDescription(&boxed_string("Comfortable spacing")?)?;
    density.SetContent(&boxed_string("Default")?)?;
    density.SetIsClickEnabled(false)?;
    expander.Items()?.Append(&density.cast::<IInspectable>()?)?;

    Ok(expander.cast()?)
}

fn create_headered_content_control_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let control = HeaderedContentControl::new()?;
    control.SetWidth(320.0)?;
    control.SetHeader(&boxed_string("Headered content")?)?;
    control.SetContent(&boxed_string("Projection-hosted content body")?)?;
    control.SetOrientation(Orientation::Vertical)?;
    Ok(control.cast()?)
}

fn create_headered_items_control_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let control = HeaderedItemsControl::new()?;
    control.SetWidth(320.0)?;
    control.SetHeader(&boxed_string("Headered items")?)?;
    control.SetFooter(&boxed_string("3 projected items")?)?;
    let items = IVector::<IInspectable>::from(
        ["Alpha", "Beta", "Gamma"]
            .into_iter()
            .map(boxed_string)
            .map(|value| value.map(Some))
            .collect::<windows::core::Result<Vec<_>>>()?,
    );
    control.SetItemsSource(&items.cast::<IInspectable>()?)?;
    Ok(control.cast()?)
}

fn create_headered_tree_view_sample(
) -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement> {
    let control = HeaderedTreeView::new()?;
    control.SetWidth(320.0)?;
    control.SetHeight(96.0)?;
    control.SetHeader(&boxed_string("Headered tree")?)?;
    control.SetFooter(&boxed_string("TreeView shell")?)?;
    Ok(control.cast()?)
}

fn sample_button(label: &str, width: f64, height: f64) -> windows::core::Result<NativeButton> {
    let button = NativeButton::new()?;
    button.SetContent(&boxed_string(label)?)?;
    button.SetWidth(width)?;
    button.SetHeight(height)?;
    button.SetMargin(thickness(4.0, 4.0, 4.0, 4.0))?;
    Ok(button)
}

fn metadata_item(label: &str) -> windows::core::Result<MetadataItem> {
    let item = MetadataItem::new()?;
    let label = HSTRING::from(label);
    item.SetLabel(&label)?;
    item.SetAccessibleLabel(&label)?;
    Ok(item)
}

fn sample_card(title: &'static str, sample: Element) -> Element {
    border(vstack((text_block(title).bold(), sample)).spacing(8.0))
        .padding(windows_reactor::Thickness::uniform(12.0))
        .border_thickness(windows_reactor::Thickness::uniform(1.0))
        .corner_radius(6.0)
        .into()
}

fn toolkit_control_host(
    name: &'static str,
    create: fn()
        -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement>,
) -> Element {
    Element::Custom(CustomElementHandle::new(ToolkitControlHost {
        name,
        create,
    }))
}

#[derive(Clone)]
struct ToolkitControlHost {
    name: &'static str,
    create:
        fn() -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement>,
}

impl CustomElement for ToolkitControlHost {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind_name(&self) -> &'static str {
        "ToolkitControlHost"
    }

    fn eq_dyn(&self, other: &dyn CustomElement) -> bool {
        other
            .as_any()
            .downcast_ref::<ToolkitControlHost>()
            .is_some_and(|other| self.name == other.name)
    }

    fn clone_dyn(&self) -> Box<dyn CustomElement> {
        Box::new(self.clone())
    }

    fn mount(&self, backend: &mut dyn Backend) -> ControlId {
        let id = backend.create(ControlKind::Border);
        mount_toolkit_control(self.name, self.create, backend.get_native_element(id));
        id
    }

    fn update(&self, prev: &dyn CustomElement, id: ControlId, backend: &mut dyn Backend) {
        let prev = prev
            .as_any()
            .downcast_ref::<ToolkitControlHost>()
            .expect("reconciler guarantees matching custom element type");
        if prev.name != self.name {
            mount_toolkit_control(self.name, self.create, backend.get_native_element(id));
        }
    }

    fn before_destroy(&self, id: ControlId, backend: &mut dyn Backend) {
        if let Some(native) = backend.get_native_element(id) {
            if let Ok(border) = native.cast::<NativeBorder>() {
                let _ = border.SetChild(None);
            }
        }
    }
}

fn mount_toolkit_control(
    name: &'static str,
    create: fn()
        -> windows::core::Result<xamltoolkit_winui_controls::Microsoft::UI::Xaml::UIElement>,
    native_host: Option<IInspectable>,
) {
    let Some(native_host) = native_host else {
        eprintln!("controls-example: {name} host native element missing");
        return;
    };

    let result = native_host
        .cast::<NativeBorder>()
        .and_then(|host| create().and_then(|control| host.SetChild(&control)));

    match result {
        Ok(()) => eprintln!("controls-example: mounted visual sample {name} OK"),
        Err(error) => eprintln!("controls-example: mounted visual sample {name} failed: {error:?}"),
    }
}

fn verify_layout_controls() -> String {
    let results = [
        verify_xaml_metadata_provider(),
        verify_wrap_panel(),
        verify_dock_panel(),
        verify_equal_panel(),
        verify_uniform_grid(),
        verify_staggered_panel(),
        verify_staggered_layout(),
        verify_constrained_box(),
        verify_case(),
        verify_color_picker_converters(),
        verify_color_previewer(),
        verify_color_picker_slider(),
        verify_color_picker(),
        verify_color_picker_button(),
        verify_radial_gauge(),
        verify_radial_gauge_automation_peer(),
        verify_layout_transform_control(),
        verify_image_cropper(),
        verify_image_cropper_thumb(),
        verify_camera_preview(),
        verify_preview_failed_event_args(),
        verify_metadata_control(),
        verify_metadata_item(),
        verify_property_sizer(),
        verify_sizer_automation_peer(),
        verify_content_sizer(),
        verify_grid_splitter(),
        verify_headered_content_control(),
        verify_headered_items_control(),
        verify_headered_tree_view(),
        verify_segmented(),
        verify_segmented_item(),
        verify_segmented_margin_converter(),
        verify_settings_card(),
        verify_settings_card_automation_peer(),
        verify_settings_expander(),
        verify_settings_expander_automation_peer(),
        verify_settings_expander_item_style_selector(),
        verify_style_extensions(),
        verify_corner_radius_converter(),
        verify_range_selector(),
        verify_case_collection(),
        verify_switch_converter(),
        verify_switch_presenter(),
        verify_tabbed_command_bar(),
        verify_tabbed_command_bar_item(),
        verify_tabbed_command_bar_item_template_selector(),
        verify_tokenizing_text_box(),
        verify_tokenizing_text_box_item(),
        verify_tokenizing_text_box_style_selector(),
        verify_token_item_adding_event_args(),
        verify_token_item_removing_event_args(),
        verify_tokenizing_text_box_automation_peer(),
        verify_rich_suggest_box(),
        verify_rich_suggest_token(),
        verify_suggestion_requested_event_args(),
        verify_suggestion_chosen_event_args(),
    ];
    format!("Controls: {}", results.join("; "))
}

fn verify_xaml_metadata_provider() -> String {
    eprintln!("controls-example: before XamlMetaDataProvider::new");
    match XamlMetaDataProvider::new() {
        Ok(provider) => {
            eprintln!("controls-example: XamlMetaDataProvider::new OK");
            let full_name = HSTRING::from("XamlToolkit.WinUI.Controls.WrapPanel");
            let xaml_type = provider.GetXamlTypeByFullName(&full_name);
            let xmlns = provider.GetXmlnsDefinitions();
            eprintln!(
                "controls-example: XamlMetaDataProvider.GetXamlTypeByFullName(WrapPanel)={xaml_type:?}, GetXmlnsDefinitions={xmlns:?}"
            );
            format!("XamlMetaDataProvider OK ({xaml_type:?}, {xmlns:?})")
        }
        Err(error) => {
            eprintln!("controls-example: XamlMetaDataProvider::new failed: {error:?}");
            format!("XamlMetaDataProvider failed: {error:?}")
        }
    }
}

fn verify_wrap_panel() -> String {
    eprintln!("controls-example: before WrapPanel::new");
    match WrapPanel::new() {
        Ok(panel) => {
            eprintln!("controls-example: WrapPanel::new OK");
            let spacing = panel.SetHorizontalSpacing(12.0);
            eprintln!("controls-example: WrapPanel.SetHorizontalSpacing returned {spacing:?}");
            let stretch = panel.SetStretchChild(StretchChild::Last);
            eprintln!("controls-example: WrapPanel.SetStretchChild returned {stretch:?}");
            format!("WrapPanel OK ({spacing:?}, {stretch:?})")
        }
        Err(error) => {
            eprintln!("controls-example: WrapPanel::new failed: {error:?}");
            format!("WrapPanel failed: {error:?}")
        }
    }
}

fn verify_dock_panel() -> String {
    eprintln!("controls-example: before DockPanel::new");
    match DockPanel::new() {
        Ok(panel) => {
            eprintln!("controls-example: DockPanel::new OK");
            let fill = panel.SetLastChildFill(true);
            let spacing = panel.SetHorizontalSpacing(10.0);
            eprintln!(
                "controls-example: DockPanel.SetLastChildFill={fill:?}, SetHorizontalSpacing={spacing:?}"
            );
            format!("DockPanel OK ({fill:?}, {spacing:?})")
        }
        Err(error) => {
            eprintln!("controls-example: DockPanel::new failed: {error:?}");
            format!("DockPanel failed: {error:?}")
        }
    }
}

fn verify_equal_panel() -> String {
    eprintln!("controls-example: before EqualPanel::new");
    match EqualPanel::new() {
        Ok(panel) => {
            eprintln!("controls-example: EqualPanel::new OK");
            let spacing = panel.SetSpacing(8.0);
            eprintln!("controls-example: EqualPanel.SetSpacing returned {spacing:?}");
            format!("EqualPanel OK ({spacing:?})")
        }
        Err(error) => {
            eprintln!("controls-example: EqualPanel::new failed: {error:?}");
            format!("EqualPanel failed: {error:?}")
        }
    }
}

fn verify_uniform_grid() -> String {
    eprintln!("controls-example: before UniformGrid::new");
    match UniformGrid::new() {
        Ok(grid) => {
            eprintln!("controls-example: UniformGrid::new OK");
            let columns = grid.SetColumns(3);
            let rows = grid.SetRows(2);
            eprintln!("controls-example: UniformGrid.SetColumns={columns:?}, SetRows={rows:?}");
            format!("UniformGrid OK ({columns:?}, {rows:?})")
        }
        Err(error) => {
            eprintln!("controls-example: UniformGrid::new failed: {error:?}");
            format!("UniformGrid failed: {error:?}")
        }
    }
}

fn verify_staggered_panel() -> String {
    eprintln!("controls-example: before StaggeredPanel::new");
    match StaggeredPanel::new() {
        Ok(panel) => {
            eprintln!("controls-example: StaggeredPanel::new OK");
            let width = panel.SetDesiredColumnWidth(180.0);
            let column = panel.SetColumnSpacing(12.0);
            let row = panel.SetRowSpacing(10.0);
            eprintln!(
                "controls-example: StaggeredPanel.SetDesiredColumnWidth={width:?}, SetColumnSpacing={column:?}, SetRowSpacing={row:?}"
            );
            format!("StaggeredPanel OK ({width:?}, {column:?}, {row:?})")
        }
        Err(error) => {
            eprintln!("controls-example: StaggeredPanel::new failed: {error:?}");
            format!("StaggeredPanel failed: {error:?}")
        }
    }
}

fn verify_staggered_layout() -> String {
    eprintln!("controls-example: before StaggeredLayout::new");
    match StaggeredLayout::new() {
        Ok(layout) => {
            eprintln!("controls-example: StaggeredLayout::new OK");
            let stretch = layout.SetItemsStretch(StaggeredLayoutItemsStretch::Fill);
            let width = layout.SetDesiredColumnWidth(160.0);
            let column = layout.SetColumnSpacing(12.0);
            let row = layout.SetRowSpacing(10.0);
            eprintln!(
                "controls-example: StaggeredLayout.SetItemsStretch={stretch:?}, SetDesiredColumnWidth={width:?}, SetColumnSpacing={column:?}, SetRowSpacing={row:?}"
            );
            format!("StaggeredLayout OK ({stretch:?}, {width:?}, {column:?}, {row:?})")
        }
        Err(error) => {
            eprintln!("controls-example: StaggeredLayout::new failed: {error:?}");
            format!("StaggeredLayout failed: {error:?}")
        }
    }
}

fn verify_constrained_box() -> String {
    eprintln!("controls-example: before ConstrainedBox::new");
    match ConstrainedBox::new() {
        Ok(box_) => {
            eprintln!("controls-example: ConstrainedBox::new OK");
            let scale_x = box_.SetScaleX(2.0);
            let scale_y = box_.SetScaleY(1.0);
            eprintln!(
                "controls-example: ConstrainedBox.SetScaleX={scale_x:?}, SetScaleY={scale_y:?}"
            );
            format!("ConstrainedBox OK ({scale_x:?}, {scale_y:?})")
        }
        Err(error) => {
            eprintln!("controls-example: ConstrainedBox::new failed: {error:?}");
            format!("ConstrainedBox failed: {error:?}")
        }
    }
}

fn verify_case() -> String {
    eprintln!("controls-example: before Case::new");
    match Case::new() {
        Ok(case) => {
            eprintln!("controls-example: Case::new OK");
            let is_default = case.SetIsDefault(true);
            eprintln!("controls-example: Case.SetIsDefault returned {is_default:?}");
            format!("Case OK ({is_default:?})")
        }
        Err(error) => {
            eprintln!("controls-example: Case::new failed: {error:?}");
            format!("Case failed: {error:?}")
        }
    }
}

fn verify_color_picker_converters() -> String {
    let accent = AccentColorConverter::new().and_then(|_| {
        AccentColorConverter::GetAccent(
            HsvColor {
                H: 210.0,
                S: 0.5,
                V: 0.7,
                A: 1.0,
            },
            2,
        )
        .map(|_| ())
    });
    let color_to_hex = ColorToHexConverter::new().map(|_| ());
    let contrast = ContrastBrushConverter::new().map(|_| ());
    let null_to_transparent = NullToTransparentConverter::new().map(|_| ());
    eprintln!(
        "controls-example: ColorPicker converters Accent/GetAccent={accent:?}, ColorToHex={color_to_hex:?}, ContrastBrush={contrast:?}, NullToTransparent={null_to_transparent:?}"
    );
    format!(
        "ColorPickerConverters OK ({accent:?}, {color_to_hex:?}, {contrast:?}, {null_to_transparent:?})"
    )
}

fn verify_color_previewer() -> String {
    eprintln!("controls-example: before ColorPreviewer::new");
    match ColorPreviewer::new() {
        Ok(previewer) => {
            eprintln!("controls-example: ColorPreviewer::new OK");
            let hsv = previewer.SetHsvColor(HsvColor {
                H: 120.0,
                S: 0.6,
                V: 0.8,
                A: 1.0,
            });
            let accents = previewer.SetShowAccentColors(true);
            eprintln!(
                "controls-example: ColorPreviewer.SetHsvColor={hsv:?}, SetShowAccentColors={accents:?}"
            );
            format!("ColorPreviewer OK ({hsv:?}, {accents:?})")
        }
        Err(error) => {
            eprintln!("controls-example: ColorPreviewer::new failed: {error:?}");
            format!("ColorPreviewer failed: {error:?}")
        }
    }
}

fn verify_color_picker_slider() -> String {
    eprintln!("controls-example: before ColorPickerSlider::new");
    match ColorPickerSlider::new() {
        Ok(slider) => {
            eprintln!("controls-example: ColorPickerSlider::new OK");
            let channel = slider.SetColorChannel(ColorChannel::Channel1);
            let representation = slider.SetColorRepresentation(ColorRepresentation::Hsva);
            let hsv = slider.SetHsvColor(HsvColor {
                H: 30.0,
                S: 0.4,
                V: 0.9,
                A: 1.0,
            });
            let alpha = slider.SetIsAlphaMaxForced(true);
            let auto = slider.SetIsAutoUpdatingEnabled(true);
            let saturation = slider.SetIsSaturationValueMaxForced(false);
            let update = slider.UpdateColors();
            eprintln!(
                "controls-example: ColorPickerSlider.SetColorChannel={channel:?}, SetColorRepresentation={representation:?}, SetHsvColor={hsv:?}, SetIsAlphaMaxForced={alpha:?}, SetIsAutoUpdatingEnabled={auto:?}, SetIsSaturationValueMaxForced={saturation:?}, UpdateColors={update:?}"
            );
            format!(
                "ColorPickerSlider OK ({channel:?}, {representation:?}, {hsv:?}, {alpha:?}, {auto:?}, {saturation:?}, {update:?})"
            )
        }
        Err(error) => {
            eprintln!("controls-example: ColorPickerSlider::new failed: {error:?}");
            format!("ColorPickerSlider failed: {error:?}")
        }
    }
}

fn verify_color_picker() -> String {
    eprintln!("controls-example: before ColorPicker::new");
    match ColorPicker::new() {
        Ok(picker) => {
            eprintln!("controls-example: ColorPicker::new OK");
            let columns = picker.SetCustomPaletteColumnCount(6);
            let palette = picker.SetIsColorPaletteVisible(true);
            let accents = picker.SetShowAccentColors(true);
            eprintln!(
                "controls-example: ColorPicker.SetCustomPaletteColumnCount={columns:?}, SetIsColorPaletteVisible={palette:?}, SetShowAccentColors={accents:?}"
            );
            format!("ColorPicker OK ({columns:?}, {palette:?}, {accents:?})")
        }
        Err(error) => {
            eprintln!("controls-example: ColorPicker::new failed: {error:?}");
            format!("ColorPicker failed: {error:?}")
        }
    }
}

fn verify_color_picker_button() -> String {
    eprintln!("controls-example: before ColorPickerButton::new");
    match ColorPickerButton::new() {
        Ok(_) => {
            eprintln!("controls-example: ColorPickerButton::new OK");
            "ColorPickerButton OK".to_string()
        }
        Err(error) => {
            eprintln!("controls-example: ColorPickerButton::new failed: {error:?}");
            format!("ColorPickerButton failed: {error:?}")
        }
    }
}

fn verify_radial_gauge() -> String {
    eprintln!("controls-example: before RadialGauge::new");
    match RadialGauge::new() {
        Ok(gauge) => {
            eprintln!("controls-example: RadialGauge::new OK");
            let minimum = gauge.SetMinimum(0.0);
            let maximum = gauge.SetMaximum(100.0);
            let interactive = gauge.SetIsInteractive(true);
            let scale_width = gauge.SetScaleWidth(26.0);
            let step = gauge.SetStepSize(2.0);
            let unit = gauge.SetUnit(&HSTRING::from("%"));
            let format = gauge.SetValueStringFormat(&HSTRING::from("{0:F0}"));
            let min_angle = gauge.SetMinAngle(-150);
            let max_angle = gauge.SetMaxAngle(150);
            let value_angle = gauge.SetValueAngle(0.0);
            eprintln!(
                "controls-example: RadialGauge.SetMinimum={minimum:?}, SetMaximum={maximum:?}, SetIsInteractive={interactive:?}, SetScaleWidth={scale_width:?}, SetStepSize={step:?}, SetUnit={unit:?}, SetValueStringFormat={format:?}, SetMinAngle={min_angle:?}, SetMaxAngle={max_angle:?}, SetValueAngle={value_angle:?}"
            );
            format!(
                "RadialGauge OK ({minimum:?}, {maximum:?}, {interactive:?}, {scale_width:?}, {step:?}, {unit:?}, {format:?}, {min_angle:?}, {max_angle:?}, {value_angle:?})"
            )
        }
        Err(error) => {
            eprintln!("controls-example: RadialGauge::new failed: {error:?}");
            format!("RadialGauge failed: {error:?}")
        }
    }
}

fn verify_radial_gauge_automation_peer() -> String {
    eprintln!("controls-example: before RadialGaugeAutomationPeer::CreateInstance");
    match RadialGauge::new().and_then(|owner| RadialGaugeAutomationPeer::CreateInstance(&owner)) {
        Ok(peer) => {
            eprintln!("controls-example: RadialGaugeAutomationPeer::CreateInstance OK");
            let raise = peer.RaiseValueChangedEvent(10.0, 20.0);
            eprintln!(
                "controls-example: RadialGaugeAutomationPeer.RaiseValueChangedEvent={raise:?}"
            );
            format!("RadialGaugeAutomationPeer OK ({raise:?})")
        }
        Err(error) => {
            eprintln!(
                "controls-example: RadialGaugeAutomationPeer::CreateInstance failed: {error:?}"
            );
            format!("RadialGaugeAutomationPeer failed: {error:?}")
        }
    }
}

fn verify_layout_transform_control() -> String {
    eprintln!("controls-example: before LayoutTransformControl::new");
    match LayoutTransformControl::new() {
        Ok(control) => {
            eprintln!("controls-example: LayoutTransformControl::new OK");
            let enabled = control.SetIsEnabled(true);
            let width = control.SetWidth(240.0);
            eprintln!(
                "controls-example: LayoutTransformControl.SetIsEnabled={enabled:?}, SetWidth={width:?}"
            );
            format!("LayoutTransformControl OK ({enabled:?}, {width:?})")
        }
        Err(error) => {
            eprintln!("controls-example: LayoutTransformControl::new failed: {error:?}");
            format!("LayoutTransformControl failed: {error:?}")
        }
    }
}

fn verify_image_cropper() -> String {
    eprintln!("controls-example: before ImageCropper::new");
    match ImageCropper::new() {
        Ok(cropper) => {
            eprintln!("controls-example: ImageCropper::new OK");
            let min_crop = cropper.SetMinCroppedPixelLength(32.0);
            let min_select = cropper.SetMinSelectedLength(24.0);
            let shape = cropper.SetCropShape(
                xamltoolkit_winui_controls::XamlToolkit::WinUI::Controls::CropShape::Rectangular,
            );
            let placement = cropper.SetThumbPlacement(ThumbPlacement::Corners);
            let region = cropper.TrySetCroppedRegion(Rect {
                X: 0.0,
                Y: 0.0,
                Width: 32.0,
                Height: 32.0,
            });
            eprintln!(
                "controls-example: ImageCropper.SetMinCroppedPixelLength={min_crop:?}, SetMinSelectedLength={min_select:?}, SetCropShape={shape:?}, SetThumbPlacement={placement:?}, TrySetCroppedRegion={region:?}"
            );
            format!(
                "ImageCropper OK ({min_crop:?}, {min_select:?}, {shape:?}, {placement:?}, {region:?})"
            )
        }
        Err(error) => {
            eprintln!("controls-example: ImageCropper::new failed: {error:?}");
            format!("ImageCropper failed: {error:?}")
        }
    }
}

fn verify_image_cropper_thumb() -> String {
    eprintln!("controls-example: before ImageCropperThumb::new");
    match ImageCropperThumb::new() {
        Ok(thumb) => {
            eprintln!("controls-example: ImageCropperThumb::new OK");
            let x = thumb.SetX(12.0);
            let y = thumb.SetY(16.0);
            let position = thumb.SetPosition(ThumbPosition::UpperLeft);
            eprintln!(
                "controls-example: ImageCropperThumb.SetX={x:?}, SetY={y:?}, SetPosition={position:?}"
            );
            format!("ImageCropperThumb OK ({x:?}, {y:?}, {position:?})")
        }
        Err(error) => {
            eprintln!("controls-example: ImageCropperThumb::new failed: {error:?}");
            format!("ImageCropperThumb failed: {error:?}")
        }
    }
}
fn verify_camera_preview() -> String {
    eprintln!("controls-example: before CameraPreview::new");
    match CameraPreview::new() {
        Ok(preview) => {
            eprintln!("controls-example: CameraPreview::new OK");
            let visible = preview.SetIsFrameSourceGroupButtonVisible(false);
            let property = CameraPreview::IsFrameSourceGroupButtonVisibleProperty().map(|_| ());
            let stop = preview.Stop();
            eprintln!(
                "controls-example: CameraPreview.SetIsFrameSourceGroupButtonVisible={visible:?}, IsFrameSourceGroupButtonVisibleProperty={property:?}, Stop={stop:?}"
            );
            format!("CameraPreview OK ({visible:?}, {property:?}, {stop:?})")
        }
        Err(error) => {
            eprintln!("controls-example: CameraPreview::new failed: {error:?}");
            format!("CameraPreview failed: {error:?}")
        }
    }
}

fn verify_preview_failed_event_args() -> String {
    eprintln!("controls-example: before PreviewFailedEventArgs::CreateInstance");
    match PreviewFailedEventArgs::CreateInstance(&HSTRING::from("camera unavailable")) {
        Ok(args) => {
            eprintln!("controls-example: PreviewFailedEventArgs::CreateInstance OK");
            let error = args.Error().map(|value| format!("{value:?}"));
            eprintln!("controls-example: PreviewFailedEventArgs.Error={error:?}");
            format!("PreviewFailedEventArgs OK ({error:?})")
        }
        Err(error) => {
            eprintln!("controls-example: PreviewFailedEventArgs::CreateInstance failed: {error:?}");
            format!("PreviewFailedEventArgs failed: {error:?}")
        }
    }
}
fn verify_metadata_control() -> String {
    eprintln!("controls-example: before MetadataControl::new");
    match MetadataControl::new() {
        Ok(control) => {
            eprintln!("controls-example: MetadataControl::new OK");
            let separator = control.SetSeparator(&HSTRING::from("/"));
            let accessible_separator = control.SetAccessibleSeparator(&HSTRING::from("slash"));
            eprintln!(
                "controls-example: MetadataControl.SetSeparator={separator:?}, SetAccessibleSeparator={accessible_separator:?}"
            );
            format!("MetadataControl OK ({separator:?}, {accessible_separator:?})")
        }
        Err(error) => {
            eprintln!("controls-example: MetadataControl::new failed: {error:?}");
            format!("MetadataControl failed: {error:?}")
        }
    }
}

fn verify_metadata_item() -> String {
    eprintln!("controls-example: before MetadataItem::new");
    match MetadataItem::new() {
        Ok(_) => {
            eprintln!("controls-example: MetadataItem::new OK");
            "MetadataItem OK".to_string()
        }
        Err(error) => {
            eprintln!("controls-example: MetadataItem::new failed: {error:?}");
            format!("MetadataItem failed: {error:?}")
        }
    }
}

fn verify_property_sizer() -> String {
    eprintln!("controls-example: before PropertySizer::new");
    match PropertySizer::new() {
        Ok(sizer) => {
            eprintln!("controls-example: PropertySizer::new OK");
            let binding = sizer.SetBinding(120.0);
            let minimum = sizer.SetMinimum(40.0);
            let maximum = sizer.SetMaximum(360.0);
            let orientation = sizer.SetOrientation(Orientation::Horizontal);
            let drag = sizer.SetDragIncrement(4.0);
            eprintln!(
                "controls-example: PropertySizer.SetBinding={binding:?}, SetMinimum={minimum:?}, SetMaximum={maximum:?}, SetOrientation={orientation:?}, SetDragIncrement={drag:?}"
            );
            format!(
                "PropertySizer OK ({binding:?}, {minimum:?}, {maximum:?}, {orientation:?}, {drag:?})"
            )
        }
        Err(error) => {
            eprintln!("controls-example: PropertySizer::new failed: {error:?}");
            format!("PropertySizer failed: {error:?}")
        }
    }
}

fn verify_sizer_automation_peer() -> String {
    eprintln!("controls-example: before SizerAutomationPeer::CreateInstance");
    match PropertySizer::new().and_then(|owner| SizerAutomationPeer::CreateInstance(&owner)) {
        Ok(_) => {
            eprintln!("controls-example: SizerAutomationPeer::CreateInstance OK");
            "SizerAutomationPeer OK".to_string()
        }
        Err(error) => {
            eprintln!("controls-example: SizerAutomationPeer::CreateInstance failed: {error:?}");
            format!("SizerAutomationPeer failed: {error:?}")
        }
    }
}

fn verify_content_sizer() -> String {
    eprintln!("controls-example: before ContentSizer::new");
    match ContentSizer::new() {
        Ok(sizer) => {
            eprintln!("controls-example: ContentSizer::new OK");
            let inverted = sizer.SetIsDragInverted(false);
            let keyboard = sizer.SetKeyboardIncrement(8.0);
            let thumb = sizer.SetIsThumbVisible(true);
            eprintln!(
                "controls-example: ContentSizer.SetIsDragInverted={inverted:?}, SetKeyboardIncrement={keyboard:?}, SetIsThumbVisible={thumb:?}"
            );
            format!("ContentSizer OK ({inverted:?}, {keyboard:?}, {thumb:?})")
        }
        Err(error) => {
            eprintln!("controls-example: ContentSizer::new failed: {error:?}");
            format!("ContentSizer failed: {error:?}")
        }
    }
}

fn verify_grid_splitter() -> String {
    eprintln!("controls-example: before GridSplitter::new");
    match GridSplitter::new() {
        Ok(splitter) => {
            eprintln!("controls-example: GridSplitter::new OK");
            let direction = splitter.SetResizeDirection(GridResizeDirection::Columns);
            let behavior = splitter.SetResizeBehavior(GridResizeBehavior::CurrentAndNext);
            let parent = splitter.SetParentLevel(1);
            let orientation = splitter.SetOrientation(Orientation::Vertical);
            eprintln!(
                "controls-example: GridSplitter.SetResizeDirection={direction:?}, SetResizeBehavior={behavior:?}, SetParentLevel={parent:?}, SetOrientation={orientation:?}"
            );
            format!("GridSplitter OK ({direction:?}, {behavior:?}, {parent:?}, {orientation:?})")
        }
        Err(error) => {
            eprintln!("controls-example: GridSplitter::new failed: {error:?}");
            format!("GridSplitter failed: {error:?}")
        }
    }
}

fn verify_headered_content_control() -> String {
    eprintln!("controls-example: before HeaderedContentControl::new");
    match HeaderedContentControl::new() {
        Ok(control) => {
            eprintln!("controls-example: HeaderedContentControl::new OK");
            let header = PropertyValue::CreateString(&HSTRING::from("Headered content"))
                .and_then(|value| control.SetHeader(&value));
            let orientation = control.SetOrientation(Orientation::Horizontal);
            eprintln!(
                "controls-example: HeaderedContentControl.SetHeader={header:?}, SetOrientation={orientation:?}"
            );
            format!("HeaderedContentControl OK ({header:?}, {orientation:?})")
        }
        Err(error) => {
            eprintln!("controls-example: HeaderedContentControl::new failed: {error:?}");
            format!("HeaderedContentControl failed: {error:?}")
        }
    }
}

fn verify_headered_items_control() -> String {
    eprintln!("controls-example: before HeaderedItemsControl::new");
    match HeaderedItemsControl::new() {
        Ok(control) => {
            eprintln!("controls-example: HeaderedItemsControl::new OK");
            let header = PropertyValue::CreateString(&HSTRING::from("Headered items"))
                .and_then(|value| control.SetHeader(&value));
            let footer = PropertyValue::CreateString(&HSTRING::from("Items footer"))
                .and_then(|value| control.SetFooter(&value));
            eprintln!(
                "controls-example: HeaderedItemsControl.SetHeader={header:?}, SetFooter={footer:?}"
            );
            format!("HeaderedItemsControl OK ({header:?}, {footer:?})")
        }
        Err(error) => {
            eprintln!("controls-example: HeaderedItemsControl::new failed: {error:?}");
            format!("HeaderedItemsControl failed: {error:?}")
        }
    }
}

fn verify_headered_tree_view() -> String {
    eprintln!("controls-example: before HeaderedTreeView::new");
    match HeaderedTreeView::new() {
        Ok(control) => {
            eprintln!("controls-example: HeaderedTreeView::new OK");
            let header = PropertyValue::CreateString(&HSTRING::from("Headered tree"))
                .and_then(|value| control.SetHeader(&value));
            let footer = PropertyValue::CreateString(&HSTRING::from("Tree footer"))
                .and_then(|value| control.SetFooter(&value));
            eprintln!(
                "controls-example: HeaderedTreeView.SetHeader={header:?}, SetFooter={footer:?}"
            );
            format!("HeaderedTreeView OK ({header:?}, {footer:?})")
        }
        Err(error) => {
            eprintln!("controls-example: HeaderedTreeView::new failed: {error:?}");
            format!("HeaderedTreeView failed: {error:?}")
        }
    }
}

fn verify_segmented() -> String {
    eprintln!("controls-example: before Segmented::new");
    match Segmented::new() {
        Ok(control) => {
            eprintln!("controls-example: Segmented::new OK");
            let orientation = control.SetOrientation(Orientation::Horizontal);
            eprintln!("controls-example: Segmented.SetOrientation={orientation:?}");
            format!("Segmented OK ({orientation:?})")
        }
        Err(error) => {
            eprintln!("controls-example: Segmented::new failed: {error:?}");
            format!("Segmented failed: {error:?}")
        }
    }
}

fn verify_segmented_item() -> String {
    eprintln!("controls-example: before SegmentedItem::new");
    match SegmentedItem::new() {
        Ok(_) => {
            eprintln!("controls-example: SegmentedItem::new OK");
            "SegmentedItem OK".to_string()
        }
        Err(error) => {
            eprintln!("controls-example: SegmentedItem::new failed: {error:?}");
            format!("SegmentedItem failed: {error:?}")
        }
    }
}

fn verify_segmented_margin_converter() -> String {
    eprintln!("controls-example: before SegmentedMarginConverter::new");
    match SegmentedMarginConverter::new() {
        Ok(converter) => {
            eprintln!("controls-example: SegmentedMarginConverter::new OK");
            let left = converter.SetLeftItemMargin(thickness(0.0, 0.0, 4.0, 0.0));
            let middle = converter.SetMiddleItemMargin(thickness(4.0, 0.0, 4.0, 0.0));
            let right = converter.SetRightItemMargin(thickness(4.0, 0.0, 0.0, 0.0));
            eprintln!(
                "controls-example: SegmentedMarginConverter.SetLeftItemMargin={left:?}, SetMiddleItemMargin={middle:?}, SetRightItemMargin={right:?}"
            );
            format!("SegmentedMarginConverter OK ({left:?}, {middle:?}, {right:?})")
        }
        Err(error) => {
            eprintln!("controls-example: SegmentedMarginConverter::new failed: {error:?}");
            format!("SegmentedMarginConverter failed: {error:?}")
        }
    }
}

fn verify_settings_card() -> String {
    eprintln!("controls-example: before SettingsCard::new");
    match SettingsCard::new() {
        Ok(card) => {
            eprintln!("controls-example: SettingsCard::new OK");
            let header =
                boxed_string("Settings card header").and_then(|value| card.SetHeader(&value));
            let description = boxed_string("Settings card description")
                .and_then(|value| card.SetDescription(&value));
            let tooltip = card.SetActionIconToolTip(&HSTRING::from("Open settings"));
            let click = card.SetIsClickEnabled(true);
            let visible = card.SetIsActionIconVisible(true);
            let alignment = card.SetContentAlignment(ContentAlignment::Vertical);
            eprintln!(
                "controls-example: SettingsCard.SetHeader={header:?}, SetDescription={description:?}, SetActionIconToolTip={tooltip:?}, SetIsClickEnabled={click:?}, SetIsActionIconVisible={visible:?}, SetContentAlignment={alignment:?}"
            );
            format!(
                "SettingsCard OK ({header:?}, {description:?}, {tooltip:?}, {click:?}, {visible:?}, {alignment:?})"
            )
        }
        Err(error) => {
            eprintln!("controls-example: SettingsCard::new failed: {error:?}");
            format!("SettingsCard failed: {error:?}")
        }
    }
}

fn verify_settings_card_automation_peer() -> String {
    eprintln!("controls-example: before SettingsCardAutomationPeer::CreateInstance");
    match SettingsCard::new().and_then(|owner| SettingsCardAutomationPeer::CreateInstance(&owner)) {
        Ok(_) => {
            eprintln!("controls-example: SettingsCardAutomationPeer::CreateInstance OK");
            "SettingsCardAutomationPeer OK".to_string()
        }
        Err(error) => {
            eprintln!(
                "controls-example: SettingsCardAutomationPeer::CreateInstance failed: {error:?}"
            );
            format!("SettingsCardAutomationPeer failed: {error:?}")
        }
    }
}

fn verify_settings_expander() -> String {
    eprintln!("controls-example: before SettingsExpander::new");
    match SettingsExpander::new() {
        Ok(expander) => {
            eprintln!("controls-example: SettingsExpander::new OK");
            let content = boxed_string("Settings expander content")
                .and_then(|value| expander.SetContent(&value));
            let header = boxed_string("Settings expander header")
                .and_then(|value| expander.SetHeader(&value));
            let description = boxed_string("Settings expander description")
                .and_then(|value| expander.SetDescription(&value));
            let expanded = expander.SetIsExpanded(true);
            eprintln!(
                "controls-example: SettingsExpander.SetContent={content:?}, SetHeader={header:?}, SetDescription={description:?}, SetIsExpanded={expanded:?}"
            );
            format!("SettingsExpander OK ({content:?}, {header:?}, {description:?}, {expanded:?})")
        }
        Err(error) => {
            eprintln!("controls-example: SettingsExpander::new failed: {error:?}");
            format!("SettingsExpander failed: {error:?}")
        }
    }
}

fn verify_settings_expander_automation_peer() -> String {
    eprintln!("controls-example: before SettingsExpanderAutomationPeer::CreateInstance");
    match SettingsExpander::new()
        .and_then(|owner| SettingsExpanderAutomationPeer::CreateInstance(&owner))
    {
        Ok(peer) => {
            eprintln!("controls-example: SettingsExpanderAutomationPeer::CreateInstance OK");
            let raise = peer.RaiseExpandedChangedEvent(true);
            eprintln!(
                "controls-example: SettingsExpanderAutomationPeer.RaiseExpandedChangedEvent={raise:?}"
            );
            format!("SettingsExpanderAutomationPeer OK ({raise:?})")
        }
        Err(error) => {
            eprintln!("controls-example: SettingsExpanderAutomationPeer::CreateInstance failed: {error:?}");
            format!("SettingsExpanderAutomationPeer failed: {error:?}")
        }
    }
}

fn verify_settings_expander_item_style_selector() -> String {
    eprintln!("controls-example: before SettingsExpanderItemStyleSelector::new");
    match SettingsExpanderItemStyleSelector::new() {
        Ok(_) => {
            eprintln!("controls-example: SettingsExpanderItemStyleSelector::new OK");
            "SettingsExpanderItemStyleSelector OK".to_string()
        }
        Err(error) => {
            eprintln!("controls-example: SettingsExpanderItemStyleSelector::new failed: {error:?}");
            format!("SettingsExpanderItemStyleSelector failed: {error:?}")
        }
    }
}

fn verify_style_extensions() -> String {
    eprintln!("controls-example: before StyleExtensions statics");
    match (Style::new(), StyleExtensionResourceDictionary::new()) {
        (Ok(style), Ok(resources)) => {
            eprintln!("controls-example: Style and StyleExtensionResourceDictionary activation OK");
            let set = StyleExtensions::SetResources(&style, &resources);
            let get = StyleExtensions::GetResources(&style).map(|_| ());
            eprintln!(
                "controls-example: StyleExtensions.SetResources={set:?}, GetResources={get:?}"
            );
            format!("StyleExtensions OK ({set:?}, {get:?})")
        }
        (Err(error), _) => {
            eprintln!("controls-example: Style::new failed: {error:?}");
            format!("StyleExtensions Style failed: {error:?}")
        }
        (_, Err(error)) => {
            eprintln!("controls-example: StyleExtensionResourceDictionary::new failed: {error:?}");
            format!("StyleExtensionResourceDictionary failed: {error:?}")
        }
    }
}

fn verify_corner_radius_converter() -> String {
    eprintln!("controls-example: before CornerRadiusConverter::new");
    match CornerRadiusConverter::new() {
        Ok(_) => {
            eprintln!("controls-example: CornerRadiusConverter::new OK");
            "CornerRadiusConverter OK".to_string()
        }
        Err(error) => {
            eprintln!("controls-example: CornerRadiusConverter::new failed: {error:?}");
            format!("CornerRadiusConverter failed: {error:?}")
        }
    }
}

fn thickness(left: f64, top: f64, right: f64, bottom: f64) -> XamlThickness {
    XamlThickness {
        Left: left,
        Top: top,
        Right: right,
        Bottom: bottom,
    }
}

fn boxed_string(value: &str) -> windows::core::Result<windows::core::IInspectable> {
    PropertyValue::CreateString(&HSTRING::from(value))
}

fn verify_range_selector() -> String {
    eprintln!("controls-example: before RangeSelector::new");
    match RangeSelector::new() {
        Ok(selector) => {
            eprintln!("controls-example: RangeSelector::new OK");
            let minimum = selector.SetMinimum(0.0);
            let maximum = selector.SetMaximum(100.0);
            let start = selector.SetRangeStart(20.0);
            let end = selector.SetRangeEnd(80.0);
            let step = selector.SetStepFrequency(5.0);
            eprintln!(
                "controls-example: RangeSelector.SetMinimum={minimum:?}, SetMaximum={maximum:?}, SetRangeStart={start:?}, SetRangeEnd={end:?}, SetStepFrequency={step:?}"
            );
            format!("RangeSelector OK ({minimum:?}, {maximum:?}, {start:?}, {end:?}, {step:?})")
        }
        Err(error) => {
            eprintln!("controls-example: RangeSelector::new failed: {error:?}");
            format!("RangeSelector failed: {error:?}")
        }
    }
}

fn verify_case_collection() -> String {
    eprintln!("controls-example: before CaseCollection::new");
    match CaseCollection::new() {
        Ok(_) => {
            eprintln!("controls-example: CaseCollection::new OK");
            "CaseCollection OK".to_string()
        }
        Err(error) => {
            eprintln!("controls-example: CaseCollection::new failed: {error:?}");
            format!("CaseCollection failed: {error:?}")
        }
    }
}

fn verify_switch_converter() -> String {
    eprintln!("controls-example: before SwitchConverter::new");
    match (SwitchConverter::new(), CaseCollection::new()) {
        (Ok(converter), Ok(cases)) => {
            eprintln!("controls-example: SwitchConverter::new OK");
            let switch_cases = converter.SetSwitchCases(&cases);
            eprintln!("controls-example: SwitchConverter.SetSwitchCases={switch_cases:?}");
            format!("SwitchConverter OK ({switch_cases:?})")
        }
        (Err(error), _) => {
            eprintln!("controls-example: SwitchConverter::new failed: {error:?}");
            format!("SwitchConverter failed: {error:?}")
        }
        (_, Err(error)) => {
            eprintln!("controls-example: SwitchConverter CaseCollection::new failed: {error:?}");
            format!("SwitchConverter cases failed: {error:?}")
        }
    }
}

fn verify_switch_presenter() -> String {
    eprintln!("controls-example: before SwitchPresenter::new");
    match (SwitchPresenter::new(), CaseCollection::new()) {
        (Ok(presenter), Ok(cases)) => {
            eprintln!("controls-example: SwitchPresenter::new OK");
            let switch_cases = presenter.SetSwitchCases(&cases);
            eprintln!("controls-example: SwitchPresenter.SetSwitchCases={switch_cases:?}");
            format!("SwitchPresenter OK ({switch_cases:?})")
        }
        (Err(error), _) => {
            eprintln!("controls-example: SwitchPresenter::new failed: {error:?}");
            format!("SwitchPresenter failed: {error:?}")
        }
        (_, Err(error)) => {
            eprintln!("controls-example: SwitchPresenter CaseCollection::new failed: {error:?}");
            format!("SwitchPresenter cases failed: {error:?}")
        }
    }
}

fn verify_tabbed_command_bar() -> String {
    eprintln!("controls-example: before TabbedCommandBar::new");
    match TabbedCommandBar::new() {
        Ok(_) => {
            eprintln!("controls-example: TabbedCommandBar::new OK");
            "TabbedCommandBar OK".to_string()
        }
        Err(error) => {
            eprintln!("controls-example: TabbedCommandBar::new failed: {error:?}");
            format!("TabbedCommandBar failed: {error:?}")
        }
    }
}

fn verify_tabbed_command_bar_item() -> String {
    eprintln!("controls-example: before TabbedCommandBarItem::new");
    match TabbedCommandBarItem::new() {
        Ok(item) => {
            eprintln!("controls-example: TabbedCommandBarItem::new OK");
            let header = boxed_string("Tabbed command").and_then(|value| item.SetHeader(&value));
            let contextual = item.SetIsContextual(true);
            let overflow = item.SetOverflowButtonAlignment(HorizontalAlignment::Right);
            let command = item.SetCommandAlignment(HorizontalAlignment::Stretch);
            eprintln!(
                "controls-example: TabbedCommandBarItem.SetHeader={header:?}, SetIsContextual={contextual:?}, SetOverflowButtonAlignment={overflow:?}, SetCommandAlignment={command:?}"
            );
            format!(
                "TabbedCommandBarItem OK ({header:?}, {contextual:?}, {overflow:?}, {command:?})"
            )
        }
        Err(error) => {
            eprintln!("controls-example: TabbedCommandBarItem::new failed: {error:?}");
            format!("TabbedCommandBarItem failed: {error:?}")
        }
    }
}

fn verify_tabbed_command_bar_item_template_selector() -> String {
    eprintln!("controls-example: before TabbedCommandBarItemTemplateSelector::new");
    match (
        TabbedCommandBarItemTemplateSelector::new(),
        DataTemplate::new(),
        DataTemplate::new(),
    ) {
        (Ok(selector), Ok(normal), Ok(contextual_template)) => {
            eprintln!("controls-example: TabbedCommandBarItemTemplateSelector::new OK");
            let normal = selector.SetNormal(&normal);
            let contextual = selector.SetContextual(&contextual_template);
            eprintln!(
                "controls-example: TabbedCommandBarItemTemplateSelector.SetNormal={normal:?}, SetContextual={contextual:?}"
            );
            format!("TabbedCommandBarItemTemplateSelector OK ({normal:?}, {contextual:?})")
        }
        (Err(error), _, _) => {
            eprintln!(
                "controls-example: TabbedCommandBarItemTemplateSelector::new failed: {error:?}"
            );
            format!("TabbedCommandBarItemTemplateSelector failed: {error:?}")
        }
        (_, Err(error), _) | (_, _, Err(error)) => {
            eprintln!("controls-example: DataTemplate::new failed: {error:?}");
            format!("TabbedCommandBarItemTemplateSelector DataTemplate failed: {error:?}")
        }
    }
}

fn verify_tokenizing_text_box() -> String {
    eprintln!("controls-example: before TokenizingTextBox::new");
    match TokenizingTextBox::new() {
        Ok(text_box) => {
            eprintln!("controls-example: TokenizingTextBox::new OK");
            let member = text_box.SetTextMemberPath(&HSTRING::from("Name"));
            let delimiter = text_box.SetTokenDelimiter(&HSTRING::from(";"));
            let spacing = text_box.SetTokenSpacing(6.0);
            let placeholder = text_box.SetPlaceholderText(&HSTRING::from("Add token"));
            let text = text_box.SetText(&HSTRING::from("alpha"));
            let suggested = boxed_string("suggested item")
                .and_then(|value| text_box.SetSuggestedItemsSource(&value));
            let tab = text_box.SetTabNavigateBackOnArrow(true);
            let maximum = text_box.SetMaximumTokens(8);
            eprintln!(
                "controls-example: TokenizingTextBox.SetTextMemberPath={member:?}, SetTokenDelimiter={delimiter:?}, SetTokenSpacing={spacing:?}, SetPlaceholderText={placeholder:?}, SetText={text:?}, SetSuggestedItemsSource={suggested:?}, SetTabNavigateBackOnArrow={tab:?}, SetMaximumTokens={maximum:?}"
            );
            format!(
                "TokenizingTextBox OK ({member:?}, {delimiter:?}, {spacing:?}, {placeholder:?}, {text:?}, {suggested:?}, {tab:?}, {maximum:?})"
            )
        }
        Err(error) => {
            eprintln!("controls-example: TokenizingTextBox::new failed: {error:?}");
            format!("TokenizingTextBox failed: {error:?}")
        }
    }
}

fn verify_tokenizing_text_box_item() -> String {
    eprintln!("controls-example: before TokenizingTextBoxItem::new");
    match (
        TokenizingTextBoxItem::new(),
        TokenizingTextBox::new(),
        Style::new(),
    ) {
        (Ok(item), Ok(owner), Ok(style)) => {
            eprintln!("controls-example: TokenizingTextBoxItem::new OK");
            let owner = item.SetOwner(&owner);
            let clear = item.SetClearButtonStyle(&style);
            eprintln!(
                "controls-example: TokenizingTextBoxItem.SetOwner={owner:?}, SetClearButtonStyle={clear:?}"
            );
            format!("TokenizingTextBoxItem OK ({owner:?}, {clear:?})")
        }
        (Err(error), _, _) => {
            eprintln!("controls-example: TokenizingTextBoxItem::new failed: {error:?}");
            format!("TokenizingTextBoxItem failed: {error:?}")
        }
        (_, Err(error), _) => {
            eprintln!("controls-example: TokenizingTextBoxItem owner TokenizingTextBox::new failed: {error:?}");
            format!("TokenizingTextBoxItem owner failed: {error:?}")
        }
        (_, _, Err(error)) => {
            eprintln!("controls-example: TokenizingTextBoxItem Style::new failed: {error:?}");
            format!("TokenizingTextBoxItem style failed: {error:?}")
        }
    }
}

fn verify_tokenizing_text_box_style_selector() -> String {
    eprintln!("controls-example: before TokenizingTextBoxStyleSelector::new");
    match (
        TokenizingTextBoxStyleSelector::new(),
        Style::new(),
        Style::new(),
    ) {
        (Ok(selector), Ok(token_style), Ok(text_style)) => {
            eprintln!("controls-example: TokenizingTextBoxStyleSelector::new OK");
            let token = selector.SetTokenStyle(&token_style);
            let text = selector.SetTextStyle(&text_style);
            eprintln!(
                "controls-example: TokenizingTextBoxStyleSelector.SetTokenStyle={token:?}, SetTextStyle={text:?}"
            );
            format!("TokenizingTextBoxStyleSelector OK ({token:?}, {text:?})")
        }
        (Err(error), _, _) => {
            eprintln!("controls-example: TokenizingTextBoxStyleSelector::new failed: {error:?}");
            format!("TokenizingTextBoxStyleSelector failed: {error:?}")
        }
        (_, Err(error), _) | (_, _, Err(error)) => {
            eprintln!(
                "controls-example: TokenizingTextBoxStyleSelector Style::new failed: {error:?}"
            );
            format!("TokenizingTextBoxStyleSelector style failed: {error:?}")
        }
    }
}

fn verify_token_item_adding_event_args() -> String {
    eprintln!("controls-example: before TokenItemAddingEventArgs::CreateInstance");
    match TokenItemAddingEventArgs::CreateInstance(&HSTRING::from("alpha")) {
        Ok(args) => {
            eprintln!("controls-example: TokenItemAddingEventArgs::CreateInstance OK");
            let token = args.SetTokenText(&HSTRING::from("beta"));
            let item = boxed_string("token item").and_then(|value| args.SetItem(&value));
            let cancel = args.SetCancel(false);
            eprintln!(
                "controls-example: TokenItemAddingEventArgs.SetTokenText={token:?}, SetItem={item:?}, SetCancel={cancel:?}"
            );
            format!("TokenItemAddingEventArgs OK ({token:?}, {item:?}, {cancel:?})")
        }
        Err(error) => {
            eprintln!(
                "controls-example: TokenItemAddingEventArgs::CreateInstance failed: {error:?}"
            );
            format!("TokenItemAddingEventArgs failed: {error:?}")
        }
    }
}

fn verify_token_item_removing_event_args() -> String {
    eprintln!("controls-example: before TokenItemRemovingEventArgs::CreateInstance");
    match (boxed_string("token item"), TokenizingTextBoxItem::new()) {
        (Ok(item), Ok(token)) => match TokenItemRemovingEventArgs::CreateInstance(&item, &token) {
            Ok(args) => {
                eprintln!("controls-example: TokenItemRemovingEventArgs::CreateInstance OK");
                let cancel = args.SetCancel(false);
                eprintln!("controls-example: TokenItemRemovingEventArgs.SetCancel={cancel:?}");
                format!("TokenItemRemovingEventArgs OK ({cancel:?})")
            }
            Err(error) => {
                eprintln!("controls-example: TokenItemRemovingEventArgs::CreateInstance failed: {error:?}");
                format!("TokenItemRemovingEventArgs failed: {error:?}")
            }
        },
        (Err(error), _) => {
            eprintln!("controls-example: TokenItemRemovingEventArgs boxed item failed: {error:?}");
            format!("TokenItemRemovingEventArgs item failed: {error:?}")
        }
        (_, Err(error)) => {
            eprintln!("controls-example: TokenItemRemovingEventArgs TokenizingTextBoxItem::new failed: {error:?}");
            format!("TokenItemRemovingEventArgs token failed: {error:?}")
        }
    }
}

fn verify_tokenizing_text_box_automation_peer() -> String {
    eprintln!("controls-example: before TokenizingTextBoxAutomationPeer::CreateInstance");
    match TokenizingTextBox::new()
        .and_then(|owner| TokenizingTextBoxAutomationPeer::CreateInstance(&owner))
    {
        Ok(_) => {
            eprintln!("controls-example: TokenizingTextBoxAutomationPeer::CreateInstance OK");
            "TokenizingTextBoxAutomationPeer OK".to_string()
        }
        Err(error) => {
            eprintln!("controls-example: TokenizingTextBoxAutomationPeer::CreateInstance failed: {error:?}");
            format!("TokenizingTextBoxAutomationPeer failed: {error:?}")
        }
    }
}

fn verify_rich_suggest_box() -> String {
    eprintln!("controls-example: before RichSuggestBox::new");
    match RichSuggestBox::new() {
        Ok(_) => {
            eprintln!("controls-example: RichSuggestBox::new OK");
            "RichSuggestBox activation OK".to_string()
        }
        Err(error) => {
            eprintln!("controls-example: RichSuggestBox::new failed: {error:?}");
            format!("RichSuggestBox failed: {error:?}")
        }
    }
}

fn verify_rich_suggest_token() -> String {
    eprintln!("controls-example: before RichSuggestToken::CreateInstance");
    match RichSuggestToken::CreateInstance(
        GUID::from_u128(0x12345678_1234_5678_9abc_def012345678),
        &HSTRING::from("Alice"),
    ) {
        Ok(token) => {
            eprintln!("controls-example: RichSuggestToken::CreateInstance OK");
            let item = boxed_string("Alice item").and_then(|value| token.SetItem(&value));
            let active = token.SetActive(true);
            let string = token.ToString().map(|_| ());
            eprintln!(
                "controls-example: RichSuggestToken.SetItem={item:?}, SetActive={active:?}, ToString={string:?}"
            );
            format!("RichSuggestToken OK ({item:?}, {active:?}, {string:?})")
        }
        Err(error) => {
            eprintln!("controls-example: RichSuggestToken::CreateInstance failed: {error:?}");
            format!("RichSuggestToken failed: {error:?}")
        }
    }
}

fn verify_suggestion_requested_event_args() -> String {
    eprintln!("controls-example: before SuggestionRequestedEventArgs::new");
    match SuggestionRequestedEventArgs::new() {
        Ok(args) => {
            eprintln!("controls-example: SuggestionRequestedEventArgs::new OK");
            let prefix = args.SetPrefix(&HSTRING::from("@"));
            let query = args.SetQueryText(&HSTRING::from("ali"));
            eprintln!(
                "controls-example: SuggestionRequestedEventArgs.SetPrefix={prefix:?}, SetQueryText={query:?}"
            );
            format!("SuggestionRequestedEventArgs OK ({prefix:?}, {query:?})")
        }
        Err(error) => {
            eprintln!("controls-example: SuggestionRequestedEventArgs::new failed: {error:?}");
            format!("SuggestionRequestedEventArgs failed: {error:?}")
        }
    }
}

fn verify_suggestion_chosen_event_args() -> String {
    eprintln!("controls-example: before SuggestionChosenEventArgs::new");
    match SuggestionChosenEventArgs::new() {
        Ok(args) => {
            eprintln!("controls-example: SuggestionChosenEventArgs::new OK");
            let display = args.SetDisplayText(&HSTRING::from("Alice"));
            eprintln!("controls-example: SuggestionChosenEventArgs.SetDisplayText={display:?}");
            format!("SuggestionChosenEventArgs OK ({display:?})")
        }
        Err(error) => {
            eprintln!("controls-example: SuggestionChosenEventArgs::new failed: {error:?}");
            format!("SuggestionChosenEventArgs failed: {error:?}")
        }
    }
}
