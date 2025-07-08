#pragma once
#include "mavsdk/core/include/mavsdk/mavsdk.h"

namespace mavsdk {
    // This makes the nested typedef usable as a proper FFI target
    using Mavsdk_ConnectionHandle = Mavsdk::ConnectionHandle;

    inline std::unique_ptr<Mavsdk> Mavsdk_new(
        const mavsdk::Mavsdk::Configuration& config) {
        return std::make_unique<Mavsdk>(config);
    }
}

int test_function() {
    return 42;
}