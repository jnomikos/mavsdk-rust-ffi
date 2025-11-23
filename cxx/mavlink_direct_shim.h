#pragma once

#include "mavlink_direct.h"
#include "shim_macros.h"

namespace mavlink_direct_subscriptions {
    DECLARE_SUBSCRIBE_SHIM_WITH_EXTRA_PARAM(mavsdk::MavlinkDirect*, subscribe_message, mavsdk::MavlinkDirect::MessageCallback, std::string)
}