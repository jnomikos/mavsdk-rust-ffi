#include <iostream>
#include <future>
#include "mavsdk_shim.h"

std::shared_ptr<mavsdk::System> mavsdk_wait_on_new_system(
    void* object
) {
    if (!object) return nullptr;

    auto* mavsdk_instance = static_cast<mavsdk::Mavsdk*>(object);

    std::promise<void> new_system_promise;
    auto new_system_future = new_system_promise.get_future();

    mavsdk_instance->subscribe_on_new_system(
        [&new_system_promise]() {
            new_system_promise.set_value();
        }
    );

    new_system_future.wait();
    return mavsdk_instance->systems().back();
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

uintptr_t subscribe_on_new_system(mavsdk::Mavsdk* mavsdk_instance, uintptr_t cb_ptr) {
    SystemCallbackFn callback = reinterpret_cast<SystemCallbackFn>(cb_ptr);
    mavsdk::Mavsdk::NewSystemHandle system_handle = mavsdk_instance->subscribe_on_new_system(
        [callback, mavsdk_instance]() {
            if (callback) {
                auto new_system = mavsdk_instance->systems().back();
                if(new_system) {
                    callback(new_system);
                }
            }
        }
    );

    return reinterpret_cast<uintptr_t>(new mavsdk::Mavsdk::NewSystemHandle(system_handle));
}

void unsubscribe_on_new_system(mavsdk::Mavsdk* mavsdk_instance, uintptr_t handle_ptr) {
    if (!handle_ptr) {
        std::cerr << "Handle pointer is null." << std::endl;
        return;
    }

    auto* handle = reinterpret_cast<mavsdk::Mavsdk::NewSystemHandle*>(handle_ptr);
    mavsdk_instance->unsubscribe_on_new_system(*handle);
    delete handle;
}