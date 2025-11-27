use mavsdk_rust_ffi::core;
use autocxx::prelude::*;

fn system_callback() {
    println!("New system detected");
}

#[unsafe(no_mangle)]
pub extern "C" fn message_callback_ffi(msg: *const core::mavsdk::Mavsdk_MavlinkMessage) -> bool {
    // SAFETY: Only if msg is valid and not null
    if msg.is_null() {
        return false;
    }
    println!("Incoming JSON message received");
    // Print out the message as JSON
    unsafe {
        let msg_ref = &*msg;
        
        match msg_ref.message_name.to_str() { 
            Ok(name) => {
                println!("Incoming MAVLink message: {}", name);
            }
            Err(e) => {
                println!("Error decoding string: {:?}", e);
            }
        }

        // For the JSON string, do the same:
        if let Ok(json) = msg_ref.fields_json.to_str() {
             println!("JSON Content: {}", json);
        }
    }
    true
}

fn main() {
    let config = core::mavsdk::Mavsdk_Configuration::new(1, 1, true).within_unique_ptr();
    let mut mavsdk_instance = core::mavsdk::Mavsdk::new1(&config).within_unique_ptr();
    cxx::let_cxx_string!(conn_str = "serial:///dev/ttyACM0:115200");
    mavsdk_instance.pin_mut().add_any_connection(&conn_str, core::mavsdk::ForwardingOption::ForwardingOff);

    let sys_cb_ptr = system_callback as usize;
    unsafe {
        let system_handle = core::core_subscriptions::subscribe_on_new_system(mavsdk_instance.as_mut_ptr(), sys_cb_ptr as libc::uintptr_t);

        let msg_cb_ptr = message_callback_ffi as usize;
        let msg_handle = core::core_subscriptions::subscribe_incoming_messages_json(
            mavsdk_instance.as_mut_ptr(),
            msg_cb_ptr as libc::uintptr_t,
        );

        // Await until we get system
        std::thread::sleep(std::time::Duration::from_secs(10));
        core::core_subscriptions::unsubscribe_on_new_system(mavsdk_instance.as_mut_ptr(),system_handle);
        core::core_subscriptions::unsubscribe_incoming_messages_json(
            mavsdk_instance.as_mut_ptr(),
            msg_handle,
        );
    }
}