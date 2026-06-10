# xamltoolkit-winui-helpers

Rust WinRT projection crate for `XamlToolkit.WinUI.Helpers`.

The default projection is intentionally incremental. It currently covers low-risk helper APIs that were useful during early projection smoke tests:

- `DesignTimeHelpers`
- `ColorHelper`
- `Windows.UI.Color`
- `XamlToolkit.WinUI.HslColor`
- `XamlToolkit.WinUI.HsvColor`

`CameraHelper` is not part of the default projection yet. It pulls in `Windows.Media.VideoFrame` and a deeper media/imaging/Direct3D type graph, so it should be handled in a dedicated camera/media phase.

## Validate

```powershell
cd C:\Users\kimika\Documents\communitytoolkit\xamltoolkit-rs
cargo check -p xamltoolkit-winui-helpers
```
