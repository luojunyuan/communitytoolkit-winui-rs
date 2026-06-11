use windows::core::HSTRING;
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
    run_smoke();
    eprintln!("converters-example: smoke OK");
}

fn run_smoke() {
    let _ = [Visibility::Visible, Visibility::Collapsed];
    let _ = TypeName {
        Name: HSTRING::from("Microsoft.UI.Xaml.Controls.TextBlock"),
        Kind: TypeKind::Metadata,
    };

    type_seen::<Option<BoolNegationConverter>>();
    type_seen::<Option<BoolToObjectConverter>>();
    type_seen::<Option<BoolToVisibilityConverter>>();
    type_seen::<Option<CollectionVisibilityConverter>>();
    type_seen::<Option<ColorToDisplayNameConverter>>();
    type_seen::<Option<DoubleToObjectConverter>>();
    type_seen::<Option<DoubleToVisibilityConverter>>();
    type_seen::<Option<EmptyCollectionToObjectConverter>>();
    type_seen::<Option<EmptyObjectToObjectConverter>>();
    type_seen::<Option<EmptyStringToObjectConverter>>();
    type_seen::<Option<FileSizeToFriendlyStringConverter>>();
    type_seen::<Option<ResourceNameToResourceStringConverter>>();
    type_seen::<Option<StringFormatConverter>>();
    type_seen::<Option<StringVisibilityConverter>>();
    type_seen::<Option<TypeToObjectConverter>>();
    type_seen::<Option<VisibilityToBoolConverter>>();
    type_seen::<Option<IValueConverter>>();
    type_seen::<Option<DependencyObject>>();
}

fn type_seen<T>() {
    let _ = std::mem::size_of::<T>();
}
