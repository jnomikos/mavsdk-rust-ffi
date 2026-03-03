pub mod core {
    autocxx::include_cpp! {

        // Shims
        #include "core_shim.h"

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
        #include "overloaded.h"
        #include "plugin_base.h"
        #include "server_component.h"
        #include "server_plugin_base.h"
        #include "system.h"
        #include "vehicle.h"

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
        generate!("mavsdk::Mavsdk_ConnectionError")
        generate!("mavsdk::Mavsdk_ConnectionHandle")
        generate!("mavsdk::Mavsdk_MavlinkMessage")
        generate_pod!("mavsdk::ForwardingOption")
        generate!("mavsdk::Mavsdk_Configuration")
        generate!("mavsdk::PluginBase")
        generate!("mavsdk::ServerComponent")
        generate!("mavsdk::ServerPluginBase")
        generate!("mavsdk::System")
        generate_pod!("mavsdk::Vehicle")
        generate!("mavsdk::to_vehicle_from_mav_type")

        // Shims
        generate_ns!("core_subscriptions")
        generate!("Mavsdk_first_autopilot")
    }
    pub use ffi::*;
}
pub mod wrappers {
    include!("../cxx/gen/mod.rs");
}