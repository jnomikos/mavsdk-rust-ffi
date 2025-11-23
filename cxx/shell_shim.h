#pragma once

#include "shell.h"
#include "shim_macros.h"

namespace shell_subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::Shell*, subscribe_receive, mavsdk::Shell::ReceiveCallback)
}