use windows::Foundation::Rect;
use windows::core::HSTRING;
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
    run_smoke();
    eprintln!("root-example: smoke OK");
}

fn run_smoke() {
    let _ = HslColor {
        H: 210.0,
        S: 0.4,
        L: 0.5,
        A: 1.0,
    };
    let _ = HsvColor {
        H: 210.0,
        S: 0.5,
        V: 0.75,
        A: 1.0,
    };
    let _ = Rect {
        X: 0.0,
        Y: 0.0,
        Width: 100.0,
        Height: 40.0,
    };
    let _ = TypeName {
        Name: HSTRING::from("XamlToolkit.WinUI.TextIconExtension"),
        Kind: TypeKind::Metadata,
    };
    let _ = [Symbol::Accept, Symbol::Add];

    type_seen::<Option<AttachedDropShadow>>();
    type_seen::<Option<ControlSizeTrigger>>();
    type_seen::<Option<Effects>>();
    type_seen::<Option<FontIcon>>();
    type_seen::<Option<FontIconExtension>>();
    type_seen::<Option<FontIconSourceExtension>>();
    type_seen::<Option<FrameworkElementExtensions>>();
    type_seen::<Option<IsEqualStateTrigger>>();
    type_seen::<Option<IsNullOrEmptyStateTrigger>>();
    type_seen::<Option<MatrixExtensions>>();
    type_seen::<Option<RectExtensions>>();
    type_seen::<Option<SymbolIconExtension>>();
    type_seen::<Option<SymbolIconSourceExtension>>();
    type_seen::<Option<TextIconExtension>>();
    type_seen::<Option<UIElementExtensions>>();
    type_seen::<Option<VisualExtensions>>();
    type_seen::<Option<DependencyObject>>();
    type_seen::<Option<FrameworkElement>>();
    type_seen::<Option<UIElement>>();
}

fn type_seen<T>() {
    let _ = std::mem::size_of::<T>();
}
