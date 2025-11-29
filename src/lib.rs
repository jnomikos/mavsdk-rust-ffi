pub mod core {
    autocxx::include_cpp! {

        // Shims
        #include "core_shim.h"
        #include "core_getters.h"

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
        generate!("MavsdkGetters::ConnectionError")
        generate!("mavsdk::Mavsdk_MavlinkMessage")
        generate!("MavsdkGetters::MavlinkMessage")
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
    }
    pub use ffi::*;
}
pub mod action {
    autocxx::include_cpp! {
        #include "action.h"
        name!(action)

        safety!(unsafe_ffi)
        generate!("mavsdk::Action")
        generate_pod!("mavsdk::Action_OrbitYawBehavior")
        generate_pod!("mavsdk::Action_Result")

        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use action::*;
}

pub mod action_server {
    autocxx::include_cpp! {
        #include "action_server.h"
        #include "action_server_shim.h"
        name!(action_server)

        safety!(unsafe_ffi)
        generate!("mavsdk::ActionServer")
        generate_pod!("mavsdk::ActionServer_FlightMode")
        generate_pod!("mavsdk::ActionServer_AllowableFlightModes")
        generate_pod!("mavsdk::ActionServer_ArmDisarm")
        generate_pod!("mavsdk::ActionServer_Result")

        generate_ns!("action_server_subscriptions")

        extern_cpp_type!("mavsdk::ServerComponent", crate::core::mavsdk::ServerComponent)
    }
    pub use action_server::*;
}

pub mod arm_authorizer_server {
    autocxx::include_cpp! {
        #include "arm_authorizer_server.h"
        #include "arm_authorizer_server_shim.h"
        name!(arm_authorizer_server)

        safety!(unsafe_ffi)
        generate!("mavsdk::ArmAuthorizerServer")
        generate_pod!("mavsdk::ArmAuthorizerServer_RejectionReason")
        generate_pod!("mavsdk::ArmAuthorizerServer_Result")

        // Shims
        generate_ns!("arm_authorizer_server_subscriptions")

        extern_cpp_type!("mavsdk::ServerComponent", crate::core::mavsdk::ServerComponent)
    }
    pub use arm_authorizer_server::*;
}

/*pub mod calibration {
    autocxx::include_cpp! {
        #include "calibration.h"
        #include "calibration_getters.h"
        name!(calibration)

        safety!(unsafe_ffi)
        generate!("mavsdk::Calibration")
        generate_pod!("mavsdk::Calibration_Result")
        generate!("mavsdk::Calibration_ProgressData")

        // Shims
        generate_ns!("ProgressData")

        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use calibration::*;
}*/

pub mod camera {
    autocxx::include_cpp! {
        #include "camera.h"
        #include "camera_shim.h"
        #include "camera_getters.h"
        name!(camera)

        safety!(unsafe_ffi)
        generate!("mavsdk::Camera")
        generate_pod!("mavsdk::Camera_Mode")
        generate_pod!("mavsdk::Camera_PhotosRange")
        generate!("mavsdk::Camera_Option")
        generate!("CameraGetters::Option")
        generate!("mavsdk::Camera_Setting")
        generate!("CameraGetters::Setting")
        generate!("mavsdk::Camera_SettingOptions")
        generate!("CameraGetters::SettingOptions")
        generate!("mavsdk::Camera_VideoStreamSettings")
        generate!("CameraGetters::VideoStreamSettings")
        generate!("mavsdk::Camera_VideoStreamInfo")
        generate!("CameraGetters::VideoStreamInfo")
        generate_pod!("mavsdk::Camera_VideoStreamInfo_VideoStreamStatus")
        generate_pod!("mavsdk::Camera_VideoStreamInfo_VideoStreamSpectrum")
        generate_pod!("mavsdk::Camera_ModeUpdate")
        generate!("mavsdk::Camera_VideoStreamUpdate")
        generate!("CameraGetters::VideoStreamUpdate")
        generate!("mavsdk::Camera_Storage")
        generate!("CameraGetters::Storage")
        generate_pod!("mavsdk::Camera_Storage_StorageStatus")
        generate_pod!("mavsdk::Camera_Storage_StorageType")
        generate!("mavsdk::Camera_StorageUpdate")
        generate!("CameraGetters::StorageUpdate")
        generate!("mavsdk::Camera_CurrentSettingsUpdate")
        generate!("CameraGetters::CurrentSettingsUpdate")
        generate!("mavsdk::Camera_PossibleSettingOptionsUpdate")
        generate!("CameraGetters::PossibleSettingOptionsUpdate")
        generate_pod!("mavsdk::Camera_Result")
        generate_pod!("mavsdk::Camera_Position")
        generate_pod!("mavsdk::Camera_Quaternion")
        generate_pod!("mavsdk::Camera_EulerAngle")
        generate!("mavsdk::Camera_CaptureInfo")
        generate!("CameraGetters::CaptureInfo")
        generate!("mavsdk::Camera_Information")
        generate!("CameraGetters::Information")
        generate!("mavsdk::Camera_CameraList")
        generate!("CameraGetters::CameraList")

        generate_ns!("camera_subscriptions")
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use camera::*;
}

pub mod camera_server {
    autocxx::include_cpp! {
        #include "camera_server.h"
        #include "camera_server_shim.h"
        #include "camera_server_getters.h"
        name!(camera_server)

        safety!(unsafe_ffi)
        generate!("mavsdk::CameraServer")
        generate_pod!("mavsdk::CameraServer_CameraFeedback")
        generate_pod!("mavsdk::CameraServer_Mode")
        generate!("mavsdk::CameraServer_Information")
        generate!("CameraServerGetters::Information")
        generate!("mavsdk::CameraServer_VideoStreaming")
        generate!("CameraServerGetters::VideoStreaming")
        generate_pod!("mavsdk::CameraServer_Position")
        generate_pod!("mavsdk::CameraServer_Quaternion")
        generate!("mavsdk::CameraServer_CaptureInfo")
        generate!("CameraServerGetters::CaptureInfo")
        generate_pod!("mavsdk::CameraServer_Result")
        generate_pod!("mavsdk::CameraServer_StorageInformation")
        generate_pod!("mavsdk::CameraServer_StorageInformation_StorageStatus")
        generate_pod!("mavsdk::CameraServer_StorageInformation_StorageType")
        generate_pod!("mavsdk::CameraServer_CaptureStatus")
        generate_pod!("mavsdk::CameraServer_CaptureStatus_ImageStatus")
        generate_pod!("mavsdk::CameraServer_CaptureStatus_VideoStatus")
        generate_pod!("mavsdk::CameraServer_TrackPoint")
        generate_pod!("mavsdk::CameraServer_TrackRectangle")

        // Shims
        generate_ns!("camera_server_subscriptions")

        extern_cpp_type!("mavsdk::ServerComponent", crate::core::mavsdk::ServerComponent)
    }
    pub use camera_server::*;
}

pub mod component_metadata {
    autocxx::include_cpp! {
        #include "component_metadata.h"
        #include "component_metadata_shim.h"
        #include "component_metadata_getters.h"
        name!(component_metadata)

        safety!(unsafe_ffi)
        generate!("mavsdk::ComponentMetadata")
        generate_pod!("mavsdk::ComponentMetadata_MetadataType")
        generate!("mavsdk::ComponentMetadata_MetadataData")
        generate!("ComponentMetadataGetters::MetadataData")
        generate_pod!("mavsdk::ComponentMetadata_Result")
        generate!("mavsdk::ComponentMetadata_MetadataUpdate")
        generate!("ComponentMetadataGetters::MetadataUpdate")

        // Shims
        generate_ns!("component_metadata_subscriptions")

        extern_cpp_type!("mavsdk::ServerComponent", crate::core::mavsdk::ServerComponent)
    }
    pub use component_metadata::*;
}

pub mod component_metadata_server {
    autocxx::include_cpp! {
        #include "component_metadata_server.h"
        #include "component_metadata_server_getters.h"
        name!(component_metadata_server)

        safety!(unsafe_ffi)
        generate!("mavsdk::ComponentMetadataServer")
        generate_pod!("mavsdk::ComponentMetadataServer_MetadataType")
        generate!("mavsdk::ComponentMetadataServer_Metadata")
        generate!("ComponentMetadataServerGetters::Metadata")

        extern_cpp_type!("mavsdk::ServerComponent", crate::core::mavsdk::ServerComponent)
    }
    pub use component_metadata_server::*;
}

pub mod events {
    autocxx::include_cpp! {
        #include "events.h"
        #include "events_shim.h"
        #include "events_getters.h"
        name!(events)

        safety!(unsafe_ffi)
        generate!("mavsdk::Events")
        generate_pod!("mavsdk::Events_LogLevel")
        generate!("mavsdk::Events_Event")
        generate!("EventsGetters::Event")
        generate!("mavsdk::Events_HealthAndArmingCheckProblem")
        generate!("EventsGetters::HealthAndArmingCheckProblem")
        generate!("mavsdk::Events_HealthAndArmingCheckMode")
        generate!("EventsGetters::HealthAndArmingCheckMode")
        generate!("mavsdk::Events_HealthComponentReport")
        generate!("EventsGetters::HealthComponentReport")
        generate!("mavsdk::Events_HealthAndArmingCheckReport")
        generate!("EventsGetters::HealthAndArmingCheckReport")
        generate_pod!("mavsdk::Events_Result")

        // Shims
        generate_ns!("events_subscriptions")

        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use events::*;
}

pub mod failure {
    autocxx::include_cpp! {
        #include "failure.h"
        name!(failure)

        safety!(unsafe_ffi)
        generate!("mavsdk::Failure")
        generate_pod!("mavsdk::Failure_FailureUnit")
        generate_pod!("mavsdk::Failure_FailureType")
        generate_pod!("mavsdk::Failure_Result")


        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use failure::*;
}

pub mod ftp {
    autocxx::include_cpp! {
        #include "ftp.h"
        #include "ftp_getters.h"
        name!(ftp)

        safety!(unsafe_ffi)
        generate!("mavsdk::Ftp")
        generate!("mavsdk::Ftp_ListDirectoryData")
        generate!("FtpGetters::ListDirectoryData")
        generate_pod!("mavsdk::Ftp_ProgressData")
        generate_pod!("mavsdk::Ftp_Result")
        block!("mavsdk::Ftp_DownloadCallback")
        block!("mavsdk::Ftp_UploadCallback")


        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use ftp::*;
}

pub mod ftp_server {
    autocxx::include_cpp! {
        #include "ftp_server.h"
        name!(ftp_server)

        safety!(unsafe_ffi)
        generate!("mavsdk::FtpServer")
        generate_pod!("mavsdk::FtpServer_Result")



        extern_cpp_type!("mavsdk::ServerComponent", crate::core::mavsdk::ServerComponent)
    }
    pub use ftp_server::*;
}

pub mod geofence {
    autocxx::include_cpp! {
        #include "geofence.h"
        #include "geofence_getters.h"
        name!(geofence)

        safety!(unsafe_ffi)
        generate!("mavsdk::Geofence")
        generate_pod!("mavsdk::Geofence_FenceType")
        generate_pod!("mavsdk::Geofence_Point")
        generate!("mavsdk::Geofence_Polygon")
        generate!("GeofenceGetters::Polygon")
        generate_pod!("mavsdk::Geofence_Circle")
        generate!("mavsdk::Geofence_GeofenceData")
        generate!("GeofenceGetters::GeofenceData")
        generate_pod!("mavsdk::Geofence_Result")

        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use geofence::*;
}

pub mod gimbal {
    autocxx::include_cpp! {
        #include "gimbal.h"
        #include "gimbal_shim.h"
        #include "gimbal_getters.h"
        name!(gimbal)

        safety!(unsafe_ffi)
        generate!("mavsdk::Gimbal")
        generate_pod!("mavsdk::Gimbal_GimbalMode")
        generate_pod!("mavsdk::Gimbal_ControlMode")
        generate_pod!("mavsdk::Gimbal_SendMode")
        generate_pod!("mavsdk::Gimbal_Quaternion")
        generate_pod!("mavsdk::Gimbal_EulerAngle")
        generate_pod!("mavsdk::Gimbal_AngularVelocityBody")
        generate_pod!("mavsdk::Gimbal_Attitude")
        generate!("mavsdk::Gimbal_GimbalItem")
        generate!("GimbalGetters::GimbalItem")
        generate!("mavsdk::Gimbal_GimbalList")
        generate!("GimbalGetters::GimbalList")
        generate_pod!("mavsdk::Gimbal_ControlStatus")
        generate_pod!("mavsdk::Gimbal_Result")

        
        // Shims
        generate_ns!("gimbal_subscriptions")

        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use gimbal::*;
}

pub mod gripper {
    autocxx::include_cpp! {
        #include "gripper.h"
        name!(gripper)

        safety!(unsafe_ffi)
        generate!("mavsdk::Gripper")
        generate_pod!("mavsdk::Gripper_GripperAction")
        generate_pod!("mavsdk::Gripper_Result")

        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use gripper::*;
}

pub mod info {
    autocxx::include_cpp! {
        #include "info.h"
        #include "info_shim.h"
        #include "info_getters.h"
        name!(info)

        safety!(unsafe_ffi)
        generate!("mavsdk::Info")
        generate_pod!("mavsdk::Info_FlightInfo")
        generate!("mavsdk::Info_Identification")
        generate!("InfoGetters::Identification")
        generate!("mavsdk::Info_Product")
        generate!("InfoGetters::Product")
        generate!("mavsdk::Info_Version")
        generate!("InfoGetters::Version")
        generate_pod!("mavsdk::Info_Version_FlightSoftwareVersionType")
        generate_pod!("mavsdk::Info_Result")

        // Shims
        generate_ns!("info_subscriptions")

        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use info::*;
}

pub mod log_files {
    autocxx::include_cpp! {
        #include "log_files.h"
        #include "log_files_getters.h"
        name!(log_files)

        safety!(unsafe_ffi)
        generate!("mavsdk::LogFiles")
        generate_pod!("mavsdk::LogFiles_ProgressData")
        generate!("mavsdk::LogFiles_Entry")
        generate!("LogFilesGetters::Entry")
        generate_pod!("mavsdk::LogFiles_Result")
        block!("mavsdk::LogFiles_DownloadLogFileCallback")

        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use log_files::*;
}

pub mod log_streaming {
    autocxx::include_cpp! {
        #include "log_streaming.h"
        #include "log_streaming_shim.h"
        #include "log_streaming_getters.h"
        name!(log_streaming)

        safety!(unsafe_ffi)
        generate!("mavsdk::LogStreaming")
        generate!("mavsdk::LogStreaming_LogStreamingRaw")
        generate!("LogStreamingGetters::LogStreamingRaw")
        generate_pod!("mavsdk::LogStreaming_Result")

        // Shims
        generate_ns!("log_streaming_subscriptions")

        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use log_streaming::*;
}

pub mod manual_control {
    autocxx::include_cpp! {
        #include "manual_control.h"
        name!(manual_control)

        safety!(unsafe_ffi)
        generate!("mavsdk::ManualControl")
        generate_pod!("mavsdk::ManualControl_Result")

        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use manual_control::*;
}

pub mod mavlink_direct {
    autocxx::include_cpp! {
        #include "mavlink_direct.h"
        #include "mavlink_direct_shim.h"
        #include "mavlink_direct_getters.h"
        name!(mavlink_direct)

        safety!(unsafe_ffi)
        generate!("mavsdk::MavlinkDirect")
        generate!("mavsdk::MavlinkDirect_MavlinkMessage")
        generate!("MavlinkDirectGetters::MavlinkMessage")
        generate_pod!("mavsdk::MavlinkDirect_Result")

        // Shims
        generate_ns!("mavlink_direct_subscriptions")


        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use mavlink_direct::*;
}

pub mod mission {
    autocxx::include_cpp! {
        #include "mission.h"
        #include "mission_shim.h"
        #include "mission_getters.h"
        name!(mission)

        safety!(unsafe_ffi)
        generate!("mavsdk::Mission")
        generate_pod!("mavsdk::Mission_MissionItem")
        generate_pod!("mavsdk::Mission_MissionItem_CameraAction")
        generate_pod!("mavsdk::Mission_MissionItem_VehicleAction")
        generate!("mavsdk::Mission_MissionPlan")
        generate!("MissionGetters::MissionPlan")
        generate_pod!("mavsdk::Mission_MissionProgress")
        generate_pod!("mavsdk::Mission_Result")
        generate_pod!("mavsdk::Mission_ProgressData")
        generate!("mavsdk::Mission_ProgressDataOrMission")
        generate!("MissionGetters::ProgressDataOrMission")
        block!("mavsdk::Mission_DownloadMissionWithProgressCallback")
        block!("mavsdk::Mission_UploadMissionWithProgressCallback")

        // Shims
        generate_ns!("mission_subscriptions")

        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use mission::*;
}

pub mod mission_raw {
    autocxx::include_cpp! {
        #include "mission_raw.h"
        #include "mission_raw_shim.h"
        #include "mission_raw_getters.h"
        name!(mission_raw)

        safety!(unsafe_ffi)
        generate!("mavsdk::MissionRaw")
        generate_pod!("mavsdk::MissionRaw_MissionProgress")
        generate_pod!("mavsdk::MissionRaw_MissionItem")
        generate!("mavsdk::MissionRaw_MissionImportData")
        generate!("MissionRawGetters::MissionImportData")
        generate_pod!("mavsdk::MissionRaw_Result")

        // Shims
        generate_ns!("mission_raw_subscriptions")

        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }

    pub use mission_raw::*;
}

pub mod mission_raw_server {
    autocxx::include_cpp! {
        #include "mission_raw_server.h"
        #include "mission_raw_server_shim.h"
        #include "mission_raw_server_getters.h"
        name!(mission_raw_server)

        safety!(unsafe_ffi)
        generate!("mavsdk::MissionRawServer")
        generate_pod!("mavsdk::MissionRawServer_MissionItem")
        generate!("mavsdk::MissionRawServer_MissionPlan")
        generate!("MissionRawServerGetters::MissionPlan")
        generate_pod!("mavsdk::MissionRawServer_MissionProgress")
        generate_pod!("mavsdk::MissionRawServer_Result")

        // Shims
        generate_ns!("mission_raw_server_subscriptions")

        extern_cpp_type!("mavsdk::ServerComponent", crate::core::mavsdk::ServerComponent)
    }

    pub use mission_raw_server::*;
}

pub mod mocap {
    autocxx::include_cpp! {
        #include "mocap.h"
        #include "mocap_getters.h"
        name!(mocap)

        safety!(unsafe_ffi)
        generate!("mavsdk::Mocap")
        generate_pod!("mavsdk::Mocap_PositionBody")
        generate_pod!("mavsdk::Mocap_AngleBody")
        generate_pod!("mavsdk::Mocap_SpeedBody")
        generate_pod!("mavsdk::Mocap_SpeedNed")
        generate_pod!("mavsdk::Mocap_AngularVelocityBody")
        generate!("mavsdk::Mocap_Covariance")
        generate!("MocapGetters::Covariance")
        generate_pod!("mavsdk::Mocap_Quaternion")
        generate!("mavsdk::Mocap_VisionPositionEstimate")
        generate!("MocapGetters::VisionPositionEstimate")
        generate!("mavsdk::Mocap_VisionSpeedEstimate")
        generate!("MocapGetters::VisionSpeedEstimate")
        generate!("mavsdk::Mocap_AttitudePositionMocap")
        generate!("MocapGetters::AttitudePositionMocap")
        generate!("mavsdk::Mocap_Odometry")
        generate!("MocapGetters::Odometry")
        generate_pod!("mavsdk::Mocap_Odometry_MavFrame")
        generate_pod!("mavsdk::Mocap_Result")


        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }

    pub use mocap::*;
}


pub mod offboard {
    autocxx::include_cpp! {
        #include "offboard.h"
        #include "offboard_getters.h"
        name!(offboard)

        safety!(unsafe_ffi)
        generate!("mavsdk::Offboard")
        generate_pod!("mavsdk::Offboard_Attitude")
        generate!("mavsdk::Offboard_ActuatorControlGroup")
        generate!("OffboardGetters::ActuatorControlGroup")
        generate!("mavsdk::Offboard_ActuatorControl")
        generate!("OffboardGetters::ActuatorControl")
        generate_pod!("mavsdk::Offboard_AttitudeRate")
        generate_pod!("mavsdk::Offboard_PositionNedYaw")
        generate_pod!("mavsdk::Offboard_PositionGlobalYaw")
        generate_pod!("mavsdk::Offboard_PositionGlobalYaw_AltitudeType")
        generate_pod!("mavsdk::Offboard_VelocityBodyYawspeed")
        generate_pod!("mavsdk::Offboard_VelocityNedYaw")
        generate_pod!("mavsdk::Offboard_AccelerationNed")
        generate_pod!("mavsdk::Offboard_Result")

        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }

    pub use offboard::*;
}


pub mod param {
    autocxx::include_cpp! {
        #include "param.h"
        #include "param_getters.h"
        name!(param)

        safety!(unsafe_ffi)

        generate!("mavsdk::Param")
        generate_pod!("mavsdk::Param_ProtocolVersion")
        generate!("mavsdk::Param_IntParam")
        generate!("ParamGetters::IntParam")
        generate!("mavsdk::Param_FloatParam")
        generate!("ParamGetters::FloatParam")
        generate!("mavsdk::Param_CustomParam")
        generate!("ParamGetters::CustomParam")
        generate!("mavsdk::Param_AllParams")
        generate!("ParamGetters::AllParams")
        generate_pod!("mavsdk::Param_Result")


        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }

    pub use param::*;
}


pub mod param_server {
    autocxx::include_cpp! {
        #include "param_server.h"
        #include "param_server_shim.h"
        #include "param_server_getters.h"
        name!(param_server)

        safety!(unsafe_ffi)
        generate!("mavsdk::ParamServer")
        generate!("mavsdk::ParamServer_IntParam")
        generate!("ParamServerGetters::IntParam")
        generate!("mavsdk::ParamServer_FloatParam")
        generate!("ParamServerGetters::FloatParam")
        generate!("mavsdk::ParamServer_CustomParam")
        generate!("ParamServerGetters::CustomParam")
        generate!("mavsdk::ParamServer_AllParams")
        generate!("ParamServerGetters::AllParams")
        generate_pod!("mavsdk::ParamServer_Result")

        // Shims
        generate_ns!("param_server_subscriptions")

        extern_cpp_type!("mavsdk::ServerComponent", crate::core::mavsdk::ServerComponent)
    }

    pub use param_server::*;
}


pub mod rtk {
    autocxx::include_cpp! {
        #include "rtk.h"
        #include "rtk_getters.h"
        name!(rtk)

        safety!(unsafe_ffi)
        generate!("mavsdk::Rtk")
        generate!("mavsdk::Rtk_RtcmData")
        generate!("RtkGetters::RtcmData")
        generate_pod!("mavsdk::Rtk_Result")


        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }

    pub use rtk::*;
}

pub mod server_utility {
    autocxx::include_cpp! {
        #include "server_utility.h"
        name!(server_utility)

        safety!(unsafe_ffi)
        generate!("mavsdk::ServerUtility")
        generate_pod!("mavsdk::ServerUtility_StatusTextType")
        generate_pod!("mavsdk::ServerUtility_Result")

        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }

    pub use server_utility::*;
}

pub mod shell {
    autocxx::include_cpp! {
        #include "shell.h"
        #include "shell_shim.h"
        name!(shell)

        safety!(unsafe_ffi)
        generate!("mavsdk::Shell")
        generate_pod!("mavsdk::Shell_Result")

        // Shims
        generate_ns!("shell_subscriptions")
        
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }

    pub use shell::*;
}

pub mod telemetry {
    autocxx::include_cpp! {
        #include "telemetry.h"
        #include "telemetry_shim.h"
        #include "telemetry_getters.h"
        name!(telemetry)

        safety!(unsafe_ffi)
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
        generate!("TelemetryGetters::StatusText")
        generate!("mavsdk::Telemetry_ActuatorControlTarget")
        generate!("TelemetryGetters::ActuatorControlTarget")
        generate!("mavsdk::Telemetry_ActuatorOutputStatus")
        generate!("TelemetryGetters::ActuatorOutputStatus")
        generate!("mavsdk::Telemetry_Covariance")
        generate!("TelemetryGetters::Covariance")
        generate_pod!("mavsdk::Telemetry_VelocityBody")
        generate_pod!("mavsdk::Telemetry_PositionBody")
        generate!("mavsdk::Telemetry_Odometry")
        generate!("TelemetryGetters::Odometry")
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

        // Shims
        generate_ns!("telemetry_subscriptions")

        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use telemetry::*;
}

pub mod telemetry_server {
    autocxx::include_cpp! {
        #include "telemetry_server.h"
        #include "telemetry_server_getters.h"
        name!(telemetry_server)

        safety!(unsafe_ffi)
        generate!("mavsdk::TelemetryServer")
        generate_pod!("mavsdk::TelemetryServer_FixType")
        generate_pod!("mavsdk::TelemetryServer_VtolState")
        generate_pod!("mavsdk::TelemetryServer_StatusTextType")
        generate_pod!("mavsdk::TelemetryServer_LandedState")
        generate_pod!("mavsdk::TelemetryServer_Position")
        generate_pod!("mavsdk::TelemetryServer_Heading")
        generate_pod!("mavsdk::TelemetryServer_Quaternion")
        generate_pod!("mavsdk::TelemetryServer_EulerAngle")
        generate_pod!("mavsdk::TelemetryServer_AngularVelocityBody")
        generate_pod!("mavsdk::TelemetryServer_GpsInfo")
        generate_pod!("mavsdk::TelemetryServer_RawGps")
        generate_pod!("mavsdk::TelemetryServer_Battery")
        generate_pod!("mavsdk::TelemetryServer_RcStatus")
        generate!("mavsdk::TelemetryServer_StatusText")
        generate!("TelemetryServerGetters::StatusText")
        generate!("mavsdk::TelemetryServer_ActuatorControlTarget")
        generate!("TelemetryServerGetters::ActuatorControlTarget")
        generate!("mavsdk::TelemetryServer_ActuatorOutputStatus")
        generate!("TelemetryServerGetters::ActuatorOutputStatus")
        generate!("mavsdk::TelemetryServer_Covariance")
        generate!("TelemetryServerGetters::Covariance")
        generate_pod!("mavsdk::TelemetryServer_VelocityBody")
        generate_pod!("mavsdk::TelemetryServer_PositionBody")
        generate!("mavsdk::TelemetryServer_Odometry")
        generate!("TelemetryServerGetters::Odometry")
        generate_pod!("mavsdk::TelemetryServer_Odometry_MavFrame")
        generate_pod!("mavsdk::TelemetryServer_DistanceSensor")
        generate_pod!("mavsdk::TelemetryServer_ScaledPressure")
        generate_pod!("mavsdk::TelemetryServer_PositionNed")
        generate_pod!("mavsdk::TelemetryServer_VelocityNed")
        generate_pod!("mavsdk::TelemetryServer_PositionVelocityNed")
        generate_pod!("mavsdk::TelemetryServer_GroundTruth")
        generate_pod!("mavsdk::TelemetryServer_FixedwingMetrics")
        generate_pod!("mavsdk::TelemetryServer_AccelerationFrd")
        generate_pod!("mavsdk::TelemetryServer_AngularVelocityFrd")
        generate_pod!("mavsdk::TelemetryServer_MagneticFieldFrd")
        generate_pod!("mavsdk::TelemetryServer_Imu")
        generate_pod!("mavsdk::TelemetryServer_Result")
        
        
        extern_cpp_type!("mavsdk::ServerComponent", crate::core::mavsdk::ServerComponent)
    }

    pub use telemetry_server::*;
}

pub mod transponder {
    autocxx::include_cpp! {
        #include "transponder.h"
        #include "transponder_shim.h"
        #include "transponder_getters.h"
        name!(transponder)

        safety!(unsafe_ffi)
        generate!("mavsdk::Transponder")
        generate_pod!("mavsdk::Transponder_AdsbEmitterType")
        generate_pod!("mavsdk::Transponder_AdsbAltitudeType")
        generate!("mavsdk::Transponder_AdsbVehicle")
        generate!("TransponderGetters::AdsbVehicle")
        generate_pod!("mavsdk::Transponder_Result")

        // Shims
        generate_ns!("transponder_subscriptions")
        
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }

    pub use transponder::*;
}

pub mod tune {
    autocxx::include_cpp! {
        #include "tune.h"
        #include "tune_getters.h"
        name!(tune)

        safety!(unsafe_ffi)
        generate!("mavsdk::Tune")
        generate_pod!("mavsdk::Tune_SongElement")
        generate!("mavsdk::Tune_TuneDescription")
        generate!("TuneGetters::TuneDescription")
        generate_pod!("mavsdk::Tune_Result")

        
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }

    pub use tune::*;
}

pub mod winch {
    autocxx::include_cpp! {
        #include "winch.h"
        #include "winch_shim.h"
        name!(winch)

        safety!(unsafe_ffi)
        generate!("mavsdk::Winch")
        generate_pod!("mavsdk::Winch_WinchAction")
        generate_pod!("mavsdk::Winch_StatusFlags")
        generate_pod!("mavsdk::Winch_Status")
        generate_pod!("mavsdk::Winch_Result")

        // Shims
        generate_ns!("winch_subscriptions")

        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }

    pub use winch::*;
}

include!("../cxx/gen/core.rs");