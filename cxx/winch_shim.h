#pragma once

#include "winch.h"
#include "shim_macros.h"

namespace winch_subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::Winch*, subscribe_status, mavsdk::Winch::StatusCallback)
}