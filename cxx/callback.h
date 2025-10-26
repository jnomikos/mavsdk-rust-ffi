#pragma once
#include <cstdint>

extern "C" {

using CallbackFn = void(*)(void* user_data, int value);

// Pass the function pointer as an integer-sized value (uintptr_t)
void register_callback_shim(uintptr_t cb_ptr, void* user_data);
void trigger_callback_shim(int value);
void unregister_callback_shim();

}