#include <iostream>
#include <future>
#include "mavsdk_shim.h"

extern "C" {
void mavsdk_wait_on_new_system(
    void* object
) {
    if (!object) return;

    auto* mavsdk_instance = static_cast<mavsdk::Mavsdk*>(object);

    std::promise<void> new_system_promise;
    auto new_system_future = new_system_promise.get_future();

    mavsdk_instance->subscribe_on_new_system(
        [&new_system_promise]() {
            new_system_promise.set_value();
        }
    );

    new_system_future.wait();
}
}