use mavsdk_rust_ffi::core;
use mavsdk_rust_ffi::mavlink_direct;
use autocxx::prelude::*;

#[unsafe(no_mangle)]
pub extern "C" fn message_callback_ffi(msg: *const core::mavsdk::Mavsdk_MavlinkMessage) -> bool {
    // SAFETY: Only if msg is valid and not null
    if msg.is_null() {
        return false;
    }
    println!("Incoming JSON message received");
    true
}

fn main() {
    let config = core::mavsdk::Mavsdk_Configuration::new(
        1, // System ID
        1, // Component ID
        true // Always send heartbeats
    ).within_unique_ptr();
    let mut mavsdk_instance = core::mavsdk::Mavsdk::new1(&config).within_unique_ptr();
    cxx::let_cxx_string!(conn_str = "serial:///dev/ttyACM0:115200");
    mavsdk_instance.pin_mut().add_any_connection(&conn_str, core::mavsdk::ForwardingOption::ForwardingOff);

    let msg_cb_ptr = message_callback_ffi as usize;
    unsafe {
        let msg_handle = core::core_subscriptions::subscribe_incoming_messages_json(
            mavsdk_instance.as_mut_ptr(),
            msg_cb_ptr as libc::uintptr_t,
        );

        // Await until we get system
        let system = core::Mavsdk_first_autopilot(
            mavsdk_instance.as_mut_ptr(),
            10.0, // Timeout in s
        );
        if system.is_null() {
            println!("No system found, exiting");
            return;
        }
        
        let mavlink_direct_plugin = mavlink_direct::mavsdk::MavlinkDirect::new(system).within_unique_ptr();


        core::core_subscriptions::unsubscribe_incoming_messages_json(
            mavsdk_instance.as_mut_ptr(),
            msg_handle,
        );
    }
}