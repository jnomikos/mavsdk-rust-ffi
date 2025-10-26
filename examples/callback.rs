use mavsdk_rust_ffi::base;
use autocxx::prelude::*;

fn system_callback(value: base::mavsdk::System) {
    println!("New system detected");
}

fn main() {
    let config = base::mavsdk::Mavsdk_Configuration::new(1, 1, true).within_unique_ptr();
    let mavsdk_instance = base::mavsdk::Mavsdk::new1(&config).within_unique_ptr();
    let sys_cb_ptr = system_callback as usize;
    unsafe {
        base::subscribe_on_new_system(mavsdk_instance.as_mut_ptr(), sys_cb_ptr as libc::uintptr_t);
    }
}