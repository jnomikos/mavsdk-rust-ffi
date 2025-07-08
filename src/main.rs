// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use autocxx::prelude::*;
include_cpp! {
    #include "../../cxx/mavsdk_shim.h"
    #include "mavsdk/core/include/mavsdk/mavsdk.h"
    #include "mavsdk/core/include/mavsdk/system.h"
    #include "mavsdk/core/include/mavsdk/handle.h"
    safety!(unsafe_ffi)
    generate!("mavsdk::Mavsdk")
    generate!("mavsdk::Mavsdk_Configuration")
    generate!("mavsdk::System")
    
    // Shims
    generate!("test_function")
    generate!("mavsdk::Mavsdk_new")
    
}
fn main() {
    println!("Hello, world!");
    let config = ffi::mavsdk::Mavsdk_Configuration::new(1, 1, true).within_unique_ptr();
    let mavsdk_instance = ffi::mavsdk::Mavsdk_new(&config);
}


