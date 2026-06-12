use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use windows_metadata::{TypeAttributes, reader};

fn main() {
    windows_reactor_setup::as_self_contained();

    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_NATIVE_PLATFORM");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_NATIVE_DIR");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_CONVERTERS_NATIVE_DIR");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_HELPERS_NATIVE_DIR");
    println!("cargo:rerun-if-env-changed=XAMLTOOLKIT_WINUI_CONTROLS_NATIVE_DIR");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let workspace_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("examples/demo should be two levels below the workspace root")
        .to_path_buf();
    let platform = env::var("XAMLTOOLKIT_NATIVE_PLATFORM").unwrap_or_else(|_| target_platform());
    let native_projects = native_projects(&workspace_root, &platform);

    let manifest_path = out_dir.join("app.manifest");
    add_toolkit_activation_to_manifest(&manifest_path, &native_projects);
    println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
    println!(
        "cargo:rustc-link-arg=/MANIFESTINPUT:{}",
        manifest_path.display()
    );

    let target_dir = target_dir_from_out(&out_dir);
    remove_stale_toolkit_winmd(&target_dir);
    copy_toolkit_native(&native_projects, &target_dir);
}

fn target_dir_from_out(out: &Path) -> PathBuf {
    out.ancestors().nth(3).unwrap_or(out).to_path_buf()
}

fn native_projects(workspace_root: &Path, platform: &str) -> Vec<(String, PathBuf)> {
    vec![
        (
            "XamlToolkit.WinUI".to_string(),
            env::var_os("XAMLTOOLKIT_WINUI_NATIVE_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|| {
                    toolkit_native_dir(workspace_root, "xamltoolkit-winui", platform)
                }),
        ),
        (
            "XamlToolkit.WinUI.Converters".to_string(),
            env::var_os("XAMLTOOLKIT_WINUI_CONVERTERS_NATIVE_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|| {
                    toolkit_native_dir(workspace_root, "xamltoolkit-winui-converters", platform)
                }),
        ),
        (
            "XamlToolkit.WinUI.Helpers".to_string(),
            env::var_os("XAMLTOOLKIT_WINUI_HELPERS_NATIVE_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|| {
                    toolkit_native_dir(workspace_root, "xamltoolkit-winui-helpers", platform)
                }),
        ),
        (
            "XamlToolkit.WinUI.Controls".to_string(),
            env::var_os("XAMLTOOLKIT_WINUI_CONTROLS_NATIVE_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|| {
                    toolkit_native_dir(workspace_root, "xamltoolkit-winui-controls", platform)
                }),
        ),
    ]
}

fn toolkit_native_dir(workspace_root: &Path, crate_name: &str, platform: &str) -> PathBuf {
    workspace_root
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

fn add_toolkit_activation_to_manifest(manifest_path: &Path, native_projects: &[(String, PathBuf)]) {
    let Ok(mut manifest) = fs::read_to_string(manifest_path) else {
        println!(
            "cargo:warning=Windows App SDK manifest not found: {}",
            manifest_path.display()
        );
        return;
    };

    let mut toolkit_entries = String::new();
    for (project, native_dir) in native_projects {
        let winmd = native_dir.join(format!("{project}.winmd"));
        println!("cargo:rerun-if-changed={}", winmd.display());

        let missing_classes: Vec<_> = collect_runtime_classes(&winmd)
            .into_iter()
            .filter(|class| !manifest.contains(&format!("name=\"{class}\"")))
            .collect();
        if missing_classes.is_empty() {
            continue;
        }

        toolkit_entries.push_str(&format!("    <asmv3:file name=\"{project}.dll\">\n"));
        for class in missing_classes {
            toolkit_entries.push_str(&format!(
                "        <winrtv1:activatableClass name=\"{class}\" threadingModel=\"both\"></winrtv1:activatableClass>\n"
            ));
        }
        toolkit_entries.push_str("    </asmv3:file>\n");
    }

    let marker = "</assembly>";
    if let Some(index) = manifest.rfind(marker) {
        manifest.insert_str(index, &toolkit_entries);
        if let Err(error) = fs::write(manifest_path, manifest) {
            println!(
                "cargo:warning=Failed to write Toolkit activation manifest {}: {error}",
                manifest_path.display()
            );
        }
    } else {
        println!(
            "cargo:warning=Windows App SDK manifest did not contain </assembly>: {}",
            manifest_path.display()
        );
    }
}

fn collect_runtime_classes(winmd: &Path) -> Vec<String> {
    let Some(index) = reader::Index::read(winmd) else {
        println!(
            "cargo:warning=Unable to read Toolkit WinMD: {}",
            winmd.display()
        );
        return Vec::new();
    };

    let mut classes = Vec::new();
    for ty in index.types() {
        if ty.flags().contains(TypeAttributes::WindowsRuntime)
            && ty.category() == reader::TypeCategory::Class
            && ty.namespace().starts_with("XamlToolkit.WinUI")
        {
            classes.push(format!("{}.{}", ty.namespace(), ty.name()));
        }
    }

    classes.sort();
    classes.dedup();
    classes
}

fn copy_toolkit_native(native_projects: &[(String, PathBuf)], target_dir: &Path) {
    for (_, native_dir) in native_projects {
        println!("cargo:rerun-if-changed={}", native_dir.display());
        let Ok(entries) = fs::read_dir(native_dir) else {
            println!(
                "cargo:warning=Toolkit native output not found: {}",
                native_dir.display()
            );
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();

            if path.is_file() && should_copy_toolkit_file(&name) {
                copy_file(&path, &target_dir.join(file_name));
            } else if path.is_dir() {
                copy_dir_contents(&path, &target_dir.join(file_name));
            }
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
