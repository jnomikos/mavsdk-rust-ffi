use autocxx::prelude::*;
use mavsdk_rust_ffi::core;
use mavsdk_rust_ffi::arm_authorizer_server;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: arm_authorizer_server <connection_path>");
        eprintln!("Example: arm_authorizer_server udpin://0.0.0.0:14030");
        process::exit(1);
    }

    let config = core::mavsdk::Mavsdk_Configuration::new1(
        core::mavsdk::ComponentType::GroundStation,
    )
    .within_unique_ptr();

    let mut mavsdk_instance = core::mavsdk::Mavsdk::new1(&config)
        .within_unique_ptr();

    cxx::let_cxx_string!(conn_str = &args[1]);
    mavsdk_instance.pin_mut().add_any_connection(&conn_str, core::mavsdk::ForwardingOption::ForwardingOff);
}
