#pragma once

#include "mission_raw_server.h"
#include "shim_macros.h"

namespace mission_raw_server_subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::MissionRawServer*, subscribe_incoming_mission, mavsdk::MissionRawServer::IncomingMissionCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::MissionRawServer*, subscribe_current_item_changed, mavsdk::MissionRawServer::CurrentItemChangedCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::MissionRawServer*, subscribe_clear_all, mavsdk::MissionRawServer::ClearAllCallback)
}