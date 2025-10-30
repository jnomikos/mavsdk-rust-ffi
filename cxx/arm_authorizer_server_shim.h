#pragma once

#include "arm_authorizer_server.h"
#include "shim_macros.h"

namespace subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::ArmAuthorizerServer*, subscribe_arm_authorization, uint32_t)
}