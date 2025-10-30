// TODO: Compile shim code so linker can find it
#pragma once

#include "camera.h"
#include "shim_macros.h"

namespace subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::Camera*, subscribe_camera_list, mavsdk::Camera::CameraList)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Camera*, subscribe_mode, mavsdk::Camera::ModeUpdate)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Camera*, subscribe_video_stream_info, mavsdk::Camera::VideoStreamUpdate)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Camera*, subscribe_capture_info, mavsdk::Camera::CaptureInfo)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Camera*, subscribe_storage, mavsdk::Camera::StorageUpdate)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Camera*, subscribe_current_settings, mavsdk::Camera::CurrentSettingsUpdate)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Camera*, subscribe_possible_setting_options, mavsdk::Camera::PossibleSettingOptionsUpdate)
}