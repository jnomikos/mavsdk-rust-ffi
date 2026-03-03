pub mod action {
    autocxx::include_cpp! {
        #include "action.h"
        name!(action)
        safety!(unsafe_ffi)

        generate!("mavsdk::Action")
        generate_pod!("mavsdk::Action_Result")
        generate_pod!("mavsdk::Action_OrbitYawBehavior")
        block!("mavsdk::Action_GetTakeoffAltitudeCallback")
        block!("mavsdk::Action_GetReturnToLaunchAltitudeCallback")
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use action::*;
}
pub mod action_server {
    autocxx::include_cpp! {
        #include "action_server.h"
        name!(action_server)
        safety!(unsafe_ffi)

        generate!("mavsdk::ActionServer")
        generate_pod!("mavsdk::ActionServer_Result")
        generate_pod!("mavsdk::ActionServer_AllowableFlightModes")
        generate_pod!("mavsdk::ActionServer_ArmDisarm")
        generate_pod!("mavsdk::ActionServer_FlightMode")
        block!("mavsdk::ActionServer_ArmDisarmCallback")
        block!("mavsdk::ActionServer_FlightModeChangeCallback")
        block!("mavsdk::ActionServer_TakeoffCallback")
        block!("mavsdk::ActionServer_LandCallback")
        block!("mavsdk::ActionServer_RebootCallback")
        block!("mavsdk::ActionServer_ShutdownCallback")
        block!("mavsdk::ActionServer_TerminateCallback")
        block!("mavsdk::ActionServer_GetAllowableFlightModesCallback")
        extern_cpp_type!("mavsdk::ServerComponent", crate::core::mavsdk::ServerComponent)
    }
    pub use action_server::*;
}
pub mod arm_authorizer_server {
    autocxx::include_cpp! {
        #include "arm_authorizer_server.h"
        name!(arm_authorizer_server)
        safety!(unsafe_ffi)

        generate!("mavsdk::ArmAuthorizerServer")
        generate_pod!("mavsdk::ArmAuthorizerServer_Result")
        generate_pod!("mavsdk::ArmAuthorizerServer_RejectionReason")
        block!("mavsdk::ArmAuthorizerServer_ArmAuthorizationCallback")
        extern_cpp_type!("mavsdk::ServerComponent", crate::core::mavsdk::ServerComponent)
    }
    pub use arm_authorizer_server::*;
}
pub mod calibration {
    autocxx::include_cpp! {
        #include "calibration.h"
        name!(calibration)
        safety!(unsafe_ffi)

        generate!("mavsdk::Calibration")
        generate_pod!("mavsdk::Calibration_Result")
        generate!("mavsdk::Calibration_ProgressData")
        block!("mavsdk::Calibration_CalibrateGyroCallback")
        block!("mavsdk::Calibration_CalibrateAccelerometerCallback")
        block!("mavsdk::Calibration_CalibrateMagnetometerCallback")
        block!("mavsdk::Calibration_CalibrateLevelHorizonCallback")
        block!("mavsdk::Calibration_CalibrateGimbalAccelerometerCallback")
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use calibration::*;
}
pub mod camera {
    autocxx::include_cpp! {
        #include "camera.h"
        name!(camera)
        safety!(unsafe_ffi)

        generate!("mavsdk::Camera")
        generate_pod!("mavsdk::Camera_Result")
        generate!("mavsdk::Camera_Option")
        generate!("mavsdk::Camera_Setting")
        generate!("mavsdk::Camera_SettingOptions")
        generate!("mavsdk::Camera_VideoStreamSettings")
        generate_pod!("mavsdk::Camera_VideoStreamInfo_VideoStreamStatus")
        generate_pod!("mavsdk::Camera_VideoStreamInfo_VideoStreamSpectrum")
        generate!("mavsdk::Camera_VideoStreamInfo")
        generate!("mavsdk::Camera_ModeUpdate")
        generate!("mavsdk::Camera_VideoStreamUpdate")
        generate_pod!("mavsdk::Camera_Storage_StorageStatus")
        generate_pod!("mavsdk::Camera_Storage_StorageType")
        generate!("mavsdk::Camera_Storage")
        generate!("mavsdk::Camera_StorageUpdate")
        generate!("mavsdk::Camera_CurrentSettingsUpdate")
        generate!("mavsdk::Camera_PossibleSettingOptionsUpdate")
        generate_pod!("mavsdk::Camera_Position")
        generate_pod!("mavsdk::Camera_Quaternion")
        generate_pod!("mavsdk::Camera_EulerAngle")
        generate!("mavsdk::Camera_CaptureInfo")
        generate!("mavsdk::Camera_Information")
        generate!("mavsdk::Camera_CameraList")
        generate_pod!("mavsdk::Camera_Mode")
        generate_pod!("mavsdk::Camera_PhotosRange")
        block!("mavsdk::Camera_ListPhotosCallback")
        block!("mavsdk::Camera_CameraListCallback")
        block!("mavsdk::Camera_ModeCallback")
        block!("mavsdk::Camera_GetModeCallback")
        block!("mavsdk::Camera_VideoStreamInfoCallback")
        block!("mavsdk::Camera_GetVideoStreamInfoCallback")
        block!("mavsdk::Camera_CaptureInfoCallback")
        block!("mavsdk::Camera_StorageCallback")
        block!("mavsdk::Camera_GetStorageCallback")
        block!("mavsdk::Camera_CurrentSettingsCallback")
        block!("mavsdk::Camera_GetCurrentSettingsCallback")
        block!("mavsdk::Camera_PossibleSettingOptionsCallback")
        block!("mavsdk::Camera_GetPossibleSettingOptionsCallback")
        block!("mavsdk::Camera_GetSettingCallback")
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use camera::*;
}
pub mod camera_server {
    autocxx::include_cpp! {
        #include "camera_server.h"
        name!(camera_server)
        safety!(unsafe_ffi)

        generate!("mavsdk::CameraServer")
        generate_pod!("mavsdk::CameraServer_Result")
        generate!("mavsdk::CameraServer_Information")
        generate!("mavsdk::CameraServer_VideoStreaming")
        generate_pod!("mavsdk::CameraServer_Position")
        generate_pod!("mavsdk::CameraServer_Quaternion")
        generate!("mavsdk::CameraServer_CaptureInfo")
        generate_pod!("mavsdk::CameraServer_StorageInformation_StorageStatus")
        generate_pod!("mavsdk::CameraServer_StorageInformation_StorageType")
        generate!("mavsdk::CameraServer_StorageInformation")
        generate_pod!("mavsdk::CameraServer_CaptureStatus_ImageStatus")
        generate_pod!("mavsdk::CameraServer_CaptureStatus_VideoStatus")
        generate!("mavsdk::CameraServer_CaptureStatus")
        generate_pod!("mavsdk::CameraServer_TrackPoint")
        generate_pod!("mavsdk::CameraServer_TrackRectangle")
        generate_pod!("mavsdk::CameraServer_CameraFeedback")
        generate_pod!("mavsdk::CameraServer_Mode")
        block!("mavsdk::CameraServer_TakePhotoCallback")
        block!("mavsdk::CameraServer_StartVideoCallback")
        block!("mavsdk::CameraServer_StopVideoCallback")
        block!("mavsdk::CameraServer_StartVideoStreamingCallback")
        block!("mavsdk::CameraServer_StopVideoStreamingCallback")
        block!("mavsdk::CameraServer_SetModeCallback")
        block!("mavsdk::CameraServer_StorageInformationCallback")
        block!("mavsdk::CameraServer_CaptureStatusCallback")
        block!("mavsdk::CameraServer_FormatStorageCallback")
        block!("mavsdk::CameraServer_ResetSettingsCallback")
        block!("mavsdk::CameraServer_ZoomInStartCallback")
        block!("mavsdk::CameraServer_ZoomOutStartCallback")
        block!("mavsdk::CameraServer_ZoomStopCallback")
        block!("mavsdk::CameraServer_ZoomRangeCallback")
        block!("mavsdk::CameraServer_TrackingPointCommandCallback")
        block!("mavsdk::CameraServer_TrackingRectangleCommandCallback")
        block!("mavsdk::CameraServer_TrackingOffCommandCallback")
        extern_cpp_type!("mavsdk::ServerComponent", crate::core::mavsdk::ServerComponent)
    }
    pub use camera_server::*;
}
pub mod component_metadata {
    autocxx::include_cpp! {
        #include "component_metadata.h"
        name!(component_metadata)
        safety!(unsafe_ffi)

        generate!("mavsdk::ComponentMetadata")
        generate_pod!("mavsdk::ComponentMetadata_Result")
        generate!("mavsdk::ComponentMetadata_MetadataData")
        generate!("mavsdk::ComponentMetadata_MetadataUpdate")
        generate_pod!("mavsdk::ComponentMetadata_MetadataType")
        block!("mavsdk::ComponentMetadata_MetadataAvailableCallback")
        block!("mavsdk::ComponentMetadata_GetMetadataCallback")
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use component_metadata::*;
}
pub mod component_metadata_server {
    autocxx::include_cpp! {
        #include "component_metadata_server.h"
        name!(component_metadata_server)
        safety!(unsafe_ffi)

        generate!("mavsdk::ComponentMetadataServer")
        generate!("mavsdk::ComponentMetadataServer_Metadata")
        generate_pod!("mavsdk::ComponentMetadataServer_MetadataType")
        
        extern_cpp_type!("mavsdk::ServerComponent", crate::core::mavsdk::ServerComponent)
    }
    pub use component_metadata_server::*;
}
pub mod events {
    autocxx::include_cpp! {
        #include "events.h"
        name!(events)
        safety!(unsafe_ffi)

        generate!("mavsdk::Events")
        generate_pod!("mavsdk::Events_Result")
        generate!("mavsdk::Events_Event")
        generate!("mavsdk::Events_HealthAndArmingCheckProblem")
        generate!("mavsdk::Events_HealthAndArmingCheckMode")
        generate!("mavsdk::Events_HealthComponentReport")
        generate!("mavsdk::Events_HealthAndArmingCheckReport")
        generate_pod!("mavsdk::Events_LogLevel")
        block!("mavsdk::Events_EventsCallback")
        block!("mavsdk::Events_HealthAndArmingChecksCallback")
        block!("mavsdk::Events_GetHealthAndArmingChecksReportCallback")
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
        generate_pod!("mavsdk::Failure_Result")
        generate_pod!("mavsdk::Failure_FailureUnit")
        generate_pod!("mavsdk::Failure_FailureType")
        
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use failure::*;
}
pub mod follow_me {
    autocxx::include_cpp! {
        #include "follow_me.h"
        name!(follow_me)
        safety!(unsafe_ffi)

        generate!("mavsdk::FollowMe")
        generate_pod!("mavsdk::FollowMe_Result")
        generate_pod!("mavsdk::FollowMe_Config_FollowAltitudeMode")
        generate!("mavsdk::FollowMe_Config")
        generate_pod!("mavsdk::FollowMe_TargetLocation")
        block!("mavsdk::FollowMe_GetConfigCallback")
        block!("mavsdk::FollowMe_IsActiveCallback")
        block!("mavsdk::FollowMe_GetLastLocationCallback")
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use follow_me::*;
}
pub mod ftp {
    autocxx::include_cpp! {
        #include "ftp.h"
        name!(ftp)
        safety!(unsafe_ffi)

        generate!("mavsdk::Ftp")
        generate_pod!("mavsdk::Ftp_Result")
        generate!("mavsdk::Ftp_ListDirectoryData")
        generate_pod!("mavsdk::Ftp_ProgressData")
        block!("mavsdk::Ftp_DownloadCallback")
        block!("mavsdk::Ftp_UploadCallback")
        block!("mavsdk::Ftp_ListDirectoryCallback")
        block!("mavsdk::Ftp_AreFilesIdenticalCallback")
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
        name!(geofence)
        safety!(unsafe_ffi)

        generate!("mavsdk::Geofence")
        generate_pod!("mavsdk::Geofence_Result")
        generate_pod!("mavsdk::Geofence_Point")
        generate!("mavsdk::Geofence_Polygon")
        generate!("mavsdk::Geofence_Circle")
        generate!("mavsdk::Geofence_GeofenceData")
        generate_pod!("mavsdk::Geofence_FenceType")
        
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use geofence::*;
}
pub mod gimbal {
    autocxx::include_cpp! {
        #include "gimbal.h"
        name!(gimbal)
        safety!(unsafe_ffi)

        generate!("mavsdk::Gimbal")
        generate_pod!("mavsdk::Gimbal_Result")
        generate_pod!("mavsdk::Gimbal_Quaternion")
        generate_pod!("mavsdk::Gimbal_EulerAngle")
        generate_pod!("mavsdk::Gimbal_AngularVelocityBody")
        generate!("mavsdk::Gimbal_Attitude")
        generate!("mavsdk::Gimbal_GimbalItem")
        generate!("mavsdk::Gimbal_GimbalList")
        generate!("mavsdk::Gimbal_ControlStatus")
        generate_pod!("mavsdk::Gimbal_GimbalMode")
        generate_pod!("mavsdk::Gimbal_ControlMode")
        generate_pod!("mavsdk::Gimbal_SendMode")
        block!("mavsdk::Gimbal_GimbalListCallback")
        block!("mavsdk::Gimbal_ControlStatusCallback")
        block!("mavsdk::Gimbal_GetControlStatusCallback")
        block!("mavsdk::Gimbal_AttitudeCallback")
        block!("mavsdk::Gimbal_GetAttitudeCallback")
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
        generate_pod!("mavsdk::Gripper_Result")
        generate_pod!("mavsdk::Gripper_GripperAction")
        
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use gripper::*;
}
pub mod info {
    autocxx::include_cpp! {
        #include "info.h"
        name!(info)
        safety!(unsafe_ffi)

        generate!("mavsdk::Info")
        generate_pod!("mavsdk::Info_Result")
        generate_pod!("mavsdk::Info_FlightInfo")
        generate!("mavsdk::Info_Identification")
        generate!("mavsdk::Info_Product")
        generate_pod!("mavsdk::Info_Version_FlightSoftwareVersionType")
        generate!("mavsdk::Info_Version")
        block!("mavsdk::Info_GetFlightInformationCallback")
        block!("mavsdk::Info_GetIdentificationCallback")
        block!("mavsdk::Info_GetProductCallback")
        block!("mavsdk::Info_GetVersionCallback")
        block!("mavsdk::Info_GetSpeedFactorCallback")
        block!("mavsdk::Info_FlightInformationCallback")
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use info::*;
}
pub mod log_files {
    autocxx::include_cpp! {
        #include "log_files.h"
        name!(log_files)
        safety!(unsafe_ffi)

        generate!("mavsdk::LogFiles")
        generate_pod!("mavsdk::LogFiles_Result")
        generate_pod!("mavsdk::LogFiles_ProgressData")
        generate!("mavsdk::LogFiles_Entry")
        block!("mavsdk::LogFiles_GetEntriesCallback")
        block!("mavsdk::LogFiles_DownloadLogFileCallback")
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use log_files::*;
}
pub mod log_streaming {
    autocxx::include_cpp! {
        #include "log_streaming.h"
        name!(log_streaming)
        safety!(unsafe_ffi)

        generate!("mavsdk::LogStreaming")
        generate_pod!("mavsdk::LogStreaming_Result")
        generate!("mavsdk::LogStreaming_LogStreamingRaw")
        block!("mavsdk::LogStreaming_LogStreamingRawCallback")
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
        name!(mavlink_direct)
        safety!(unsafe_ffi)

        generate!("mavsdk::MavlinkDirect")
        generate_pod!("mavsdk::MavlinkDirect_Result")
        generate!("mavsdk::MavlinkDirect_MavlinkMessage")
        block!("mavsdk::MavlinkDirect_MessageCallback")
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use mavlink_direct::*;
}
pub mod mission {
    autocxx::include_cpp! {
        #include "mission.h"
        name!(mission)
        safety!(unsafe_ffi)

        generate!("mavsdk::Mission")
        generate_pod!("mavsdk::Mission_Result")
        generate_pod!("mavsdk::Mission_MissionItem_CameraAction")
        generate_pod!("mavsdk::Mission_MissionItem_VehicleAction")
        generate!("mavsdk::Mission_MissionItem")
        generate!("mavsdk::Mission_MissionPlan")
        generate_pod!("mavsdk::Mission_MissionProgress")
        generate_pod!("mavsdk::Mission_ProgressData")
        generate!("mavsdk::Mission_ProgressDataOrMission")
        block!("mavsdk::Mission_UploadMissionWithProgressCallback")
        block!("mavsdk::Mission_DownloadMissionCallback")
        block!("mavsdk::Mission_DownloadMissionWithProgressCallback")
        block!("mavsdk::Mission_IsMissionFinishedCallback")
        block!("mavsdk::Mission_MissionProgressCallback")
        block!("mavsdk::Mission_GetReturnToLaunchAfterMissionCallback")
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use mission::*;
}
pub mod mission_raw {
    autocxx::include_cpp! {
        #include "mission_raw.h"
        name!(mission_raw)
        safety!(unsafe_ffi)

        generate!("mavsdk::MissionRaw")
        generate_pod!("mavsdk::MissionRaw_Result")
        generate_pod!("mavsdk::MissionRaw_MissionProgress")
        generate_pod!("mavsdk::MissionRaw_MissionItem")
        generate!("mavsdk::MissionRaw_MissionImportData")
        block!("mavsdk::MissionRaw_DownloadMissionCallback")
        block!("mavsdk::MissionRaw_DownloadGeofenceCallback")
        block!("mavsdk::MissionRaw_DownloadRallypointsCallback")
        block!("mavsdk::MissionRaw_MissionProgressCallback")
        block!("mavsdk::MissionRaw_MissionChangedCallback")
        block!("mavsdk::MissionRaw_ImportQgroundcontrolMissionCallback")
        block!("mavsdk::MissionRaw_ImportQgroundcontrolMissionFromStringCallback")
        block!("mavsdk::MissionRaw_ImportMissionPlannerMissionCallback")
        block!("mavsdk::MissionRaw_ImportMissionPlannerMissionFromStringCallback")
        block!("mavsdk::MissionRaw_IsMissionFinishedCallback")
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use mission_raw::*;
}
pub mod mission_raw_server {
    autocxx::include_cpp! {
        #include "mission_raw_server.h"
        name!(mission_raw_server)
        safety!(unsafe_ffi)

        generate!("mavsdk::MissionRawServer")
        generate_pod!("mavsdk::MissionRawServer_Result")
        generate_pod!("mavsdk::MissionRawServer_MissionItem")
        generate!("mavsdk::MissionRawServer_MissionPlan")
        generate_pod!("mavsdk::MissionRawServer_MissionProgress")
        block!("mavsdk::MissionRawServer_IncomingMissionCallback")
        block!("mavsdk::MissionRawServer_CurrentItemChangedCallback")
        block!("mavsdk::MissionRawServer_ClearAllCallback")
        extern_cpp_type!("mavsdk::ServerComponent", crate::core::mavsdk::ServerComponent)
    }
    pub use mission_raw_server::*;
}
pub mod mocap {
    autocxx::include_cpp! {
        #include "mocap.h"
        name!(mocap)
        safety!(unsafe_ffi)

        generate!("mavsdk::Mocap")
        generate_pod!("mavsdk::Mocap_Result")
        generate_pod!("mavsdk::Mocap_PositionBody")
        generate_pod!("mavsdk::Mocap_AngleBody")
        generate_pod!("mavsdk::Mocap_SpeedBody")
        generate_pod!("mavsdk::Mocap_SpeedNed")
        generate_pod!("mavsdk::Mocap_AngularVelocityBody")
        generate!("mavsdk::Mocap_Covariance")
        generate_pod!("mavsdk::Mocap_Quaternion")
        generate!("mavsdk::Mocap_VisionPositionEstimate")
        generate!("mavsdk::Mocap_VisionSpeedEstimate")
        generate!("mavsdk::Mocap_AttitudePositionMocap")
        generate_pod!("mavsdk::Mocap_Odometry_MavFrame")
        generate!("mavsdk::Mocap_Odometry")
        
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use mocap::*;
}
pub mod offboard {
    autocxx::include_cpp! {
        #include "offboard.h"
        name!(offboard)
        safety!(unsafe_ffi)

        generate!("mavsdk::Offboard")
        generate_pod!("mavsdk::Offboard_Result")
        generate_pod!("mavsdk::Offboard_Attitude")
        generate!("mavsdk::Offboard_ActuatorControlGroup")
        generate!("mavsdk::Offboard_ActuatorControl")
        generate_pod!("mavsdk::Offboard_AttitudeRate")
        generate_pod!("mavsdk::Offboard_PositionNedYaw")
        generate_pod!("mavsdk::Offboard_PositionGlobalYaw_AltitudeType")
        generate!("mavsdk::Offboard_PositionGlobalYaw")
        generate_pod!("mavsdk::Offboard_VelocityBodyYawspeed")
        generate_pod!("mavsdk::Offboard_VelocityNedYaw")
        generate_pod!("mavsdk::Offboard_AccelerationNed")
        block!("mavsdk::Offboard_IsActiveCallback")
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use offboard::*;
}
pub mod param {
    autocxx::include_cpp! {
        #include "param.h"
        name!(param)
        safety!(unsafe_ffi)

        generate!("mavsdk::Param")
        generate_pod!("mavsdk::Param_Result")
        generate!("mavsdk::Param_IntParam")
        generate!("mavsdk::Param_FloatParam")
        generate!("mavsdk::Param_CustomParam")
        generate!("mavsdk::Param_AllParams")
        generate_pod!("mavsdk::Param_ProtocolVersion")
        block!("mavsdk::Param_GetParamIntCallback")
        block!("mavsdk::Param_GetParamFloatCallback")
        block!("mavsdk::Param_GetParamCustomCallback")
        block!("mavsdk::Param_GetAllParamsCallback")
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use param::*;
}
pub mod param_server {
    autocxx::include_cpp! {
        #include "param_server.h"
        name!(param_server)
        safety!(unsafe_ffi)

        generate!("mavsdk::ParamServer")
        generate_pod!("mavsdk::ParamServer_Result")
        generate!("mavsdk::ParamServer_IntParam")
        generate!("mavsdk::ParamServer_FloatParam")
        generate!("mavsdk::ParamServer_CustomParam")
        generate!("mavsdk::ParamServer_AllParams")
        block!("mavsdk::ParamServer_RetrieveParamIntCallback")
        block!("mavsdk::ParamServer_RetrieveParamFloatCallback")
        block!("mavsdk::ParamServer_RetrieveParamCustomCallback")
        block!("mavsdk::ParamServer_RetrieveAllParamsCallback")
        block!("mavsdk::ParamServer_ChangedParamIntCallback")
        block!("mavsdk::ParamServer_ChangedParamFloatCallback")
        block!("mavsdk::ParamServer_ChangedParamCustomCallback")
        extern_cpp_type!("mavsdk::ServerComponent", crate::core::mavsdk::ServerComponent)
    }
    pub use param_server::*;
}
pub mod rtk {
    autocxx::include_cpp! {
        #include "rtk.h"
        name!(rtk)
        safety!(unsafe_ffi)

        generate!("mavsdk::Rtk")
        generate_pod!("mavsdk::Rtk_Result")
        generate!("mavsdk::Rtk_RtcmData")
        
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
        generate_pod!("mavsdk::ServerUtility_Result")
        generate_pod!("mavsdk::ServerUtility_StatusTextType")
        
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use server_utility::*;
}
pub mod shell {
    autocxx::include_cpp! {
        #include "shell.h"
        name!(shell)
        safety!(unsafe_ffi)

        generate!("mavsdk::Shell")
        generate_pod!("mavsdk::Shell_Result")
        block!("mavsdk::Shell_ReceiveCallback")
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use shell::*;
}
pub mod telemetry {
    autocxx::include_cpp! {
        #include "telemetry.h"
        name!(telemetry)
        safety!(unsafe_ffi)

        generate!("mavsdk::Telemetry")
        generate_pod!("mavsdk::Telemetry_Result")
        generate_pod!("mavsdk::Telemetry_Position")
        generate_pod!("mavsdk::Telemetry_Heading")
        generate_pod!("mavsdk::Telemetry_Quaternion")
        generate_pod!("mavsdk::Telemetry_EulerAngle")
        generate_pod!("mavsdk::Telemetry_AngularVelocityBody")
        generate!("mavsdk::Telemetry_GpsInfo")
        generate_pod!("mavsdk::Telemetry_RawGps")
        generate!("mavsdk::Telemetry_Battery")
        generate_pod!("mavsdk::Telemetry_Health")
        generate_pod!("mavsdk::Telemetry_RcStatus")
        generate!("mavsdk::Telemetry_StatusText")
        generate!("mavsdk::Telemetry_ActuatorControlTarget")
        generate!("mavsdk::Telemetry_ActuatorOutputStatus")
        generate!("mavsdk::Telemetry_Covariance")
        generate_pod!("mavsdk::Telemetry_VelocityBody")
        generate_pod!("mavsdk::Telemetry_PositionBody")
        generate_pod!("mavsdk::Telemetry_Odometry_MavFrame")
        generate_pod!("mavsdk::Telemetry_Odometry_MavFrame")
        generate!("mavsdk::Telemetry_Odometry")
        generate!("mavsdk::Telemetry_DistanceSensor")
        generate_pod!("mavsdk::Telemetry_ScaledPressure")
        generate_pod!("mavsdk::Telemetry_PositionNed")
        generate_pod!("mavsdk::Telemetry_VelocityNed")
        generate!("mavsdk::Telemetry_PositionVelocityNed")
        generate_pod!("mavsdk::Telemetry_GroundTruth")
        generate_pod!("mavsdk::Telemetry_FixedwingMetrics")
        generate_pod!("mavsdk::Telemetry_AccelerationFrd")
        generate_pod!("mavsdk::Telemetry_AngularVelocityFrd")
        generate_pod!("mavsdk::Telemetry_MagneticFieldFrd")
        generate!("mavsdk::Telemetry_Imu")
        generate_pod!("mavsdk::Telemetry_GpsGlobalOrigin")
        generate_pod!("mavsdk::Telemetry_Altitude")
        generate_pod!("mavsdk::Telemetry_Wind")
        generate_pod!("mavsdk::Telemetry_FixType")
        generate_pod!("mavsdk::Telemetry_BatteryFunction")
        generate_pod!("mavsdk::Telemetry_FlightMode")
        generate_pod!("mavsdk::Telemetry_StatusTextType")
        generate_pod!("mavsdk::Telemetry_LandedState")
        generate_pod!("mavsdk::Telemetry_VtolState")
        block!("mavsdk::Telemetry_PositionCallback")
        block!("mavsdk::Telemetry_HomeCallback")
        block!("mavsdk::Telemetry_InAirCallback")
        block!("mavsdk::Telemetry_LandedStateCallback")
        block!("mavsdk::Telemetry_ArmedCallback")
        block!("mavsdk::Telemetry_VtolStateCallback")
        block!("mavsdk::Telemetry_AttitudeQuaternionCallback")
        block!("mavsdk::Telemetry_AttitudeEulerCallback")
        block!("mavsdk::Telemetry_AttitudeAngularVelocityBodyCallback")
        block!("mavsdk::Telemetry_VelocityNedCallback")
        block!("mavsdk::Telemetry_GpsInfoCallback")
        block!("mavsdk::Telemetry_RawGpsCallback")
        block!("mavsdk::Telemetry_BatteryCallback")
        block!("mavsdk::Telemetry_FlightModeCallback")
        block!("mavsdk::Telemetry_HealthCallback")
        block!("mavsdk::Telemetry_RcStatusCallback")
        block!("mavsdk::Telemetry_StatusTextCallback")
        block!("mavsdk::Telemetry_ActuatorControlTargetCallback")
        block!("mavsdk::Telemetry_ActuatorOutputStatusCallback")
        block!("mavsdk::Telemetry_OdometryCallback")
        block!("mavsdk::Telemetry_PositionVelocityNedCallback")
        block!("mavsdk::Telemetry_GroundTruthCallback")
        block!("mavsdk::Telemetry_FixedwingMetricsCallback")
        block!("mavsdk::Telemetry_ImuCallback")
        block!("mavsdk::Telemetry_ScaledImuCallback")
        block!("mavsdk::Telemetry_RawImuCallback")
        block!("mavsdk::Telemetry_HealthAllOkCallback")
        block!("mavsdk::Telemetry_UnixEpochTimeCallback")
        block!("mavsdk::Telemetry_DistanceSensorCallback")
        block!("mavsdk::Telemetry_ScaledPressureCallback")
        block!("mavsdk::Telemetry_HeadingCallback")
        block!("mavsdk::Telemetry_AltitudeCallback")
        block!("mavsdk::Telemetry_WindCallback")
        block!("mavsdk::Telemetry_GetGpsGlobalOriginCallback")
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use telemetry::*;
}
pub mod telemetry_server {
    autocxx::include_cpp! {
        #include "telemetry_server.h"
        name!(telemetry_server)
        safety!(unsafe_ffi)

        generate!("mavsdk::TelemetryServer")
        generate_pod!("mavsdk::TelemetryServer_Result")
        generate_pod!("mavsdk::TelemetryServer_Position")
        generate_pod!("mavsdk::TelemetryServer_Heading")
        generate_pod!("mavsdk::TelemetryServer_Quaternion")
        generate_pod!("mavsdk::TelemetryServer_EulerAngle")
        generate_pod!("mavsdk::TelemetryServer_AngularVelocityBody")
        generate!("mavsdk::TelemetryServer_GpsInfo")
        generate_pod!("mavsdk::TelemetryServer_RawGps")
        generate_pod!("mavsdk::TelemetryServer_Battery")
        generate_pod!("mavsdk::TelemetryServer_RcStatus")
        generate!("mavsdk::TelemetryServer_StatusText")
        generate!("mavsdk::TelemetryServer_ActuatorControlTarget")
        generate!("mavsdk::TelemetryServer_ActuatorOutputStatus")
        generate!("mavsdk::TelemetryServer_Covariance")
        generate_pod!("mavsdk::TelemetryServer_VelocityBody")
        generate_pod!("mavsdk::TelemetryServer_PositionBody")
        generate_pod!("mavsdk::TelemetryServer_Odometry_MavFrame")
        generate_pod!("mavsdk::TelemetryServer_Odometry_MavFrame")
        generate!("mavsdk::TelemetryServer_Odometry")
        generate_pod!("mavsdk::TelemetryServer_DistanceSensor")
        generate_pod!("mavsdk::TelemetryServer_ScaledPressure")
        generate_pod!("mavsdk::TelemetryServer_PositionNed")
        generate_pod!("mavsdk::TelemetryServer_VelocityNed")
        generate!("mavsdk::TelemetryServer_PositionVelocityNed")
        generate_pod!("mavsdk::TelemetryServer_GroundTruth")
        generate_pod!("mavsdk::TelemetryServer_FixedwingMetrics")
        generate_pod!("mavsdk::TelemetryServer_AccelerationFrd")
        generate_pod!("mavsdk::TelemetryServer_AngularVelocityFrd")
        generate_pod!("mavsdk::TelemetryServer_MagneticFieldFrd")
        generate!("mavsdk::TelemetryServer_Imu")
        generate_pod!("mavsdk::TelemetryServer_FixType")
        generate_pod!("mavsdk::TelemetryServer_VtolState")
        generate_pod!("mavsdk::TelemetryServer_StatusTextType")
        generate_pod!("mavsdk::TelemetryServer_LandedState")
        
        extern_cpp_type!("mavsdk::ServerComponent", crate::core::mavsdk::ServerComponent)
    }
    pub use telemetry_server::*;
}
pub mod transponder {
    autocxx::include_cpp! {
        #include "transponder.h"
        name!(transponder)
        safety!(unsafe_ffi)

        generate!("mavsdk::Transponder")
        generate_pod!("mavsdk::Transponder_Result")
        generate!("mavsdk::Transponder_AdsbVehicle")
        generate_pod!("mavsdk::Transponder_AdsbEmitterType")
        generate_pod!("mavsdk::Transponder_AdsbAltitudeType")
        block!("mavsdk::Transponder_TransponderCallback")
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use transponder::*;
}
pub mod tune {
    autocxx::include_cpp! {
        #include "tune.h"
        name!(tune)
        safety!(unsafe_ffi)

        generate!("mavsdk::Tune")
        generate_pod!("mavsdk::Tune_Result")
        generate!("mavsdk::Tune_TuneDescription")
        generate_pod!("mavsdk::Tune_SongElement")
        
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use tune::*;
}
pub mod winch {
    autocxx::include_cpp! {
        #include "winch.h"
        name!(winch)
        safety!(unsafe_ffi)

        generate!("mavsdk::Winch")
        generate_pod!("mavsdk::Winch_Result")
        generate_pod!("mavsdk::Winch_StatusFlags")
        generate!("mavsdk::Winch_Status")
        generate_pod!("mavsdk::Winch_WinchAction")
        block!("mavsdk::Winch_StatusCallback")
        extern_cpp_type!("mavsdk::System", crate::core::mavsdk::System)
    }
    pub use winch::*;
}
