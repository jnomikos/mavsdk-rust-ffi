use std::env;
use std::process::Command;
use std::path::Path;

fn main() -> miette::Result<()> {
    let dst = cmake::Config::new("MAVSDK")
        .profile("Release")
        .build();

    let src_include = "MAVSDK/src/mavsdk/core/include/mavsdk";
    let generated_include = dst.join("build/src/mavsdk/core/include/mavsdk");
    let mavlink_include = dst.join("build/third_party/mavlink/mavlink/src/mavlink-build/include");
    let lib_dir = dst.join("lib");
    let path = std::path::PathBuf::from("MAVSDK/src");

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
        "geofence", 
        "gimbal", 
        "gripper", 
        "info",
        "log_files", 
        "log_streaming", 
        "manual_control", 
        "mavlink_direct",
        //"mavlink_passthrough", 
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

    let mut b = autocxx_build::Builder::new("src/lib.rs", &["cxx", "cxx/gen", path.to_str().unwrap()])
        .extra_clang_args(&extra_clang_args_refs)
        .build()?;

    for plugin in plugin_includes {
        b.include(format!("MAVSDK/src/mavsdk/plugins/{plugin}/include/plugins/{plugin}"));
    }

    b.flag_if_supported("-std=c++17")
        .flag_if_supported("-Wno-address-of-packed-member")
        .include("MAVSDK/src/mavsdk/core/include/mavsdk")
        .include("MAVSDK/src/mavsdk/plugins")
        .include(src_include)
        .include(&generated_include)
        .include(&mavlink_include)
        .compile("autocxx-mavssdk-example");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=mavsdk");
    Ok(())
}