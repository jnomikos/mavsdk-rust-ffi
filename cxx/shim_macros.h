#ifndef SHIM_MACRO_H_INC
#define SHIM_MACRO_H_INC

/**
 * @brief Declares a subscribe function in an autocxx friendly way. 
 *
 * Autocxx has issues with functions that take in std::function or function
 * objects. So, we need to create shims for each subscribe and unsubscribe
 * function. We take advantage of the fact that uintptr_t can be used to pass 
 * function pointers and opaque handles across the FFI boundary.
 * 
 * @param INSTANCE_TYPE The type of the instance on which the subscribe function is called. (i.e mavsdk::Mavsdk*)
 * @param RETURN_TYPE The return type of the callback function. (i.e void, bool, etc)
 * @param SUBSCRIBE_FN The name of the subscribe function. (i.e subscribe_connection_errors)
 * @param ... The argument types of the callback function. (i.e mavsdk::Mavsdk::ConnectionError)
 */
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
            return; \
        } \
        using HandleType = decltype(std::declval<INSTANCE_TYPE>()->SUBSCRIBE_FN( \
            std::declval<std::function<RETURN_TYPE(__VA_ARGS__)>>() \
        )); \
        auto* handle = reinterpret_cast<HandleType*>(handle_ptr); \
        instance->un##SUBSCRIBE_FN(*handle); \
        delete handle; \
    }

// In almost all cases, subscribe functions do not return anything
#define DECLARE_SUBSCRIBE_SHIM(INSTANCE_TYPE, SUBSCRIBE_FN, ...) \
    DECLARE_SUBSCRIBE_SHIM_RET(INSTANCE_TYPE, void, SUBSCRIBE_FN, __VA_ARGS__)

#endif
