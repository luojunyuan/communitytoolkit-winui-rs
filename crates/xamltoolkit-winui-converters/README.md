# toolkit-winui-converters

Rust WinRT projection crate for the `XamlToolkit.WinUI.Converters` native component.

This crate consumes checked-in WinMD metadata under `metadata/` and generates Rust bindings with `windows-bindgen` during `cargo check` or `cargo build`. It does not generate WinMD from IDL and does not require `midlrt`.

## Metadata source

The regular source of `metadata/XamlToolkit.WinUI.Converters.winmd` is the native Release build output from the upstream Toolkit repository:

```text
CommunityToolkit.WinUI\x64\Release\XamlToolkit.WinUI.Converters\XamlToolkit.WinUI.Converters.winmd
```

WinAppSDK dependency metadata is centralized in `crates\wasdk\metadata\deps`. The regular upstream source for those files is the restored Windows App SDK package metadata:

```text
CommunityToolkit.WinUI\packages\Microsoft.WindowsAppSDK.WinUI.*\metadata\Microsoft.UI.Xaml.winmd
CommunityToolkit.WinUI\packages\Microsoft.WindowsAppSDK.WinUI.*\metadata\Microsoft.UI.Text.winmd
CommunityToolkit.WinUI\packages\Microsoft.WindowsAppSDK.InteractiveExperiences.*\metadata\<target>\Microsoft.UI.winmd
CommunityToolkit.WinUI\packages\Microsoft.WindowsAppSDK.InteractiveExperiences.*\metadata\<target>\Microsoft.Foundation.winmd
```

Run the sync helper after rebuilding upstream metadata:

```powershell
.\tools\sync-metadata.ps1 -Project Converters
```

`tools\sync-metadata.ps1` copies produced metadata and matching native runtime artifacts into `metadata/native/<platform>`. By default it discovers `CommunityToolkit.WinUI` as `xamltoolkit-rs\submodules\CommunityToolkit.WinUI`; use `-SourceRoot` for a different checkout. It infers Windows App SDK package versions from the upstream project.

The checked-in native runtime layout is:

```text
metadata\native\ARM64\XamlToolkit.WinUI.Converters.dll
metadata\native\ARM64\XamlToolkit.WinUI.Converters.pri
metadata\native\ARM64\XamlToolkit.WinUI.Converters.winmd
metadata\native\x64\XamlToolkit.WinUI.Converters.dll
metadata\native\x64\XamlToolkit.WinUI.Converters.pri
metadata\native\x64\XamlToolkit.WinUI.Converters.winmd
metadata\native\Win32\XamlToolkit.WinUI.Converters.dll
metadata\native\Win32\XamlToolkit.WinUI.Converters.pri
metadata\native\Win32\XamlToolkit.WinUI.Converters.winmd
```

## Projection scope

The default filter covers the full `XamlToolkit.WinUI.Converters` WinRT surface exposed by the produced WinMD:

- bool, double, string, type, and visibility converters
- empty object/string/collection converters
- display/resource/string-format/file-size converters

The Converters project does not depend on the root `XamlToolkit.WinUI` component. The generated projection includes only the Converters namespace. `Windows.*` APIs are referenced from the `windows` crate where available; WinAppSDK/WinUI support types such as `Microsoft.UI.Xaml.*` and `Windows.UI.Xaml.Interop.TypeName` are referenced from the shared `wasdk` crate. This crate's local `metadata\deps` directory does not need Microsoft or Windows WinMD files. The crate also re-exports its Toolkit namespace at crate root, so consumers can use `toolkit_winui_converters::BoolNegationConverter` or `toolkit_winui_converters::Converters::BoolNegationConverter`.

## Build

```powershell
cargo check -p toolkit-winui-converters
cargo check --example converters
```

Environment overrides are available for metadata experiments:

```powershell
$env:XAMLTOOLKIT_WINUI_CONVERTERS_WINMD = "C:\path\to\XamlToolkit.WinUI.Converters.winmd"
$env:XAMLTOOLKIT_WINUI_CONVERTERS_METADATA_DEPS = "C:\path\to\metadata-deps"
$env:XAMLTOOLKIT_WINUI_CONVERTERS_FILTERS = "XamlToolkit.WinUI.Converters.BoolNegationConverter"
$env:XAMLTOOLKIT_WINUI_CONVERTERS_BINDGEN_WARNINGS = "1"
```

`XAMLTOOLKIT_WINUI_CONVERTERS_BINDGEN_WARNINGS=1` prints the `windows-bindgen` skip summary. By default the crate writes that detail to `target/.../bindgen-warnings.txt` without surfacing a Cargo warning.
