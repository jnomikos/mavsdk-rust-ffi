#ifndef SHIM_MACRO_H_INC
#define SHIM_MACRO_H_INC

#include <functional>
#include "handle.h"

template<typename T>
struct function_traits;
template<typename ReturnType, typename... Args>
struct function_traits<std::function<ReturnType(Args...)>> {
    using return_type = ReturnType;
    using as_handle = mavsdk::Handle<Args...>;
    using type = ReturnType(*)(Args...);
};

#define DECLARE_SUBSCRIBE_SHIM(INSTANCE_TYPE, SUBSCRIBE_FN, CALLBACK_FUNCTION) \
    DECLARE_SUBSCRIBE_SHIM_WITH_HANDLE(INSTANCE_TYPE, SUBSCRIBE_FN, CALLBACK_FUNCTION, typename function_traits<CALLBACK_FUNCTION>::as_handle) \

#define DECLARE_SUBSCRIBE_SHIM_WITH_HANDLE(INSTANCE_TYPE, SUBSCRIBE_FN, CALLBACK_FUNCTION, HANDLE_TYPE) \
    uintptr_t SUBSCRIBE_FN(INSTANCE_TYPE instance, uintptr_t cb_ptr) { \
        using CallbackType = typename function_traits<CALLBACK_FUNCTION>::type; \
        using ReturnType = typename function_traits<CALLBACK_FUNCTION>::return_type; \
        auto callback = reinterpret_cast<CallbackType>(cb_ptr); \
        auto handle = instance->SUBSCRIBE_FN( \
            [callback, instance](auto&&... args) -> ReturnType { \
                return callback(std::forward<decltype(args)>(args)...); \
            } \
        ); \
        auto handle_ptr = new decltype(handle)(handle); \
        return reinterpret_cast<uintptr_t>(handle_ptr); \
    } \
    void un##SUBSCRIBE_FN(INSTANCE_TYPE instance, uintptr_t handle_ptr) { \
        if (!handle_ptr) { \
            return; \
        } \
        auto* handle = reinterpret_cast<HANDLE_TYPE*>(handle_ptr); \
        instance->un##SUBSCRIBE_FN(*handle); \
        delete handle; \
    }

#define DECLARE_SUBSCRIBE_SHIM_WITH_HANDLE_AND_EXTRA_PARAM(INSTANCE_TYPE, SUBSCRIBE_FN, CALLBACK_FUNCTION, HANDLE_TYPE, EXTRA_PARAM_TYPE) \
uintptr_t SUBSCRIBE_FN(INSTANCE_TYPE instance, EXTRA_PARAM_TYPE extra, uintptr_t cb_ptr) { \
    using CallbackType = typename function_traits<CALLBACK_FUNCTION>::type; \
    using ReturnType = typename function_traits<CALLBACK_FUNCTION>::return_type; \
    auto callback = reinterpret_cast<CallbackType>(cb_ptr); \
    auto handle = instance->SUBSCRIBE_FN(extra, \
        [callback, instance](auto&&... args) -> ReturnType { \
            return callback(std::forward<decltype(args)>(args)...); \
        } \
    ); \
    auto handle_ptr = new decltype(handle)(handle); \
    return reinterpret_cast<uintptr_t>(handle_ptr); \
} \
void un##SUBSCRIBE_FN(INSTANCE_TYPE instance, uintptr_t handle_ptr) { \
    if (!handle_ptr) { \
        return; \
    } \
    auto* handle = reinterpret_cast<HANDLE_TYPE*>(handle_ptr); \
    instance->un##SUBSCRIBE_FN(*handle); \
    delete handle; \
}

#define DECLARE_SUBSCRIBE_SHIM_WITH_EXTRA_PARAM(INSTANCE_TYPE, SUBSCRIBE_FN, CALLBACK_FUNCTION, EXTRA_PARAM_TYPE) \
    DECLARE_SUBSCRIBE_SHIM_WITH_HANDLE_AND_EXTRA_PARAM(INSTANCE_TYPE, SUBSCRIBE_FN, CALLBACK_FUNCTION, typename function_traits<CALLBACK_FUNCTION>::as_handle, EXTRA_PARAM_TYPE)

#endif
