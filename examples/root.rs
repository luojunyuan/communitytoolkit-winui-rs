use std::env;

use windows::Foundation::{PropertyValue, Rect};
use windows::core::{HSTRING, IInspectable, Interface, Result};
use windows_reactor::{App, Element, RenderCx, text_block};
use xamltoolkit_winui::Microsoft::UI::Xaml::Controls::{FontIcon, Symbol};
use xamltoolkit_winui::Microsoft::UI::Xaml::{DependencyObject, FrameworkElement, UIElement};
use xamltoolkit_winui::Windows::UI::Xaml::Interop::{TypeKind, TypeName};
use xamltoolkit_winui::XamlToolkit::WinUI::{
    AttachedDropShadow, ControlSizeTrigger, Effects, FontIconExtension, FontIconSourceExtension,
    FrameworkElementExtensions, HslColor, HsvColor, IsEqualStateTrigger, IsNullOrEmptyStateTrigger,
    MatrixExtensions, RectExtensions, SymbolIconExtension, SymbolIconSourceExtension,
    TextIconExtension, UIElementExtensions, VisualExtensions,
};

fn main() {
    if let Err(error) = App::new().title("XamlToolkit WinUI root smoke").render(app) {
        eprintln!("root-example failed: {error:?}");
        std::process::exit(1);
    }
}

fn app(_cx: &mut RenderCx) -> Element {
    match run_smoke() {
        Ok(()) => {
            eprintln!("root-example: smoke OK");
            if env::var_os("XAMLTOOLKIT_ROOT_SMOKE_EXIT").is_some() {
                std::process::exit(0);
            }
        }
        Err(error) => {
            eprintln!("root-example: smoke failed: {error:?}");
            std::process::exit(1);
        }
    }

    text_block("XamlToolkit.WinUI root smoke").into()
}

fn run_smoke() -> Result<()> {
    let _hsl = HslColor {
        H: 210.0,
        S: 0.4,
        L: 0.5,
        A: 1.0,
    };
    let _hsv = HsvColor {
        H: 210.0,
        S: 0.5,
        V: 0.75,
        A: 1.0,
    };

    smoke_step("rect extensions", || {
        let rect = Rect {
            X: 0.0,
            Y: 0.0,
            Width: 100.0,
            Height: 40.0,
        };
        let _ = RectExtensions::IntersectsWith(rect, rect)?;
        let _ = RectExtensions::ToSize(rect)?;
        Ok(())
    })?;

    smoke_step("matrix extensions", || {
        let matrix = MatrixExtensions::CreateScaling(2.0, 3.0)?;
        let _ = MatrixExtensions::HasInverse(matrix)?;
        let _ = MatrixExtensions::Round(matrix, 2)?;
        Ok(())
    })?;

    smoke_step("triggers", smoke_triggers)?;
    smoke_step("markup extensions", smoke_markup_extensions)?;
    smoke_step("attached properties", smoke_attached_properties)?;
    smoke_step("shadow properties", smoke_shadow_properties)?;
    smoke_typename_compile_path();

    Ok(())
}

fn smoke_step(name: &str, f: impl FnOnce() -> Result<()>) -> Result<()> {
    eprintln!("root-example: smoke {name}");
    f()
}

fn smoke_triggers() -> Result<()> {
    let control_size = ControlSizeTrigger::new()?;
    control_size.SetCanTrigger(true)?;
    control_size.SetMinWidth(32.0)?;
    control_size.SetMaxWidth(640.0)?;
    let _ = control_size.IsActive()?;
    let _ = ControlSizeTrigger::CanTriggerProperty()?;
    let _ = ControlSizeTrigger::MaxWidthProperty()?;

    let equal = IsEqualStateTrigger::new()?;
    let left = boxed_string("alpha")?;
    let right = boxed_string("alpha")?;
    equal.SetValue2(&left)?;
    equal.SetTo(&right)?;
    let _ = IsEqualStateTrigger::ValueProperty()?;
    let _ = IsEqualStateTrigger::ToProperty()?;

    let null_or_empty = IsNullOrEmptyStateTrigger::new()?;
    let value = boxed_string("")?;
    null_or_empty.SetValue2(&value)?;
    let _ = IsNullOrEmptyStateTrigger::ValueProperty()?;

    Ok(())
}

fn smoke_markup_extensions() -> Result<()> {
    let text = TextIconExtension::new()?;
    text.SetFontSize(18.0)?;
    text.SetIsTextScaleFactorEnabled(true)?;
    text.SetMirroredWhenRightToLeft(false)?;
    let _ = TextIconExtension::SymbolThemeFontFamily()?;

    let font = FontIconExtension::new()?;
    font.SetGlyph(&HSTRING::from("E10F"))?;
    font.SetFontSize(20.0)?;

    let font_source = FontIconSourceExtension::new()?;
    font_source.SetGlyph(&HSTRING::from("E115"))?;
    font_source.SetFontSize(20.0)?;

    let symbol = SymbolIconExtension::new()?;
    symbol.SetSymbol(Symbol::Accept)?;
    symbol.SetFontSize(20.0)?;

    let symbol_source = SymbolIconSourceExtension::new()?;
    symbol_source.SetSymbol(Symbol::Add)?;
    symbol_source.SetFontSize(20.0)?;

    let _native_icon = FontIcon::new()?;

    Ok(())
}

fn smoke_attached_properties() -> Result<()> {
    let icon = FontIcon::new()?;
    let element: DependencyObject = icon.cast()?;
    VisualExtensions::SetOffset(&element, &HSTRING::from("1,2,3"))?;
    let _ = VisualExtensions::GetOffset(&element)?;
    VisualExtensions::SetOpacity(&element, 0.75)?;
    let _ = VisualExtensions::GetOpacity(&element)?;

    let framework_element: FrameworkElement = icon.cast()?;
    let type_name = TypeName {
        Name: HSTRING::from("Microsoft.UI.Xaml.Controls.FontIcon"),
        Kind: TypeKind::Metadata,
    };
    FrameworkElementExtensions::SetAncestorType(&framework_element, &type_name)?;
    let _ = FrameworkElementExtensions::GetAncestorType(&framework_element)?;

    let ui_element: UIElement = framework_element.cast()?;
    UIElementExtensions::SetClipToBounds(&ui_element, true)?;
    let _ = UIElementExtensions::GetClipToBounds(&ui_element)?;

    Ok(())
}

fn smoke_shadow_properties() -> Result<()> {
    let shadow = AttachedDropShadow::new()?;
    shadow.SetOpacity(0.4)?;
    shadow.SetOffset(&HSTRING::from("0,2,8"))?;
    shadow.SetIsMasked(false)?;
    shadow.SetCornerRadius(4.0)?;
    let _ = AttachedDropShadow::IsMaskedProperty()?;

    let framework_element: FrameworkElement = FontIcon::new()?.cast()?;
    Effects::SetShadow(&framework_element, &shadow)?;
    let _ = Effects::GetShadow(&framework_element)?;
    let _ = Effects::ShadowProperty()?;

    Ok(())
}

fn smoke_typename_compile_path() {
    let _ = TypeName {
        Name: HSTRING::from("XamlToolkit.WinUI.TextIconExtension"),
        Kind: TypeKind::Metadata,
    };
}

fn boxed_string(value: &str) -> Result<IInspectable> {
    PropertyValue::CreateString(&HSTRING::from(value))
}
