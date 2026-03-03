// Auto-generated wrapper. Do not edit.
use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct AllowableFlightModesOwned {
    pub can_auto_mode: bool,
    pub can_guided_mode: bool,
    pub can_stabilize_mode: bool,
}

/*  */
pub struct AllowableFlightModes<'a> {
    inner: &'a crate::action_server::mavsdk::ActionServer_AllowableFlightModes,
}

impl<'a> AllowableFlightModes<'a> {
    pub fn new(inner: &'a crate::action_server::mavsdk::ActionServer_AllowableFlightModes) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> AllowableFlightModesOwned {
        AllowableFlightModesOwned {
            can_auto_mode: self.can_auto_mode(),
            can_guided_mode: self.can_guided_mode(),
            can_stabilize_mode: self.can_stabilize_mode(),
        }
    }
    ///  Auto/mission mode

    pub fn can_auto_mode(&self) -> bool {
        self.inner.action_server_get_can_auto_mode()
    }
    ///  Guided mode

    pub fn can_guided_mode(&self) -> bool {
        self.inner.action_server_get_can_guided_mode()
    }
    ///  Stabilize mode

    pub fn can_stabilize_mode(&self) -> bool {
        self.inner.action_server_get_can_stabilize_mode()
    }
}

pub struct ArmDisarmOwned {
    pub arm: bool,
    pub force: bool,
}

/*  */
pub struct ArmDisarm<'a> {
    inner: &'a crate::action_server::mavsdk::ActionServer_ArmDisarm,
}

impl<'a> ArmDisarm<'a> {
    pub fn new(inner: &'a crate::action_server::mavsdk::ActionServer_ArmDisarm) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> ArmDisarmOwned {
        ArmDisarmOwned {
            arm: self.arm(),
            force: self.force(),
        }
    }
    ///  Should vehicle arm

    pub fn arm(&self) -> bool {
        self.inner.action_server_get_arm()
    }
    ///  Should arm override pre-flight checks

    pub fn force(&self) -> bool {
        self.inner.action_server_get_force()
    }
}

struct ActionServerInner {
    plugin: cxx::UniquePtr<crate::action_server::mavsdk::ActionServer>,

    /// ArmDisarm State
    arm_disarm_handle: Option<usize>,
    arm_disarm_user_data: *mut c_void,
    /// FlightModeChange State
    flight_mode_change_handle: Option<usize>,
    flight_mode_change_user_data: *mut c_void,
    /// Takeoff State
    takeoff_handle: Option<usize>,
    takeoff_user_data: *mut c_void,
    /// Land State
    land_handle: Option<usize>,
    land_user_data: *mut c_void,
    /// Reboot State
    reboot_handle: Option<usize>,
    reboot_user_data: *mut c_void,
    /// Shutdown State
    shutdown_handle: Option<usize>,
    shutdown_user_data: *mut c_void,
    /// Terminate State
    terminate_handle: Option<usize>,
    terminate_user_data: *mut c_void,
}

pub struct ActionServerClient {
    inner: Mutex<ActionServerInner>,

    // Producer for the latest ArmDisarm state; broadcast to all active subscribers.
    arm_disarm_tx: watch::Sender<std::option::Option<ArmDisarmStatus>>,
    // Producer for the latest FlightModeChange state; broadcast to all active subscribers.
    flight_mode_change_tx: watch::Sender<std::option::Option<FlightModeStatus>>,
    // Producer for the latest Takeoff state; broadcast to all active subscribers.
    takeoff_tx: watch::Sender<std::option::Option<TakeoffStatus>>,
    // Producer for the latest Land state; broadcast to all active subscribers.
    land_tx: watch::Sender<std::option::Option<LandStatus>>,
    // Producer for the latest Reboot state; broadcast to all active subscribers.
    reboot_tx: watch::Sender<std::option::Option<RebootStatus>>,
    // Producer for the latest Shutdown state; broadcast to all active subscribers.
    shutdown_tx: watch::Sender<std::option::Option<ShutdownStatus>>,
    // Producer for the latest Terminate state; broadcast to all active subscribers.
    terminate_tx: watch::Sender<std::option::Option<TerminateStatus>>,
}

impl ActionServerClient {
    pub fn new(plugin: cxx::UniquePtr<crate::action_server::mavsdk::ActionServer>) -> Self {
        let (arm_disarm_tx, _) = watch::channel(None);
        let (flight_mode_change_tx, _) = watch::channel(None);
        let (takeoff_tx, _) = watch::channel(None);
        let (land_tx, _) = watch::channel(None);
        let (reboot_tx, _) = watch::channel(None);
        let (shutdown_tx, _) = watch::channel(None);
        let (terminate_tx, _) = watch::channel(None);

        Self {
            inner: Mutex::new(ActionServerInner {
                plugin,

                arm_disarm_handle: None,
                arm_disarm_user_data: std::ptr::null_mut(),
                flight_mode_change_handle: None,
                flight_mode_change_user_data: std::ptr::null_mut(),
                takeoff_handle: None,
                takeoff_user_data: std::ptr::null_mut(),
                land_handle: None,
                land_user_data: std::ptr::null_mut(),
                reboot_handle: None,
                reboot_user_data: std::ptr::null_mut(),
                shutdown_handle: None,
                shutdown_user_data: std::ptr::null_mut(),
                terminate_handle: None,
                terminate_user_data: std::ptr::null_mut(),
            }),

            arm_disarm_tx,
            flight_mode_change_tx,
            takeoff_tx,
            land_tx,
            reboot_tx,
            shutdown_tx,
            terminate_tx,
        }
    }

    pub fn subscribe_arm_disarm(&self) -> watch::Receiver<std::option::Option<ArmDisarmStatus>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.arm_disarm_handle.is_none() {
            // Lazy initialization of C++ 'arm_disarm' action_server stream.
            let user_data_ptr = Box::into_raw(Box::new(self.arm_disarm_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_arm_disarm(
                action_server_arm_disarm_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.arm_disarm_handle = Some(handle);
            inner.arm_disarm_user_data = user_data_ptr;
        }

        self.arm_disarm_tx.subscribe()
    }
    pub fn subscribe_flight_mode_change(
        &self,
    ) -> watch::Receiver<std::option::Option<FlightModeStatus>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.flight_mode_change_handle.is_none() {
            // Lazy initialization of C++ 'flight_mode_change' action_server stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.flight_mode_change_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_flight_mode_change(
                action_server_flight_mode_change_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.flight_mode_change_handle = Some(handle);
            inner.flight_mode_change_user_data = user_data_ptr;
        }

        self.flight_mode_change_tx.subscribe()
    }
    pub fn subscribe_takeoff(&self) -> watch::Receiver<std::option::Option<TakeoffStatus>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.takeoff_handle.is_none() {
            // Lazy initialization of C++ 'takeoff' action_server stream.
            let user_data_ptr = Box::into_raw(Box::new(self.takeoff_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_takeoff(
                action_server_takeoff_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.takeoff_handle = Some(handle);
            inner.takeoff_user_data = user_data_ptr;
        }

        self.takeoff_tx.subscribe()
    }
    pub fn subscribe_land(&self) -> watch::Receiver<std::option::Option<LandStatus>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.land_handle.is_none() {
            // Lazy initialization of C++ 'land' action_server stream.
            let user_data_ptr = Box::into_raw(Box::new(self.land_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_land(
                action_server_land_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.land_handle = Some(handle);
            inner.land_user_data = user_data_ptr;
        }

        self.land_tx.subscribe()
    }
    pub fn subscribe_reboot(&self) -> watch::Receiver<std::option::Option<RebootStatus>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.reboot_handle.is_none() {
            // Lazy initialization of C++ 'reboot' action_server stream.
            let user_data_ptr = Box::into_raw(Box::new(self.reboot_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_reboot(
                action_server_reboot_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.reboot_handle = Some(handle);
            inner.reboot_user_data = user_data_ptr;
        }

        self.reboot_tx.subscribe()
    }
    pub fn subscribe_shutdown(&self) -> watch::Receiver<std::option::Option<ShutdownStatus>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.shutdown_handle.is_none() {
            // Lazy initialization of C++ 'shutdown' action_server stream.
            let user_data_ptr = Box::into_raw(Box::new(self.shutdown_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_shutdown(
                action_server_shutdown_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.shutdown_handle = Some(handle);
            inner.shutdown_user_data = user_data_ptr;
        }

        self.shutdown_tx.subscribe()
    }
    pub fn subscribe_terminate(&self) -> watch::Receiver<std::option::Option<TerminateStatus>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.terminate_handle.is_none() {
            // Lazy initialization of C++ 'terminate' action_server stream.
            let user_data_ptr = Box::into_raw(Box::new(self.terminate_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_terminate(
                action_server_terminate_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.terminate_handle = Some(handle);
            inner.terminate_user_data = user_data_ptr;
        }

        self.terminate_tx.subscribe()
    }
}

pub struct ArmDisarmStatus {
    pub result: crate::action_server::mavsdk::ActionServer_Result,
    pub arm_disarm: ArmDisarmOwned,
}
#[unsafe(no_mangle)]
pub extern "C" fn action_server_arm_disarm_callback_ffi(
    user_data: *mut c_void,
    result: crate::action_server::mavsdk::ActionServer_Result,
    arm_disarm: &crate::action_server::mavsdk::ActionServer_ArmDisarm,
) {
    if user_data.is_null() {
        return;
    }
    let send_arm_disarm = ArmDisarmStatus {
        result,
        arm_disarm: ArmDisarm::new(arm_disarm).into_owned(),
    };

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<ArmDisarmStatus>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_arm_disarm));
}
pub struct FlightModeStatus {
    pub result: crate::action_server::mavsdk::ActionServer_Result,
    pub flight_mode_change: crate::action_server::mavsdk::ActionServer_FlightMode,
}
#[unsafe(no_mangle)]
pub extern "C" fn action_server_flight_mode_change_callback_ffi(
    user_data: *mut c_void,
    result: crate::action_server::mavsdk::ActionServer_Result,
    flight_mode_change: &crate::action_server::mavsdk::ActionServer_FlightMode,
) {
    if user_data.is_null() {
        return;
    }
    let send_flight_mode_change = FlightModeStatus {
        result,
        flight_mode_change: flight_mode_change.clone(),
    };

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<FlightModeStatus>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_flight_mode_change));
}
pub struct TakeoffStatus {
    pub result: crate::action_server::mavsdk::ActionServer_Result,
    pub takeoff: bool,
}
#[unsafe(no_mangle)]
pub extern "C" fn action_server_takeoff_callback_ffi(
    user_data: *mut c_void,
    result: crate::action_server::mavsdk::ActionServer_Result,
    takeoff: bool,
) {
    if user_data.is_null() {
        return;
    }
    let send_takeoff = TakeoffStatus {
        result,
        takeoff: takeoff.clone(),
    };

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<TakeoffStatus>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_takeoff));
}
pub struct LandStatus {
    pub result: crate::action_server::mavsdk::ActionServer_Result,
    pub land: bool,
}
#[unsafe(no_mangle)]
pub extern "C" fn action_server_land_callback_ffi(
    user_data: *mut c_void,
    result: crate::action_server::mavsdk::ActionServer_Result,
    land: bool,
) {
    if user_data.is_null() {
        return;
    }
    let send_land = LandStatus {
        result,
        land: land.clone(),
    };

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<LandStatus>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_land));
}
pub struct RebootStatus {
    pub result: crate::action_server::mavsdk::ActionServer_Result,
    pub reboot: bool,
}
#[unsafe(no_mangle)]
pub extern "C" fn action_server_reboot_callback_ffi(
    user_data: *mut c_void,
    result: crate::action_server::mavsdk::ActionServer_Result,
    reboot: bool,
) {
    if user_data.is_null() {
        return;
    }
    let send_reboot = RebootStatus {
        result,
        reboot: reboot.clone(),
    };

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<RebootStatus>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_reboot));
}
pub struct ShutdownStatus {
    pub result: crate::action_server::mavsdk::ActionServer_Result,
    pub shutdown: bool,
}
#[unsafe(no_mangle)]
pub extern "C" fn action_server_shutdown_callback_ffi(
    user_data: *mut c_void,
    result: crate::action_server::mavsdk::ActionServer_Result,
    shutdown: bool,
) {
    if user_data.is_null() {
        return;
    }
    let send_shutdown = ShutdownStatus {
        result,
        shutdown: shutdown.clone(),
    };

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<ShutdownStatus>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_shutdown));
}
pub struct TerminateStatus {
    pub result: crate::action_server::mavsdk::ActionServer_Result,
    pub terminate: bool,
}
#[unsafe(no_mangle)]
pub extern "C" fn action_server_terminate_callback_ffi(
    user_data: *mut c_void,
    result: crate::action_server::mavsdk::ActionServer_Result,
    terminate: bool,
) {
    if user_data.is_null() {
        return;
    }
    let send_terminate = TerminateStatus {
        result,
        terminate: terminate.clone(),
    };

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<TerminateStatus>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_terminate));
}
