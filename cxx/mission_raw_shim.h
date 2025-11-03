#pragma once

#include "mission_raw.h"
#include "shim_macros.h"

namespace subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::MissionRaw*, subscribe_mission_progress, mavsdk::MissionRaw::MissionProgressCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::MissionRaw*, subscribe_mission_changed, mavsdk::MissionRaw::MissionChangedCallback)
}