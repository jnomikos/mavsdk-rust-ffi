#pragma once

#include "mavsdk.h"
#include "system.h"
#include "shim_macros.h"

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

// Modify mavsdk class std::optional<std::shared_ptr<System>> first_autopilot(double timeout_s) const; to not use std::optional, and just return nullptr if no system found

/*
std::optional<std::shared_ptr<System>> Mavsdk::first_autopilot(double timeout_s) const
{
    return _impl->first_autopilot(timeout_s);
}

*/

std::shared_ptr<mavsdk::System> Mavsdk_first_autopilot(mavsdk::Mavsdk* mavsdk_instance, double timeout_s) {
    auto result = mavsdk_instance->first_autopilot(timeout_s);
    if (result.has_value()) {
        return result.value();
    }
    return nullptr;
}