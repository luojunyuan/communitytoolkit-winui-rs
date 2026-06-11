# xamltoolkit-winui-helpers

Rust WinRT projection crate for `XamlToolkit.WinUI.Helpers`.

This crate consumes checked-in WinMD metadata under `metadata/` and generates Rust bindings with `windows-bindgen` during `cargo check` or `cargo build`. It does not generate WinMD from IDL and does not require `midlrt`.

## Metadata source

The regular source of `metadata/XamlToolkit.WinUI.Helpers.winmd` is the native Release build output from the upstream Toolkit repository:

```text
CommunityToolkit.WinUI\XamlToolkit.WinUI.Helpers\x64\Release\XamlToolkit.WinUI.Helpers\XamlToolkit.WinUI.Helpers.winmd
```

Dependency metadata comes from the upstream repository's restored Windows App SDK packages plus the root Toolkit WinMD:

```text
CommunityToolkit.WinUI\packages\Microsoft.WindowsAppSDK.WinUI.*\metadata\Microsoft.UI.Xaml.winmd
CommunityToolkit.WinUI\packages\Microsoft.WindowsAppSDK.WinUI.*\metadata\Microsoft.UI.Text.winmd
CommunityToolkit.WinUI\packages\Microsoft.WindowsAppSDK.InteractiveExperiences.*\metadata\<target>\Microsoft.UI.winmd
CommunityToolkit.WinUI\packages\Microsoft.WindowsAppSDK.InteractiveExperiences.*\metadata\<target>\Microsoft.Foundation.winmd
crates\xamltoolkit-winui\metadata\XamlToolkit.WinUI.winmd
```

Run the sync helper after rebuilding upstream metadata:

```powershell
.\tools\sync-metadata.ps1 -Project Helpers
```

`tools\sync-metadata.ps1` copies produced metadata and matching native runtime artifacts into `metadata/native/<platform>`. By default it discovers `CommunityToolkit.WinUI` as `xamltoolkit-rs\submodules\CommunityToolkit.WinUI`; use `-SourceRoot` for a different checkout. It infers Windows App SDK package versions from the upstream project.

The checked-in native runtime layout is:

```text
metadata\native\ARM64\XamlToolkit.WinUI.Helpers.dll
metadata\native\ARM64\XamlToolkit.WinUI.Helpers.pri
metadata\native\ARM64\XamlToolkit.WinUI.Helpers.winmd
metadata\native\x64\XamlToolkit.WinUI.Helpers.dll
metadata\native\x64\XamlToolkit.WinUI.Helpers.pri
metadata\native\x64\XamlToolkit.WinUI.Helpers.winmd
metadata\native\Win32\XamlToolkit.WinUI.Helpers.dll
metadata\native\Win32\XamlToolkit.WinUI.Helpers.pri
metadata\native\Win32\XamlToolkit.WinUI.Helpers.winmd
```

## Projection scope

The default filter covers the full `XamlToolkit.WinUI.Helpers` WinRT surface exposed by the produced WinMD:

- `CameraHelper`
- `CameraHelperResult`
- `ColorHelper`
- `DesignTimeHelpers`
- `FrameEventArgs`
- `ThemeChangedHandler`
- `ThemeListener`

The upstream native project references the root `XamlToolkit.WinUI` component. This crate depends on `xamltoolkit-winui` and keeps the root WinMD in `metadata/deps`; `ColorHelper::ToHsl` and `ColorHelper::ToHsv` return the root crate's `XamlToolkit::WinUI::HslColor` and `XamlToolkit::WinUI::HsvColor` types instead of generating duplicate root structs.

## Validate

```powershell
cargo check -p xamltoolkit-winui-helpers
cargo check --example helpers
```
