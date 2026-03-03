// Auto-generated wrapper. Do not edit.
use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct IntParamOwned {
    pub name: String,
    pub value: i32,
}

/*  */
pub struct IntParam<'a> {
    inner: &'a crate::param_server::mavsdk::ParamServer_IntParam,
}

impl<'a> IntParam<'a> {
    pub fn new(inner: &'a crate::param_server::mavsdk::ParamServer_IntParam) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> IntParamOwned {
        IntParamOwned {
            name: self.name(),
            value: self.value(),
        }
    }
    ///  Name of the parameter

    pub fn name(&self) -> String {
        self.inner
            .param_server_get_name()
            .to_string_lossy()
            .into_owned()
    }
    ///  Value of the parameter

    pub fn value(&self) -> i32 {
        self.inner.param_server_get_value()
    }
}

pub struct FloatParamOwned {
    pub name: String,
    pub value: f32,
}

/*  */
pub struct FloatParam<'a> {
    inner: &'a crate::param_server::mavsdk::ParamServer_FloatParam,
}

impl<'a> FloatParam<'a> {
    pub fn new(inner: &'a crate::param_server::mavsdk::ParamServer_FloatParam) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> FloatParamOwned {
        FloatParamOwned {
            name: self.name(),
            value: self.value(),
        }
    }
    ///  Name of the parameter

    pub fn name(&self) -> String {
        self.inner
            .param_server_get_name()
            .to_string_lossy()
            .into_owned()
    }
    ///  Value of the parameter

    pub fn value(&self) -> f32 {
        self.inner.param_server_get_value()
    }
}

pub struct CustomParamOwned {
    pub name: String,
    pub value: String,
}

/*  */
pub struct CustomParam<'a> {
    inner: &'a crate::param_server::mavsdk::ParamServer_CustomParam,
}

impl<'a> CustomParam<'a> {
    pub fn new(inner: &'a crate::param_server::mavsdk::ParamServer_CustomParam) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> CustomParamOwned {
        CustomParamOwned {
            name: self.name(),
            value: self.value(),
        }
    }
    ///  Name of the parameter

    pub fn name(&self) -> String {
        self.inner
            .param_server_get_name()
            .to_string_lossy()
            .into_owned()
    }
    ///  Value of the parameter

    pub fn value(&self) -> String {
        self.inner
            .param_server_get_value()
            .to_string_lossy()
            .into_owned()
    }
}

pub struct AllParamsOwned {
    pub int_params: std::vec::Vec<IntParamOwned>,
    pub float_params: std::vec::Vec<FloatParamOwned>,
    pub custom_params: std::vec::Vec<CustomParamOwned>,
}

/*  */
pub struct AllParams<'a> {
    inner: &'a crate::param_server::mavsdk::ParamServer_AllParams,
}

impl<'a> AllParams<'a> {
    pub fn new(inner: &'a crate::param_server::mavsdk::ParamServer_AllParams) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> AllParamsOwned {
        AllParamsOwned {
            int_params: self.int_params(),
            float_params: self.float_params(),
            custom_params: self.custom_params(),
        }
    }
    ///  Collection of all parameter names and values of type int

    pub fn int_params(&self) -> std::vec::Vec<IntParamOwned> {
        self.inner
            .param_server_get_int_params()
            .iter()
            .map(|item| IntParam::new(item).into_owned())
            .collect()
    }
    ///  Collection of all parameter names and values of type float

    pub fn float_params(&self) -> std::vec::Vec<FloatParamOwned> {
        self.inner
            .param_server_get_float_params()
            .iter()
            .map(|item| FloatParam::new(item).into_owned())
            .collect()
    }
    ///  Collection of all parameter names and values of type custom

    pub fn custom_params(&self) -> std::vec::Vec<CustomParamOwned> {
        self.inner
            .param_server_get_custom_params()
            .iter()
            .map(|item| CustomParam::new(item).into_owned())
            .collect()
    }
}

struct ParamServerInner {
    plugin: cxx::UniquePtr<crate::param_server::mavsdk::ParamServer>,

    /// ChangedParamInt State
    changed_param_int_handle: Option<usize>,
    changed_param_int_user_data: *mut c_void,
    /// ChangedParamFloat State
    changed_param_float_handle: Option<usize>,
    changed_param_float_user_data: *mut c_void,
    /// ChangedParamCustom State
    changed_param_custom_handle: Option<usize>,
    changed_param_custom_user_data: *mut c_void,
}

pub struct ParamServerClient {
    inner: Mutex<ParamServerInner>,

    // Producer for the latest ChangedParamInt state; broadcast to all active subscribers.
    changed_param_int_tx: watch::Sender<std::option::Option<IntParamOwned>>,
    // Producer for the latest ChangedParamFloat state; broadcast to all active subscribers.
    changed_param_float_tx: watch::Sender<std::option::Option<FloatParamOwned>>,
    // Producer for the latest ChangedParamCustom state; broadcast to all active subscribers.
    changed_param_custom_tx: watch::Sender<std::option::Option<CustomParamOwned>>,
}

impl ParamServerClient {
    pub fn new(plugin: cxx::UniquePtr<crate::param_server::mavsdk::ParamServer>) -> Self {
        let (changed_param_int_tx, _) = watch::channel(None);
        let (changed_param_float_tx, _) = watch::channel(None);
        let (changed_param_custom_tx, _) = watch::channel(None);

        Self {
            inner: Mutex::new(ParamServerInner {
                plugin,

                changed_param_int_handle: None,
                changed_param_int_user_data: std::ptr::null_mut(),
                changed_param_float_handle: None,
                changed_param_float_user_data: std::ptr::null_mut(),
                changed_param_custom_handle: None,
                changed_param_custom_user_data: std::ptr::null_mut(),
            }),

            changed_param_int_tx,
            changed_param_float_tx,
            changed_param_custom_tx,
        }
    }

    pub fn subscribe_changed_param_int(
        &self,
    ) -> watch::Receiver<std::option::Option<IntParamOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.changed_param_int_handle.is_none() {
            // Lazy initialization of C++ 'changed_param_int' param_server stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.changed_param_int_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_changed_param_int(
                param_server_changed_param_int_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.changed_param_int_handle = Some(handle);
            inner.changed_param_int_user_data = user_data_ptr;
        }

        self.changed_param_int_tx.subscribe()
    }
    pub fn subscribe_changed_param_float(
        &self,
    ) -> watch::Receiver<std::option::Option<FloatParamOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.changed_param_float_handle.is_none() {
            // Lazy initialization of C++ 'changed_param_float' param_server stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.changed_param_float_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_changed_param_float(
                param_server_changed_param_float_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.changed_param_float_handle = Some(handle);
            inner.changed_param_float_user_data = user_data_ptr;
        }

        self.changed_param_float_tx.subscribe()
    }
    pub fn subscribe_changed_param_custom(
        &self,
    ) -> watch::Receiver<std::option::Option<CustomParamOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.changed_param_custom_handle.is_none() {
            // Lazy initialization of C++ 'changed_param_custom' param_server stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.changed_param_custom_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_changed_param_custom(
                param_server_changed_param_custom_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.changed_param_custom_handle = Some(handle);
            inner.changed_param_custom_user_data = user_data_ptr;
        }

        self.changed_param_custom_tx.subscribe()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn param_server_changed_param_int_callback_ffi(
    user_data: *mut c_void,
    changed_param_int: &crate::param_server::mavsdk::ParamServer_IntParam,
) {
    if user_data.is_null() {
        return;
    }
    let send_changed_param_int = IntParam::new(changed_param_int).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<IntParamOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_changed_param_int));
}
#[unsafe(no_mangle)]
pub extern "C" fn param_server_changed_param_float_callback_ffi(
    user_data: *mut c_void,
    changed_param_float: &crate::param_server::mavsdk::ParamServer_FloatParam,
) {
    if user_data.is_null() {
        return;
    }
    let send_changed_param_float = FloatParam::new(changed_param_float).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<FloatParamOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_changed_param_float));
}
#[unsafe(no_mangle)]
pub extern "C" fn param_server_changed_param_custom_callback_ffi(
    user_data: *mut c_void,
    changed_param_custom: &crate::param_server::mavsdk::ParamServer_CustomParam,
) {
    if user_data.is_null() {
        return;
    }
    let send_changed_param_custom = CustomParam::new(changed_param_custom).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<CustomParamOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_changed_param_custom));
}
