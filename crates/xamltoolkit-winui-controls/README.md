# xamltoolkit-winui-controls

Rust WinRT projection crate for `XamlToolkit.WinUI.Controls`.

This crate is generated from `XamlToolkit.WinUI.Controls.winmd` with an intentionally curated `windows-bindgen` filter list. The filter list is broad enough for the current Controls example, but still narrower than the full Toolkit metadata so that inherited WinUI members and media-heavy APIs can be expanded in controlled steps.

## Current Coverage

Validated by `examples/controls.rs` smoke tests and visual sample mounts:

- Layout / panels: `WrapPanel`, `DockPanel`, `EqualPanel`, `UniformGrid`, `StaggeredPanel`, `StaggeredLayout`.
- Basic controls: `ConstrainedBox`, `AspectRatio`, `LayoutTransformControl`, `MetadataControl`, `MetadataItem`; `AspectRatio` constructors/static string conversion are covered by focused smoke.
- Range / sizers: `RangeSelector`, `RangeChangedEventArgs`, `RangeSelectorProperty`, `SizerBase`, `PropertySizer`, `ContentSizer`, `GridSplitter`.
- Headered / segmented / settings controls: `HeaderedContentControl`, `HeaderedItemsControl`, `HeaderedTreeView`, `Segmented`, `SegmentedItem`, `SettingsCard`, `SettingsExpander`.
- Color controls: `ColorPicker`, `ColorPickerButton`, `IColorPalette`, `ColorPreviewer`, `ColorPickerSlider`, color converters, `HsvColor`; the example implements `IColorPalette` in Rust and passes it to `ColorPicker.SetCustomPalette`.
- Other controls: `RadialGauge`, `TabbedCommandBar`, `TokenizingTextBox`, `TokenizingTextBoxItem`, `PretokenStringContainer`, `InterspersedObservableVector`, `RichSuggestBox`, `RichSuggestToken`, RichSuggest event args, `SwitchPresenter`.
- Image/media-light coverage: `ImageCropper`, `ImageCropperThumb`, `BitmapFileFormat`, `CameraPreview` minimal activation/property surface, `PreviewFailedEventArgs`.

## Known Gaps

- `CameraPreview.StartAsync`, `StartAsync(CameraHelper)`, `CameraHelper`, and `FrameEventArgs.VideoFrame` are not part of the default controls projection yet. A first attempt showed that `Windows.Media.VideoFrame` pulls in a deeper `IPropertySet` / imaging / Direct3D type graph, so that should be handled as a dedicated media-camera phase.
- `RichSuggestBox` now has a visual sample that mounts the RichEditBox/Popup template and item source path. `RichSuggestTokenSelectedEventArgs` and `RichSuggestTokenPointerOverEventArgs` have focused token-property smoke coverage, but real editor selection/range behavior still needs dedicated coverage.
- `TokenizingTextBox` full control visual mounting is still isolated because the current template path crashes in `Microsoft.UI.Xaml.dll`; `TokenizingTextBoxItem`, `PretokenStringContainer`, `ITokenStringContainer`, and `InterspersedObservableVector` are covered by focused samples/smoke tests.

## Validate

```powershell
cargo check -p xamltoolkit-winui-controls
cargo check --example controls
cargo build --example controls
```

Run the example with:

```powershell
cargo run --example controls
```

The example is a GUI process and should remain running. For automated smoke verification, start `target\debug\examples\controls.exe`, wait about 10 seconds, confirm the process is still alive, then stop it and inspect stderr.

The current visual sample set contains 28 mounts: `WrapPanel`, `DockPanel`, `UniformGrid`, `EqualPanel`, `StaggeredPanel`, `ConstrainedBox`, `LayoutTransformControl`, `ImageCropper`, `CameraPreview`, `TabbedCommandBar`, `TabbedCommandBarItem`, `SwitchPresenter`, `MetadataControl`, `ColorPreviewer`, `ColorPickerSlider`, `ColorPickerButton`, `ColorPicker`, `Sizers`, `RangeSelector`, `RichSuggestBox`, `TokenizingTextBoxItem`, `Segmented`, `RadialGauge`, `SettingsCard`, `SettingsExpander`, `HeaderedContentControl`, `HeaderedItemsControl`, and `HeaderedTreeView`.

## Expanding The Projection

Add required runtimeclasses/enums/interfaces to `crates/xamltoolkit-winui-controls/build.rs`, then run:

```powershell
cargo check -p xamltoolkit-winui-controls
```

For WinUI controls that inherit from WinUI runtimeclasses, keep the necessary WinUI base class chain in this crate. `windows-bindgen --reference` works for ordinary referenced types, but inherited methods can require access to crate-private vtable fields.
