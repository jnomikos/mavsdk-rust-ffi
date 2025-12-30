// Intermediary representation of structs and their fields. For IR step of codegen.

use clang::{EntityKind, TypeKind};
use log::{error, debug};

#[derive(Debug, Clone)]
pub struct StructIR {
    pub name: String,
    pub semantic_name: String,
    pub semantic_name_cpp: String,
    pub fields: Vec<FieldIR>,
}

#[derive(Debug, Clone)]
pub struct FieldIR {
    pub name: String,
    pub rust_type_name: String,
    pub cpp_type_name: String,
    pub is_pod: bool,
}

pub fn struct_members_are_all_pod(entity: &clang::Entity) -> bool {
    for child in entity.get_children() {
        if child.get_kind() != EntityKind::FieldDecl {
            continue;
        }
        if let Some(field_type) = child.get_type() {
            if field_type.is_pod() {
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

pub fn parse_struct(entity: &clang::Entity, module_name: &str) -> Option<StructIR> {
    let name = entity.get_name().unwrap_or_default();
    let semantic_name = get_entity_semantic_name_ffi(module_name, entity);
    let semantic_name_cpp = get_entity_semantic_name(entity);
    
    let mut fields = Vec::new();
    for child in entity.get_children() {
        if child.get_kind() != EntityKind::FieldDecl {
            continue;
        }

        let field_type = match child.get_type() {
            Some(t) => t,
            None => continue, // skip fields with no type
        };
        
        let rust_type = resolve_rust_type(module_name, &child);
        let cpp_type = resolve_cpp_type(&field_type);

        let field_name = child.get_display_name().unwrap_or_default();
        let is_pod = field_type.is_pod();
        let field_ir = FieldIR {
            name: field_name,
            rust_type_name: rust_type,
            cpp_type_name: cpp_type,
            is_pod,
        };
        fields.push(field_ir);
    }

    Some(StructIR {
        name,
        semantic_name,
        semantic_name_cpp,
        fields,
    })
}

fn get_entity_semantic_name(entity: &clang::Entity) -> String {
    let mut parts = Vec::new();
    let mut current = Some(*entity);
    while let Some(ent) = current {
        let kind = ent.get_kind();
        if kind == EntityKind::TranslationUnit || 
            kind == EntityKind::NotImplemented ||
            kind == EntityKind::UnexposedDecl
        {
            break;
        }
        
        if let Some(name) = ent.get_name() {
            parts.push(name);
        }

        // semantic parent reflects C++ scoping, unlike lexical
        current = ent.get_semantic_parent();
    }
    parts.reverse();
    parts.join("::")
}

// In rust, we use '_' instead of '::' for module paths, but still use :: for namespaces
fn get_entity_semantic_name_ffi(module_name: &str, entity: &clang::Entity) -> String {
    let mut parts = Vec::new();
    let mut current = Some(*entity);
    while let Some(ent) = current {
        let kind = ent.get_kind();
        debug!("Entity kind: {:?}", kind);
        if kind == EntityKind::TranslationUnit || 
            kind == EntityKind::NotImplemented ||
            kind == EntityKind::UnexposedDecl
        {
            break;
        }
        
        if let Some(name) = ent.get_name() {
            parts.push(name);
        }

        if parts.len() > 1 {
            if kind == EntityKind::Namespace {
                parts.insert(parts.len() - 1, "::".to_string());
            } else {
                parts.insert(parts.len() - 1, "_".to_string());
            }
        }

        // semantic parent reflects C++ scoping, unlike lexical
        current = ent.get_semantic_parent();
    }
    parts.reverse();
    let joined = parts.join("");

    // Check if the entity is a primitive type
    let is_complex = entity.get_type()
        .map(|t| matches!(
            t.get_kind(),
            TypeKind::Elaborated | 
            TypeKind::Record | 
            TypeKind::Enum | 
            TypeKind::Vector
        ))
        .unwrap_or(false);

    if is_complex {
        format!("{}::{}", module_name, joined)
    } else {
        joined
    }
}

fn resolve_cpp_type(field_type: &clang::Type) -> String {
    let display_name = field_type.get_display_name();
    if display_name.contains('<') {
        let mut inner_type_name = String::new();
        if let Some(arg_types) = field_type.get_template_argument_types() {
            for arg_type in arg_types {
                if let Some(arg_type) = arg_type {
                    inner_type_name = format!("{}", resolve_cpp_type(&arg_type));
                    debug!("Template argument type: {}", inner_type_name);
                }
            }

            if display_name.starts_with("std::vector") {
                format!("std::vector<{}>", inner_type_name)
            }
            else {
                display_name
            }
        } else {
            display_name
        }
    } else {
        if let Some(declaration) = field_type.get_declaration() {
            let decl_name = get_entity_semantic_name(&declaration);
            if !decl_name.is_empty() {
                decl_name
            } else { 
                display_name 
            }
        } else {
            display_name
        }
    }
}

fn resolve_simple_rust_type(field_kind: TypeKind) -> Option<String> {
    match field_kind {
        TypeKind::Void => Some("()".to_string()),
        TypeKind::Bool => Some("bool".to_string()),
        TypeKind::CharS | TypeKind::SChar => Some("std::os::raw::c_schar".to_string()),
        TypeKind::CharU | TypeKind::UChar => Some("std::os::raw::c_uchar".to_string()),
        TypeKind::Short => Some("std::os::raw::c_short".to_string()),
        TypeKind::UShort => Some("std::os::raw::c_ushort".to_string()),
        TypeKind::Int => Some("std::os::raw::c_int".to_string()),
        TypeKind::UInt => Some("std::os::raw::c_uint".to_string()),
        TypeKind::Long => Some("std::os::raw::c_long".to_string()),
        TypeKind::ULong => Some("std::os::raw::c_ulong".to_string()),
        TypeKind::LongLong => Some("std::os::raw::c_longlong".to_string()),
        TypeKind::ULongLong => Some("std::os::raw::c_ulonglong".to_string()),
        TypeKind::Float => Some("f32".to_string()),
        TypeKind::Double => Some("f64".to_string()),
        TypeKind::Pointer => Some("usize".to_string()),
        _ => None,
    }
}

fn stdint_type_to_rust_type(stdint_type: &str) -> Option<String> {
    match stdint_type {
        "int8_t" => Some("i8".to_string()),
        "uint8_t" => Some("u8".to_string()),
        "int16_t" => Some("i16".to_string()),
        "uint16_t" => Some("u16".to_string()),
        "int32_t" => Some("i32".to_string()),
        "uint32_t" => Some("u32".to_string()),
        "int64_t" => Some("i64".to_string()),
        "uint64_t" => Some("u64".to_string()),
        _ => Some(stdint_type.to_string()),
    }
}

fn resolve_rust_type(module_name: &str, entity: &clang::Entity) -> String {
    let field_type = entity.get_type().unwrap_or_else(|| {
        error!("Entity {:?} has no type", entity.get_name());
        error!("Entity kind: {:?}", entity.get_kind());
        error!("entity details: {:?}", entity);
        std::process::exit(1);
    });
    let entity_kind = entity.get_kind();

    debug!("Processing entity: {:?} of kind {:?}", entity.get_name(), entity_kind);

    // libclang's definition technically determines some things as non-POD when they are POD in autocxx,
    // such as a struct with all POD members that have them declared as {}. So we instead check each member if struct.
    let field_type_is_pod = 
        if entity_kind == EntityKind::StructDecl {
            struct_members_are_all_pod(entity)
        } else {
            field_type.is_pod()
        };

    let display_name = field_type.get_display_name();
    let field_kind = field_type.get_kind();
    let declaration = field_type.get_declaration();
    let field_type_name = if display_name.contains('<') {
        field_type.get_canonical_type().get_display_name()
    } else {
        if let Some(decl) = declaration {
            let decl_name = get_entity_semantic_name_ffi(module_name, &decl);
            if !decl_name.is_empty() {
                decl_name
            } else { 
                display_name 
            }
        } else {
            display_name
        }
    };

    if let Some(decl) = declaration {
        if decl.get_kind() == EntityKind::TypeAliasDecl {
            return String::new(); // Skip type aliases
        }
    }

    debug!("Field type name: {}", field_type_name);
    debug!("Declaration: {:?}", declaration);
    match field_kind {
        TypeKind::Typedef => {
            if field_type_name == "std::string" || field_type_name == "std::basic_string<char>" {
                return "CxxString".to_string();
            }
            match declaration {
                Some(decl) => {
                    let decl_name = decl.get_name().unwrap_or_default();
                    debug!("Typedef found: {}", decl_name);
                    stdint_type_to_rust_type(&decl_name).unwrap_or_else(|| {
                        error!("Unsupported typedef type: {} for entity {:?}", decl_name, entity.get_name());
                        std::process::exit(1);
                    })
                },
                None => {
                    error!("Typedef without declaration");
                    std::process::exit(1);
                }
            }
        },
        TypeKind::Elaborated | TypeKind::Record | TypeKind::Enum | TypeKind::Vector => {
            debug!("Complex type found: {}", field_type_name);

            if field_type_is_pod {
                debug!("Field type is POD with name: {}", field_type_name);
                field_type_name
            }
            else if field_type_name == "std::string" || field_type_name == "std::basic_string<char>" {
                "CxxString".to_string()
            }
            else if field_type_name.contains("<") && field_type_name.contains(">") {
                // Loop through template argument types
                let mut inner_entity_type_name = String::new();
                if let Some(arg_types) = field_type.get_template_argument_types() {
                    for arg_type in arg_types {
                        if let Some(arg_type) = arg_type {
                            if let Some(inner_entity) = arg_type.get_declaration() {
                                inner_entity_type_name = format!("{}", resolve_rust_type(module_name, &inner_entity));

                                debug!("Template argument type: {}", inner_entity_type_name);
                            } else {
                                inner_entity_type_name = resolve_simple_rust_type(arg_type.get_kind()).unwrap_or_else(|| {
                                    error!("Unsupported template argument type kind: {:?} for entity {:?}", arg_type.get_kind(), entity.get_name());
                                    std::process::exit(1);
                                });
                            }
                        }
                    }

                    if inner_entity_type_name.is_empty() {
                        error!("Unable to determine inner type for template type: {}", field_type_name);
                        std::process::exit(1);
                    }

                    if field_type_name.starts_with("std::vector") {
                        format!("CxxVector<{}>", inner_entity_type_name)
                    }
                    else {
                        // Error
                        error!("Unsupported template type: {}", field_type_name);
                        std::process::exit(1);
                    }
                } else {
                    format!("{}", field_type_name)
                }
            }
            else {
                format!("{}", field_type_name)
            }
        },
        _ => {
            resolve_simple_rust_type(field_kind).unwrap_or_else(|| {
                error!("Unsupported field type kind: {:?} for entity {:?}", field_kind, entity.get_name());
                std::process::exit(1);
            })
        }
    }
}