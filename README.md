# xamltoolkit-rs

Rust projection workspace for `CommunityToolkit.WinUI` / `XamlToolkit.WinUI`.

The `xamltoolkit-winui` crate projects the public root `XamlToolkit.WinUI` WinRT surface from the produced Toolkit WinMD. The `xamltoolkit-winui-converters`, `xamltoolkit-winui-helpers`, and `xamltoolkit-winui-controls` crates project the `XamlToolkit.WinUI.Converters`, `XamlToolkit.WinUI.Helpers`, and `XamlToolkit.WinUI.Controls` WinRT surfaces from their produced WinMD files.

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
crates/xamltoolkit-winui-helpers     XamlToolkit.WinUI.Helpers projection crate
crates/xamltoolkit-winui-controls    XamlToolkit.WinUI.Controls projection crate
examples/root.rs                     root projection smoke executable
examples/converters.rs               Converters projection smoke executable
examples/helpers.rs                  Helpers projection smoke executable
examples/controls.rs                 Controls projection smoke executable
examples/demo                        Windows Reactor Controls GUI demo workspace
```

## Metadata

Each active crate's `metadata` directory is checked in so the crates can build and run smoke examples without the upstream repository being present. Each metadata directory contains the projection WinMD, dependency WinMD files, and `native/<platform>` runtime artifacts (`dll`, `pri`, `winmd`).

Use the top-level sync helper to refresh metadata from upstream build output and package metadata:

```powershell
.\tools\sync-metadata.ps1
```

By default this syncs the active Root, Converters, Helpers, and Controls crates for `x64|Release`. Common variants:

```powershell
.\tools\sync-metadata.ps1 -Platform All
.\tools\sync-metadata.ps1 -Project Root -Platform x64
.\tools\sync-metadata.ps1 -Project Converters -Platform ARM64
.\tools\sync-metadata.ps1 -Project Helpers -Platform All
.\tools\sync-metadata.ps1 -Project Controls -Platform All
```

The sync helper copies produced WinMD/native runtime artifacts from the selected upstream Release output and discovers Windows App SDK metadata versions from `submodules\CommunityToolkit.WinUI\packages`. `Win32` output is accepted from `submodules\CommunityToolkit.WinUI\Release\<project>`. `x64` and `ARM64` output is accepted from either `submodules\CommunityToolkit.WinUI\<platform>\Release\<project>` or the project-local `submodules\CommunityToolkit.WinUI\<project>\<platform>\Release\<project>` layout used by Helpers. When syncing `-Platform All`, the checked-in top-level projection WinMD is taken from `x64` unless `-MetadataPlatform` is specified.

## Validate

```powershell
cargo fmt --check
cargo check -p xamltoolkit-winui
cargo check -p xamltoolkit-winui-converters
cargo check -p xamltoolkit-winui-helpers
cargo check -p xamltoolkit-winui-controls
cargo check --example root
cargo check --example converters
cargo check --example helpers
cargo check --example controls
cargo check --workspace
```

Run the smoke examples with:

```powershell
cargo run --example root
cargo run --example converters
cargo run --example helpers
cargo run --example controls
```

The examples are console smoke executables that validate projection type paths and selected interface paths without starting a WinUI application. The workspace `build.rs` still copies checked-in Toolkit native artifacts from each active crate's `metadata\native\<platform>` directory by default, where `<platform>` follows the Cargo target architecture (`ARM64`, `x64`, or `Win32`). Use `XAMLTOOLKIT_NATIVE_PLATFORM`, `XAMLTOOLKIT_WINUI_NATIVE_DIR`, `XAMLTOOLKIT_WINUI_CONVERTERS_NATIVE_DIR`, `XAMLTOOLKIT_WINUI_HELPERS_NATIVE_DIR`, or `XAMLTOOLKIT_WINUI_CONTROLS_NATIVE_DIR` to override that mapping.

Run the GUI demo with:

```powershell
cargo run --manifest-path examples/demo/Cargo.toml
```

The GUI demo is isolated under `examples/demo` as a separate workspace. It is the only package that depends on `windows-reactor`.

## Expansion Plan

1. Add real visual sample applications separately from the projection smoke examples.
2. Treat `XamlToolkit.Labs.WinUI.Native` as a separate crate/library later.
3. Add or expand sibling binding crates for other native Toolkit projects after the root, Converters, Helpers, and Controls paths are stable.
4. Commit after each verified module or coherent documentation/update step.
