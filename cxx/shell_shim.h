#pragma once

#include "shell.h"
#include "shim_macros.h"

namespace subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::Shell*, subscribe_receive, mavsdk::Shell::ReceiveCallback)
}