#pragma once

#include "param_server.h"
#include "shim_macros.h"

namespace subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::ParamServer*, subscribe_changed_param_int, mavsdk::ParamServer::ChangedParamIntCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::ParamServer*, subscribe_changed_param_float, mavsdk::ParamServer::ChangedParamFloatCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::ParamServer*, subscribe_changed_param_custom, mavsdk::ParamServer::ChangedParamCustomCallback)
}