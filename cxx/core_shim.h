// TODO: Compile shim code so linker can find it
#pragma once

#include "mavsdk.h"
#include "system.h"
#include "shim_macros.h"

namespace mavsdk {
    // This makes the nested typedef usable as a proper FFI target
    using Mavsdk_ConnectionHandle = Mavsdk::ConnectionHandle;
}

namespace core_subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::Mavsdk*, subscribe_connection_errors, mavsdk::Mavsdk::ConnectionErrorCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Mavsdk*, subscribe_on_new_system, mavsdk::Mavsdk::NewSystemCallback)

    DECLARE_SUBSCRIBE_SHIM_WITH_HANDLE(mavsdk::Mavsdk*, subscribe_incoming_messages_json, mavsdk::Mavsdk::InterceptJsonCallback, mavsdk::Mavsdk::InterceptJsonHandle)

    DECLARE_SUBSCRIBE_SHIM_WITH_HANDLE(mavsdk::Mavsdk*,
    subscribe_raw_bytes_to_be_sent,
    mavsdk::Mavsdk::RawBytesCallback,
    mavsdk::Mavsdk::RawBytesHandle)

    DECLARE_SUBSCRIBE_SHIM_WITH_HANDLE(mavsdk::Mavsdk*, subscribe_outgoing_messages_json, mavsdk::Mavsdk::InterceptJsonCallback, mavsdk::Mavsdk::InterceptJsonHandle)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::System*, subscribe_is_connected, mavsdk::System::IsConnectedCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::System*, subscribe_component_discovered, mavsdk::System::ComponentDiscoveredCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::System*, subscribe_component_discovered_id, mavsdk::System::ComponentDiscoveredIdCallback)
}