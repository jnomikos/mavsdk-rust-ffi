#ifndef SHIM_MACRO_H_INC
#define SHIM_MACRO_H_INC
#include <iostream>
#include "mavsdk.h"


#define DECLARE_SUBSCRIBE_SHIM(INSTANCE_TYPE, SUBSCRIBE_FN, CALLBACK_PARAMETER_TYPE) \
    uintptr_t SUBSCRIBE_FN(INSTANCE_TYPE instance, uintptr_t cb_ptr) { \
        using CallbackType = void(*)(CALLBACK_PARAMETER_TYPE); \
        auto callback = reinterpret_cast<CallbackType>(cb_ptr); \
        auto handle = instance->SUBSCRIBE_FN( \
            [callback, instance](CALLBACK_PARAMETER_TYPE param) { \
                callback(param); \
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
            std::declval<std::function<void(CALLBACK_PARAMETER_TYPE)>>() \
        )); \
        auto* handle = reinterpret_cast<HandleType*>(handle_ptr); \
        instance->un##SUBSCRIBE_FN(*handle); \
        delete handle; \
    }
        
#define DECLARE_SUBSCRIBE_SHIM_NO_PARAM(INSTANCE_TYPE, SUBSCRIBE_FN) \
    uintptr_t SUBSCRIBE_FN(INSTANCE_TYPE instance, uintptr_t cb_ptr) { \
        using CallbackType = void(*)(); \
        auto callback = reinterpret_cast<CallbackType>(cb_ptr); \
        auto handle = instance->SUBSCRIBE_FN( \
            [callback, instance]() { \
                callback(); \
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
            std::declval<std::function<void()>>() \
        )); \
        auto* handle = reinterpret_cast<HandleType*>(handle_ptr); \
        instance->un##SUBSCRIBE_FN(*handle); \
        delete handle; \
    }

#define DECLARE_SUBSCRIBE_SHIM_BOOL(INSTANCE_TYPE, SUBSCRIBE_FN, CALLBACK_PARAMETER_TYPE) \
    uintptr_t SUBSCRIBE_FN(INSTANCE_TYPE instance, uintptr_t cb_ptr) { \
        using CallbackType = bool(*)(CALLBACK_PARAMETER_TYPE); \
        auto callback = reinterpret_cast<CallbackType>(cb_ptr); \
        auto handle = instance->SUBSCRIBE_FN( \
            [callback](CALLBACK_PARAMETER_TYPE param) -> bool { \
                return callback(param); \
            } \
        ); \
        auto* handle_ptr = new decltype(handle)(std::move(handle)); \
        return reinterpret_cast<uintptr_t>(handle_ptr); \
    } \
    void un##SUBSCRIBE_FN(INSTANCE_TYPE instance, uintptr_t handle_ptr) { \
        if (!handle_ptr) return; \
        using HandleType = decltype(std::declval<INSTANCE_TYPE>()->SUBSCRIBE_FN( \
            std::declval<std::function<bool(CALLBACK_PARAMETER_TYPE)>>())); \
        auto* handle = reinterpret_cast<HandleType*>(handle_ptr); \
        instance->un##SUBSCRIBE_FN(*handle); \
        delete handle; \
    }
#endif
