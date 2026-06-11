use windows::core::{HSTRING, IInspectable, Interface, Result};
use windows_reactor::{App, Element, RenderCx, text_block};
use xamltoolkit_winui_converters::Microsoft::UI::Xaml::Data::IValueConverter;
use xamltoolkit_winui_converters::Microsoft::UI::Xaml::{DependencyObject, Visibility};
use xamltoolkit_winui_converters::Windows::UI::Xaml::Interop::{TypeKind, TypeName};
use xamltoolkit_winui_converters::XamlToolkit::WinUI::Converters::{
    BoolNegationConverter, BoolToObjectConverter, BoolToVisibilityConverter,
    CollectionVisibilityConverter, ColorToDisplayNameConverter, DoubleToObjectConverter,
    DoubleToVisibilityConverter, EmptyCollectionToObjectConverter, EmptyObjectToObjectConverter,
    EmptyStringToObjectConverter, FileSizeToFriendlyStringConverter,
    ResourceNameToResourceStringConverter, StringFormatConverter, StringVisibilityConverter,
    TypeToObjectConverter, VisibilityToBoolConverter,
};

fn main() {
    if let Err(error) = App::new()
        .title("XamlToolkit WinUI Converters smoke")
        .render(app)
    {
        eprintln!("converters-example failed: {error:?}");
        std::process::exit(1);
    }
}

fn app(_cx: &mut RenderCx) -> Element {
    match run_smoke() {
        Ok(()) => {
            eprintln!("converters-example: smoke OK");
            if std::env::var_os("XAMLTOOLKIT_CONVERTERS_SMOKE_EXIT").is_some() {
                std::process::exit(0);
            }
        }
        Err(error) => {
            eprintln!("converters-example: smoke failed: {error:?}");
            std::process::exit(1);
        }
    }

    text_block("XamlToolkit.WinUI.Converters smoke").into()
}

fn run_smoke() -> Result<()> {
    let bool_negation = BoolNegationConverter::new()?;
    let _: IValueConverter = bool_negation.cast()?;

    let bool_to_object = BoolToObjectConverter::new()?;
    bool_to_object.SetTrueValue(&boxed_string("yes")?)?;
    bool_to_object.SetFalseValue(&boxed_string("no")?)?;
    let _ = bool_to_object.TrueValue()?;
    let _: DependencyObject = bool_to_object.cast()?;
    let _ = BoolToObjectConverter::TrueValueProperty()?;
    let _ = BoolToObjectConverter::FalseValueProperty()?;

    let _ = BoolToVisibilityConverter::new()?;
    let _ = CollectionVisibilityConverter::new()?;
    let _ = ColorToDisplayNameConverter::new()?;

    let double_to_object = DoubleToObjectConverter::new()?;
    double_to_object.SetTrueValue(&boxed_string("high")?)?;
    double_to_object.SetFalseValue(&boxed_string("low")?)?;
    double_to_object.SetNullValue(&boxed_string("none")?)?;
    double_to_object.SetGreaterThan(1.0)?;
    double_to_object.SetLessThan(10.0)?;
    let _ = DoubleToObjectConverter::GreaterThanProperty()?;
    let _ = DoubleToObjectConverter::LessThanProperty()?;
    let _ = DoubleToObjectConverter::NullValueProperty()?;

    let _ = DoubleToVisibilityConverter::new()?;
    let _ = EmptyCollectionToObjectConverter::new()?;

    let empty_object = EmptyObjectToObjectConverter::new()?;
    empty_object.SetNotEmptyValue(&boxed_string("not empty")?)?;
    empty_object.SetEmptyValue(&boxed_string("empty")?)?;
    let _ = EmptyObjectToObjectConverter::NotEmptyValueProperty()?;
    let _ = EmptyObjectToObjectConverter::EmptyValueProperty()?;

    let empty_string = EmptyStringToObjectConverter::new()?;
    empty_string.SetNotEmptyValue(&boxed_string("text")?)?;
    empty_string.SetEmptyValue(&boxed_string("blank")?)?;

    let _ = FileSizeToFriendlyStringConverter::new()?;
    let _ = ResourceNameToResourceStringConverter::new()?;

    let string_format = StringFormatConverter::new()?;
    let _: IValueConverter = string_format.cast()?;

    let _ = StringVisibilityConverter::new()?;

    let type_converter = TypeToObjectConverter::new()?;
    type_converter.SetTrueValue(&boxed_string("match")?)?;
    type_converter.SetFalseValue(&boxed_string("miss")?)?;
    type_converter.SetType(&TypeName {
        Name: HSTRING::from("Microsoft.UI.Xaml.Controls.TextBlock"),
        Kind: TypeKind::Metadata,
    })?;
    let _ = TypeToObjectConverter::TypeProperty()?;

    let visibility = VisibilityToBoolConverter::new()?;
    let _: IValueConverter = visibility.cast()?;
    let _ = Visibility::Visible;

    Ok(())
}

fn boxed_string(value: &str) -> Result<IInspectable> {
    windows::Foundation::PropertyValue::CreateString(&HSTRING::from(value))
}
