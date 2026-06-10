use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    windows_reactor_setup::as_self_contained();

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let source_root = manifest_dir
        .parent()
        .unwrap_or(&manifest_dir)
        .join("CommunityToolkit.WinUI");
    let manifest_path = out_dir.join("app.manifest");
    add_toolkit_activation_to_manifest(&manifest_path, &source_root);
    println!("cargo:rustc-link-arg-examples=/MANIFEST:EMBED");
    println!(
        "cargo:rustc-link-arg-examples=/MANIFESTINPUT:{}",
        manifest_path.display()
    );

    let target_dir = target_dir_from_out(&out_dir);
    let examples_dir = target_dir.join("examples");
    copy_runtime_to_examples(&target_dir, &examples_dir);
    copy_toolkit_native_to_examples(&source_root, &examples_dir);
}

fn target_dir_from_out(out: &Path) -> PathBuf {
    out.ancestors().nth(3).unwrap_or(out).to_path_buf()
}

fn add_toolkit_activation_to_manifest(manifest_path: &Path, source_root: &Path) {
    let Ok(mut manifest) = fs::read_to_string(manifest_path) else {
        println!(
            "cargo:warning=Windows App SDK manifest not found: {}",
            manifest_path.display()
        );
        return;
    };

    if manifest.contains("XamlToolkit.WinUI.Controls.XamlMetaDataProvider") {
        return;
    }

    let mut toolkit_entries = String::new();
    for project in [
        "XamlToolkit.WinUI",
        "XamlToolkit.WinUI.Helpers",
        "XamlToolkit.WinUI.Converters",
        "XamlToolkit.WinUI.Controls",
    ] {
        let project_dir = source_root.join(project);
        let classes = collect_runtime_classes(&project_dir);
        if classes.is_empty() {
            println!(
                "cargo:warning=No runtime classes found in {}",
                project_dir.display()
            );
            continue;
        }

        toolkit_entries.push_str(&format!("    <asmv3:file name=\"{project}.dll\">\n"));
        for class in classes {
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

fn collect_runtime_classes(project_dir: &Path) -> Vec<String> {
    let mut files = Vec::new();
    collect_idl_files(project_dir, &mut files);

    let mut classes = Vec::new();
    for file in files {
        println!("cargo:rerun-if-changed={}", file.display());
        let Ok(content) = fs::read_to_string(&file) else {
            continue;
        };

        let mut namespace = String::new();
        for line in content.lines() {
            let clean = line.split("//").next().unwrap_or("").trim();
            if let Some(rest) = clean.strip_prefix("namespace ") {
                namespace = read_ident(rest);
                continue;
            }

            let Some(index) = clean.find("runtimeclass ") else {
                continue;
            };
            let name = read_ident(&clean[index + "runtimeclass ".len()..]);
            if !namespace.is_empty() && !name.is_empty() {
                classes.push(format!("{namespace}.{name}"));
            }
        }
    }

    classes.sort();
    classes.dedup();
    classes
}

fn collect_idl_files(dir: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_idl_files(&path, files);
        } else if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("idl"))
        {
            files.push(path);
        }
    }
}

fn read_ident(input: &str) -> String {
    input
        .trim_start()
        .chars()
        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_' || *ch == '.')
        .collect()
}

fn copy_runtime_to_examples(target_dir: &Path, examples_dir: &Path) {
    let _ = fs::create_dir_all(examples_dir);

    let Ok(entries) = fs::read_dir(target_dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let name_text = name.to_string_lossy();

        if name_text.eq_ignore_ascii_case("examples")
            || name_text.eq_ignore_ascii_case("build")
            || name_text.eq_ignore_ascii_case("deps")
            || name_text.eq_ignore_ascii_case("incremental")
            || name_text.eq_ignore_ascii_case(".fingerprint")
        {
            continue;
        }

        if path.is_file() && is_runtime_file(&name_text) {
            copy_file(&path, &examples_dir.join(name));
        } else if path.is_dir() && is_runtime_dir(&path) {
            copy_dir_contents(&path, &examples_dir.join(name));
        }
    }
}

fn is_runtime_file(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.ends_with(".dll") || lower.ends_with(".pri")
}

fn is_runtime_dir(path: &Path) -> bool {
    if path
        .file_name()
        .is_some_and(|name| name == "Microsoft.UI.Xaml")
    {
        return true;
    }

    let Ok(entries) = fs::read_dir(path) else {
        return false;
    };

    entries.flatten().any(|entry| {
        entry
            .path()
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.eq_ignore_ascii_case("Microsoft.ui.xaml.dll.mui"))
    })
}

fn copy_toolkit_native_to_examples(source_root: &Path, examples_dir: &Path) {
    for project in [
        "XamlToolkit.WinUI",
        "XamlToolkit.WinUI.Helpers",
        "XamlToolkit.WinUI.Converters",
        "XamlToolkit.WinUI.Controls",
    ] {
        let native_dir = source_root
            .join(project)
            .join("ARM64")
            .join("Release")
            .join(project);
        println!("cargo:rerun-if-changed={}", native_dir.display());
        copy_native_project_to_examples(&native_dir, examples_dir);
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
                if name.eq_ignore_ascii_case("Microsoft.WindowsAppRuntime.Bootstrap.dll") {
                    continue;
                }
                copy_file(&path, &examples_dir.join(file_name));
            }
        } else if path.is_dir() {
            copy_dir_contents(&path, &examples_dir.join(file_name));
        }
    }
}

fn should_copy_toolkit_file(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.ends_with(".dll") || lower.ends_with(".pri") || lower.ends_with(".winmd")
}

fn copy_dir_contents(src: &Path, dest: &Path) {
    let _ = fs::create_dir_all(dest);
    let Ok(entries) = fs::read_dir(src) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let dest_path = dest.join(entry.file_name());
        if path.is_file() {
            copy_file(&path, &dest_path);
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
