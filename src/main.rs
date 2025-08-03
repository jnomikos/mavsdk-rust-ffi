use std::os::raw::c_void;

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
    #include "action.h"
    #include "action_server.h"
    #include "arm_authorizer_server.h"
    //#include "calibration.h"
    #include "camera.h"
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
    generate!("mavsdk::Mavsdk_NewSystemHandle")
    generate_pod!("mavsdk::log::Level")
    generate!("mavsdk::log::Callback")
    generate!("mavsdk::log::get_callback")
    generate!("mavsdk::log::subscribe")
    generate_pod!("MavlinkAddress")
    generate!("mavsdk::Mavsdk")
    generate_pod!("mavsdk::ForwardingOption")
    generate!("mavsdk::Mavsdk_Configuration")
    generate!("mavsdk::PluginBase")
    generate!("mavsdk::ServerComponent")
    generate!("mavsdk::ServerPluginBase")
    generate!("mavsdk::System")
    generate_pod!("mavsdk::Vehicle")
    generate!("mavsdk::to_vehicle_from_mav_type")

    // --- Plugins --- //

    // Action
    generate!("mavsdk::Action")
    generate_pod!("mavsdk::Action_OrbitYawBehavior")
    generate_pod!("mavsdk::Action_Result")

    // ActionServer
    generate!("mavsdk::ActionServer")
    generate_pod!("mavsdk::ActionServer_FlightMode")
    generate_pod!("mavsdk::ActionServer_AllowableFlightModes")
    generate_pod!("mavsdk::ActionServer_ArmDisarm")
    generate_pod!("mavsdk::ActionServer_Result")

    // ArmAuthorizerServer
    generate!("mavsdk::ArmAuthorizerServer")
    generate_pod!("mavsdk::ArmAuthorizerServer_RejectionReason")
    generate_pod!("mavsdk::ArmAuthorizerServer_Result")

    generate!("mavsdk::Camera")
    generate_pod!("mavsdk::Camera_Mode")
    generate_pod!("mavsdk::Camera_PhotosRange")
    generate!("mavsdk::Camera_Option")
    generate!("mavsdk::Camera_Setting")
    generate!("mavsdk::Camera_SettingOptions")
    generate!("mavsdk::Camera_VideoStreamSettings")
    generate!("mavsdk::Camera_VideoStreamInfo")
    generate_pod!("mavsdk::Camera_VideoStreamInfo_VideoStreamStatus")
    generate_pod!("mavsdk::Camera_VideoStreamInfo_VideoStreamSpectrum")
    generate_pod!("mavsdk::Camera_ModeUpdate")
    generate!("mavsdk::Camera_VideoStreamUpdate")
    generate!("mavsdk::Camera_Storage")
    generate_pod!("mavsdk::Camera_Storage_StorageStatus")
    generate_pod!("mavsdk::Camera_Storage_StorageType")
    generate!("mavsdk::Camera_StorageUpdate")
    generate!("mavsdk::Camera_CurrentSettingsUpdate")
    generate!("mavsdk::Camera_PossibleSettingOptionsUpdate")
    generate_pod!("mavsdk::Camera_Result")
    generate_pod!("mavsdk::Camera_Position")
    generate_pod!("mavsdk::Camera_Quaternion")
    generate_pod!("mavsdk::Camera_EulerAngle")
    generate!("mavsdk::Camera_CaptureInfo")
    generate!("mavsdk::Camera_Information")
    generate!("mavsdk::Camera_CameraList")


    // Telemetry
    generate!("mavsdk::Telemetry")
    generate_pod!("mavsdk::Telemetry_FixType")
    generate_pod!("mavsdk::Telemetry_BatteryFunction")
    generate_pod!("mavsdk::Telemetry_FlightMode")
    generate_pod!("mavsdk::Telemetry_StatusTextType")
    generate_pod!("mavsdk::Telemetry_LandedState")
    generate_pod!("mavsdk::Telemetry_VtolState")
    generate_pod!("mavsdk::Telemetry_Position")
    generate_pod!("mavsdk::Telemetry_Heading")
    generate_pod!("mavsdk::Telemetry_Quaternion")
    generate_pod!("mavsdk::Telemetry_EulerAngle")
    generate_pod!("mavsdk::Telemetry_AngularVelocityBody")
    generate_pod!("mavsdk::Telemetry_GpsInfo")
    generate_pod!("mavsdk::Telemetry_RawGps")
    generate_pod!("mavsdk::Telemetry_Battery")
    generate_pod!("mavsdk::Telemetry_Health")
    generate_pod!("mavsdk::Telemetry_RcStatus")
    generate!("mavsdk::Telemetry_StatusText")
    generate!("mavsdk::Telemetry_ActuatorControlTarget")
    generate!("mavsdk::Telemetry_ActuatorOutputStatus")
    generate!("mavsdk::Telemetry_Covariance")
    generate_pod!("mavsdk::Telemetry_VelocityBody")
    generate_pod!("mavsdk::Telemetry_PositionBody")
    generate!("mavsdk::Telemetry_Odometry")
    generate_pod!("mavsdk::Telemetry_Odometry_MavFrame")
    generate_pod!("mavsdk::Telemetry_DistanceSensor")
    generate_pod!("mavsdk::Telemetry_ScaledPressure")
    generate_pod!("mavsdk::Telemetry_PositionNed")
    generate_pod!("mavsdk::Telemetry_VelocityNed")
    generate_pod!("mavsdk::Telemetry_PositionVelocityNed")
    generate_pod!("mavsdk::Telemetry_GroundTruth")
    generate_pod!("mavsdk::Telemetry_FixedwingMetrics")
    generate_pod!("mavsdk::Telemetry_AccelerationFrd")
    generate_pod!("mavsdk::Telemetry_AngularVelocityFrd")
    generate_pod!("mavsdk::Telemetry_MagneticFieldFrd")
    generate_pod!("mavsdk::Telemetry_Imu")
    generate_pod!("mavsdk::Telemetry_GpsGlobalOrigin")
    generate_pod!("mavsdk::Telemetry_Altitude")
    generate_pod!("mavsdk::Telemetry_Wind")
    generate_pod!("mavsdk::Telemetry_Result")
    
    // Shim
    generate!("mavsdk_wait_on_new_system")
    
}
fn main() {
    println!("Hello, world!");
    let config = ffi::mavsdk::Mavsdk_Configuration::new(1, 1, true).within_unique_ptr();
    // Must use new1 because this is mavsdk::Mavsdk's second constructor
    let mut mavsdk_instance = ffi::mavsdk::Mavsdk::new1(&config).within_unique_ptr();

    cxx::let_cxx_string!(conn_str = "serial:///dev/ttyACM0:115200");
    let mavsdk_connection_result = mavsdk_instance.pin_mut().add_any_connection(&conn_str, ffi::mavsdk::ForwardingOption::ForwardingOff);

    let mavsdk_version = mavsdk_instance.version();
    println!("MAVSDK Version: {}", mavsdk_version);


    let number: c_int = autocxx::c_int(5);

    extern "C" fn my_callback(userdata: *mut std::ffi::c_void) {
        println!("New system discovered! Userdata: {:?}", userdata);
    }

    unsafe {
        ffi::mavsdk_wait_on_new_system(
            mavsdk_instance.as_mut_ptr() as *mut autocxx::c_void,
        );
    }
    println!("New system discovered!");
}


