// TODO: Compile shim code so linker can find it
#pragma once
#include <map>
#include <mutex>

#include "mavsdk.h"
#include "system.h"


typedef void (*new_system_cb_t)(void* userdata);

static std::unique_ptr<mavsdk::Mavsdk::NewSystemHandle> g_handle;


namespace mavsdk {
    // This makes the nested typedef usable as a proper FFI target
    using Mavsdk_ConnectionHandle = Mavsdk::ConnectionHandle;
}

extern "C" {
void mavsdk_wait_on_new_system(
    void* object
);
}

std::shared_ptr<mavsdk::System> mavsdk_system_get(
    mavsdk::Mavsdk* mavsdk_instance,
    size_t index
);

void* test_function_handle(int number);

int test_function();