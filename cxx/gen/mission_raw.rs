// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct MissionProgressOwned {
    pub current: i32,
    pub total: i32,
}

/*  */
pub struct MissionProgress<'a> {
    inner: &'a crate::mission_raw::mavsdk::MissionRaw_MissionProgress,
}

impl<'a> MissionProgress<'a> {
    pub fn new(inner: &'a crate::mission_raw::mavsdk::MissionRaw_MissionProgress) -> Self {
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
        self.inner.mission_raw_get_current()
    }
    ///  Total number of mission items

    pub fn total(&self) -> i32 {
        self.inner.mission_raw_get_total()
    }
}

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
    inner: &'a crate::mission_raw::mavsdk::MissionRaw_MissionItem,
}

impl<'a> MissionItem<'a> {
    pub fn new(inner: &'a crate::mission_raw::mavsdk::MissionRaw_MissionItem) -> Self {
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
        self.inner.mission_raw_get_seq()
    }
    ///  The coordinate system of the waypoint (actually uint8_t)

    pub fn frame(&self) -> u32 {
        self.inner.mission_raw_get_frame()
    }
    ///  The scheduled action for the waypoint (actually uint16_t)

    pub fn command(&self) -> u32 {
        self.inner.mission_raw_get_command()
    }
    ///  false:0, true:1 (actually uint8_t)

    pub fn current(&self) -> u32 {
        self.inner.mission_raw_get_current()
    }
    ///  Autocontinue to next waypoint (actually uint8_t)

    pub fn autocontinue(&self) -> u32 {
        self.inner.mission_raw_get_autocontinue()
    }
    ///  PARAM1, see MAV_CMD enum

    pub fn param1(&self) -> f32 {
        self.inner.mission_raw_get_param1()
    }
    ///  PARAM2, see MAV_CMD enum

    pub fn param2(&self) -> f32 {
        self.inner.mission_raw_get_param2()
    }
    ///  PARAM3, see MAV_CMD enum

    pub fn param3(&self) -> f32 {
        self.inner.mission_raw_get_param3()
    }
    ///  PARAM4, see MAV_CMD enum

    pub fn param4(&self) -> f32 {
        self.inner.mission_raw_get_param4()
    }
    ///  PARAM5 / local: x position in meters * 1e4, global: latitude in degrees * 10^7

    pub fn x(&self) -> i32 {
        self.inner.mission_raw_get_x()
    }
    ///  PARAM6 / y position: local: x position in meters * 1e4, global: longitude in degrees *10^7

    pub fn y(&self) -> i32 {
        self.inner.mission_raw_get_y()
    }
    ///  PARAM7 / local: Z coordinate, global: altitude (relative or absolute, depending on frame)

    pub fn z(&self) -> f32 {
        self.inner.mission_raw_get_z()
    }
    ///  Mission type (actually uint8_t)

    pub fn mission_type(&self) -> u32 {
        self.inner.mission_raw_get_mission_type()
    }
}

pub struct MissionImportDataOwned {
    pub mission_items: std::vec::Vec<MissionItemOwned>,
    pub geofence_items: std::vec::Vec<MissionItemOwned>,
    pub rally_items: std::vec::Vec<MissionItemOwned>,
}

/*  */
pub struct MissionImportData<'a> {
    inner: &'a crate::mission_raw::mavsdk::MissionRaw_MissionImportData,
}

impl<'a> MissionImportData<'a> {
    pub fn new(inner: &'a crate::mission_raw::mavsdk::MissionRaw_MissionImportData) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> MissionImportDataOwned {
        MissionImportDataOwned {
            mission_items: self.mission_items(),
            geofence_items: self.geofence_items(),
            rally_items: self.rally_items(),
        }
    }
    ///  Mission items

    pub fn mission_items(&self) -> std::vec::Vec<MissionItemOwned> {
        self.inner
            .mission_raw_get_mission_items()
            .iter()
            .map(|item| MissionItem::new(item).into_owned())
            .collect()
    }
    ///  Geofence items

    pub fn geofence_items(&self) -> std::vec::Vec<MissionItemOwned> {
        self.inner
            .mission_raw_get_geofence_items()
            .iter()
            .map(|item| MissionItem::new(item).into_owned())
            .collect()
    }
    ///  Rally items

    pub fn rally_items(&self) -> std::vec::Vec<MissionItemOwned> {
        self.inner
            .mission_raw_get_rally_items()
            .iter()
            .map(|item| MissionItem::new(item).into_owned())
            .collect()
    }
}

struct MissionRawInner {
    plugin: cxx::UniquePtr<crate::mission_raw::mavsdk::MissionRaw>,

    /// MissionProgress State
    mission_progress_handle: Option<usize>,
    mission_progress_user_data: *mut c_void,
    /// MissionChanged State
    mission_changed_handle: Option<usize>,
    mission_changed_user_data: *mut c_void,
}

pub struct MissionRawClient {
    inner: Mutex<MissionRawInner>,

    // Producer for the latest MissionProgress state; broadcast to all active subscribers.
    mission_progress_tx: watch::Sender<std::option::Option<MissionProgressOwned>>,
    // Producer for the latest MissionChanged state; broadcast to all active subscribers.
    mission_changed_tx: watch::Sender<std::option::Option<bool>>,
}

impl MissionRawClient {
    pub fn new(plugin: cxx::UniquePtr<crate::mission_raw::mavsdk::MissionRaw>) -> Self {
        let (mission_progress_tx, _) = watch::channel(None);
        let (mission_changed_tx, _) = watch::channel(None);

        Self {
            inner: Mutex::new(MissionRawInner {
                plugin,

                mission_progress_handle: None,
                mission_progress_user_data: std::ptr::null_mut(),
                mission_changed_handle: None,
                mission_changed_user_data: std::ptr::null_mut(),
            }),

            mission_progress_tx,
            mission_changed_tx,
        }
    }

    pub fn subscribe_mission_progress(
        &self,
    ) -> watch::Receiver<std::option::Option<MissionProgressOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.mission_progress_handle.is_none() {
            // Lazy initialization of C++ 'mission_progress' mission_raw stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.mission_progress_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_mission_progress(
                mission_raw_mission_progress_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.mission_progress_handle = Some(handle);
            inner.mission_progress_user_data = user_data_ptr;
        }

        self.mission_progress_tx.subscribe()
    }
    pub fn subscribe_mission_changed(&self) -> watch::Receiver<std::option::Option<bool>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.mission_changed_handle.is_none() {
            // Lazy initialization of C++ 'mission_changed' mission_raw stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.mission_changed_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_mission_changed(
                mission_raw_mission_changed_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.mission_changed_handle = Some(handle);
            inner.mission_changed_user_data = user_data_ptr;
        }

        self.mission_changed_tx.subscribe()
    }
}

impl Drop for MissionRawClient {
    fn drop(&mut self) {
        if let Ok(mut inner) = self.inner.lock() {
            if let Some(handle) = inner.mission_progress_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_mission_progress(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.mission_progress_user_data
                            as *mut watch::Sender<std::option::Option<MissionProgressOwned>>,
                    );
                }
            }
            if let Some(handle) = inner.mission_changed_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_mission_changed(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.mission_changed_user_data
                            as *mut watch::Sender<std::option::Option<bool>>,
                    );
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn mission_raw_mission_progress_callback_ffi(
    user_data: *mut c_void,
    mission_progress: &crate::mission_raw::mavsdk::MissionRaw_MissionProgress,
) {
    if user_data.is_null() {
        return;
    }
    let send_mission_progress = MissionProgress::new(mission_progress).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<MissionProgressOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_mission_progress));
}
#[unsafe(no_mangle)]
pub extern "C" fn mission_raw_mission_changed_callback_ffi(
    user_data: *mut c_void,
    mission_changed: bool,
) {
    if user_data.is_null() {
        return;
    }
    let send_mission_changed = mission_changed.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<bool>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_mission_changed));
}
