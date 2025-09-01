fn main() -> miette::Result<()> {
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
        "gripper",
        "info",
        "log_files",
        "log_streaming",
        "manual_control",
        "mavlink_passthrough",
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
        "-IMAVSDK/src/mavsdk/core/include/mavsdk".to_string(),
        "-I/usr/include/c++/11".to_string(),
        "-I/usr/include/x86_64-linux-gnu/c++/11".to_string(),
    ];
    
    for plugin in &plugin_includes {
        extra_clang_args.push(format!("-IMAVSDK/src/mavsdk/plugins/{plugin}/include/plugins/{plugin}"));
    }

    let extra_clang_args_refs: Vec<&str> = extra_clang_args.iter().map(|s| s.as_str()).collect();
    
    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&path])
        .extra_clang_args(&extra_clang_args_refs)
        .build()?;


    for plugin in plugin_includes {
        b.include(format!("MAVSDK/src/mavsdk/plugins/{plugin}/include/plugins/{plugin}"));
    }

    b.flag_if_supported("-std=c++17")
        .flag_if_supported("-Wno-address-of-packed-member")
        .include("MAVSDK/src/mavsdk/core/include/mavsdk")
        .include("MAVSDK/src/mavsdk/plugins")
        .file("cxx/mavsdk_shim.cpp")
        .compile("autocxx-mavssdk-example");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rustc-link-search=native=MAVSDK/build");
    // Link dynamic library
    println!("cargo:rustc-link-lib=dylib=mavsdk");
    Ok(())
}