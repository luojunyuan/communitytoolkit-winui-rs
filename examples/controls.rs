use toolkit_winui::HsvColor;
use toolkit_winui_controls::Primitives::{ColorPickerSlider, ColorPreviewer};
use toolkit_winui_controls::Windows::UI::Color;
use toolkit_winui_controls::{
    AccentColorConverter, AspectRatio, BitmapFileFormat, CameraPreview, Case, CaseCollection,
    ColorChannel, ColorPicker, ColorPickerButton, ColorRepresentation, ColorToHexConverter,
    ConstrainedBox, ContentAlignment, ContentSizer, ContrastBrushConverter, CornerRadiusConverter,
    CropShape, Dock, DockPanel, EqualPanel, GridResizeBehavior, GridResizeDirection, GridSplitter,
    HeaderedContentControl, HeaderedItemsControl, HeaderedTreeView, IColorPalette,
    IColorPalette_Impl, ITokenStringContainer, ImageCropper, ImageCropperThumb,
    InterspersedObservableVector, LayoutTransformControl, MetadataControl, MetadataItem,
    NullToTransparentConverter, PretokenStringContainer, PreviewFailedEventArgs, PropertySizer,
    RadialGauge, RadialGaugeAutomationPeer, RangeChangedEventArgs, RangeSelector,
    RangeSelectorProperty, RichSuggestBox, RichSuggestToken, RichSuggestTokenPointerOverEventArgs,
    RichSuggestTokenSelectedEventArgs, Segmented, SegmentedItem, SegmentedMarginConverter,
    SettingsCard, SettingsCardAutomationPeer, SettingsExpander, SettingsExpanderAutomationPeer,
    SettingsExpanderItemStyleSelector, SizerAutomationPeer, SizerBase, StaggeredLayout,
    StaggeredLayoutItemsStretch, StaggeredPanel, StretchChild, StyleExtensionResourceDictionary,
    StyleExtensions, SuggestionChosenEventArgs, SuggestionPopupPlacementMode,
    SuggestionRequestedEventArgs, SwitchConverter, SwitchPresenter, TabbedCommandBar,
    TabbedCommandBarItem, TabbedCommandBarItemTemplateSelector, ThumbPlacement, ThumbPosition,
    TokenItemAddingEventArgs, TokenItemRemovingEventArgs, TokenizingTextBox,
    TokenizingTextBoxAutomationPeer, TokenizingTextBoxItem, TokenizingTextBoxStyleSelector,
    UniformGrid, WrapPanel, XamlMetaDataProvider,
};
use toolkit_winui_helpers::CameraHelper;
use windows_core::{HSTRING, Result};

#[windows_core::implement(IColorPalette)]
struct SmokeColorPalette;

impl IColorPalette_Impl for SmokeColorPalette_Impl {
    fn ColorCount(&self) -> Result<i32> {
        Ok(1)
    }

    fn ShadeCount(&self) -> Result<i32> {
        Ok(1)
    }

    fn GetColor(&self, _color_index: i32, _shade_index: i32) -> Result<Color> {
        Ok(Color {
            A: 255,
            R: 48,
            G: 96,
            B: 192,
        })
    }
}

fn main() {
    if let Err(error) = run_smoke() {
        eprintln!("controls-example: smoke failed: {error:?}");
        std::process::exit(1);
    }

    eprintln!("controls-example: smoke OK");
}

fn run_smoke() -> Result<()> {
    verify_enum_surface();
    verify_type_surface();
    verify_interface_surface()?;
    verify_root_dependency_surface();
    Ok(())
}

fn verify_enum_surface() {
    let _ = [
        BitmapFileFormat::Bmp,
        BitmapFileFormat::Png,
        BitmapFileFormat::Jpeg,
        BitmapFileFormat::Tiff,
        BitmapFileFormat::Gif,
        BitmapFileFormat::JpegXR,
    ];
    let _ = [
        ColorChannel::Alpha,
        ColorChannel::Channel1,
        ColorChannel::Channel2,
        ColorChannel::Channel3,
    ];
    let _ = [ColorRepresentation::Hsva, ColorRepresentation::Rgba];
    let _ = [
        ContentAlignment::Right,
        ContentAlignment::Left,
        ContentAlignment::Vertical,
    ];
    let _ = [CropShape::Rectangular, CropShape::Circular];
    let _ = [Dock::Left, Dock::Top, Dock::Right, Dock::Bottom];
    let _ = [
        GridResizeBehavior::BasedOnAlignment,
        GridResizeBehavior::CurrentAndNext,
        GridResizeBehavior::PreviousAndCurrent,
        GridResizeBehavior::PreviousAndNext,
    ];
    let _ = [
        GridResizeDirection::Auto,
        GridResizeDirection::Columns,
        GridResizeDirection::Rows,
    ];
    let _ = [
        RangeSelectorProperty::MinimumValue,
        RangeSelectorProperty::MaximumValue,
    ];
    let _ = [
        StaggeredLayoutItemsStretch::None,
        StaggeredLayoutItemsStretch::Fill,
    ];
    let _ = [StretchChild::None, StretchChild::Last];
    let _ = [
        SuggestionPopupPlacementMode::Floating,
        SuggestionPopupPlacementMode::Attached,
    ];
    let _ = [ThumbPlacement::All, ThumbPlacement::Corners];
    let _ = [
        ThumbPosition::Top,
        ThumbPosition::Bottom,
        ThumbPosition::Left,
        ThumbPosition::Right,
        ThumbPosition::UpperLeft,
        ThumbPosition::UpperRight,
        ThumbPosition::LowerLeft,
        ThumbPosition::LowerRight,
    ];
}

fn verify_type_surface() {
    type_seen::<Option<AccentColorConverter>>();
    type_seen::<Option<AspectRatio>>();
    type_seen::<Option<CameraPreview>>();
    type_seen::<Option<Case>>();
    type_seen::<Option<CaseCollection>>();
    type_seen::<Option<ColorPicker>>();
    type_seen::<Option<ColorPickerButton>>();
    type_seen::<Option<ColorPickerSlider>>();
    type_seen::<Option<ColorPreviewer>>();
    type_seen::<Option<ColorToHexConverter>>();
    type_seen::<Option<ConstrainedBox>>();
    type_seen::<Option<ContentSizer>>();
    type_seen::<Option<ContrastBrushConverter>>();
    type_seen::<Option<CornerRadiusConverter>>();
    type_seen::<Option<DockPanel>>();
    type_seen::<Option<EqualPanel>>();
    type_seen::<Option<GridSplitter>>();
    type_seen::<Option<HeaderedContentControl>>();
    type_seen::<Option<HeaderedItemsControl>>();
    type_seen::<Option<HeaderedTreeView>>();
    type_seen::<Option<ImageCropper>>();
    type_seen::<Option<ImageCropperThumb>>();
    type_seen::<Option<InterspersedObservableVector>>();
    type_seen::<Option<LayoutTransformControl>>();
    type_seen::<Option<MetadataControl>>();
    type_seen::<Option<MetadataItem>>();
    type_seen::<Option<NullToTransparentConverter>>();
    type_seen::<Option<PretokenStringContainer>>();
    type_seen::<Option<PreviewFailedEventArgs>>();
    type_seen::<Option<PropertySizer>>();
    type_seen::<Option<RadialGauge>>();
    type_seen::<Option<RadialGaugeAutomationPeer>>();
    type_seen::<Option<RangeChangedEventArgs>>();
    type_seen::<Option<RangeSelector>>();
    type_seen::<Option<RichSuggestBox>>();
    type_seen::<Option<RichSuggestToken>>();
    type_seen::<Option<RichSuggestTokenPointerOverEventArgs>>();
    type_seen::<Option<RichSuggestTokenSelectedEventArgs>>();
    type_seen::<Option<Segmented>>();
    type_seen::<Option<SegmentedItem>>();
    type_seen::<Option<SegmentedMarginConverter>>();
    type_seen::<Option<SettingsCard>>();
    type_seen::<Option<SettingsCardAutomationPeer>>();
    type_seen::<Option<SettingsExpander>>();
    type_seen::<Option<SettingsExpanderAutomationPeer>>();
    type_seen::<Option<SettingsExpanderItemStyleSelector>>();
    type_seen::<Option<SizerAutomationPeer>>();
    type_seen::<Option<SizerBase>>();
    type_seen::<Option<StaggeredLayout>>();
    type_seen::<Option<StaggeredPanel>>();
    type_seen::<Option<StyleExtensionResourceDictionary>>();
    type_seen::<Option<StyleExtensions>>();
    type_seen::<Option<SuggestionChosenEventArgs>>();
    type_seen::<Option<SuggestionRequestedEventArgs>>();
    type_seen::<Option<SwitchConverter>>();
    type_seen::<Option<SwitchPresenter>>();
    type_seen::<Option<TabbedCommandBar>>();
    type_seen::<Option<TabbedCommandBarItem>>();
    type_seen::<Option<TabbedCommandBarItemTemplateSelector>>();
    type_seen::<Option<TokenItemAddingEventArgs>>();
    type_seen::<Option<TokenItemRemovingEventArgs>>();
    type_seen::<Option<TokenizingTextBox>>();
    type_seen::<Option<TokenizingTextBoxAutomationPeer>>();
    type_seen::<Option<TokenizingTextBoxItem>>();
    type_seen::<Option<TokenizingTextBoxStyleSelector>>();
    type_seen::<Option<UniformGrid>>();
    type_seen::<Option<WrapPanel>>();
    type_seen::<Option<XamlMetaDataProvider>>();
}

fn verify_interface_surface() -> Result<()> {
    let palette: IColorPalette = SmokeColorPalette.into();
    assert_eq!(palette.ColorCount()?, 1);
    assert_eq!(palette.ShadeCount()?, 1);
    let color = palette.GetColor(0, 0)?;
    assert_eq!(color.A, 255);
    type_seen::<Option<ITokenStringContainer>>();
    Ok(())
}

fn verify_root_dependency_surface() {
    let _ = HsvColor {
        H: 210.0,
        S: 0.5,
        V: 0.7,
        A: 1.0,
    };
    let _ = HSTRING::from("XamlToolkit.WinUI.Controls");
}

#[allow(dead_code)]
fn compile_camera_preview_dependency_methods(
    _preview: &CameraPreview,
    _helper: &CameraHelper,
) -> Result<()> {
    let _ = CameraPreview::IsFrameSourceGroupButtonVisibleProperty()?;
    Ok(())
}

#[allow(dead_code)]
fn compile_hsv_dependency_methods(
    _previewer: &ColorPreviewer,
    _slider: &ColorPickerSlider,
) -> Result<()> {
    let _ = ColorPreviewer::HsvColorProperty()?;
    let _ = ColorPickerSlider::HsvColorProperty()?;
    Ok(())
}

fn type_seen<T>() {
    let _ = std::mem::size_of::<T>();
}
