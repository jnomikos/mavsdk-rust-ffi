#pragma once

#include "mission.h"
#include "shim_macros.h"

namespace subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::Mission*, subscribe_mission_progress, mavsdk::Mission::MissionProgressCallback)
}