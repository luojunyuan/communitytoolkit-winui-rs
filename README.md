# xamltoolkit-rs

Rust projection workspace for `CommunityToolkit.WinUI` / `XamlToolkit.WinUI`.

The goal is to generate Rust WinRT binding crates for the native Toolkit projects under `crates/`, then consume those crates from a Rust Labs demo executable.

The source C++/WinRT repository stays as a sibling directory:

```text
C:\Users\kimika\Documents\communitytoolkit\CommunityToolkit.WinUI
C:\Users\kimika\Documents\communitytoolkit\xamltoolkit-rs
```

## Layout

```text
crates/xamltoolkit-winui             root XamlToolkit.WinUI projection crate
crates/xamltoolkit-winui-controls    XamlToolkit.WinUI.Controls projection crate
crates/xamltoolkit-winui-helpers     XamlToolkit.WinUI.Helpers projection crate
crates/xamltoolkit-winui-converters  XamlToolkit.WinUI.Converters projection crate
examples/xamltoolkit-labs            Rust demo executable for consuming projected controls
```

`examples/xamltoolkit-labs` replaces the role of the original Labs demo app on the Rust side. It is not a projection crate for `XamlToolkit.Labs.WinUI`; it is a runnable demo that will gradually port the original Labs pages for whichever Toolkit modules already have Rust projections.

## Current Scope

The first major scope is `xamltoolkit-winui-controls`.

The controls crate currently projects and smoke-tests a broad Controls subset, including layout panels, basic controls, range/sizer controls, headered/segmented/settings controls, ColorPicker, RadialGauge, TabbedCommandBar, TokenizingTextBox, RichSuggestBox activation, ImageCropper, and a minimal CameraPreview surface.

Known deeper gaps remain: real Labs pages are not yet ported, `CameraHelper` / true camera preview is not enabled by default, and some collection/event-heavy controls still only have light smoke tests.

## Validate

```powershell
cd C:\Users\kimika\Documents\communitytoolkit\xamltoolkit-rs
cargo check --workspace
cargo check -p xamltoolkit-winui-controls
cargo check --example xamltoolkit-labs
cargo build --example xamltoolkit-labs
```

Run the demo with:

```powershell
cargo run --example xamltoolkit-labs
```

Runtime activation requires the Toolkit DLL/PRI assets to be deployed next to the executable. The root `build.rs` follows the `windows-reactor-setup` self-contained pattern and emits activatable class manifest entries for Toolkit runtimeclasses.

## Expansion Plan

1. Keep expanding `crates/xamltoolkit-winui-controls` module by module, with compile and demo smoke tests for each module.
2. Port the corresponding original Labs pages into `examples/xamltoolkit-labs` for projected controls.
3. Add or expand sibling binding crates for other native Toolkit projects when needed, for example helpers, converters, media, behaviors, animations, and interactivity.
4. Commit after each verified module or coherent documentation/update step.
