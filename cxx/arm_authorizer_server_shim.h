#pragma once

#include "arm_authorizer_server.h"
#include "shim_macros.h"

namespace arm_authorizer_server_subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::ArmAuthorizerServer*, subscribe_arm_authorization, mavsdk::ArmAuthorizerServer::ArmAuthorizationCallback)
}