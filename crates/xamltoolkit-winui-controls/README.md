# toolkit-winui-controls

Rust WinRT projection crate for `XamlToolkit.WinUI.Controls`.

This crate consumes checked-in WinMD metadata under `metadata/` and generates Rust bindings with `windows-bindgen` during `cargo check` or `cargo build`. It does not generate WinMD from IDL and does not require `midlrt`.

## Metadata source

The regular source of `metadata/XamlToolkit.WinUI.Controls.winmd` is the native Release build output from the upstream Toolkit repository:

```text
CommunityToolkit.WinUI\XamlToolkit.WinUI.Controls\x64\Release\XamlToolkit.WinUI.Controls\XamlToolkit.WinUI.Controls.winmd
```

WinAppSDK dependency metadata is centralized in `crates\wasdk\metadata\deps`. This crate's local dependency metadata only needs Toolkit WinMD dependencies:

```text
crates\xamltoolkit-winui\metadata\XamlToolkit.WinUI.winmd
crates\xamltoolkit-winui-helpers\metadata\XamlToolkit.WinUI.Helpers.winmd
crates\xamltoolkit-winui-converters\metadata\XamlToolkit.WinUI.Converters.winmd
```

Run the sync helper after rebuilding upstream metadata:

```powershell
.\tools\sync-metadata.ps1 -Project Controls -Platform All
```

By default the sync helper discovers upstream as `xamltoolkit-rs\submodules\CommunityToolkit.WinUI`; use `-SourceRoot` for a different checkout.

## Projection scope

The default filter covers the public `XamlToolkit.WinUI.Controls` surface exposed by the produced WinMD, including:

- Layout/panel controls: `WrapPanel`, `DockPanel`, `EqualPanel`, `UniformGrid`, `StaggeredPanel`, `StaggeredLayout`.
- Basic controls and helpers: `ConstrainedBox`, `AspectRatio`, `LayoutTransformControl`, `MetadataControl`, `MetadataItem`.
- Range and sizing controls: `RangeSelector`, `RangeChangedEventArgs`, `RangeSelectorProperty`, `SizerBase`, `PropertySizer`, `ContentSizer`, `GridSplitter`.
- Headered, segmented, and settings controls: `HeaderedContentControl`, `HeaderedItemsControl`, `HeaderedTreeView`, `Segmented`, `SegmentedItem`, `SettingsCard`, `SettingsExpander`.
- Color controls: `ColorPicker`, `ColorPickerButton`, `Primitives::ColorPreviewer`, `Primitives::ColorPickerSlider`, `IColorPalette`, color converters, and `HsvColor`-based members.
- Text/token/suggestion controls: `TokenizingTextBox`, `TokenizingTextBoxItem`, `PretokenStringContainer`, `ITokenStringContainer`, `InterspersedObservableVector`, `RichSuggestBox`, `RichSuggestToken`, RichSuggest event args.
- Other controls: `RadialGauge`, `TabbedCommandBar`, `TabbedCommandBarItem`, `SwitchPresenter`, `ImageCropper`, `ImageCropperThumb`, `CameraPreview`, `PreviewFailedEventArgs`, and `XamlMetaDataProvider`.

The crate directly depends on `toolkit-winui`, `toolkit-winui-helpers`, and `toolkit-winui-converters`. Controls methods that expose root or helper types reuse those crates; for example `HsvColor` comes from `toolkit-winui`, and `CameraPreview::CameraHelper` / `StartAsync` use `toolkit_winui_helpers::CameraHelper`. WinAppSDK/WinUI support types are referenced from the shared `wasdk` crate, with WinAppSDK WinMD files coming from `crates\wasdk\metadata\deps`. The Controls namespace is re-exported at crate root, so consumers can use `toolkit_winui_controls::ColorPicker` or `toolkit_winui_controls::primitives::ColorPickerSlider`.

`windows-bindgen` may still report skipped inherited members for dependency projection graphs when `XAMLTOOLKIT_WINUI_CONTROLS_BINDGEN_WARNINGS` is set. The build script treats skipped `XamlToolkit.WinUI.*` members as an error so the Toolkit projection surface does not silently regress.

## Validate

```powershell
cargo check -p toolkit-winui-controls
cargo check --example controls
cargo run --example controls
```

The `controls` example is a console projection smoke executable and does not start a WinUI application.
