// TODO: Compile shim code so linker can find it
#pragma once
#include <mutex>
#include <cstdint>

#include "mavsdk.h"
#include "system.h"


typedef void (*new_system_cb_t)(void* userdata);

static std::unique_ptr<mavsdk::Mavsdk::NewSystemHandle> g_handle;


namespace mavsdk {
    // This makes the nested typedef usable as a proper FFI target
    using Mavsdk_ConnectionHandle = Mavsdk::ConnectionHandle;
}

std::shared_ptr<mavsdk::System> mavsdk_wait_on_new_system(
    void* object
);

std::shared_ptr<mavsdk::System> mavsdk_system_get(
    mavsdk::Mavsdk* mavsdk_instance,
    size_t index
);

using SystemCallbackFn = void(*)(std::shared_ptr<mavsdk::System> system);
uintptr_t subscribe_on_new_system(mavsdk::Mavsdk* mavsdk_instance, uintptr_t cb_ptr);

void unsubscribe_on_new_system(mavsdk::Mavsdk* mavsdk_instance, uintptr_t handle_ptr);