#pragma once

#include "camera.h"
#include "shim_macros.h"

namespace subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::Camera*, subscribe_camera_list, mavsdk::Camera::CameraListCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Camera*, subscribe_mode, mavsdk::Camera::ModeCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Camera*, subscribe_video_stream_info, mavsdk::Camera::VideoStreamInfoCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Camera*, subscribe_capture_info, mavsdk::Camera::CaptureInfoCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Camera*, subscribe_storage, mavsdk::Camera::StorageCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Camera*, subscribe_current_settings, mavsdk::Camera::CurrentSettingsCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Camera*, subscribe_possible_setting_options, mavsdk::Camera::PossibleSettingOptionsCallback)
}