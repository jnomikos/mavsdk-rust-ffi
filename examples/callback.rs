use mavsdk_rust_ffi::core;
use autocxx::prelude::*;

fn system_callback() {
    println!("New system detected");
}

fn main() {
    let config = core::mavsdk::Mavsdk_Configuration::new(1, 1, true).within_unique_ptr();
    let mut mavsdk_instance = core::mavsdk::Mavsdk::new1(&config).within_unique_ptr();
    cxx::let_cxx_string!(conn_str = "serial:///dev/ttyACM0:115200");
    mavsdk_instance.pin_mut().add_any_connection(&conn_str, core::mavsdk::ForwardingOption::ForwardingOff);

    let sys_cb_ptr = system_callback as usize;
    unsafe {
        let system_handle = core::subscriptions::subscribe_on_new_system(mavsdk_instance.as_mut_ptr(), sys_cb_ptr as libc::uintptr_t);

        // Await until we get system
        std::thread::sleep(std::time::Duration::from_secs(10));
        core::subscriptions::unsubscribe_on_new_system(mavsdk_instance.as_mut_ptr(),system_handle);
    }
}