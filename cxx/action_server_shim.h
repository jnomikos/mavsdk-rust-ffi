#pragma once

#include "action_server.h"
#include "shim_macros.h"

namespace subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::ActionServer*, subscribe_arm_disarm, mavsdk::ActionServer::Result, mavsdk::ActionServer::ArmDisarm)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::ActionServer*, subscribe_flight_mode_change, mavsdk::ActionServer::Result, mavsdk::ActionServer::FlightMode)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::ActionServer*, subscribe_takeoff, mavsdk::ActionServer::Result, bool)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::ActionServer*, subscribe_land, mavsdk::ActionServer::Result, bool)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::ActionServer*, subscribe_reboot, mavsdk::ActionServer::Result, bool)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::ActionServer*, subscribe_shutdown, mavsdk::ActionServer::Result, bool)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::ActionServer*, subscribe_terminate, mavsdk::ActionServer::Result, bool)
}