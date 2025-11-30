use clang::{Clang, EntityKind};
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::fs::{self};
use std::process::Command;
use log::{info, warn, error, debug};

#[derive(Debug, Clone)]
struct FieldInfo {
    name: String,
    cpp_type: String,
    is_pod: bool,
}

#[derive(Debug)]
struct StructInfo {
    name: String,
    qualified_name: String,
    parent_name: String,
    fields: Vec<FieldInfo>,
}

fn get_qualified_name(entity: &clang::Entity) -> Option<String> {
    // Get the qualified name which includes namespace
    entity.get_name().map(|name| {
        let mut qualified_parts = Vec::new();
        let mut current = entity.clone();
        
        // Walk up the parent chain to collect namespace/class names
        while let Some(parent) = current.get_semantic_parent() {
            match parent.get_kind() {
                EntityKind::Namespace | EntityKind::ClassDecl | EntityKind::StructDecl => {
                    if let Some(parent_name) = parent.get_name() {
                        qualified_parts.push(parent_name);
                    }
                }
                _ => break,
            }
            current = parent;
        }
        
        // Reverse to get correct order (namespace::class::struct)
        qualified_parts.reverse();
        qualified_parts.push(name);
        qualified_parts.join("::")
    })
}

fn struct_members_are_all_pod(entity: &clang::Entity) -> bool {
    for child in entity.get_children() {
        if child.get_kind() != EntityKind::FieldDecl {
            continue;
        }
        if let Some(field_type) = child.get_type() {
            if field_type.is_pod() {
                debug!("Member {} is POD", child.get_name().unwrap_or_default());
                continue;
            }

            // If it's a struct, recursively check its members
            if let Some(decl) = field_type.get_declaration() {
                debug!("Member {} is a struct, checking members", child.get_name().unwrap_or_default());
                if decl.get_kind() == EntityKind::StructDecl {
                    debug!("Struct {} found, checking if all members are POD", decl.get_name().unwrap_or_default());
                    if !struct_members_are_all_pod(&decl) {
                        debug!("Struct {} has non-POD members", decl.get_name().unwrap_or_default());
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    true
}

fn generate_cpp_getters(struct_info: &StructInfo) -> String {
    let mut content = String::new();
    
    content.push_str(&format!("\tstruct {} {{\n", struct_info.name));
    content.push_str(&format!("\t\t{}() = delete;\n", struct_info.name));
    
    for field in &struct_info.fields {
        let ret_prefix = if field.is_pod {
            format!("static {}", field.cpp_type)
        } else {
            format!("static const {}&", field.cpp_type)
        };
        
        content.push_str(&format!(
            "\t\t{} get_{}(const {}& s) {{ return s.{}; }}\n",
            ret_prefix, field.name, struct_info.qualified_name, field.name
        ));
    }
    
    content.push_str(&format!("\t}};\n"));
    content
}

fn analyze_struct(entity: &clang::Entity) -> Option<StructInfo> {
    let qualified_name = get_qualified_name(entity)?;
    let struct_name = entity.get_name()?;
    let struct_parent_name = entity
        .get_semantic_parent()
        .and_then(|p| p.get_name())
        .unwrap_or_default();

    let mut fields = Vec::new();
    let mut all_members_pod = true;

    for field in entity.get_children() {
        if field.get_kind() == EntityKind::FieldDecl {
            let field_name = field.get_name().unwrap();
            let field_type = field.get_type().unwrap();
            let display_name = field_type.get_display_name();
            let field_type_name = if display_name.contains('<') {
                // Complex/template type (e.g., vector)
                field_type.get_canonical_type().get_display_name()
            } else {
                // Simple type
                display_name
            };

            let field_is_pod = field_type.is_pod();
            debug!("Field: {} of type {} is_pod: {}", field_name, field_type_name, field_is_pod);

            // Check if this field affects whether we need getters
            if let Some(field_type_decl) = field_type.get_declaration() {
                if field_type_decl.get_kind() == EntityKind::StructDecl {
                    debug!("Field {} is a struct, checking if all members are pod", field_name);
                    if !struct_members_are_all_pod(&field_type_decl) {
                        all_members_pod = false;
                        debug!("Field {} has non-POD members", field_name);
                    }
                } else if !field_is_pod {
                    debug!("Field {} is non-POD type", field_name);
                    all_members_pod = false;
                }
            } else if !field_is_pod {
                debug!("Field {} is non-POD type with no declaration", field_name);
                all_members_pod = false;
            }

            fields.push(FieldInfo {
                name: field_name,
                cpp_type: field_type_name,
                is_pod: field_is_pod,
            });
        }
    }

    // Only return struct info if we actually need getters
    if all_members_pod {
        debug!("Skipping generation for {} as all members are POD", qualified_name);
        return None;
    }

    Some(StructInfo {
        name: struct_name,
        qualified_name,
        parent_name: struct_parent_name,
        fields,
    })
}

fn visit_entity(
    entity: &clang::Entity, 
    target_file: &str, 
    collected_structs: &mut Vec<StructInfo>
) -> i32 {
    // Only process entities defined in the target file
    if let Some(location) = entity.get_location() {
        if let Some(file) = location.get_file_location().file {
            if file.get_path().to_str() != Some(target_file) {
                // Not in our target file, skip
                return 0;
            }
        }
    }

    let mut gen_count = 0;

    debug!(
        "Visiting entity: {:?}, kind: {:?}, is_pod: {}, location: {:?}",
        entity.get_name(),
        entity.get_kind(),
        entity.get_type().map_or(false, |ty| ty.is_pod()),
        entity.get_location()
    );

    // Process struct declarations
    match entity.get_kind() {
        EntityKind::StructDecl if entity.get_type().map_or(false, |ty| !ty.is_pod()) => {
            if let Some(struct_info) = analyze_struct(entity) {
                debug!("Struct: {}", struct_info.qualified_name);
                gen_count += struct_info.fields.len() as i32;
                collected_structs.push(struct_info);
                debug!("Collected struct for getter generation");
            }
        }
        _ => {}
    }

    // Recurse into children
    for child in entity.get_children() {
        gen_count += visit_entity(&child, target_file, collected_structs);
    }

    gen_count
}

fn write_getters_to_file(
    collected_structs: &[StructInfo], 
    write_file: &mut fs::File
) -> Result<(), std::io::Error> {
    let mut namespaces_written = std::collections::HashSet::new();
    
    for struct_info in collected_structs {
        // Write namespace declaration if not already written
        if !namespaces_written.contains(&struct_info.parent_name) {
            writeln!(write_file, "namespace {}Getters {{", struct_info.parent_name)?;
            namespaces_written.insert(struct_info.parent_name.clone());
        }
        
        // Generate and write the getter code
        let getter_code = generate_cpp_getters(struct_info);
        write_file.write_all(getter_code.as_bytes())?;
        
        debug!("Generated getters for {}", struct_info.qualified_name);
    }
    
    // Close all namespaces
    for ns in namespaces_written {
        writeln!(write_file, "}} // namespace {}Getters", ns)?;
    }
    
    Ok(())
}

fn parse_file(target_file: &str, write_to: &Path, clang_args: &[String]) {
    debug!("Parsing file: {}", target_file);
    debug!("Output file: {:?}", write_to);
    
    let clang = Clang::new().unwrap();
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
    let gen_count = visit_entity(&tu.get_entity(), target_file, &mut collected_structs);

    if gen_count == 0 {
        // Remove the file if nothing was generated
        if write_to.exists() {
            fs::remove_file(write_to).unwrap();
        }
        return;
    }

    // Create output file and write header
    let mut write_file = fs::File::create(write_to).expect("Unable to create output file");
    writeln!(write_file, "// WARNING: THIS FILE IS AUTOGENERATED! As such, it should not be edited.").unwrap();
    writeln!(write_file, "#pragma once").unwrap();

    // Write all the getter code
    if let Err(e) = write_getters_to_file(&collected_structs, &mut write_file) {
        error!("Failed to write getters to file: {}", e);
        return;
    }

    info!("Generated {} getters for {}", gen_count, target_file);
}

fn generate_from_files_in_dir(dir: &Path, cb: &dyn Fn(&str), recursive: bool) -> io::Result<()> {
    if !dir.is_dir() {
       return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && recursive {
            generate_from_files_in_dir(&path, cb, recursive)?;
        } else if path.is_file() {
            // Convert path to string
            if let Some(path_str) = path.to_str() {
                cb(path_str);
            }
        }
    }
    Ok(())
}

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    let mut clang_args = vec![
        "-I.".to_string(), // your current directory (project headers)
        "-I/usr/lib/llvm-14/lib/clang/14.0.0/include".to_string(),
        "-x".to_string(), "c++".to_string(), "-std=c++17".to_string(),
    ];

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


    let mavsdk_path = Path::new("../../MAVSDK/src/mavsdk");
    let core_include = mavsdk_path.join("core/include/mavsdk");
    let cxx_gen_path = Path::new("../../cxx/gen");
    let mavsdk_cxx_gen_path = cxx_gen_path.join("core_getters.h");
    let plugin_include = mavsdk_path.join("plugins");

    clang_args.push(format!("-I{}", core_include.to_str().unwrap()));

    // Debug clang args
    info!("Clang args: {:?}", clang_args);

    if cxx_gen_path.exists() {
        fs::remove_dir_all(&cxx_gen_path).unwrap();
    }
    fs::create_dir_all(&cxx_gen_path).unwrap();

    generate_from_files_in_dir(&core_include, &|entry: &str| {
        parse_file(&entry, &mavsdk_cxx_gen_path, &clang_args);
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
            
            let plugin_cxx_gen_path = cxx_gen_path.join(format!("{}_getters.h", plugin_name));
            generate_from_files_in_dir(&plugin_include_path, &|entry: &str| {
                parse_file(&entry, &plugin_cxx_gen_path, &clang_args);
            }, false).unwrap();
        }
    }
}