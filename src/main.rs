// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use autocxx::prelude::*;
include_cpp! {

    // Shims
    #include "../../cxx/mavsdk_shim.h"

    // Core MAVSDK headers
    #include "autopilot.h"
    #include "base64.h"
    #include "component_type.h"
    #include "connection_result.h"
    #include "geometry.h"
    #include "handle.h"
    #include "log_callback.h"
    #include "mavlink_address.h"
    #include "mavsdk.h"
    #include "plugin_base.h"
    #include "server_component.h"
    #include "server_plugin_base.h"
    #include "system.h"
    #include "vehicle.h"

    // Plugins
    #include "telemetry.h"

    safety!(unsafe_ffi)
    generate!("mavsdk::Autopilot")
    generate!("mavsdk::base64_encode")
    generate!("mavsdk::base64_decode")
    generate_pod!("mavsdk::ComponentType")
    generate_pod!("mavsdk::ConnectionResult")
    generate!("mavsdk::geometry::CoordinateTransformation")
    generate_pod!("mavsdk::geometry::CoordinateTransformation_GlobalCoordinate")
    generate_pod!("mavsdk::geometry::CoordinateTransformation_LocalCoordinate")
    generate!("mavsdk::Handle")
    generate_pod!("mavsdk::log::Level")
    generate!("mavsdk::log::Callback")
    generate!("mavsdk::log::get_callback")
    generate!("mavsdk::log::subscribe")
    generate_pod!("MavlinkAddress")
    generate!("mavsdk::Mavsdk")
    generate!("mavsdk::Mavsdk_Configuration")
    generate!("mavsdk::PluginBase")
    generate!("mavsdk::ServerComponent")
    generate!("mavsdk::ServerPluginBase")
    generate!("mavsdk::System")
    generate_pod!("mavsdk::Vehicle")
    generate!("mavsdk::to_vehicle_from_mav_type")
    
    // Shims
    generate!("test_function")
    
}
fn main() {
    println!("Hello, world!");
    let config = ffi::mavsdk::Mavsdk_Configuration::new(1, 1, true).within_unique_ptr();
    // Must use new1 because this is mavsdk::Mavsdk's second constructor
    let mavsdk_instance = ffi::mavsdk::Mavsdk::new1(&config).within_unique_ptr();

    let mavsdk_version = mavsdk_instance.version();
    println!("MAVSDK Version: {}", mavsdk_version);

    
}


