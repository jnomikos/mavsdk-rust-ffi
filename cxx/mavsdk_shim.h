#pragma once
#include "mavsdk.h"

namespace mavsdk {
    // This makes the nested typedef usable as a proper FFI target
    using Mavsdk_ConnectionHandle = Mavsdk::ConnectionHandle;
}

int test_function() {
    return 42;
}