use std::env;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::fs;
use std::io;

fn copy_recursively_if_changed(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let entry_type = entry.file_type()?;
        let dest_path = destination.as_ref().join(entry.file_name());

        if entry_type.is_dir() {
            copy_recursively_if_changed(entry.path(), dest_path)?;
        } else {
            // Check if we need to copy to avoid touching timestamps unnecessarily
            let should_copy = match fs::metadata(&dest_path) {
                Ok(dest_meta) => {
                    let src_meta = entry.metadata()?;
                    // Only copy if the source file is newer than the destination file
                    src_meta.modified()? > dest_meta.modified()?
                }
                Err(_) => true, // Destination doesn't exist, so we must copy
            };

            if should_copy {
                fs::copy(entry.path(), dest_path)?;
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let src_include = "MAVSDK/src/mavsdk/core/include/mavsdk";
    let generated_src = Path::new("generated-mavsdk/mavsdk");

    if generated_src.exists() {
        copy_recursively_if_changed("MAVSDK", &out_dir.join("MAVSDK"))?;
        copy_recursively_if_changed(generated_src, &out_dir.join("MAVSDK/src/mavsdk"))?;
    } else {
        eprintln!("cargo:warning=Generated MAVSDK source not found; please run the code generator first.");
        std::process::exit(1);
    }

    let mavsdk_src_path = out_dir.join("MAVSDK/src");

    let dst = cmake::Config::new(&out_dir.join("MAVSDK"))
        .profile("Release")
        .define("BUILD_TESTING", "OFF") // The testing is incompatible with our modifications
        .build();

    let generated_include = dst.join("build/src/mavsdk/core/include/mavsdk");
    let mavlink_include = dst.join("build/third_party/mavlink/mavlink/src/mavlink-build/include");
    let lib_dir = dst.join("lib");

    let plugin_includes = vec![
        "action",
        "action_server",
        "arm_authorizer_server", 
        "calibration", 
        "camera",
        "camera_server", 
        "component_metadata", 
        "component_metadata_server", 
        "events",
        "failure", 
        "ftp", 
        "ftp_server",
        "follow_me",
        "geofence", 
        "gimbal", 
        "gripper", 
        "info",
        "log_files", 
        "log_streaming", 
        "manual_control", 
        "mavlink_direct",
        "mission", 
        "mission_raw", 
        "mission_raw_server",
        "mocap", 
        "offboard", 
        "param", 
        "param_server", 
        "rtk", 
        "server_utility",
        "shell", 
        "telemetry", 
        "telemetry_server",
        "transponder", 
        "tune", 
        "winch"
    ];

    let mut extra_clang_args = vec![
        "-std=c++17".to_string(),
        format!("-I{}", src_include),
        format!("-I{}", generated_include.display()),
        format!("-I{}", mavlink_include.display()),
    ];

    // Dynamically detect C++ standard library include paths on Linux
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux") {
        // Try to get the GCC version
        if let Ok(output) = Command::new("g++").arg("-dumpversion").output() {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let gcc_base = format!("/usr/include/c++/{}", version);
            if Path::new(&gcc_base).exists() {
                extra_clang_args.push(format!("-I{}", gcc_base));
            }
            let multiarch = format!("/usr/include/x86_64-linux-gnu/c++/{}", version);
            if Path::new(&multiarch).exists() {
                extra_clang_args.push(format!("-I{}", multiarch));
            }
        }
    }

    // Windows: usually MSVC/Clang auto-detects

    for plugin in &plugin_includes {
        extra_clang_args.push(format!("-IMAVSDK/src/mavsdk/plugins/{plugin}/include/plugins/{plugin}"));
    }

    let extra_clang_args_refs: Vec<&str> = extra_clang_args.iter().map(|s| s.as_str()).collect();

    let mut b = autocxx_build::Builder::new("src/lib.rs", &["cxx", mavsdk_src_path.to_str().unwrap()])
        .extra_clang_args(&extra_clang_args_refs)
        .build()?;

    for plugin in plugin_includes {
        b.include(format!("MAVSDK/src/mavsdk/plugins/{plugin}/include/plugins/{plugin}"));
    }

    b.flag_if_supported("-std=c++17")
        .flag_if_supported("-Wno-address-of-packed-member")
        .include(src_include)
        .include(&generated_include)
        .include(&mavlink_include)
        .compile("autocxx-mavsdk");

    // Watch rust source files for changes
    println!("cargo:rerun-if-changed=src/lib.rs");
    // Watch the original C++ source files
    println!("cargo:rerun-if-changed=MAVSDK");
    println!("cargo:rerun-if-changed=generated-mavsdk/mavsdk");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=mavsdk");
    Ok(())
}