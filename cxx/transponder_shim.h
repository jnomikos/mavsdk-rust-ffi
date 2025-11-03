#pragma once

#include "transponder.h"
#include "shim_macros.h"

namespace subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::Transponder*, subscribe_transponder, mavsdk::Transponder::TransponderCallback)
}