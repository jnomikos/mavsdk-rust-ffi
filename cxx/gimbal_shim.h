#pragma once

#include "gimbal.h"
#include "shim_macros.h"

namespace gimbal_subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::Gimbal*, subscribe_gimbal_list, mavsdk::Gimbal::GimbalListCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Gimbal*, subscribe_control_status, mavsdk::Gimbal::ControlStatusCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Gimbal*, subscribe_attitude, mavsdk::Gimbal::AttitudeCallback)
}