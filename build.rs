use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_NATIVE_PLATFORM");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_NATIVE_DIR");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_CONVERTERS_NATIVE_DIR");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_HELPERS_NATIVE_DIR");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_CONTROLS_NATIVE_DIR");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let platform = env::var("XAMLTOOLKIT_NATIVE_PLATFORM").unwrap_or_else(|_| target_platform());
    let native_projects = native_projects(&manifest_dir, &platform);

    let target_dir = target_dir_from_out(&out_dir);
    let examples_dir = target_dir.join("examples");
    remove_stale_toolkit_winmd(&examples_dir);
    copy_toolkit_native_to_examples(&native_projects, &examples_dir);
}

fn target_dir_from_out(out: &Path) -> PathBuf {
    out.ancestors().nth(3).unwrap_or(out).to_path_buf()
}

fn native_projects(manifest_dir: &Path, platform: &str) -> Vec<PathBuf> {
    vec![
        env::var_os("XAMLTOOLKIT_WINUI_NATIVE_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| toolkit_native_dir(manifest_dir, "xamltoolkit-winui", platform)),
        env::var_os("XAMLTOOLKIT_WINUI_CONVERTERS_NATIVE_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                toolkit_native_dir(manifest_dir, "xamltoolkit-winui-converters", platform)
            }),
        env::var_os("XAMLTOOLKIT_WINUI_HELPERS_NATIVE_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                toolkit_native_dir(manifest_dir, "xamltoolkit-winui-helpers", platform)
            }),
        env::var_os("XAMLTOOLKIT_WINUI_CONTROLS_NATIVE_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                toolkit_native_dir(manifest_dir, "xamltoolkit-winui-controls", platform)
            }),
    ]
}

fn toolkit_native_dir(manifest_dir: &Path, crate_name: &str, platform: &str) -> PathBuf {
    manifest_dir
        .join("crates")
        .join(crate_name)
        .join("metadata")
        .join("native")
        .join(platform)
}

fn target_platform() -> String {
    match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("aarch64") => "ARM64".to_string(),
        Ok("x86") => "Win32".to_string(),
        _ => "x64".to_string(),
    }
}

fn copy_toolkit_native_to_examples(native_projects: &[PathBuf], examples_dir: &Path) {
    for native_dir in native_projects {
        println!("cargo:rerun-if-changed={}", native_dir.display());
        copy_native_project_to_examples(native_dir, examples_dir);
    }
}

fn copy_native_project_to_examples(native_dir: &Path, examples_dir: &Path) {
    let Ok(entries) = fs::read_dir(native_dir) else {
        println!(
            "cargo:warning=Toolkit native output not found: {}",
            native_dir.display()
        );
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if path.is_file() {
            if should_copy_toolkit_file(&name) {
                copy_file(&path, &examples_dir.join(file_name));
            }
        } else if path.is_dir() {
            copy_dir_contents(&path, &examples_dir.join(file_name));
        }
    }
}

fn should_copy_toolkit_file(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.ends_with(".dll") || lower.ends_with(".pri")
}

fn remove_stale_toolkit_winmd(dir: &Path) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if path.is_file()
            && name.starts_with("XamlToolkit.WinUI")
            && name.to_ascii_lowercase().ends_with(".winmd")
        {
            let _ = fs::remove_file(path);
        }
    }
}

fn copy_dir_contents(src: &Path, dest: &Path) {
    let _ = fs::create_dir_all(dest);
    let Ok(entries) = fs::read_dir(src) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let dest_path = dest.join(entry.file_name());
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if path.is_file() {
            if should_copy_toolkit_resource_file(&name) {
                copy_file(&path, &dest_path);
            }
        } else if path.is_dir() {
            copy_dir_contents(&path, &dest_path);
        }
    }
}

fn copy_file(src: &Path, dest: &Path) {
    if let Some(parent) = dest.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::copy(src, dest);
}

fn should_copy_toolkit_resource_file(name: &str) -> bool {
    !name.to_ascii_lowercase().ends_with(".winmd")
}
