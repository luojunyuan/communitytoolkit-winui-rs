# xamltoolkit-rs

Rust projection workspace for `CommunityToolkit.WinUI` / `XamlToolkit.WinUI`.

The current priority is to generate and validate the Rust WinRT projection for `XamlToolkit.WinUI.Controls`. Other Toolkit projects can stay as workspace crates for later phases, but the runnable example is now a Controls consumer, not a Labs crate.

The source C++/WinRT repository stays as a sibling directory:

```text
C:\Users\kimika\Documents\communitytoolkit\CommunityToolkit.WinUI
C:\Users\kimika\Documents\communitytoolkit\xamltoolkit-rs
C:\Users\kimika\Documents\communitytoolkit\windows-rs-62af965-patched
```

`windows-rs-62af965-patched` is a local copy of the `microsoft/windows-rs` commit used by the original lockfile, with a small `windows-reactor` patch that lets external WinRT component libraries register their generated XAML metadata provider. The Controls example registers `XamlToolkit.WinUI.Controls.XamlMetaDataProvider` through that hook so templated controls can resolve Toolkit XAML types.

## Layout

```text
crates/xamltoolkit-winui             root XamlToolkit.WinUI projection crate
crates/xamltoolkit-winui-controls    XamlToolkit.WinUI.Controls projection crate
crates/xamltoolkit-winui-helpers     XamlToolkit.WinUI.Helpers projection crate, parked for later
crates/xamltoolkit-winui-converters  XamlToolkit.WinUI.Converters projection crate, parked for later
examples/controls.rs                 Controls sample/smoke executable
```

`examples/controls.rs` is a standard Cargo example and runs with `cargo run --example controls`. It intentionally does not use the `xamltoolkit-labs` name: `XamlToolkit.Labs.WinUI.Native` should be treated as its own crate/library if we project it later.

## Current Scope

The first major scope is `xamltoolkit-winui-controls`.

The controls crate currently projects and smoke-tests a broad Controls subset, including layout panels, basic controls, range/sizer controls, headered/segmented/settings controls, ColorPicker, RadialGauge, TabbedCommandBar, TokenizingTextBox, RichSuggestBox activation, ImageCropper, and a minimal CameraPreview surface.

The example also mounts a growing set of real visual samples. The current `all` run hosts `WrapPanel`, `DockPanel`, `UniformGrid`, `RangeSelector`, `Segmented`, `RadialGauge`, and `SettingsCard` through the generated `xamltoolkit-winui-controls` crate.

Known deeper gaps remain: real visual sample pages are only starting, `CameraHelper` / true camera preview is not enabled by default, and some collection/event-heavy controls still only have light smoke tests.

## Validate

```powershell
cd C:\Users\kimika\Documents\communitytoolkit\xamltoolkit-rs
cargo check -p xamltoolkit-winui-controls
cargo check --example controls
cargo build --example controls
```

Run the Controls example with:

```powershell
cargo run --example controls
```

The default visual sample is `WrapPanel`. More samples can be selected with:

```powershell
$env:XAMLTOOLKIT_CONTROLS_VISUAL_SAMPLES = "SettingsCard"
$env:XAMLTOOLKIT_CONTROLS_VISUAL_SAMPLES = "RadialGauge"
$env:XAMLTOOLKIT_CONTROLS_VISUAL_SAMPLES = "WrapPanel,DockPanel,UniformGrid,RangeSelector,Segmented"
$env:XAMLTOOLKIT_CONTROLS_VISUAL_SAMPLES = "all"
cargo run --example controls
```

Runtime activation requires the Toolkit DLL/PRI assets to be deployed next to the executable. The root `build.rs` follows the `windows-reactor-setup` self-contained pattern and emits activatable class manifest entries for Toolkit runtimeclasses.

## Expansion Plan

1. Keep expanding `crates/xamltoolkit-winui-controls` module by module, with compile and example smoke tests for each module.
2. Turn `examples/controls.rs` from a smoke list into real Controls sample pages that exercise the projected controls visually.
3. Treat `XamlToolkit.Labs.WinUI.Native` as a separate crate/library later instead of naming the example after it.
4. Add or expand sibling binding crates for other native Toolkit projects only after the Controls path is stable.
5. Commit after each verified module or coherent documentation/update step.
