# xamltoolkit-winui

Rust WinRT projection crate for the root `XamlToolkit.WinUI` native component.

This crate consumes the checked-in WinMD metadata under `metadata/` and generates Rust bindings with `windows-bindgen` during `cargo check` or `cargo build`. It does not generate WinMD from IDL and does not require `midlrt`.

## Metadata source

The regular source of `metadata/XamlToolkit.WinUI.winmd` is the native build output from the upstream Toolkit repository:

```text
CommunityToolkit.WinUI\x64\Release\XamlToolkit.WinUI\XamlToolkit.WinUI.winmd
```

The dependency metadata comes from the upstream repository's restored packages, especially:

```text
CommunityToolkit.WinUI\packages\Microsoft.WindowsAppSDK.WinUI.*\metadata\Microsoft.UI.Xaml.winmd
CommunityToolkit.WinUI\packages\Microsoft.WindowsAppSDK.WinUI.*\metadata\Microsoft.UI.Text.winmd
CommunityToolkit.WinUI\packages\Microsoft.WindowsAppSDK.InteractiveExperiences.*\metadata\<target>\Microsoft.UI.winmd
CommunityToolkit.WinUI\packages\Microsoft.WindowsAppSDK.InteractiveExperiences.*\metadata\<target>\Microsoft.Foundation.winmd
```

Run the sync helper after rebuilding upstream metadata:

```powershell
cd C:\Users\kimika\Documents\communitytoolkit\xamltoolkit-rs\crates\xamltoolkit-winui
.\sync-metadata.ps1
```

`sync-metadata.ps1` only copies produced metadata. It discovers `CommunityToolkit.WinUI` first as a future submodule under `xamltoolkit-rs`, then as the current sibling directory, and it infers Windows App SDK package versions from `CommunityToolkit.WinUI\packages`.

## Projection scope

The default filter covers the root `XamlToolkit.WinUI` public WinRT surface exposed by the produced WinMD, including:

- color structs: `HslColor`, `HsvColor`
- UI and framework extensions
- rect, matrix, transform, and visual extensions
- icon markup extensions
- state triggers
- attached shadow and effects contracts

`Windows.*` APIs are referenced from the `windows` crate where available. The generated projection includes only the Toolkit root namespace and the Microsoft WinUI support types needed to compile against the Toolkit WinMD.

## Build

```powershell
cd C:\Users\kimika\Documents\communitytoolkit\xamltoolkit-rs
cargo check -p xamltoolkit-winui
cargo check --example root
```

Environment overrides are available for metadata experiments:

```powershell
$env:XAMLTOOLKIT_WINUI_WINMD = "C:\path\to\XamlToolkit.WinUI.winmd"
$env:XAMLTOOLKIT_WINUI_METADATA_DEPS = "C:\path\to\metadata-deps"
$env:XAMLTOOLKIT_WINUI_FILTERS = "XamlToolkit.WinUI.HsvColor;XamlToolkit.WinUI.TextIconExtension"
```

## Runtime smoke

The workspace provides a light root smoke example:

```powershell
cargo run --example root
```

Runtime activation requires the matching Toolkit DLL/PRI files next to the executable. The workspace `build.rs` copies native outputs from `CommunityToolkit.WinUI\<platform>\Release\<Project>` by default, where `<platform>` follows the Cargo target architecture (`ARM64`, `x64`, or `Win32`). Override with `XAMLTOOLKIT_NATIVE_PLATFORM` and `XAMLTOOLKIT_NATIVE_CONFIGURATION` if needed.
