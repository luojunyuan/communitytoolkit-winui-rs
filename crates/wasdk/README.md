# wasdk

Shared Rust projection crate for the WinAppSDK/WinUI metadata used by the Toolkit crates.

This crate owns the checked-in WinAppSDK metadata under `metadata/deps`, including `Microsoft.UI.Xaml.winmd`, `Microsoft.UI.winmd`, `Microsoft.Foundation.winmd`, `Microsoft.UI.Text.winmd`, and `Microsoft.Windows.ApplicationModel.Resources.winmd`. Toolkit crates consume the generated Rust projection from this crate and reuse these WinMD files at build time to resolve Toolkit WinMD signatures. Windows SDK metadata is resolved from windows-rs/default bindgen metadata, so this crate does not check in `Windows.winmd`.

Validate with:

```powershell
cargo check -p wasdk
```
