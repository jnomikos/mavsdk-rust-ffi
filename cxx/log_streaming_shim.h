#pragma once

#include "log_streaming.h"
#include "shim_macros.h"

namespace subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::LogStreaming*, subscribe_log_streaming_raw, mavsdk::LogStreaming::LogStreamingRawCallback)
}