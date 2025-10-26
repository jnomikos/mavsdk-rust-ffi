/*fn main() {
    println!("Hello, world!");
    let config = core::mavsdk::Mavsdk_Configuration::new(1, 1, true).within_unique_ptr();
    // Must use new1 because this is mavsdk::Mavsdk's second constructor
    let mut mavsdk_instance = core::mavsdk::Mavsdk::new1(&config).within_unique_ptr();

    cxx::let_cxx_string!(conn_str = "serial:///dev/ttyACM0:115200");
    let mavsdk_connection_result = mavsdk_instance.pin_mut().add_any_connection(&conn_str, core::mavsdk::ForwardingOption::ForwardingOff);

    let mavsdk_version = mavsdk_instance.version();
    println!("MAVSDK Version: {}", mavsdk_version);


    let number: c_int = autocxx::c_int(5);


    extern "C" fn my_callback(userdata: *mut std::ffi::c_void) {
        println!("New system discovered! Userdata: {:?}", userdata);
    }

    unsafe {
        core::mavsdk_wait_on_new_system(
            mavsdk_instance.as_mut_ptr() as *mut autocxx::c_void,
        );
    }
    println!("New system discovered!");

    // Get the first system
    let system_index: usize = 0;
    unsafe {
        let system_ptr = core::mavsdk_system_get(mavsdk_instance.as_mut_ptr(), system_index);

        // Now that we have the system pointer, let's create a new instance of Action
        let action_instance = action::mavsdk::Action::new1(system_ptr);
    }
}*/

use mavsdk_rust_ffi::core;
use autocxx::prelude::*;
fn main() {
    let config = core::mavsdk::Mavsdk_Configuration::new(1, 1, true).within_unique_ptr();

    // Must use new1 because this is mavsdk::Mavsdk's second constructor
    let mavsdk_instance = core::mavsdk::Mavsdk::new1(&config).within_unique_ptr();

    let mavsdk_version = mavsdk_instance.version();
    println!("MAVSDK Version: {}", mavsdk_version);
}
