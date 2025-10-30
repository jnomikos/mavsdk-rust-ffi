// TODO: Compile shim code so linker can find it
#pragma once

#include "mavsdk.h"
#include "system.h"
#include "shim_macros.h"

namespace mavsdk {
    // This makes the nested typedef usable as a proper FFI target
    using Mavsdk_ConnectionHandle = Mavsdk::ConnectionHandle;
}

namespace subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::Mavsdk*, subscribe_connection_errors, mavsdk::Mavsdk::ConnectionError)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Mavsdk*, subscribe_on_new_system)

    DECLARE_SUBSCRIBE_SHIM_RET(mavsdk::Mavsdk*, bool, subscribe_incoming_messages_json, mavsdk::Mavsdk::MavlinkMessage)

    DECLARE_SUBSCRIBE_SHIM_RET(mavsdk::Mavsdk*, bool, subscribe_outgoing_messages_json, mavsdk::Mavsdk::MavlinkMessage)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::System*, subscribe_is_connected, bool)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::System*, subscribe_component_discovered, mavsdk::ComponentType)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::System*, subscribe_component_discovered_id, mavsdk::ComponentType, uint8_t)
}