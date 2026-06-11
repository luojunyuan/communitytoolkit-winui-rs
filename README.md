# xamltoolkit-rs

Rust projection workspace for `CommunityToolkit.WinUI` / `XamlToolkit.WinUI`.

The `xamltoolkit-winui` crate projects the public root `XamlToolkit.WinUI` WinRT surface from the produced Toolkit WinMD. The `xamltoolkit-winui-converters` crate projects the full `XamlToolkit.WinUI.Converters` WinRT surface from its produced WinMD. Other projection crates remain parked for later expansion.

The source C++/WinRT repository is checked out as a git submodule under this workspace:

```text
xamltoolkit-rs
xamltoolkit-rs\submodules\CommunityToolkit.WinUI
```

The Rust Windows projection/runtime crates are pulled from the official `microsoft/windows-rs` git repository.

## Layout

```text
crates/xamltoolkit-winui             root XamlToolkit.WinUI projection crate
crates/xamltoolkit-winui-converters  XamlToolkit.WinUI.Converters projection crate
crates/xamltoolkit-winui-controls    XamlToolkit.WinUI.Controls projection crate
crates/xamltoolkit-winui-helpers     XamlToolkit.WinUI.Helpers projection crate, parked for later
examples/root.rs                     root projection smoke executable
examples/converters.rs               Converters projection smoke executable
examples/controls.rs                 Controls sample source, parked for later
```

## Metadata

`crates/xamltoolkit-winui/metadata` and `crates/xamltoolkit-winui-converters/metadata` are checked in so the crates can build and run smoke examples without the upstream repository being present. Each metadata directory contains the projection WinMD, dependency WinMD files, and `native/<platform>` runtime artifacts (`dll`, `pri`, `winmd`).

Use the top-level sync helper to refresh metadata from upstream build output and package metadata:

```powershell
.\tools\sync-metadata.ps1
```

By default this syncs the active Root and Converters crates for `x64|Release`. Common variants:

```powershell
.\tools\sync-metadata.ps1 -Platform All
.\tools\sync-metadata.ps1 -Project Root -Platform x64
.\tools\sync-metadata.ps1 -Project Converters -Platform ARM64
```

The sync helper copies produced WinMD/native runtime artifacts from the selected upstream Release output and discovers Windows App SDK metadata versions from `submodules\CommunityToolkit.WinUI\packages`. `Win32` output is accepted from `submodules\CommunityToolkit.WinUI\Release\<project>`; `x64` and `ARM64` use `submodules\CommunityToolkit.WinUI\<platform>\Release\<project>`. When syncing `-Platform All`, the checked-in top-level projection WinMD is taken from `x64` unless `-MetadataPlatform` is specified.

## Validate

```powershell
cargo fmt --check
cargo check -p xamltoolkit-winui
cargo check -p xamltoolkit-winui-converters
cargo check --example root
cargo check --example converters
cargo check --workspace
```

Run the smoke examples with:

```powershell
cargo run --example root
cargo run --example converters
```

Runtime activation requires Toolkit DLL/PRI assets next to the executable. The workspace `build.rs` copies the checked-in native artifacts from each active crate's `metadata\native\<platform>` directory by default, where `<platform>` follows the Cargo target architecture (`ARM64`, `x64`, or `Win32`). Use `XAMLTOOLKIT_NATIVE_PLATFORM`, `XAMLTOOLKIT_WINUI_NATIVE_DIR`, or `XAMLTOOLKIT_WINUI_CONVERTERS_NATIVE_DIR` to override that mapping.

## Expansion Plan

1. Keep expanding `crates/xamltoolkit-winui-controls` module by module, with compile and example smoke tests for each module.
2. Turn `examples/controls.rs` from a smoke list into real Controls sample pages that exercise the projected controls visually.
3. Treat `XamlToolkit.Labs.WinUI.Native` as a separate crate/library later instead of naming the example after it.
4. Add or expand sibling binding crates for other native Toolkit projects after the root and Controls paths are stable.
5. Commit after each verified module or coherent documentation/update step.
