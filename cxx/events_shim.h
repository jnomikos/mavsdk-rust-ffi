#pragma once

#include "events.h"
#include "shim_macros.h"

namespace events_subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::Events*, subscribe_events, mavsdk::Events::EventsCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Events*, subscribe_health_and_arming_checks, mavsdk::Events::HealthAndArmingChecksCallback)
}