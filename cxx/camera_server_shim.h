#pragma once

#include "camera_server.h"
#include "shim_macros.h"

namespace subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::CameraServer*, subscribe_take_photo, mavsdk::CameraServer::TakePhotoCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::CameraServer*, subscribe_start_video, mavsdk::CameraServer::StartVideoCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::CameraServer*, subscribe_stop_video, mavsdk::CameraServer::StopVideoCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::CameraServer*, subscribe_start_video_streaming, mavsdk::CameraServer::StartVideoStreamingCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::CameraServer*, subscribe_stop_video_streaming, mavsdk::CameraServer::StopVideoStreamingCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::CameraServer*, subscribe_set_mode, mavsdk::CameraServer::SetModeCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::CameraServer*, subscribe_storage_information, mavsdk::CameraServer::StorageInformationCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::CameraServer*, subscribe_capture_status, mavsdk::CameraServer::CaptureStatusCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::CameraServer*, subscribe_format_storage, mavsdk::CameraServer::FormatStorageCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::CameraServer*, subscribe_reset_settings, mavsdk::CameraServer::ResetSettingsCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::CameraServer*, subscribe_zoom_in_start, mavsdk::CameraServer::ZoomInStartCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::CameraServer*, subscribe_zoom_out_start, mavsdk::CameraServer::ZoomOutStartCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::CameraServer*, subscribe_zoom_stop, mavsdk::CameraServer::ZoomStopCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::CameraServer*, subscribe_zoom_range, mavsdk::CameraServer::ZoomRangeCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::CameraServer*, subscribe_tracking_point_command, mavsdk::CameraServer::TrackingPointCommandCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::CameraServer*, subscribe_tracking_rectangle_command, mavsdk::CameraServer::TrackingRectangleCommandCallback)
    
    DECLARE_SUBSCRIBE_SHIM(mavsdk::CameraServer*, subscribe_tracking_off_command, mavsdk::CameraServer::TrackingOffCommandCallback)
}