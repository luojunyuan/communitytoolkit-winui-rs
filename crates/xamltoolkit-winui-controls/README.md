# xamltoolkit-winui-controls

Rust WinRT projection crate for `XamlToolkit.WinUI.Controls`.

This crate is generated from `XamlToolkit.WinUI.Controls.winmd` with an intentionally curated `windows-bindgen` filter list. The filter list is broad enough for the current Rust Labs demo, but still narrower than the full Toolkit metadata so that inherited WinUI members and media-heavy APIs can be expanded in controlled steps.

## Current Coverage

Validated by `examples/xamltoolkit-labs` smoke tests:

- Layout / panels: `WrapPanel`, `DockPanel`, `EqualPanel`, `UniformGrid`, `StaggeredPanel`, `StaggeredLayout`.
- Basic controls: `ConstrainedBox`, `AspectRatio`, `LayoutTransformControl`, `MetadataControl`, `MetadataItem`.
- Range / sizers: `RangeSelector`, `RangeChangedEventArgs`, `SizerBase`, `PropertySizer`, `ContentSizer`, `GridSplitter`.
- Headered / segmented / settings controls: `HeaderedContentControl`, `HeaderedItemsControl`, `HeaderedTreeView`, `Segmented`, `SegmentedItem`, `SettingsCard`, `SettingsExpander`.
- Color controls: `ColorPicker`, `ColorPickerButton`, `ColorPreviewer`, `ColorPickerSlider`, color converters, `HsvColor`.
- Other controls: `RadialGauge`, `TabbedCommandBar`, `TokenizingTextBox`, `RichSuggestBox` activation, `SwitchPresenter`.
- Image/media-light coverage: `ImageCropper`, `ImageCropperThumb`, `CameraPreview` minimal activation/property surface, `PreviewFailedEventArgs`.

## Known Gaps

- `CameraPreview.StartAsync`, `StartAsync(CameraHelper)`, `CameraHelper`, and `FrameEventArgs.VideoFrame` are not part of the default controls projection yet. A first attempt showed that `Windows.Media.VideoFrame` pulls in a deeper `IPropertySet` / imaging / Direct3D type graph, so that should be handled as a dedicated media-camera phase.
- `RichSuggestBox` is currently activation-only in the demo because deeper render-time setters previously hung the app.
- Some collection-heavy types such as `InterspersedObservableVector` are projected but only lightly smoke-tested.

## Validate

```powershell
cd C:\Users\kimika\Documents\communitytoolkit\xamltoolkit-rs
cargo check -p xamltoolkit-winui-controls
cargo check --example xamltoolkit-labs
cargo build --example xamltoolkit-labs
```

Run the demo with:

```powershell
cargo run --example xamltoolkit-labs
```

The demo is a GUI process and should remain running. For automated smoke verification, start `target\debug\examples\xamltoolkit-labs.exe`, wait about 10 seconds, confirm the process is still alive, then stop it and inspect stderr.

## Expanding The Projection

Add required runtimeclasses/enums/interfaces to `crates/xamltoolkit-winui-controls/build.rs`, then run:

```powershell
cargo check -p xamltoolkit-winui-controls
```

For WinUI controls that inherit from WinUI runtimeclasses, keep the necessary WinUI base class chain in this crate. `windows-bindgen --reference` works for ordinary referenced types, but inherited methods can require access to crate-private vtable fields.
