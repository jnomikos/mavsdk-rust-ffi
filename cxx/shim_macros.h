#ifndef SHIM_MACRO_H_INC
#define SHIM_MACRO_H_INC
#include <iostream>
#include "mavsdk.h"


#define DECLARE_SUBSCRIBE_SHIM(INSTANCE_TYPE, SUBSCRIBE_FN, ...) \
    DECLARE_SUBSCRIBE_SHIM_RET(INSTANCE_TYPE, void, SUBSCRIBE_FN, __VA_ARGS__)

#define DECLARE_SUBSCRIBE_SHIM_RET(INSTANCE_TYPE, RETURN_TYPE, SUBSCRIBE_FN, ...) \
    uintptr_t SUBSCRIBE_FN(INSTANCE_TYPE instance, uintptr_t cb_ptr) { \
        using CallbackType = RETURN_TYPE(*)(__VA_ARGS__); \
        auto callback = reinterpret_cast<CallbackType>(cb_ptr); \
        auto handle = instance->SUBSCRIBE_FN( \
            [callback, instance](auto&&... args) -> RETURN_TYPE { \
                return callback(std::forward<decltype(args)>(args)...); \
            } \
        ); \
        auto handle_ptr = new decltype(handle)(handle); \
        return reinterpret_cast<uintptr_t>(handle_ptr); \
    } \
    void un##SUBSCRIBE_FN(INSTANCE_TYPE instance, uintptr_t handle_ptr) { \
        if (!handle_ptr) { \
            std::cerr << "Handle pointer is null." << std::endl; \
            return; \
        } \
        using HandleType = decltype(std::declval<INSTANCE_TYPE>()->SUBSCRIBE_FN( \
            std::declval<std::function<RETURN_TYPE(__VA_ARGS__)>>() \
        )); \
        auto* handle = reinterpret_cast<HandleType*>(handle_ptr); \
        instance->un##SUBSCRIBE_FN(*handle); \
        delete handle; \
    }
#endif
