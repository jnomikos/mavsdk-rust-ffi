#pragma once

#include "info.h"
#include "shim_macros.h"

namespace subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::Info*, subscribe_flight_information, mavsdk::Info::FlightInformationCallback)
}