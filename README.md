# xamltoolkit-rs

Rust projection workspace for `CommunityToolkit.WinUI` / `XamlToolkit.WinUI`.

The root `xamltoolkit-winui` crate now projects the public root `XamlToolkit.WinUI` WinRT surface from the produced Toolkit WinMD. The Controls crate remains the broadest visual/sample consumer.

The source C++/WinRT repository is currently a sibling directory and may later become a submodule under this workspace:

```text
C:\Users\kimika\Documents\communitytoolkit\CommunityToolkit.WinUI
C:\Users\kimika\Documents\communitytoolkit\xamltoolkit-rs
C:\Users\kimika\Documents\communitytoolkit\windows-rs-62af965-patched
```

`windows-rs-62af965-patched` is a local copy of the `microsoft/windows-rs` commit used by the original lockfile, with a small `windows-reactor` patch that lets external WinRT component libraries register generated XAML metadata providers.

## Layout

```text
crates/xamltoolkit-winui             root XamlToolkit.WinUI projection crate
crates/xamltoolkit-winui-controls    XamlToolkit.WinUI.Controls projection crate
crates/xamltoolkit-winui-helpers     XamlToolkit.WinUI.Helpers projection crate, parked for later
crates/xamltoolkit-winui-converters  XamlToolkit.WinUI.Converters projection crate, parked for later
examples/root.rs                     root projection smoke executable
examples/controls.rs                 Controls sample/smoke executable
```

## Metadata

`crates/xamltoolkit-winui/metadata` is checked in so the crate can build without the upstream repository being present. To refresh it from upstream build output and package metadata:

```powershell
cd C:\Users\kimika\Documents\communitytoolkit\xamltoolkit-rs\crates\xamltoolkit-winui
.\sync-metadata.ps1
```

The sync helper copies `XamlToolkit.WinUI.winmd` from `CommunityToolkit.WinUI\x64\Release\XamlToolkit.WinUI` by default and discovers Windows App SDK metadata versions from `CommunityToolkit.WinUI\packages`.

## Validate

```powershell
cd C:\Users\kimika\Documents\communitytoolkit\xamltoolkit-rs
cargo fmt --check
cargo check -p xamltoolkit-winui
cargo check --example root
cargo check --example controls
cargo check --workspace
```

Run the root smoke example with:

```powershell
cargo run --example root
```

Run the Controls example with:

```powershell
cargo run --example controls
```

The default Controls visual sample is `WrapPanel`. More samples can be selected with:

```powershell
$env:XAMLTOOLKIT_CONTROLS_VISUAL_SAMPLES = "SettingsCard"
$env:XAMLTOOLKIT_CONTROLS_VISUAL_SAMPLES = "RadialGauge"
$env:XAMLTOOLKIT_CONTROLS_VISUAL_SAMPLES = "all"
cargo run --example controls
```

Runtime activation requires Toolkit DLL/PRI assets next to the executable. The workspace `build.rs` copies native outputs from `CommunityToolkit.WinUI\<platform>\Release\<Project>` by default, where `<platform>` follows the Cargo target architecture (`ARM64`, `x64`, or `Win32`). Use `XAMLTOOLKIT_NATIVE_PLATFORM` and `XAMLTOOLKIT_NATIVE_CONFIGURATION` to override that mapping.

## Expansion Plan

1. Keep expanding `crates/xamltoolkit-winui-controls` module by module, with compile and example smoke tests for each module.
2. Turn `examples/controls.rs` from a smoke list into real Controls sample pages that exercise the projected controls visually.
3. Treat `XamlToolkit.Labs.WinUI.Native` as a separate crate/library later instead of naming the example after it.
4. Add or expand sibling binding crates for other native Toolkit projects after the root and Controls paths are stable.
5. Commit after each verified module or coherent documentation/update step.
