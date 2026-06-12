# xamltoolkit-winui

Rust WinRT projection crate for the root `XamlToolkit.WinUI` native component.

This crate consumes the checked-in WinMD metadata under `metadata/` and generates Rust bindings with `windows-bindgen` during `cargo check` or `cargo build`. It does not generate WinMD from IDL and does not require `midlrt`.

## Metadata source

The regular source of `metadata/XamlToolkit.WinUI.winmd` is the native Release build output from the upstream Toolkit repository:

```text
CommunityToolkit.WinUI\x64\Release\XamlToolkit.WinUI\XamlToolkit.WinUI.winmd
```

WinAppSDK dependency metadata is centralized in `crates\wasdk\metadata\deps`. The regular upstream source for those files is the restored package metadata, especially:

```text
CommunityToolkit.WinUI\packages\Microsoft.WindowsAppSDK.WinUI.*\metadata\Microsoft.UI.Xaml.winmd
CommunityToolkit.WinUI\packages\Microsoft.WindowsAppSDK.WinUI.*\metadata\Microsoft.UI.Text.winmd
CommunityToolkit.WinUI\packages\Microsoft.WindowsAppSDK.InteractiveExperiences.*\metadata\<target>\Microsoft.UI.winmd
CommunityToolkit.WinUI\packages\Microsoft.WindowsAppSDK.InteractiveExperiences.*\metadata\<target>\Microsoft.Foundation.winmd
```

Run the sync helper after rebuilding upstream metadata:

```powershell
.\tools\sync-metadata.ps1 -Project Root
```

`tools\sync-metadata.ps1` copies produced metadata and the matching native runtime artifacts into `metadata/native/<platform>`. By default it discovers `CommunityToolkit.WinUI` as `xamltoolkit-rs\submodules\CommunityToolkit.WinUI`; use `-SourceRoot` for a different checkout. It infers Windows App SDK package versions from `CommunityToolkit.WinUI\packages`.

The checked-in native runtime layout is:

```text
metadata\native\ARM64\XamlToolkit.WinUI.dll
metadata\native\ARM64\XamlToolkit.WinUI.pri
metadata\native\ARM64\XamlToolkit.WinUI.winmd
metadata\native\x64\XamlToolkit.WinUI.dll
metadata\native\x64\XamlToolkit.WinUI.pri
metadata\native\x64\XamlToolkit.WinUI.winmd
metadata\native\Win32\XamlToolkit.WinUI.dll
metadata\native\Win32\XamlToolkit.WinUI.pri
metadata\native\Win32\XamlToolkit.WinUI.winmd
```

## Projection scope

The default filter covers the root `XamlToolkit.WinUI` public WinRT surface exposed by the produced WinMD, including:

- color structs: `HslColor`, `HsvColor`
- UI and framework extensions
- rect, matrix, transform, and visual extensions
- icon markup extensions
- state triggers
- attached shadow and effects contracts

`Windows.*` APIs are referenced from the `windows` crate where available. WinAppSDK/WinUI support types such as `Microsoft.UI.Xaml.*` and `Windows.UI.Xaml.Interop.*` are referenced from the shared `wasdk` crate instead of being regenerated here. This crate's local `metadata\deps` directory does not need Microsoft or Windows WinMD files. The crate also re-exports its root Toolkit namespace at crate root, so consumers can use `xamltoolkit_winui::HsvColor` in addition to the generated `xamltoolkit_winui::XamlToolkit::WinUI::HsvColor` path.

## Build

```powershell
cargo check -p xamltoolkit-winui
cargo check --example root
```

Environment overrides are available for metadata experiments:

```powershell
$env:XAMLTOOLKIT_WINUI_WINMD = "C:\path\to\XamlToolkit.WinUI.winmd"
$env:XAMLTOOLKIT_WINUI_METADATA_DEPS = "C:\path\to\metadata-deps"
$env:XAMLTOOLKIT_WINUI_FILTERS = "XamlToolkit.WinUI.HsvColor;XamlToolkit.WinUI.TextIconExtension"
$env:XAMLTOOLKIT_WINUI_BINDGEN_WARNINGS = "1"
```

`XAMLTOOLKIT_WINUI_BINDGEN_WARNINGS=1` prints the `windows-bindgen` skip summary. By default the crate writes that detail to `target/.../bindgen-warnings.txt` without surfacing a Cargo warning, because the default projection intentionally keeps WinUI/Composition support types minimal instead of generating the full inherited WinUI surface.

## Runtime smoke

The workspace provides a light root smoke example:

```powershell
cargo run --example root
```

Runtime activation requires the matching Toolkit DLL/PRI files next to the executable. The workspace `build.rs` copies native artifacts from `metadata\native\<platform>` by default, where `<platform>` follows the Cargo target architecture (`ARM64`, `x64`, or `Win32`). Override with `XAMLTOOLKIT_NATIVE_PLATFORM` or `XAMLTOOLKIT_WINUI_NATIVE_DIR` if needed.
