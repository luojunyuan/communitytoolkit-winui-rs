# wasdk

Shared Rust projection crate for the WinAppSDK/WinUI metadata used by the Toolkit crates.

This crate owns the checked-in WinAppSDK metadata under `metadata/deps`, including `Microsoft.UI.Xaml.winmd`, `Microsoft.UI.winmd`, `Microsoft.Foundation.winmd`, `Microsoft.UI.Text.winmd`, `Microsoft.Windows.ApplicationModel.Resources.winmd`, and `Windows.winmd`. Toolkit crates consume the generated Rust projection from this crate and reuse these WinMD files at build time to resolve Toolkit WinMD signatures.

Validate with:

```powershell
cargo check -p wasdk
```
