# xamltoolkit-rs

Rust projection workspace for `CommunityToolkit.WinUI` / `XamlToolkit.WinUI`.

The root `xamltoolkit-winui` crate projects the public root `XamlToolkit.WinUI` WinRT surface from the produced Toolkit WinMD. The other projection crates are kept in the repository for later expansion, but the workspace currently builds only the root crate.

The source C++/WinRT repository is checked out as a git submodule under this workspace:

```text
xamltoolkit-rs
xamltoolkit-rs\submodules\CommunityToolkit.WinUI
windows-rs-62af965-patched
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

`crates/xamltoolkit-winui/metadata` is checked in so the crate can build and run the root smoke example without the upstream repository being present. It contains the projection WinMD, dependency WinMD files, and `native/<platform>` runtime artifacts (`dll`, `pri`, `winmd`). To refresh it from upstream build output and package metadata:

```powershell
cd crates\xamltoolkit-winui
.\sync-metadata.ps1
```

The sync helper copies `XamlToolkit.WinUI.winmd` and native runtime artifacts from the selected upstream Release output and discovers Windows App SDK metadata versions from `submodules\CommunityToolkit.WinUI\packages`. `Win32` output is accepted from `submodules\CommunityToolkit.WinUI\Release\XamlToolkit.WinUI`; `x64` and `ARM64` use `submodules\CommunityToolkit.WinUI\<platform>\Release\XamlToolkit.WinUI`.

## Validate

```powershell
cargo fmt --check
cargo check -p xamltoolkit-winui
cargo check --example root
cargo check --workspace
```

Run the root smoke example with:

```powershell
cargo run --example root
```

Runtime activation requires Toolkit DLL/PRI assets next to the executable. The workspace `build.rs` copies the checked-in native artifacts from `crates\xamltoolkit-winui\metadata\native\<platform>` by default, where `<platform>` follows the Cargo target architecture (`ARM64`, `x64`, or `Win32`). Use `XAMLTOOLKIT_NATIVE_PLATFORM` or `XAMLTOOLKIT_WINUI_NATIVE_DIR` to override that mapping.

## Expansion Plan

1. Keep expanding `crates/xamltoolkit-winui-controls` module by module, with compile and example smoke tests for each module.
2. Turn `examples/controls.rs` from a smoke list into real Controls sample pages that exercise the projected controls visually.
3. Treat `XamlToolkit.Labs.WinUI.Native` as a separate crate/library later instead of naming the example after it.
4. Add or expand sibling binding crates for other native Toolkit projects after the root and Controls paths are stable.
5. Commit after each verified module or coherent documentation/update step.
