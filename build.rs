fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("MAVSDK/src");
    
    let mut b = autocxx_build::Builder::new("src/main.rs", &[&path]).extra_clang_args(&["-std=c++17", 
                                            "-I/usr/include/c++/11",
                                            "-I/usr/include/x86_64-linux-gnu/c++/11"
    ])
    .build()?;
    
    b.flag_if_supported("-std=c++17")
        .flag_if_supported("-Wno-address-of-packed-member")
        .file("MAVSDK/src/mavsdk/core/mavsdk.cpp")
        .include("MAVSDK/src/mavsdk/core/include/mavsdk")
        .compile("autocxx-mavssdk-example");
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rustc-link-search=native=MAVSDK/build");
    // Link dynamic library
    println!("cargo:rustc-link-lib=dylib=mavsdk");
    Ok(())
}