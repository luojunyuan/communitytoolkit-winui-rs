use windows::core::{GUID, HSTRING};
use windows::Foundation::PropertyValue;
use windows_reactor::*;
use xamltoolkit_winui_controls::Microsoft::UI::Xaml::Controls::Orientation;
use xamltoolkit_winui_controls::Microsoft::UI::Xaml::Thickness;
use xamltoolkit_winui_controls::Microsoft::UI::Xaml::{DataTemplate, HorizontalAlignment, Style};
use xamltoolkit_winui_controls::Windows::Foundation::Rect;
use xamltoolkit_winui_controls::XamlToolkit::WinUI::Controls::Primitives::{
    ColorPickerSlider, ColorPreviewer,
};
use xamltoolkit_winui_controls::XamlToolkit::WinUI::Controls::{
    AccentColorConverter, CameraPreview, Case, CaseCollection, ColorChannel, ColorPicker,
    ColorPickerButton, ColorRepresentation, ColorToHexConverter, ConstrainedBox, ContentAlignment,
    ContentSizer, ContrastBrushConverter, CornerRadiusConverter, DockPanel, EqualPanel,
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
    TokenizingTextBoxStyleSelector, UniformGrid, WrapPanel,
};
use xamltoolkit_winui_controls::XamlToolkit::WinUI::HsvColor;
use xamltoolkit_winui_converters::XamlToolkit::WinUI::Converters::BoolNegationConverter;
use xamltoolkit_winui_helpers::XamlToolkit::WinUI::Helpers::{ColorHelper, DesignTimeHelpers};

fn main() {
    eprintln!("xamltoolkit-labs: starting");
    if let Err(error) = App::new().title("XamlToolkit Labs").render(app) {
        eprintln!("xamltoolkit-labs failed: {error:?}");
        std::process::exit(1);
    }
}

fn app(_cx: &mut RenderCx) -> Element {
    eprintln!("xamltoolkit-labs: rendering app");

    let controls_status = verify_layout_controls();
    let helpers_status = verify_helpers();
    let converters_status = verify_bool_negation_converter();

    vstack((
        text_block("XamlToolkit Labs").font_size(22.0).bold(),
        text_block("Rust Labs demo consuming generated XamlToolkit WinRT projection crates."),
        text_block(controls_status),
        text_block(helpers_status),
        text_block(converters_status),
    ))
    .spacing(8.0)
    .padding(24.0)
    .into()
}

fn verify_layout_controls() -> String {
    let results = [
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

fn verify_wrap_panel() -> String {
    eprintln!("xamltoolkit-labs: before WrapPanel::new");
    match WrapPanel::new() {
        Ok(panel) => {
            eprintln!("xamltoolkit-labs: WrapPanel::new OK");
            let spacing = panel.SetHorizontalSpacing(12.0);
            eprintln!("xamltoolkit-labs: WrapPanel.SetHorizontalSpacing returned {spacing:?}");
            let stretch = panel.SetStretchChild(StretchChild::Last);
            eprintln!("xamltoolkit-labs: WrapPanel.SetStretchChild returned {stretch:?}");
            format!("WrapPanel OK ({spacing:?}, {stretch:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: WrapPanel::new failed: {error:?}");
            format!("WrapPanel failed: {error:?}")
        }
    }
}

fn verify_dock_panel() -> String {
    eprintln!("xamltoolkit-labs: before DockPanel::new");
    match DockPanel::new() {
        Ok(panel) => {
            eprintln!("xamltoolkit-labs: DockPanel::new OK");
            let fill = panel.SetLastChildFill(true);
            let spacing = panel.SetHorizontalSpacing(10.0);
            eprintln!(
                "xamltoolkit-labs: DockPanel.SetLastChildFill={fill:?}, SetHorizontalSpacing={spacing:?}"
            );
            format!("DockPanel OK ({fill:?}, {spacing:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: DockPanel::new failed: {error:?}");
            format!("DockPanel failed: {error:?}")
        }
    }
}

fn verify_equal_panel() -> String {
    eprintln!("xamltoolkit-labs: before EqualPanel::new");
    match EqualPanel::new() {
        Ok(panel) => {
            eprintln!("xamltoolkit-labs: EqualPanel::new OK");
            let spacing = panel.SetSpacing(8.0);
            eprintln!("xamltoolkit-labs: EqualPanel.SetSpacing returned {spacing:?}");
            format!("EqualPanel OK ({spacing:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: EqualPanel::new failed: {error:?}");
            format!("EqualPanel failed: {error:?}")
        }
    }
}

fn verify_uniform_grid() -> String {
    eprintln!("xamltoolkit-labs: before UniformGrid::new");
    match UniformGrid::new() {
        Ok(grid) => {
            eprintln!("xamltoolkit-labs: UniformGrid::new OK");
            let columns = grid.SetColumns(3);
            let rows = grid.SetRows(2);
            eprintln!("xamltoolkit-labs: UniformGrid.SetColumns={columns:?}, SetRows={rows:?}");
            format!("UniformGrid OK ({columns:?}, {rows:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: UniformGrid::new failed: {error:?}");
            format!("UniformGrid failed: {error:?}")
        }
    }
}

fn verify_staggered_panel() -> String {
    eprintln!("xamltoolkit-labs: before StaggeredPanel::new");
    match StaggeredPanel::new() {
        Ok(panel) => {
            eprintln!("xamltoolkit-labs: StaggeredPanel::new OK");
            let width = panel.SetDesiredColumnWidth(180.0);
            let column = panel.SetColumnSpacing(12.0);
            let row = panel.SetRowSpacing(10.0);
            eprintln!(
                "xamltoolkit-labs: StaggeredPanel.SetDesiredColumnWidth={width:?}, SetColumnSpacing={column:?}, SetRowSpacing={row:?}"
            );
            format!("StaggeredPanel OK ({width:?}, {column:?}, {row:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: StaggeredPanel::new failed: {error:?}");
            format!("StaggeredPanel failed: {error:?}")
        }
    }
}

fn verify_staggered_layout() -> String {
    eprintln!("xamltoolkit-labs: before StaggeredLayout::new");
    match StaggeredLayout::new() {
        Ok(layout) => {
            eprintln!("xamltoolkit-labs: StaggeredLayout::new OK");
            let stretch = layout.SetItemsStretch(StaggeredLayoutItemsStretch::Fill);
            let width = layout.SetDesiredColumnWidth(160.0);
            let column = layout.SetColumnSpacing(12.0);
            let row = layout.SetRowSpacing(10.0);
            eprintln!(
                "xamltoolkit-labs: StaggeredLayout.SetItemsStretch={stretch:?}, SetDesiredColumnWidth={width:?}, SetColumnSpacing={column:?}, SetRowSpacing={row:?}"
            );
            format!("StaggeredLayout OK ({stretch:?}, {width:?}, {column:?}, {row:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: StaggeredLayout::new failed: {error:?}");
            format!("StaggeredLayout failed: {error:?}")
        }
    }
}

fn verify_constrained_box() -> String {
    eprintln!("xamltoolkit-labs: before ConstrainedBox::new");
    match ConstrainedBox::new() {
        Ok(box_) => {
            eprintln!("xamltoolkit-labs: ConstrainedBox::new OK");
            let scale_x = box_.SetScaleX(2.0);
            let scale_y = box_.SetScaleY(1.0);
            eprintln!(
                "xamltoolkit-labs: ConstrainedBox.SetScaleX={scale_x:?}, SetScaleY={scale_y:?}"
            );
            format!("ConstrainedBox OK ({scale_x:?}, {scale_y:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: ConstrainedBox::new failed: {error:?}");
            format!("ConstrainedBox failed: {error:?}")
        }
    }
}

fn verify_case() -> String {
    eprintln!("xamltoolkit-labs: before Case::new");
    match Case::new() {
        Ok(case) => {
            eprintln!("xamltoolkit-labs: Case::new OK");
            let is_default = case.SetIsDefault(true);
            eprintln!("xamltoolkit-labs: Case.SetIsDefault returned {is_default:?}");
            format!("Case OK ({is_default:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: Case::new failed: {error:?}");
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
        "xamltoolkit-labs: ColorPicker converters Accent/GetAccent={accent:?}, ColorToHex={color_to_hex:?}, ContrastBrush={contrast:?}, NullToTransparent={null_to_transparent:?}"
    );
    format!(
        "ColorPickerConverters OK ({accent:?}, {color_to_hex:?}, {contrast:?}, {null_to_transparent:?})"
    )
}

fn verify_color_previewer() -> String {
    eprintln!("xamltoolkit-labs: before ColorPreviewer::new");
    match ColorPreviewer::new() {
        Ok(previewer) => {
            eprintln!("xamltoolkit-labs: ColorPreviewer::new OK");
            let hsv = previewer.SetHsvColor(HsvColor {
                H: 120.0,
                S: 0.6,
                V: 0.8,
                A: 1.0,
            });
            let accents = previewer.SetShowAccentColors(true);
            eprintln!(
                "xamltoolkit-labs: ColorPreviewer.SetHsvColor={hsv:?}, SetShowAccentColors={accents:?}"
            );
            format!("ColorPreviewer OK ({hsv:?}, {accents:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: ColorPreviewer::new failed: {error:?}");
            format!("ColorPreviewer failed: {error:?}")
        }
    }
}

fn verify_color_picker_slider() -> String {
    eprintln!("xamltoolkit-labs: before ColorPickerSlider::new");
    match ColorPickerSlider::new() {
        Ok(slider) => {
            eprintln!("xamltoolkit-labs: ColorPickerSlider::new OK");
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
                "xamltoolkit-labs: ColorPickerSlider.SetColorChannel={channel:?}, SetColorRepresentation={representation:?}, SetHsvColor={hsv:?}, SetIsAlphaMaxForced={alpha:?}, SetIsAutoUpdatingEnabled={auto:?}, SetIsSaturationValueMaxForced={saturation:?}, UpdateColors={update:?}"
            );
            format!(
                "ColorPickerSlider OK ({channel:?}, {representation:?}, {hsv:?}, {alpha:?}, {auto:?}, {saturation:?}, {update:?})"
            )
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: ColorPickerSlider::new failed: {error:?}");
            format!("ColorPickerSlider failed: {error:?}")
        }
    }
}

fn verify_color_picker() -> String {
    eprintln!("xamltoolkit-labs: before ColorPicker::new");
    match ColorPicker::new() {
        Ok(picker) => {
            eprintln!("xamltoolkit-labs: ColorPicker::new OK");
            let columns = picker.SetCustomPaletteColumnCount(6);
            let palette = picker.SetIsColorPaletteVisible(true);
            let accents = picker.SetShowAccentColors(true);
            eprintln!(
                "xamltoolkit-labs: ColorPicker.SetCustomPaletteColumnCount={columns:?}, SetIsColorPaletteVisible={palette:?}, SetShowAccentColors={accents:?}"
            );
            format!("ColorPicker OK ({columns:?}, {palette:?}, {accents:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: ColorPicker::new failed: {error:?}");
            format!("ColorPicker failed: {error:?}")
        }
    }
}

fn verify_color_picker_button() -> String {
    eprintln!("xamltoolkit-labs: before ColorPickerButton::new");
    match ColorPickerButton::new() {
        Ok(_) => {
            eprintln!("xamltoolkit-labs: ColorPickerButton::new OK");
            "ColorPickerButton OK".to_string()
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: ColorPickerButton::new failed: {error:?}");
            format!("ColorPickerButton failed: {error:?}")
        }
    }
}

fn verify_radial_gauge() -> String {
    eprintln!("xamltoolkit-labs: before RadialGauge::new");
    match RadialGauge::new() {
        Ok(gauge) => {
            eprintln!("xamltoolkit-labs: RadialGauge::new OK");
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
                "xamltoolkit-labs: RadialGauge.SetMinimum={minimum:?}, SetMaximum={maximum:?}, SetIsInteractive={interactive:?}, SetScaleWidth={scale_width:?}, SetStepSize={step:?}, SetUnit={unit:?}, SetValueStringFormat={format:?}, SetMinAngle={min_angle:?}, SetMaxAngle={max_angle:?}, SetValueAngle={value_angle:?}"
            );
            format!(
                "RadialGauge OK ({minimum:?}, {maximum:?}, {interactive:?}, {scale_width:?}, {step:?}, {unit:?}, {format:?}, {min_angle:?}, {max_angle:?}, {value_angle:?})"
            )
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: RadialGauge::new failed: {error:?}");
            format!("RadialGauge failed: {error:?}")
        }
    }
}

fn verify_radial_gauge_automation_peer() -> String {
    eprintln!("xamltoolkit-labs: before RadialGaugeAutomationPeer::CreateInstance");
    match RadialGauge::new().and_then(|owner| RadialGaugeAutomationPeer::CreateInstance(&owner)) {
        Ok(peer) => {
            eprintln!("xamltoolkit-labs: RadialGaugeAutomationPeer::CreateInstance OK");
            let raise = peer.RaiseValueChangedEvent(10.0, 20.0);
            eprintln!(
                "xamltoolkit-labs: RadialGaugeAutomationPeer.RaiseValueChangedEvent={raise:?}"
            );
            format!("RadialGaugeAutomationPeer OK ({raise:?})")
        }
        Err(error) => {
            eprintln!(
                "xamltoolkit-labs: RadialGaugeAutomationPeer::CreateInstance failed: {error:?}"
            );
            format!("RadialGaugeAutomationPeer failed: {error:?}")
        }
    }
}

fn verify_layout_transform_control() -> String {
    eprintln!("xamltoolkit-labs: before LayoutTransformControl::new");
    match LayoutTransformControl::new() {
        Ok(control) => {
            eprintln!("xamltoolkit-labs: LayoutTransformControl::new OK");
            let enabled = control.SetIsEnabled(true);
            let width = control.SetWidth(240.0);
            eprintln!(
                "xamltoolkit-labs: LayoutTransformControl.SetIsEnabled={enabled:?}, SetWidth={width:?}"
            );
            format!("LayoutTransformControl OK ({enabled:?}, {width:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: LayoutTransformControl::new failed: {error:?}");
            format!("LayoutTransformControl failed: {error:?}")
        }
    }
}

fn verify_image_cropper() -> String {
    eprintln!("xamltoolkit-labs: before ImageCropper::new");
    match ImageCropper::new() {
        Ok(cropper) => {
            eprintln!("xamltoolkit-labs: ImageCropper::new OK");
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
                "xamltoolkit-labs: ImageCropper.SetMinCroppedPixelLength={min_crop:?}, SetMinSelectedLength={min_select:?}, SetCropShape={shape:?}, SetThumbPlacement={placement:?}, TrySetCroppedRegion={region:?}"
            );
            format!(
                "ImageCropper OK ({min_crop:?}, {min_select:?}, {shape:?}, {placement:?}, {region:?})"
            )
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: ImageCropper::new failed: {error:?}");
            format!("ImageCropper failed: {error:?}")
        }
    }
}

fn verify_image_cropper_thumb() -> String {
    eprintln!("xamltoolkit-labs: before ImageCropperThumb::new");
    match ImageCropperThumb::new() {
        Ok(thumb) => {
            eprintln!("xamltoolkit-labs: ImageCropperThumb::new OK");
            let x = thumb.SetX(12.0);
            let y = thumb.SetY(16.0);
            let position = thumb.SetPosition(ThumbPosition::UpperLeft);
            eprintln!(
                "xamltoolkit-labs: ImageCropperThumb.SetX={x:?}, SetY={y:?}, SetPosition={position:?}"
            );
            format!("ImageCropperThumb OK ({x:?}, {y:?}, {position:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: ImageCropperThumb::new failed: {error:?}");
            format!("ImageCropperThumb failed: {error:?}")
        }
    }
}
fn verify_camera_preview() -> String {
    eprintln!("xamltoolkit-labs: before CameraPreview::new");
    match CameraPreview::new() {
        Ok(preview) => {
            eprintln!("xamltoolkit-labs: CameraPreview::new OK");
            let visible = preview.SetIsFrameSourceGroupButtonVisible(false);
            let property = CameraPreview::IsFrameSourceGroupButtonVisibleProperty().map(|_| ());
            let stop = preview.Stop();
            eprintln!(
                "xamltoolkit-labs: CameraPreview.SetIsFrameSourceGroupButtonVisible={visible:?}, IsFrameSourceGroupButtonVisibleProperty={property:?}, Stop={stop:?}"
            );
            format!("CameraPreview OK ({visible:?}, {property:?}, {stop:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: CameraPreview::new failed: {error:?}");
            format!("CameraPreview failed: {error:?}")
        }
    }
}

fn verify_preview_failed_event_args() -> String {
    eprintln!("xamltoolkit-labs: before PreviewFailedEventArgs::CreateInstance");
    match PreviewFailedEventArgs::CreateInstance(&HSTRING::from("camera unavailable")) {
        Ok(args) => {
            eprintln!("xamltoolkit-labs: PreviewFailedEventArgs::CreateInstance OK");
            let error = args.Error().map(|value| format!("{value:?}"));
            eprintln!("xamltoolkit-labs: PreviewFailedEventArgs.Error={error:?}");
            format!("PreviewFailedEventArgs OK ({error:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: PreviewFailedEventArgs::CreateInstance failed: {error:?}");
            format!("PreviewFailedEventArgs failed: {error:?}")
        }
    }
}
fn verify_metadata_control() -> String {
    eprintln!("xamltoolkit-labs: before MetadataControl::new");
    match MetadataControl::new() {
        Ok(control) => {
            eprintln!("xamltoolkit-labs: MetadataControl::new OK");
            let separator = control.SetSeparator(&HSTRING::from("/"));
            let accessible_separator = control.SetAccessibleSeparator(&HSTRING::from("slash"));
            eprintln!(
                "xamltoolkit-labs: MetadataControl.SetSeparator={separator:?}, SetAccessibleSeparator={accessible_separator:?}"
            );
            format!("MetadataControl OK ({separator:?}, {accessible_separator:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: MetadataControl::new failed: {error:?}");
            format!("MetadataControl failed: {error:?}")
        }
    }
}

fn verify_metadata_item() -> String {
    eprintln!("xamltoolkit-labs: before MetadataItem::new");
    match MetadataItem::new() {
        Ok(_) => {
            eprintln!("xamltoolkit-labs: MetadataItem::new OK");
            "MetadataItem OK".to_string()
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: MetadataItem::new failed: {error:?}");
            format!("MetadataItem failed: {error:?}")
        }
    }
}

fn verify_property_sizer() -> String {
    eprintln!("xamltoolkit-labs: before PropertySizer::new");
    match PropertySizer::new() {
        Ok(sizer) => {
            eprintln!("xamltoolkit-labs: PropertySizer::new OK");
            let binding = sizer.SetBinding(120.0);
            let minimum = sizer.SetMinimum(40.0);
            let maximum = sizer.SetMaximum(360.0);
            let orientation = sizer.SetOrientation(Orientation::Horizontal);
            let drag = sizer.SetDragIncrement(4.0);
            eprintln!(
                "xamltoolkit-labs: PropertySizer.SetBinding={binding:?}, SetMinimum={minimum:?}, SetMaximum={maximum:?}, SetOrientation={orientation:?}, SetDragIncrement={drag:?}"
            );
            format!(
                "PropertySizer OK ({binding:?}, {minimum:?}, {maximum:?}, {orientation:?}, {drag:?})"
            )
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: PropertySizer::new failed: {error:?}");
            format!("PropertySizer failed: {error:?}")
        }
    }
}

fn verify_sizer_automation_peer() -> String {
    eprintln!("xamltoolkit-labs: before SizerAutomationPeer::CreateInstance");
    match PropertySizer::new().and_then(|owner| SizerAutomationPeer::CreateInstance(&owner)) {
        Ok(_) => {
            eprintln!("xamltoolkit-labs: SizerAutomationPeer::CreateInstance OK");
            "SizerAutomationPeer OK".to_string()
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: SizerAutomationPeer::CreateInstance failed: {error:?}");
            format!("SizerAutomationPeer failed: {error:?}")
        }
    }
}

fn verify_content_sizer() -> String {
    eprintln!("xamltoolkit-labs: before ContentSizer::new");
    match ContentSizer::new() {
        Ok(sizer) => {
            eprintln!("xamltoolkit-labs: ContentSizer::new OK");
            let inverted = sizer.SetIsDragInverted(false);
            let keyboard = sizer.SetKeyboardIncrement(8.0);
            let thumb = sizer.SetIsThumbVisible(true);
            eprintln!(
                "xamltoolkit-labs: ContentSizer.SetIsDragInverted={inverted:?}, SetKeyboardIncrement={keyboard:?}, SetIsThumbVisible={thumb:?}"
            );
            format!("ContentSizer OK ({inverted:?}, {keyboard:?}, {thumb:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: ContentSizer::new failed: {error:?}");
            format!("ContentSizer failed: {error:?}")
        }
    }
}

fn verify_grid_splitter() -> String {
    eprintln!("xamltoolkit-labs: before GridSplitter::new");
    match GridSplitter::new() {
        Ok(splitter) => {
            eprintln!("xamltoolkit-labs: GridSplitter::new OK");
            let direction = splitter.SetResizeDirection(GridResizeDirection::Columns);
            let behavior = splitter.SetResizeBehavior(GridResizeBehavior::CurrentAndNext);
            let parent = splitter.SetParentLevel(1);
            let orientation = splitter.SetOrientation(Orientation::Vertical);
            eprintln!(
                "xamltoolkit-labs: GridSplitter.SetResizeDirection={direction:?}, SetResizeBehavior={behavior:?}, SetParentLevel={parent:?}, SetOrientation={orientation:?}"
            );
            format!("GridSplitter OK ({direction:?}, {behavior:?}, {parent:?}, {orientation:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: GridSplitter::new failed: {error:?}");
            format!("GridSplitter failed: {error:?}")
        }
    }
}

fn verify_headered_content_control() -> String {
    eprintln!("xamltoolkit-labs: before HeaderedContentControl::new");
    match HeaderedContentControl::new() {
        Ok(control) => {
            eprintln!("xamltoolkit-labs: HeaderedContentControl::new OK");
            let header = PropertyValue::CreateString(&HSTRING::from("Headered content"))
                .and_then(|value| control.SetHeader(&value));
            let orientation = control.SetOrientation(Orientation::Horizontal);
            eprintln!(
                "xamltoolkit-labs: HeaderedContentControl.SetHeader={header:?}, SetOrientation={orientation:?}"
            );
            format!("HeaderedContentControl OK ({header:?}, {orientation:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: HeaderedContentControl::new failed: {error:?}");
            format!("HeaderedContentControl failed: {error:?}")
        }
    }
}

fn verify_headered_items_control() -> String {
    eprintln!("xamltoolkit-labs: before HeaderedItemsControl::new");
    match HeaderedItemsControl::new() {
        Ok(control) => {
            eprintln!("xamltoolkit-labs: HeaderedItemsControl::new OK");
            let header = PropertyValue::CreateString(&HSTRING::from("Headered items"))
                .and_then(|value| control.SetHeader(&value));
            let footer = PropertyValue::CreateString(&HSTRING::from("Items footer"))
                .and_then(|value| control.SetFooter(&value));
            eprintln!(
                "xamltoolkit-labs: HeaderedItemsControl.SetHeader={header:?}, SetFooter={footer:?}"
            );
            format!("HeaderedItemsControl OK ({header:?}, {footer:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: HeaderedItemsControl::new failed: {error:?}");
            format!("HeaderedItemsControl failed: {error:?}")
        }
    }
}

fn verify_headered_tree_view() -> String {
    eprintln!("xamltoolkit-labs: before HeaderedTreeView::new");
    match HeaderedTreeView::new() {
        Ok(control) => {
            eprintln!("xamltoolkit-labs: HeaderedTreeView::new OK");
            let header = PropertyValue::CreateString(&HSTRING::from("Headered tree"))
                .and_then(|value| control.SetHeader(&value));
            let footer = PropertyValue::CreateString(&HSTRING::from("Tree footer"))
                .and_then(|value| control.SetFooter(&value));
            eprintln!(
                "xamltoolkit-labs: HeaderedTreeView.SetHeader={header:?}, SetFooter={footer:?}"
            );
            format!("HeaderedTreeView OK ({header:?}, {footer:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: HeaderedTreeView::new failed: {error:?}");
            format!("HeaderedTreeView failed: {error:?}")
        }
    }
}

fn verify_segmented() -> String {
    eprintln!("xamltoolkit-labs: before Segmented::new");
    match Segmented::new() {
        Ok(control) => {
            eprintln!("xamltoolkit-labs: Segmented::new OK");
            let orientation = control.SetOrientation(Orientation::Horizontal);
            eprintln!("xamltoolkit-labs: Segmented.SetOrientation={orientation:?}");
            format!("Segmented OK ({orientation:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: Segmented::new failed: {error:?}");
            format!("Segmented failed: {error:?}")
        }
    }
}

fn verify_segmented_item() -> String {
    eprintln!("xamltoolkit-labs: before SegmentedItem::new");
    match SegmentedItem::new() {
        Ok(_) => {
            eprintln!("xamltoolkit-labs: SegmentedItem::new OK");
            "SegmentedItem OK".to_string()
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: SegmentedItem::new failed: {error:?}");
            format!("SegmentedItem failed: {error:?}")
        }
    }
}

fn verify_segmented_margin_converter() -> String {
    eprintln!("xamltoolkit-labs: before SegmentedMarginConverter::new");
    match SegmentedMarginConverter::new() {
        Ok(converter) => {
            eprintln!("xamltoolkit-labs: SegmentedMarginConverter::new OK");
            let left = converter.SetLeftItemMargin(thickness(0.0, 0.0, 4.0, 0.0));
            let middle = converter.SetMiddleItemMargin(thickness(4.0, 0.0, 4.0, 0.0));
            let right = converter.SetRightItemMargin(thickness(4.0, 0.0, 0.0, 0.0));
            eprintln!(
                "xamltoolkit-labs: SegmentedMarginConverter.SetLeftItemMargin={left:?}, SetMiddleItemMargin={middle:?}, SetRightItemMargin={right:?}"
            );
            format!("SegmentedMarginConverter OK ({left:?}, {middle:?}, {right:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: SegmentedMarginConverter::new failed: {error:?}");
            format!("SegmentedMarginConverter failed: {error:?}")
        }
    }
}

fn verify_settings_card() -> String {
    eprintln!("xamltoolkit-labs: before SettingsCard::new");
    match SettingsCard::new() {
        Ok(card) => {
            eprintln!("xamltoolkit-labs: SettingsCard::new OK");
            let header =
                boxed_string("Settings card header").and_then(|value| card.SetHeader(&value));
            let description = boxed_string("Settings card description")
                .and_then(|value| card.SetDescription(&value));
            let tooltip = card.SetActionIconToolTip(&HSTRING::from("Open settings"));
            let click = card.SetIsClickEnabled(true);
            let visible = card.SetIsActionIconVisible(true);
            let alignment = card.SetContentAlignment(ContentAlignment::Vertical);
            eprintln!(
                "xamltoolkit-labs: SettingsCard.SetHeader={header:?}, SetDescription={description:?}, SetActionIconToolTip={tooltip:?}, SetIsClickEnabled={click:?}, SetIsActionIconVisible={visible:?}, SetContentAlignment={alignment:?}"
            );
            format!(
                "SettingsCard OK ({header:?}, {description:?}, {tooltip:?}, {click:?}, {visible:?}, {alignment:?})"
            )
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: SettingsCard::new failed: {error:?}");
            format!("SettingsCard failed: {error:?}")
        }
    }
}

fn verify_settings_card_automation_peer() -> String {
    eprintln!("xamltoolkit-labs: before SettingsCardAutomationPeer::CreateInstance");
    match SettingsCard::new().and_then(|owner| SettingsCardAutomationPeer::CreateInstance(&owner)) {
        Ok(_) => {
            eprintln!("xamltoolkit-labs: SettingsCardAutomationPeer::CreateInstance OK");
            "SettingsCardAutomationPeer OK".to_string()
        }
        Err(error) => {
            eprintln!(
                "xamltoolkit-labs: SettingsCardAutomationPeer::CreateInstance failed: {error:?}"
            );
            format!("SettingsCardAutomationPeer failed: {error:?}")
        }
    }
}

fn verify_settings_expander() -> String {
    eprintln!("xamltoolkit-labs: before SettingsExpander::new");
    match SettingsExpander::new() {
        Ok(expander) => {
            eprintln!("xamltoolkit-labs: SettingsExpander::new OK");
            let content = boxed_string("Settings expander content")
                .and_then(|value| expander.SetContent(&value));
            let header = boxed_string("Settings expander header")
                .and_then(|value| expander.SetHeader(&value));
            let description = boxed_string("Settings expander description")
                .and_then(|value| expander.SetDescription(&value));
            let expanded = expander.SetIsExpanded(true);
            eprintln!(
                "xamltoolkit-labs: SettingsExpander.SetContent={content:?}, SetHeader={header:?}, SetDescription={description:?}, SetIsExpanded={expanded:?}"
            );
            format!("SettingsExpander OK ({content:?}, {header:?}, {description:?}, {expanded:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: SettingsExpander::new failed: {error:?}");
            format!("SettingsExpander failed: {error:?}")
        }
    }
}

fn verify_settings_expander_automation_peer() -> String {
    eprintln!("xamltoolkit-labs: before SettingsExpanderAutomationPeer::CreateInstance");
    match SettingsExpander::new()
        .and_then(|owner| SettingsExpanderAutomationPeer::CreateInstance(&owner))
    {
        Ok(peer) => {
            eprintln!("xamltoolkit-labs: SettingsExpanderAutomationPeer::CreateInstance OK");
            let raise = peer.RaiseExpandedChangedEvent(true);
            eprintln!(
                "xamltoolkit-labs: SettingsExpanderAutomationPeer.RaiseExpandedChangedEvent={raise:?}"
            );
            format!("SettingsExpanderAutomationPeer OK ({raise:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: SettingsExpanderAutomationPeer::CreateInstance failed: {error:?}");
            format!("SettingsExpanderAutomationPeer failed: {error:?}")
        }
    }
}

fn verify_settings_expander_item_style_selector() -> String {
    eprintln!("xamltoolkit-labs: before SettingsExpanderItemStyleSelector::new");
    match SettingsExpanderItemStyleSelector::new() {
        Ok(_) => {
            eprintln!("xamltoolkit-labs: SettingsExpanderItemStyleSelector::new OK");
            "SettingsExpanderItemStyleSelector OK".to_string()
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: SettingsExpanderItemStyleSelector::new failed: {error:?}");
            format!("SettingsExpanderItemStyleSelector failed: {error:?}")
        }
    }
}

fn verify_style_extensions() -> String {
    eprintln!("xamltoolkit-labs: before StyleExtensions statics");
    match (Style::new(), StyleExtensionResourceDictionary::new()) {
        (Ok(style), Ok(resources)) => {
            eprintln!("xamltoolkit-labs: Style and StyleExtensionResourceDictionary activation OK");
            let set = StyleExtensions::SetResources(&style, &resources);
            let get = StyleExtensions::GetResources(&style).map(|_| ());
            eprintln!(
                "xamltoolkit-labs: StyleExtensions.SetResources={set:?}, GetResources={get:?}"
            );
            format!("StyleExtensions OK ({set:?}, {get:?})")
        }
        (Err(error), _) => {
            eprintln!("xamltoolkit-labs: Style::new failed: {error:?}");
            format!("StyleExtensions Style failed: {error:?}")
        }
        (_, Err(error)) => {
            eprintln!("xamltoolkit-labs: StyleExtensionResourceDictionary::new failed: {error:?}");
            format!("StyleExtensionResourceDictionary failed: {error:?}")
        }
    }
}

fn verify_corner_radius_converter() -> String {
    eprintln!("xamltoolkit-labs: before CornerRadiusConverter::new");
    match CornerRadiusConverter::new() {
        Ok(_) => {
            eprintln!("xamltoolkit-labs: CornerRadiusConverter::new OK");
            "CornerRadiusConverter OK".to_string()
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: CornerRadiusConverter::new failed: {error:?}");
            format!("CornerRadiusConverter failed: {error:?}")
        }
    }
}

fn thickness(left: f64, top: f64, right: f64, bottom: f64) -> Thickness {
    Thickness {
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
    eprintln!("xamltoolkit-labs: before RangeSelector::new");
    match RangeSelector::new() {
        Ok(selector) => {
            eprintln!("xamltoolkit-labs: RangeSelector::new OK");
            let minimum = selector.SetMinimum(0.0);
            let maximum = selector.SetMaximum(100.0);
            let start = selector.SetRangeStart(20.0);
            let end = selector.SetRangeEnd(80.0);
            let step = selector.SetStepFrequency(5.0);
            eprintln!(
                "xamltoolkit-labs: RangeSelector.SetMinimum={minimum:?}, SetMaximum={maximum:?}, SetRangeStart={start:?}, SetRangeEnd={end:?}, SetStepFrequency={step:?}"
            );
            format!("RangeSelector OK ({minimum:?}, {maximum:?}, {start:?}, {end:?}, {step:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: RangeSelector::new failed: {error:?}");
            format!("RangeSelector failed: {error:?}")
        }
    }
}

fn verify_case_collection() -> String {
    eprintln!("xamltoolkit-labs: before CaseCollection::new");
    match CaseCollection::new() {
        Ok(_) => {
            eprintln!("xamltoolkit-labs: CaseCollection::new OK");
            "CaseCollection OK".to_string()
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: CaseCollection::new failed: {error:?}");
            format!("CaseCollection failed: {error:?}")
        }
    }
}

fn verify_switch_converter() -> String {
    eprintln!("xamltoolkit-labs: before SwitchConverter::new");
    match (SwitchConverter::new(), CaseCollection::new()) {
        (Ok(converter), Ok(cases)) => {
            eprintln!("xamltoolkit-labs: SwitchConverter::new OK");
            let switch_cases = converter.SetSwitchCases(&cases);
            eprintln!("xamltoolkit-labs: SwitchConverter.SetSwitchCases={switch_cases:?}");
            format!("SwitchConverter OK ({switch_cases:?})")
        }
        (Err(error), _) => {
            eprintln!("xamltoolkit-labs: SwitchConverter::new failed: {error:?}");
            format!("SwitchConverter failed: {error:?}")
        }
        (_, Err(error)) => {
            eprintln!("xamltoolkit-labs: SwitchConverter CaseCollection::new failed: {error:?}");
            format!("SwitchConverter cases failed: {error:?}")
        }
    }
}

fn verify_switch_presenter() -> String {
    eprintln!("xamltoolkit-labs: before SwitchPresenter::new");
    match (SwitchPresenter::new(), CaseCollection::new()) {
        (Ok(presenter), Ok(cases)) => {
            eprintln!("xamltoolkit-labs: SwitchPresenter::new OK");
            let switch_cases = presenter.SetSwitchCases(&cases);
            eprintln!("xamltoolkit-labs: SwitchPresenter.SetSwitchCases={switch_cases:?}");
            format!("SwitchPresenter OK ({switch_cases:?})")
        }
        (Err(error), _) => {
            eprintln!("xamltoolkit-labs: SwitchPresenter::new failed: {error:?}");
            format!("SwitchPresenter failed: {error:?}")
        }
        (_, Err(error)) => {
            eprintln!("xamltoolkit-labs: SwitchPresenter CaseCollection::new failed: {error:?}");
            format!("SwitchPresenter cases failed: {error:?}")
        }
    }
}

fn verify_tabbed_command_bar() -> String {
    eprintln!("xamltoolkit-labs: before TabbedCommandBar::new");
    match TabbedCommandBar::new() {
        Ok(_) => {
            eprintln!("xamltoolkit-labs: TabbedCommandBar::new OK");
            "TabbedCommandBar OK".to_string()
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: TabbedCommandBar::new failed: {error:?}");
            format!("TabbedCommandBar failed: {error:?}")
        }
    }
}

fn verify_tabbed_command_bar_item() -> String {
    eprintln!("xamltoolkit-labs: before TabbedCommandBarItem::new");
    match TabbedCommandBarItem::new() {
        Ok(item) => {
            eprintln!("xamltoolkit-labs: TabbedCommandBarItem::new OK");
            let header = boxed_string("Tabbed command").and_then(|value| item.SetHeader(&value));
            let contextual = item.SetIsContextual(true);
            let overflow = item.SetOverflowButtonAlignment(HorizontalAlignment::Right);
            let command = item.SetCommandAlignment(HorizontalAlignment::Stretch);
            eprintln!(
                "xamltoolkit-labs: TabbedCommandBarItem.SetHeader={header:?}, SetIsContextual={contextual:?}, SetOverflowButtonAlignment={overflow:?}, SetCommandAlignment={command:?}"
            );
            format!(
                "TabbedCommandBarItem OK ({header:?}, {contextual:?}, {overflow:?}, {command:?})"
            )
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: TabbedCommandBarItem::new failed: {error:?}");
            format!("TabbedCommandBarItem failed: {error:?}")
        }
    }
}

fn verify_tabbed_command_bar_item_template_selector() -> String {
    eprintln!("xamltoolkit-labs: before TabbedCommandBarItemTemplateSelector::new");
    match (
        TabbedCommandBarItemTemplateSelector::new(),
        DataTemplate::new(),
        DataTemplate::new(),
    ) {
        (Ok(selector), Ok(normal), Ok(contextual_template)) => {
            eprintln!("xamltoolkit-labs: TabbedCommandBarItemTemplateSelector::new OK");
            let normal = selector.SetNormal(&normal);
            let contextual = selector.SetContextual(&contextual_template);
            eprintln!(
                "xamltoolkit-labs: TabbedCommandBarItemTemplateSelector.SetNormal={normal:?}, SetContextual={contextual:?}"
            );
            format!("TabbedCommandBarItemTemplateSelector OK ({normal:?}, {contextual:?})")
        }
        (Err(error), _, _) => {
            eprintln!(
                "xamltoolkit-labs: TabbedCommandBarItemTemplateSelector::new failed: {error:?}"
            );
            format!("TabbedCommandBarItemTemplateSelector failed: {error:?}")
        }
        (_, Err(error), _) | (_, _, Err(error)) => {
            eprintln!("xamltoolkit-labs: DataTemplate::new failed: {error:?}");
            format!("TabbedCommandBarItemTemplateSelector DataTemplate failed: {error:?}")
        }
    }
}

fn verify_tokenizing_text_box() -> String {
    eprintln!("xamltoolkit-labs: before TokenizingTextBox::new");
    match TokenizingTextBox::new() {
        Ok(text_box) => {
            eprintln!("xamltoolkit-labs: TokenizingTextBox::new OK");
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
                "xamltoolkit-labs: TokenizingTextBox.SetTextMemberPath={member:?}, SetTokenDelimiter={delimiter:?}, SetTokenSpacing={spacing:?}, SetPlaceholderText={placeholder:?}, SetText={text:?}, SetSuggestedItemsSource={suggested:?}, SetTabNavigateBackOnArrow={tab:?}, SetMaximumTokens={maximum:?}"
            );
            format!(
                "TokenizingTextBox OK ({member:?}, {delimiter:?}, {spacing:?}, {placeholder:?}, {text:?}, {suggested:?}, {tab:?}, {maximum:?})"
            )
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: TokenizingTextBox::new failed: {error:?}");
            format!("TokenizingTextBox failed: {error:?}")
        }
    }
}

fn verify_tokenizing_text_box_item() -> String {
    eprintln!("xamltoolkit-labs: before TokenizingTextBoxItem::new");
    match (
        TokenizingTextBoxItem::new(),
        TokenizingTextBox::new(),
        Style::new(),
    ) {
        (Ok(item), Ok(owner), Ok(style)) => {
            eprintln!("xamltoolkit-labs: TokenizingTextBoxItem::new OK");
            let owner = item.SetOwner(&owner);
            let clear = item.SetClearButtonStyle(&style);
            eprintln!(
                "xamltoolkit-labs: TokenizingTextBoxItem.SetOwner={owner:?}, SetClearButtonStyle={clear:?}"
            );
            format!("TokenizingTextBoxItem OK ({owner:?}, {clear:?})")
        }
        (Err(error), _, _) => {
            eprintln!("xamltoolkit-labs: TokenizingTextBoxItem::new failed: {error:?}");
            format!("TokenizingTextBoxItem failed: {error:?}")
        }
        (_, Err(error), _) => {
            eprintln!("xamltoolkit-labs: TokenizingTextBoxItem owner TokenizingTextBox::new failed: {error:?}");
            format!("TokenizingTextBoxItem owner failed: {error:?}")
        }
        (_, _, Err(error)) => {
            eprintln!("xamltoolkit-labs: TokenizingTextBoxItem Style::new failed: {error:?}");
            format!("TokenizingTextBoxItem style failed: {error:?}")
        }
    }
}

fn verify_tokenizing_text_box_style_selector() -> String {
    eprintln!("xamltoolkit-labs: before TokenizingTextBoxStyleSelector::new");
    match (
        TokenizingTextBoxStyleSelector::new(),
        Style::new(),
        Style::new(),
    ) {
        (Ok(selector), Ok(token_style), Ok(text_style)) => {
            eprintln!("xamltoolkit-labs: TokenizingTextBoxStyleSelector::new OK");
            let token = selector.SetTokenStyle(&token_style);
            let text = selector.SetTextStyle(&text_style);
            eprintln!(
                "xamltoolkit-labs: TokenizingTextBoxStyleSelector.SetTokenStyle={token:?}, SetTextStyle={text:?}"
            );
            format!("TokenizingTextBoxStyleSelector OK ({token:?}, {text:?})")
        }
        (Err(error), _, _) => {
            eprintln!("xamltoolkit-labs: TokenizingTextBoxStyleSelector::new failed: {error:?}");
            format!("TokenizingTextBoxStyleSelector failed: {error:?}")
        }
        (_, Err(error), _) | (_, _, Err(error)) => {
            eprintln!(
                "xamltoolkit-labs: TokenizingTextBoxStyleSelector Style::new failed: {error:?}"
            );
            format!("TokenizingTextBoxStyleSelector style failed: {error:?}")
        }
    }
}

fn verify_token_item_adding_event_args() -> String {
    eprintln!("xamltoolkit-labs: before TokenItemAddingEventArgs::CreateInstance");
    match TokenItemAddingEventArgs::CreateInstance(&HSTRING::from("alpha")) {
        Ok(args) => {
            eprintln!("xamltoolkit-labs: TokenItemAddingEventArgs::CreateInstance OK");
            let token = args.SetTokenText(&HSTRING::from("beta"));
            let item = boxed_string("token item").and_then(|value| args.SetItem(&value));
            let cancel = args.SetCancel(false);
            eprintln!(
                "xamltoolkit-labs: TokenItemAddingEventArgs.SetTokenText={token:?}, SetItem={item:?}, SetCancel={cancel:?}"
            );
            format!("TokenItemAddingEventArgs OK ({token:?}, {item:?}, {cancel:?})")
        }
        Err(error) => {
            eprintln!(
                "xamltoolkit-labs: TokenItemAddingEventArgs::CreateInstance failed: {error:?}"
            );
            format!("TokenItemAddingEventArgs failed: {error:?}")
        }
    }
}

fn verify_token_item_removing_event_args() -> String {
    eprintln!("xamltoolkit-labs: before TokenItemRemovingEventArgs::CreateInstance");
    match (boxed_string("token item"), TokenizingTextBoxItem::new()) {
        (Ok(item), Ok(token)) => match TokenItemRemovingEventArgs::CreateInstance(&item, &token) {
            Ok(args) => {
                eprintln!("xamltoolkit-labs: TokenItemRemovingEventArgs::CreateInstance OK");
                let cancel = args.SetCancel(false);
                eprintln!("xamltoolkit-labs: TokenItemRemovingEventArgs.SetCancel={cancel:?}");
                format!("TokenItemRemovingEventArgs OK ({cancel:?})")
            }
            Err(error) => {
                eprintln!("xamltoolkit-labs: TokenItemRemovingEventArgs::CreateInstance failed: {error:?}");
                format!("TokenItemRemovingEventArgs failed: {error:?}")
            }
        },
        (Err(error), _) => {
            eprintln!("xamltoolkit-labs: TokenItemRemovingEventArgs boxed item failed: {error:?}");
            format!("TokenItemRemovingEventArgs item failed: {error:?}")
        }
        (_, Err(error)) => {
            eprintln!("xamltoolkit-labs: TokenItemRemovingEventArgs TokenizingTextBoxItem::new failed: {error:?}");
            format!("TokenItemRemovingEventArgs token failed: {error:?}")
        }
    }
}

fn verify_tokenizing_text_box_automation_peer() -> String {
    eprintln!("xamltoolkit-labs: before TokenizingTextBoxAutomationPeer::CreateInstance");
    match TokenizingTextBox::new()
        .and_then(|owner| TokenizingTextBoxAutomationPeer::CreateInstance(&owner))
    {
        Ok(_) => {
            eprintln!("xamltoolkit-labs: TokenizingTextBoxAutomationPeer::CreateInstance OK");
            "TokenizingTextBoxAutomationPeer OK".to_string()
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: TokenizingTextBoxAutomationPeer::CreateInstance failed: {error:?}");
            format!("TokenizingTextBoxAutomationPeer failed: {error:?}")
        }
    }
}

fn verify_rich_suggest_box() -> String {
    eprintln!("xamltoolkit-labs: before RichSuggestBox::new");
    match RichSuggestBox::new() {
        Ok(_) => {
            eprintln!("xamltoolkit-labs: RichSuggestBox::new OK");
            "RichSuggestBox activation OK".to_string()
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: RichSuggestBox::new failed: {error:?}");
            format!("RichSuggestBox failed: {error:?}")
        }
    }
}

fn verify_rich_suggest_token() -> String {
    eprintln!("xamltoolkit-labs: before RichSuggestToken::CreateInstance");
    match RichSuggestToken::CreateInstance(
        GUID::from_u128(0x12345678_1234_5678_9abc_def012345678),
        &HSTRING::from("Alice"),
    ) {
        Ok(token) => {
            eprintln!("xamltoolkit-labs: RichSuggestToken::CreateInstance OK");
            let item = boxed_string("Alice item").and_then(|value| token.SetItem(&value));
            let active = token.SetActive(true);
            let string = token.ToString().map(|_| ());
            eprintln!(
                "xamltoolkit-labs: RichSuggestToken.SetItem={item:?}, SetActive={active:?}, ToString={string:?}"
            );
            format!("RichSuggestToken OK ({item:?}, {active:?}, {string:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: RichSuggestToken::CreateInstance failed: {error:?}");
            format!("RichSuggestToken failed: {error:?}")
        }
    }
}

fn verify_suggestion_requested_event_args() -> String {
    eprintln!("xamltoolkit-labs: before SuggestionRequestedEventArgs::new");
    match SuggestionRequestedEventArgs::new() {
        Ok(args) => {
            eprintln!("xamltoolkit-labs: SuggestionRequestedEventArgs::new OK");
            let prefix = args.SetPrefix(&HSTRING::from("@"));
            let query = args.SetQueryText(&HSTRING::from("ali"));
            eprintln!(
                "xamltoolkit-labs: SuggestionRequestedEventArgs.SetPrefix={prefix:?}, SetQueryText={query:?}"
            );
            format!("SuggestionRequestedEventArgs OK ({prefix:?}, {query:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: SuggestionRequestedEventArgs::new failed: {error:?}");
            format!("SuggestionRequestedEventArgs failed: {error:?}")
        }
    }
}

fn verify_suggestion_chosen_event_args() -> String {
    eprintln!("xamltoolkit-labs: before SuggestionChosenEventArgs::new");
    match SuggestionChosenEventArgs::new() {
        Ok(args) => {
            eprintln!("xamltoolkit-labs: SuggestionChosenEventArgs::new OK");
            let display = args.SetDisplayText(&HSTRING::from("Alice"));
            eprintln!("xamltoolkit-labs: SuggestionChosenEventArgs.SetDisplayText={display:?}");
            format!("SuggestionChosenEventArgs OK ({display:?})")
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: SuggestionChosenEventArgs::new failed: {error:?}");
            format!("SuggestionChosenEventArgs failed: {error:?}")
        }
    }
}

fn verify_helpers() -> String {
    format!(
        "{}; {}",
        verify_design_time_helpers(),
        verify_color_helper()
    )
}

fn verify_color_helper() -> String {
    eprintln!("xamltoolkit-labs: before ColorHelper statics");
    match ColorHelper::ToColor(&HSTRING::from("#336699")) {
        Ok(color) => {
            let hex = ColorHelper::ToHex(color).map(|value| format!("{value:?}"));
            let hsl = ColorHelper::ToHsl(color);
            let hsv = ColorHelper::ToHsv(color);
            let from_hsl = ColorHelper::FromHsl(210.0, 0.5, 0.4, 1.0);
            let from_hsv = ColorHelper::FromHsv(210.0, 0.66, 0.6, 1.0);
            eprintln!(
                "xamltoolkit-labs: ColorHelper color={color:?}, ToHex={hex:?}, ToHsl={hsl:?}, ToHsv={hsv:?}, FromHsl={from_hsl:?}, FromHsv={from_hsv:?}"
            );
            format!(
                "ColorHelper OK (color={color:?}, hex={hex:?}, hsl={hsl:?}, hsv={hsv:?}, from_hsl={from_hsl:?}, from_hsv={from_hsv:?})"
            )
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: ColorHelper::ToColor failed: {error:?}");
            format!("ColorHelper failed: {error:?}")
        }
    }
}
fn verify_design_time_helpers() -> String {
    eprintln!("xamltoolkit-labs: before DesignTimeHelpers statics");
    let runtime = DesignTimeHelpers::IsRunningInApplicationRuntimeMode();
    let legacy = DesignTimeHelpers::IsRunningInLegacyDesignerMode();
    let enhanced = DesignTimeHelpers::IsRunningInEnhancedDesignerMode();
    eprintln!(
        "xamltoolkit-labs: DesignTimeHelpers runtime={runtime:?}, legacy={legacy:?}, enhanced={enhanced:?}"
    );
    format!(
        "Helpers: DesignTimeHelpers runtime={:?}; legacy={:?}; enhanced={:?}",
        runtime, legacy, enhanced
    )
}

fn verify_bool_negation_converter() -> String {
    eprintln!("xamltoolkit-labs: before BoolNegationConverter::new");
    match BoolNegationConverter::new() {
        Ok(_) => {
            eprintln!("xamltoolkit-labs: BoolNegationConverter::new OK");
            "Converters: BoolNegationConverter activation OK".to_string()
        }
        Err(error) => {
            eprintln!("xamltoolkit-labs: BoolNegationConverter::new failed: {error:?}");
            format!("Converters: BoolNegationConverter activation failed: {error:?}")
        }
    }
}
