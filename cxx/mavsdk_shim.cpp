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

std::shared_ptr<mavsdk::System> mavsdk_system_get(
    mavsdk::Mavsdk* mavsdk_instance,
    size_t index
) {
    if (!mavsdk_instance) {
        std::cerr << "Mavsdk instance is null." << std::endl;
        return nullptr;
    }

    auto systems = mavsdk_instance->systems();
    if (index >= systems.size()) {
        std::cerr << "Index out of bounds: " << index << " for systems size: " << systems.size() << std::endl;
        return nullptr;
    }
    return systems[index];
}