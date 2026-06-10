# xamltoolkit-winui

Minimal Rust projection experiment for `XamlToolkit.WinUI`.

This crate validates the intended Rust support path:

```text
IDL -> native C++/WinRT build or MIDL metadata generation -> XamlToolkit.WinUI.winmd -> windows-bindgen -> Rust bindings
```

The default generation target is deliberately small:

```text
XamlToolkit.WinUI.HslColor
XamlToolkit.WinUI.HsvColor
```

These two structs come from the root `XamlToolkit.WinUI` module and avoid the large WinUI/XAML object dependency graph. This proves that custom toolkit WinMD metadata can be consumed by `windows-rs` before expanding to runtime classes and controls.

## Required metadata

The checked-in experiment expects metadata here:

```text
xamltoolkit-rs/metadata/XamlToolkit.WinUI.winmd
xamltoolkit-rs/metadata/deps/*.winmd
```

`metadata/deps` should contain the WinUI/Windows metadata needed by the toolkit WinMD, for example:

```text
Microsoft.Foundation.winmd
Microsoft.UI.winmd
Microsoft.UI.Text.winmd
Microsoft.UI.Xaml.winmd
Windows.winmd
```

Equivalent environment variables are supported:

```powershell
$env:XAMLTOOLKIT_WINUI_WINMD = "C:\path\to\XamlToolkit.WinUI.winmd"
$env:XAMLTOOLKIT_WINUI_METADATA_DEPS = "C:\path\to\metadata-deps"
cargo check
```

## Generate bindings

From this directory:

```powershell
cargo check
```

The generated Rust bindings are written to Cargo's `OUT_DIR` and included by `src/lib.rs`.

To try a broader filter:

```powershell
$env:XAMLTOOLKIT_WINUI_FILTERS = "XamlToolkit.WinUI"
cargo check
```

Expect broader filters to require a more complete strategy for projecting WinUI dependencies. The minimal validation intentionally starts with the pure structs.

## Runtime note

Generating bindings only validates metadata projection. Calling runtime classes at runtime also requires the matching native `XamlToolkit.WinUI.dll` to be deployed next to the Rust executable or otherwise discoverable by the WinRT activation path.
