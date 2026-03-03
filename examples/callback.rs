use std::env;
use autocxx::prelude::*;
use mavsdk_rust_ffi::core;
use mavsdk_rust_ffi::mavlink_direct;
use mavsdk_rust_ffi::wrappers;
use mavsdk_rust_ffi::wrappers::telemetry::*;
use mavsdk_rust_ffi::telemetry;

fn usage() {
    eprintln!(r#"
    Usage: {} <connection_url> 
    Connection URL format should be :
     For TCP server: tcpin://<our_ip>:<port>
     For TCP client: tcpout://<remote_ip>:<port>
     For UDP server: udpin://<our_ip>:<port>
     For UDP client: udpout://<remote_ip>:<port>
     For Serial : serial://</path/to/serial/dev>:<baudrate>]
    For example, to connect to the simulator use URL: udpin://0.0.0.0:14540
    "#, env::args().next().unwrap());
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
        return;
    }
    println!("Starting MAVSDK instance with Telemetry plugin...");
    let config = core::mavsdk::Mavsdk_Configuration::new(
        1, // System ID
        1, // Component ID
        true // Always send heartbeats
    ).within_unique_ptr();
    let mut mavsdk_instance = core::mavsdk::Mavsdk::new1(&config).within_unique_ptr();
    cxx::let_cxx_string!(conn_str = args[1].as_str());
    mavsdk_instance.pin_mut().add_any_connection(&conn_str, core::mavsdk::ForwardingOption::ForwardingOff);

    // Await until we get system
    let system = unsafe {
        core::Mavsdk_first_autopilot(
            mavsdk_instance.as_mut_ptr(),
            10.0, // Timeout in s
        )
    };
    if system.is_null() {
        println!("No system found, exiting");
        return;
    }

    let mut telemetry = telemetry::mavsdk::Telemetry::new(system).within_unique_ptr();
    let mut telemetry_client = TelemetryClient::new(telemetry);
    let mut position_rx = telemetry_client.subscribe_position();
    tokio::spawn(async move {
        while position_rx.changed().await.is_ok() {
            let latest_pos = position_rx.borrow();
            if let Some(pos) = latest_pos.as_ref() {
                println!("Received position update: lat {}, lon {}, alt {} m", pos.latitude_deg, pos.longitude_deg, pos.absolute_altitude_m);
            }
        }
        println!("Position subscription ended");
    });
    
    tokio::signal::ctrl_c().await.expect("failed to listen for event");
}