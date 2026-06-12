#![windows_subsystem = "windows"]

use std::any::Any;
use std::env;
use std::sync::Once;

use windows::ApplicationModel::Resources::Core::ResourceManager;
use windows::Foundation::{PropertyValue, Uri};
use windows::Storage::{IStorageFile, StorageFile};
use windows::core::{HSTRING, IInspectable, Interface, Result};
use windows_collections::IVector;
use windows_reactor::core::backend::{Backend, ControlId, ControlKind};
use windows_reactor::core::custom::{CustomElement, CustomElementHandle};
use windows_reactor::{
    App, Element, ElementExt, GridLength, RenderCx, border, grid, scroll_viewer, text_block, vstack,
};
use xamltoolkit_winui_controls::Microsoft::UI::Xaml::Controls::{
    Border as NativeBorder, Button as NativeButton, Orientation,
};
use xamltoolkit_winui_controls::Microsoft::UI::Xaml::{
    Application as NativeApplication, ResourceDictionary, Thickness as XamlThickness, UIElement,
};
use xamltoolkit_winui_controls::XamlToolkit::WinUI::Controls::{
    ConstrainedBox, Dock, DockPanel, EqualPanel, RadialGauge, RangeSelector, Segmented,
    SettingsCard, SettingsExpander, UniformGrid, WrapPanel, XamlMetaDataProvider,
};

fn main() -> Result<()> {
    windows_reactor::register_xaml_metadata_provider_factory(|| {
        let provider = XamlMetaDataProvider::new()?;
        provider.cast::<IInspectable>()
    });

    App::new()
        .title("XamlToolkit WinUI Controls Demo")
        .render(app)
}

fn app(_cx: &mut RenderCx) -> Element {
    install_toolkit_resources_once();

    scroll_viewer(
        vstack((
            text_block("XamlToolkit.WinUI.Controls")
                .font_size(24.0)
                .bold(),
            text_block("Windows Reactor demo consuming the generated Controls projection crate."),
            visual_samples(),
        ))
        .spacing(14.0)
        .padding(24.0),
    )
    .into()
}

static INSTALL_TOOLKIT_RESOURCES: Once = Once::new();

fn install_toolkit_resources_once() {
    INSTALL_TOOLKIT_RESOURCES.call_once(|| {
        if let Err(error) = verify_toolkit_metadata_provider()
            .and_then(|_| load_toolkit_pri_files())
            .and_then(|_| install_toolkit_resources())
        {
            eprintln!("demo: Toolkit XAML metadata/resources failed: {error:?}");
        }
    });
}

fn verify_toolkit_metadata_provider() -> Result<()> {
    let provider = XamlMetaDataProvider::new()?;
    let full_name = HSTRING::from("XamlToolkit.WinUI.Controls.SettingsCard");
    provider.GetXamlTypeByFullName(&full_name).map(|_| ())
}

fn load_toolkit_pri_files() -> Result<()> {
    let exe_dir = env::current_exe()?
        .parent()
        .expect("current_exe always has a parent directory")
        .to_path_buf();
    let mut files = Vec::new();

    for name in [
        "XamlToolkit.WinUI.pri",
        "XamlToolkit.WinUI.Converters.pri",
        "XamlToolkit.WinUI.Helpers.pri",
        "XamlToolkit.WinUI.Controls.pri",
    ] {
        let path = exe_dir.join(name);
        if path.exists() {
            let path = HSTRING::from(path.to_string_lossy().as_ref());
            let file = StorageFile::GetFileFromPathAsync(&path)?.join()?;
            files.push(Some(file.cast::<IStorageFile>()?));
        }
    }

    if !files.is_empty() {
        let files = IVector::<IStorageFile>::from(files);
        ResourceManager::Current()?.LoadPriFiles(&files)?;
    }

    Ok(())
}

fn install_toolkit_resources() -> Result<()> {
    let app = NativeApplication::Current()?;
    let resources = app.Resources()?;
    let merged = resources.MergedDictionaries()?;
    for source in [
        "ms-appx:///XamlToolkit.WinUI.Controls/SettingsControls/SettingsCard/SettingsCard.xaml",
        "ms-appx:///XamlToolkit.WinUI.Controls/SettingsControls/SettingsExpander/SettingsExpander.xaml",
        "ms-appx:///XamlToolkit.WinUI.Controls/Segmented/Segmented.xaml",
        "ms-appx:///XamlToolkit.WinUI.Controls/Segmented/SegmentedItem.xaml",
        "ms-appx:///XamlToolkit.WinUI.Controls/RangeSelector/RangeSelector.xaml",
        "ms-appx:///XamlToolkit.WinUI.Controls/RadialGauge/RadialGauge.xaml",
    ] {
        let dictionary = ResourceDictionary::new()?;
        let uri = Uri::CreateUri(&HSTRING::from(source))?;
        dictionary.SetSource(&uri)?;
        merged.Append(&dictionary)?;
    }
    Ok(())
}

fn visual_samples() -> Element {
    grid((
        sample_card(
            "SettingsCard",
            toolkit_control_host("SettingsCard", create_settings_card_sample),
        )
        .grid_row(0)
        .grid_column(0),
        sample_card(
            "SettingsExpander",
            toolkit_control_host("SettingsExpander", create_settings_expander_sample),
        )
        .grid_row(0)
        .grid_column(1),
        sample_card(
            "Segmented",
            toolkit_control_host("Segmented", create_segmented_sample),
        )
        .grid_row(1)
        .grid_column(0),
        sample_card(
            "RangeSelector",
            toolkit_control_host("RangeSelector", create_range_selector_sample),
        )
        .grid_row(1)
        .grid_column(1),
        sample_card(
            "RadialGauge",
            toolkit_control_host("RadialGauge", create_radial_gauge_sample),
        )
        .grid_row(2)
        .grid_column(0),
        sample_card(
            "WrapPanel",
            toolkit_control_host("WrapPanel", create_wrap_panel_sample),
        )
        .grid_row(2)
        .grid_column(1),
        sample_card(
            "DockPanel",
            toolkit_control_host("DockPanel", create_dock_panel_sample),
        )
        .grid_row(3)
        .grid_column(0),
        sample_card(
            "UniformGrid",
            toolkit_control_host("UniformGrid", create_uniform_grid_sample),
        )
        .grid_row(3)
        .grid_column(1),
        sample_card(
            "EqualPanel",
            toolkit_control_host("EqualPanel", create_equal_panel_sample),
        )
        .grid_row(4)
        .grid_column(0),
        sample_card(
            "ConstrainedBox",
            toolkit_control_host("ConstrainedBox", create_constrained_box_sample),
        )
        .grid_row(4)
        .grid_column(1),
    ))
    .rows([
        GridLength::Auto,
        GridLength::Auto,
        GridLength::Auto,
        GridLength::Auto,
        GridLength::Auto,
    ])
    .columns([GridLength::Star(1.0), GridLength::Star(1.0)])
    .row_spacing(12.0)
    .column_spacing(12.0)
    .into()
}

fn create_wrap_panel_sample() -> Result<UIElement> {
    let panel = WrapPanel::new()?;
    panel.SetWidth(320.0)?;
    panel.SetHeight(96.0)?;
    panel.SetHorizontalSpacing(8.0)?;
    panel.SetVerticalSpacing(8.0)?;

    let children = panel.Children()?;
    for label in ["Alpha", "Beta", "Gamma", "Delta"] {
        children.Append(&sample_button(label, 74.0, 32.0)?)?;
    }

    panel.cast()
}

fn create_dock_panel_sample() -> Result<UIElement> {
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

    panel.cast()
}

fn create_uniform_grid_sample() -> Result<UIElement> {
    let grid = UniformGrid::new()?;
    grid.SetWidth(320.0)?;
    grid.SetHeight(132.0)?;
    grid.SetRows(2)?;
    grid.SetColumns(3)?;

    for label in ["One", "Two", "Three", "Four", "Five", "Six"] {
        grid.Children()?
            .Append(&sample_button(label, 96.0, 44.0)?)?;
    }

    grid.cast()
}

fn sample_button(label: &str, width: f64, height: f64) -> Result<NativeButton> {
    let button = NativeButton::new()?;
    button.SetContent(&boxed_string(label)?)?;
    button.SetWidth(width)?;
    button.SetHeight(height)?;
    button.SetMargin(thickness(4.0, 4.0, 4.0, 4.0))?;
    Ok(button)
}

fn create_equal_panel_sample() -> Result<UIElement> {
    let panel = EqualPanel::new()?;
    panel.SetWidth(320.0)?;
    panel.SetHeight(96.0)?;
    panel.SetSpacing(8.0)?;

    for label in ["One", "Two", "Three"] {
        panel
            .Children()?
            .Append(&sample_button(label, 88.0, 40.0)?)?;
    }

    panel.cast()
}

fn create_constrained_box_sample() -> Result<UIElement> {
    let box_control = ConstrainedBox::new()?;
    box_control.SetWidth(320.0)?;
    box_control.SetHeight(80.0)?;
    box_control.SetScaleX(1.0)?;
    box_control.SetScaleY(1.0)?;
    box_control.SetMultipleX(2)?;
    box_control.SetMultipleY(1)?;
    box_control.SetContent(&sample_button("Constrained", 160.0, 44.0)?.cast::<IInspectable>()?)?;

    box_control.cast()
}

fn create_settings_card_sample() -> Result<UIElement> {
    let card = SettingsCard::new()?;
    card.SetWidth(320.0)?;
    card.SetHeader(&boxed_string("Rust projection")?)?;
    card.SetDescription(&boxed_string("Hosted from xamltoolkit-winui-controls")?)?;
    card.SetContent(&boxed_string("Ready")?)?;
    card.SetIsClickEnabled(false)?;
    card.cast()
}

fn create_settings_expander_sample() -> Result<UIElement> {
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

    expander.cast()
}

fn create_segmented_sample() -> Result<UIElement> {
    let segmented = Segmented::new()?;
    segmented.SetWidth(320.0)?;
    segmented.SetHeight(48.0)?;
    segmented.SetOrientation(Orientation::Horizontal)?;
    let items = IVector::<IInspectable>::from(
        ["Daily", "Weekly", "Monthly"]
            .into_iter()
            .map(boxed_string)
            .map(|value| value.map(Some))
            .collect::<Result<Vec<_>>>()?,
    );
    segmented.SetItemsSource(&items.cast::<IInspectable>()?)?;
    segmented.SetSelectedIndex(1)?;
    segmented.cast()
}

fn create_range_selector_sample() -> Result<UIElement> {
    let selector = RangeSelector::new()?;
    selector.SetWidth(320.0)?;
    selector.SetHeight(64.0)?;
    selector.SetMinimum(0.0)?;
    selector.SetMaximum(100.0)?;
    selector.SetRangeStart(20.0)?;
    selector.SetRangeEnd(80.0)?;
    selector.SetStepFrequency(5.0)?;
    selector.cast()
}

fn create_radial_gauge_sample() -> Result<UIElement> {
    let gauge = RadialGauge::new()?;
    gauge.SetWidth(180.0)?;
    gauge.SetHeight(180.0)?;
    gauge.SetMinimum(0.0)?;
    gauge.SetMaximum(100.0)?;
    gauge.SetValue2(72.0)?;
    gauge.SetUnit(&HSTRING::from("%"))?;
    gauge.SetIsInteractive(false)?;
    gauge.cast()
}

fn sample_card(title: &'static str, sample: Element) -> Element {
    border(vstack((text_block(title).bold(), sample)).spacing(8.0))
        .padding(windows_reactor::Thickness::uniform(12.0))
        .border_thickness(windows_reactor::Thickness::uniform(1.0))
        .corner_radius(6.0)
        .into()
}

fn toolkit_control_host(name: &'static str, create: fn() -> Result<UIElement>) -> Element {
    Element::Custom(CustomElementHandle::new(ToolkitControlHost {
        name,
        create,
    }))
}

#[derive(Clone)]
struct ToolkitControlHost {
    name: &'static str,
    create: fn() -> Result<UIElement>,
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
    create: fn() -> Result<UIElement>,
    native_host: Option<IInspectable>,
) {
    let Some(native_host) = native_host else {
        eprintln!("demo: {name} host native element missing");
        return;
    };

    let result = native_host
        .cast::<NativeBorder>()
        .and_then(|host| create().and_then(|control| host.SetChild(&control)));

    match result {
        Ok(()) => eprintln!("demo: mounted {name} OK"),
        Err(error) => eprintln!("demo: mounted {name} failed: {error:?}"),
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

fn boxed_string(value: &str) -> Result<IInspectable> {
    PropertyValue::CreateString(&HSTRING::from(value))
}
