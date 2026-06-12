# XamlToolkit Controls Demo

Small Windows Reactor app that consumes `xamltoolkit-winui-controls` and mounts several native Toolkit controls.

Run from the workspace root:

```powershell
cargo run --manifest-path examples/demo/Cargo.toml
```

The demo build script uses `windows-reactor-setup` for Windows App SDK self-contained runtime files, embeds Toolkit reg-free WinRT activation entries, and copies checked-in Toolkit native artifacts from `crates/*/metadata/native/<platform>` next to the executable.
