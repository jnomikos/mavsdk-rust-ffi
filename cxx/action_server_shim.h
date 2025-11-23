#pragma once

#include "action_server.h"
#include "shim_macros.h"

namespace action_server_subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::ActionServer*, subscribe_arm_disarm, mavsdk::ActionServer::ArmDisarmCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::ActionServer*, subscribe_flight_mode_change, mavsdk::ActionServer::FlightModeChangeCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::ActionServer*, subscribe_takeoff, mavsdk::ActionServer::TakeoffCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::ActionServer*, subscribe_land, mavsdk::ActionServer::LandCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::ActionServer*, subscribe_reboot, mavsdk::ActionServer::RebootCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::ActionServer*, subscribe_shutdown, mavsdk::ActionServer::ShutdownCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::ActionServer*, subscribe_terminate, mavsdk::ActionServer::TerminateCallback)
}