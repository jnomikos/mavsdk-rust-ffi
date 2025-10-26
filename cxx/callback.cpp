#include "callback.h"
#include <cstdint>

static CallbackFn g_cb = nullptr;
static void* g_user_data = nullptr;

extern "C" {

void register_callback_shim(uintptr_t cb_ptr, void* user_data) {
    g_cb = reinterpret_cast<CallbackFn>(cb_ptr);
    g_user_data = user_data;
}

void unregister_callback_shim() {
    g_cb = nullptr;
    g_user_data = nullptr;
}

void trigger_callback_shim(int value) {
    if (g_cb) {
        g_cb(g_user_data, value);
    }
}

}