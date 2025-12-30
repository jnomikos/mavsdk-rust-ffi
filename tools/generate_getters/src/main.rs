// Generation steps:
// 1. IR: Parse C++ files and build an intermediary representation of structs and their fields.
// 2. Codegen: From the IR, generate C++ and Rust getter functions.

mod ir;
mod generator;

use clang::{EntityKind};
use std::collections::BTreeMap;
use std::io;
use std::path::Path;
use std::fs::{self};
use std::process::Command;
use log::{debug, info, warn, error};
use clang::Clang;

use crate::ir::{StructIR};

fn visit_entity<'tu>(
    entity: &clang::Entity<'tu>, 
    target_file: &str, 
    collected_structs: &mut Vec<clang::Entity<'tu>>
) {
    info!("Visiting entity: {:?}", entity.get_name());
    // Only process entities defined in the target file
    if let Some(location) = entity.get_location() {
        if let Some(file) = location.get_file_location().file {
            if file.get_path().to_str() != Some(target_file) {
                // Not in our target file, skip
                return;
            }
        }
    }

    for child in entity.get_children() {
        if child.get_kind() == EntityKind::StructDecl
            && !ir::struct_members_are_all_pod(&child)
        {
            collected_structs.push(child);
        }
    }

    // Recurse into child entities
    for child in entity.get_children() {
        visit_entity(&child, target_file, collected_structs);
    }
}

fn parse_file(
    module_name: &str,
    target_file: &str,
    clang_args:  &[String],
    all_modules: &mut BTreeMap<String, Vec<StructIR>>,
) {
    let clang = match Clang::new() {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to initialize Clang: {}", e);
            std::process::exit(1);
        }
    };
    let index = clang::Index::new(&clang, false, true);
    let tu = index.parser(target_file)
        .arguments(&clang_args)
        .parse()
        .expect("Failed to parse file");

    // Print diagnostics to see errors/warnings
    let diagnostics = tu.get_diagnostics();
    if !diagnostics.is_empty() {
        warn!("Diagnostics for {}:", target_file);
        for diag in diagnostics {
            warn!("{}", diag);
        }
    } else {
        debug!("No diagnostics for {}", target_file);
    }
    
    // Collect all structs that need getters
    let mut collected_structs = Vec::new();
    visit_entity(&tu.get_entity(), target_file, &mut collected_structs);

    // Either create new module entry or append to existing
    let module_vec = all_modules.entry(module_name.to_string()).or_insert_with(Vec::new);

    for struct_info in &collected_structs {
        debug!("Collected struct: {}", struct_info.get_name().unwrap_or_default());
        // Generate IR for the struct
        if let Some(struct_ir) = ir::parse_struct(struct_info, module_name) {
            module_vec.push(struct_ir);
        }
    }


}

fn parse_from_files_in_dir(
    dir: &Path,
    cb: &mut dyn FnMut(&str),
    recursive: bool
) -> io::Result<()> {
    if !dir.is_dir() {
       return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && recursive {
            parse_from_files_in_dir(&path, cb, recursive)?;
        } else if path.is_file() {
            // Convert path to string
            if let Some(path_str) = path.to_str() {
                cb(path_str);
            }
        }
    }
    Ok(())
}

// TODO: Go through and ensure compatability with multiple operating systems. Confirmed working on Ubuntu Linux.
fn setup_clang(clang_args: &mut Vec<String>) {
    // Check if LIBCLANG_PATH is set
    if let Ok(libclang_path) = std::env::var("LIBCLANG_PATH") {
        info!("Using LIBCLANG_PATH: {}", libclang_path);
    } else {
        warn!("LIBCLANG_PATH env var is not set. If you encounter issues, please set it to the directory containing your libclang shared library. e.g /usr/lib/llvm-18/lib");
    }

    clang_args.extend_from_slice(&[
        "-I.".to_string(),
        "-x".to_string(), "c++".to_string(), "-std=c++17".to_string(),
    ]);

    if cfg!(target_os = "linux") {
        info!("Detected Linux target");
        // Try to get the GCC version
        if let Ok(output) = Command::new("g++").arg("-dumpversion").output() {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            info!("GCC version: {}", version);
            let gcc_base = format!("/usr/include/c++/{}", version);
            if Path::new(&gcc_base).exists() {
                clang_args.push(format!("-I{}", gcc_base));
                info!("Added: {}", gcc_base);
            } else {
                warn!("Path does not exist: {}", gcc_base);
            }
            let multiarch = format!("/usr/include/x86_64-linux-gnu/c++/{}", version);
            if Path::new(&multiarch).exists() {
                clang_args.push(format!("-I{}", multiarch));
                info!("Added: {}", multiarch);
            } else {
                warn!("Path does not exist: {}", multiarch);
            }
        } else {
            warn!("Failed to get GCC version, trying fallbacks");
            // Fallback: try common versions
            for version in &["11", "12", "13", "10", "9"] {
                let gcc_base = format!("/usr/include/c++/{}", version);
                if Path::new(&gcc_base).exists() {
                    clang_args.push(format!("-I{}", gcc_base));
                    info!("Added fallback: {}", gcc_base);
                    let multiarch = format!("/usr/include/x86_64-linux-gnu/c++/{}", version);
                    if Path::new(&multiarch).exists() {
                        clang_args.push(format!("-I{}", multiarch));
                        info!("Added fallback: {}", multiarch);
                    }
                    break;
                }
            }
        }
    }
}

fn main() {
    env_logger::init();
    let mut clang_args = Vec::new();
    setup_clang(&mut clang_args);


    let mavsdk_path = Path::new("MAVSDK/src/mavsdk");
    let core_include = mavsdk_path.join("core/include/mavsdk");
    let cxx_gen_path = Path::new("cxx/gen");
    let plugin_include = mavsdk_path.join("plugins");

    clang_args.push(format!("-I{}", core_include.to_str().unwrap()));

    let mut all_modules: BTreeMap<String, Vec<StructIR>> = BTreeMap::new();

    info!("Clang args: {:?}", clang_args);

    if cxx_gen_path.exists() {
        fs::remove_dir_all(&cxx_gen_path).unwrap();
    }
    fs::create_dir_all(&cxx_gen_path).unwrap();

    parse_from_files_in_dir(&core_include, &mut |entry: &str| {
        parse_file("core", &entry, &clang_args, &mut all_modules);
    }, false).unwrap();

    // Navigate each plugin in plugin_include and generate getters
    if let Ok(plugins) = fs::read_dir(&plugin_include) {
        for plugin in plugins {
            let Ok(plugin) = plugin else {
                continue;
            };
            let plugin_path = plugin.path();
            let plugin_name = plugin_path.file_name().unwrap().to_str().unwrap();
            let plugin_include_path = plugin_path.join("include").join("plugins").join(plugin_name);

            // Ensure include path exists
            if !plugin_include_path.exists() {
                continue;
            }
            
            parse_from_files_in_dir(&plugin_include_path, &mut |entry: &str| {
                parse_file(plugin_name, &entry, &clang_args, &mut all_modules);
            }, false).unwrap();
        }
    }

    // Generator step, iterate over all modules and generate files
    for (module_name, structs) in &all_modules {
        if structs.is_empty() {
            continue;
        }
        let file_path = cxx_gen_path.join(format!("{}_getters", module_name));
        generator::generate_rust_file(&file_path.with_extension("rs"), module_name, structs).expect("Failed to generate Rust file");
        generator::generate_cpp_file(&file_path.with_extension("h"), module_name, structs).expect("Failed to generate C++ file");
    }
}