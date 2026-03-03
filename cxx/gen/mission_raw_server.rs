// Auto-generated wrapper. Do not edit.
use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct MissionItemOwned {
    pub seq: u32,
    pub frame: u32,
    pub command: u32,
    pub current: u32,
    pub autocontinue: u32,
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub param4: f32,
    pub x: i32,
    pub y: i32,
    pub z: f32,
    pub mission_type: u32,
}

/*  */
pub struct MissionItem<'a> {
    inner: &'a crate::mission_raw_server::mavsdk::MissionRawServer_MissionItem,
}

impl<'a> MissionItem<'a> {
    pub fn new(inner: &'a crate::mission_raw_server::mavsdk::MissionRawServer_MissionItem) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> MissionItemOwned {
        MissionItemOwned {
            seq: self.seq(),
            frame: self.frame(),
            command: self.command(),
            current: self.current(),
            autocontinue: self.autocontinue(),
            param1: self.param1(),
            param2: self.param2(),
            param3: self.param3(),
            param4: self.param4(),
            x: self.x(),
            y: self.y(),
            z: self.z(),
            mission_type: self.mission_type(),
        }
    }
    ///  Sequence (uint16_t)

    pub fn seq(&self) -> u32 {
        self.inner.mission_raw_server_get_seq()
    }
    ///  The coordinate system of the waypoint (actually uint8_t)

    pub fn frame(&self) -> u32 {
        self.inner.mission_raw_server_get_frame()
    }
    ///  The scheduled action for the waypoint (actually uint16_t)

    pub fn command(&self) -> u32 {
        self.inner.mission_raw_server_get_command()
    }
    ///  false:0, true:1 (actually uint8_t)

    pub fn current(&self) -> u32 {
        self.inner.mission_raw_server_get_current()
    }
    ///  Autocontinue to next waypoint (actually uint8_t)

    pub fn autocontinue(&self) -> u32 {
        self.inner.mission_raw_server_get_autocontinue()
    }
    ///  PARAM1, see MAV_CMD enum

    pub fn param1(&self) -> f32 {
        self.inner.mission_raw_server_get_param1()
    }
    ///  PARAM2, see MAV_CMD enum

    pub fn param2(&self) -> f32 {
        self.inner.mission_raw_server_get_param2()
    }
    ///  PARAM3, see MAV_CMD enum

    pub fn param3(&self) -> f32 {
        self.inner.mission_raw_server_get_param3()
    }
    ///  PARAM4, see MAV_CMD enum

    pub fn param4(&self) -> f32 {
        self.inner.mission_raw_server_get_param4()
    }
    ///  PARAM5 / local: x position in meters * 1e4, global: latitude in degrees * 10^7

    pub fn x(&self) -> i32 {
        self.inner.mission_raw_server_get_x()
    }
    ///  PARAM6 / y position: local: x position in meters * 1e4, global: longitude in degrees *10^7

    pub fn y(&self) -> i32 {
        self.inner.mission_raw_server_get_y()
    }
    ///  PARAM7 / local: Z coordinate, global: altitude (relative or absolute, depending on frame)

    pub fn z(&self) -> f32 {
        self.inner.mission_raw_server_get_z()
    }
    ///  Mission type (actually uint8_t)

    pub fn mission_type(&self) -> u32 {
        self.inner.mission_raw_server_get_mission_type()
    }
}

pub struct MissionPlanOwned {
    pub mission_items: std::vec::Vec<MissionItemOwned>,
}

/*  */
pub struct MissionPlan<'a> {
    inner: &'a crate::mission_raw_server::mavsdk::MissionRawServer_MissionPlan,
}

impl<'a> MissionPlan<'a> {
    pub fn new(inner: &'a crate::mission_raw_server::mavsdk::MissionRawServer_MissionPlan) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> MissionPlanOwned {
        MissionPlanOwned {
            mission_items: self.mission_items(),
        }
    }
    ///  The mission items

    pub fn mission_items(&self) -> std::vec::Vec<MissionItemOwned> {
        self.inner
            .mission_raw_server_get_mission_items()
            .iter()
            .map(|item| MissionItem::new(item).into_owned())
            .collect()
    }
}

pub struct MissionProgressOwned {
    pub current: i32,
    pub total: i32,
}

/*  */
pub struct MissionProgress<'a> {
    inner: &'a crate::mission_raw_server::mavsdk::MissionRawServer_MissionProgress,
}

impl<'a> MissionProgress<'a> {
    pub fn new(
        inner: &'a crate::mission_raw_server::mavsdk::MissionRawServer_MissionProgress,
    ) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> MissionProgressOwned {
        MissionProgressOwned {
            current: self.current(),
            total: self.total(),
        }
    }
    ///  Current mission item index (0-based), if equal to total, the mission is finished

    pub fn current(&self) -> i32 {
        self.inner.mission_raw_server_get_current()
    }
    ///  Total number of mission items

    pub fn total(&self) -> i32 {
        self.inner.mission_raw_server_get_total()
    }
}

struct MissionRawServerInner {
    plugin: cxx::UniquePtr<crate::mission_raw_server::mavsdk::MissionRawServer>,

    /// IncomingMission State
    incoming_mission_handle: Option<usize>,
    incoming_mission_user_data: *mut c_void,
    /// CurrentItemChanged State
    current_item_changed_handle: Option<usize>,
    current_item_changed_user_data: *mut c_void,
    /// ClearAll State
    clear_all_handle: Option<usize>,
    clear_all_user_data: *mut c_void,
}

pub struct MissionRawServerClient {
    inner: Mutex<MissionRawServerInner>,

    // Producer for the latest IncomingMission state; broadcast to all active subscribers.
    incoming_mission_tx: watch::Sender<std::option::Option<MissionPlanStatus>>,
    // Producer for the latest CurrentItemChanged state; broadcast to all active subscribers.
    current_item_changed_tx: watch::Sender<std::option::Option<MissionItemOwned>>,
    // Producer for the latest ClearAll state; broadcast to all active subscribers.
    clear_all_tx: watch::Sender<std::option::Option<u32>>,
}

impl MissionRawServerClient {
    pub fn new(
        plugin: cxx::UniquePtr<crate::mission_raw_server::mavsdk::MissionRawServer>,
    ) -> Self {
        let (incoming_mission_tx, _) = watch::channel(None);
        let (current_item_changed_tx, _) = watch::channel(None);
        let (clear_all_tx, _) = watch::channel(None);

        Self {
            inner: Mutex::new(MissionRawServerInner {
                plugin,

                incoming_mission_handle: None,
                incoming_mission_user_data: std::ptr::null_mut(),
                current_item_changed_handle: None,
                current_item_changed_user_data: std::ptr::null_mut(),
                clear_all_handle: None,
                clear_all_user_data: std::ptr::null_mut(),
            }),

            incoming_mission_tx,
            current_item_changed_tx,
            clear_all_tx,
        }
    }

    pub fn subscribe_incoming_mission(
        &self,
    ) -> watch::Receiver<std::option::Option<MissionPlanStatus>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.incoming_mission_handle.is_none() {
            // Lazy initialization of C++ 'incoming_mission' mission_raw_server stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.incoming_mission_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_incoming_mission(
                mission_raw_server_incoming_mission_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.incoming_mission_handle = Some(handle);
            inner.incoming_mission_user_data = user_data_ptr;
        }

        self.incoming_mission_tx.subscribe()
    }
    pub fn subscribe_current_item_changed(
        &self,
    ) -> watch::Receiver<std::option::Option<MissionItemOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.current_item_changed_handle.is_none() {
            // Lazy initialization of C++ 'current_item_changed' mission_raw_server stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.current_item_changed_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_current_item_changed(
                mission_raw_server_current_item_changed_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.current_item_changed_handle = Some(handle);
            inner.current_item_changed_user_data = user_data_ptr;
        }

        self.current_item_changed_tx.subscribe()
    }
    pub fn subscribe_clear_all(&self) -> watch::Receiver<std::option::Option<u32>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.clear_all_handle.is_none() {
            // Lazy initialization of C++ 'clear_all' mission_raw_server stream.
            let user_data_ptr = Box::into_raw(Box::new(self.clear_all_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_clear_all(
                mission_raw_server_clear_all_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.clear_all_handle = Some(handle);
            inner.clear_all_user_data = user_data_ptr;
        }

        self.clear_all_tx.subscribe()
    }
}

pub struct MissionPlanStatus {
    pub result: crate::mission_raw_server::mavsdk::MissionRawServer_Result,
    pub incoming_mission: MissionPlanOwned,
}
#[unsafe(no_mangle)]
pub extern "C" fn mission_raw_server_incoming_mission_callback_ffi(
    user_data: *mut c_void,
    result: crate::mission_raw_server::mavsdk::MissionRawServer_Result,
    incoming_mission: &crate::mission_raw_server::mavsdk::MissionRawServer_MissionPlan,
) {
    if user_data.is_null() {
        return;
    }
    let send_incoming_mission = MissionPlanStatus {
        result,
        incoming_mission: MissionPlan::new(incoming_mission).into_owned(),
    };

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<MissionPlanStatus>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_incoming_mission));
}
#[unsafe(no_mangle)]
pub extern "C" fn mission_raw_server_current_item_changed_callback_ffi(
    user_data: *mut c_void,
    current_item_changed: &crate::mission_raw_server::mavsdk::MissionRawServer_MissionItem,
) {
    if user_data.is_null() {
        return;
    }
    let send_current_item_changed = MissionItem::new(current_item_changed).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<MissionItemOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_current_item_changed));
}
#[unsafe(no_mangle)]
pub extern "C" fn mission_raw_server_clear_all_callback_ffi(
    user_data: *mut c_void,
    clear_all: u32,
) {
    if user_data.is_null() {
        return;
    }
    let send_clear_all = clear_all.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<u32>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_clear_all));
}
