// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use tokio::sync::watch;
use std::sync::Mutex;






pub struct MissionItemOwned {
        pub latitude_deg: f64,
        pub longitude_deg: f64,
        pub relative_altitude_m: f32,
        pub speed_m_s: f32,
        pub is_fly_through: bool,
        pub gimbal_pitch_deg: f32,
        pub gimbal_yaw_deg: f32,
        pub camera_action: crate::mission::mavsdk::Mission_MissionItem_CameraAction,
        pub loiter_time_s: f32,
        pub camera_photo_interval_s: f64,
        pub acceptance_radius_m: f32,
        pub yaw_deg: f32,
        pub camera_photo_distance_m: f32,
        pub vehicle_action: crate::mission::mavsdk::Mission_MissionItem_VehicleAction,
}

/*  */
pub struct MissionItem<'a> {
    inner: &'a crate::mission::mavsdk::Mission_MissionItem,
}

impl<'a> MissionItem<'a> {
    pub fn new(inner: &'a crate::mission::mavsdk::Mission_MissionItem) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> MissionItemOwned {
        MissionItemOwned {
            
            latitude_deg: self.latitude_deg(),
            longitude_deg: self.longitude_deg(),
            relative_altitude_m: self.relative_altitude_m(),
            speed_m_s: self.speed_m_s(),
            is_fly_through: self.is_fly_through(),
            gimbal_pitch_deg: self.gimbal_pitch_deg(),
            gimbal_yaw_deg: self.gimbal_yaw_deg(),
            camera_action: self.camera_action(),
            loiter_time_s: self.loiter_time_s(),
            camera_photo_interval_s: self.camera_photo_interval_s(),
            acceptance_radius_m: self.acceptance_radius_m(),
            yaw_deg: self.yaw_deg(),
            camera_photo_distance_m: self.camera_photo_distance_m(),
            vehicle_action: self.vehicle_action(),
        }
    }
        ///  Latitude in degrees (range: -90 to +90)

    pub fn latitude_deg(&self) -> f64 {
        self.inner.mission_get_latitude_deg()
    }
        ///  Longitude in degrees (range: -180 to +180)

    pub fn longitude_deg(&self) -> f64 {
        self.inner.mission_get_longitude_deg()
    }
        ///  Altitude relative to takeoff altitude in metres

    pub fn relative_altitude_m(&self) -> f32 {
        self.inner.mission_get_relative_altitude_m()
    }
        ///  Speed to use after this mission item (in metres/second)

    pub fn speed_m_s(&self) -> f32 {
        self.inner.mission_get_speed_m_s()
    }
        ///  True will make the drone fly through without stopping, while false will make the drone stop on the waypoint

    pub fn is_fly_through(&self) -> bool {
        self.inner.mission_get_is_fly_through()
    }
        ///  Gimbal pitch (in degrees)

    pub fn gimbal_pitch_deg(&self) -> f32 {
        self.inner.mission_get_gimbal_pitch_deg()
    }
        ///  Gimbal yaw (in degrees)

    pub fn gimbal_yaw_deg(&self) -> f32 {
        self.inner.mission_get_gimbal_yaw_deg()
    }
        ///  Camera action to trigger at this mission item

    pub fn camera_action(&self) -> crate::mission::mavsdk::Mission_MissionItem_CameraAction {
        self.inner.mission_get_camera_action().clone()
    }
        ///  Loiter time (in seconds)

    pub fn loiter_time_s(&self) -> f32 {
        self.inner.mission_get_loiter_time_s()
    }
        ///  Camera photo interval to use after this mission item (in seconds)

    pub fn camera_photo_interval_s(&self) -> f64 {
        self.inner.mission_get_camera_photo_interval_s()
    }
        ///  Radius for completing a mission item (in metres)

    pub fn acceptance_radius_m(&self) -> f32 {
        self.inner.mission_get_acceptance_radius_m()
    }
        ///  Absolute yaw angle (in degrees)

    pub fn yaw_deg(&self) -> f32 {
        self.inner.mission_get_yaw_deg()
    }
        ///  Camera photo distance to use after this mission item (in meters)

    pub fn camera_photo_distance_m(&self) -> f32 {
        self.inner.mission_get_camera_photo_distance_m()
    }
        ///  Vehicle action to trigger at this mission item.

    pub fn vehicle_action(&self) -> crate::mission::mavsdk::Mission_MissionItem_VehicleAction {
        self.inner.mission_get_vehicle_action().clone()
    }
}


pub struct MissionPlanOwned {
        pub mission_items: 
            std::vec::Vec<MissionItemOwned>,
}

/*  */
pub struct MissionPlan<'a> {
    inner: &'a crate::mission::mavsdk::Mission_MissionPlan,
}

impl<'a> MissionPlan<'a> {
    pub fn new(inner: &'a crate::mission::mavsdk::Mission_MissionPlan) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> MissionPlanOwned {
        MissionPlanOwned {
            
            mission_items: self.mission_items(),
        }
    }
        ///  The mission items

    pub fn mission_items(&self) -> 
            std::vec::Vec<MissionItemOwned> {
            self.inner.mission_get_mission_items()
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
    inner: &'a crate::mission::mavsdk::Mission_MissionProgress,
}

impl<'a> MissionProgress<'a> {
    pub fn new(inner: &'a crate::mission::mavsdk::Mission_MissionProgress) -> Self {
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
        self.inner.mission_get_current()
    }
        ///  Total number of mission items

    pub fn total(&self) -> i32 {
        self.inner.mission_get_total()
    }
}


pub struct ProgressDataOwned {
        pub progress: f32,
}

/*  */
pub struct ProgressData<'a> {
    inner: &'a crate::mission::mavsdk::Mission_ProgressData,
}

impl<'a> ProgressData<'a> {
    pub fn new(inner: &'a crate::mission::mavsdk::Mission_ProgressData) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> ProgressDataOwned {
        ProgressDataOwned {
            
            progress: self.progress(),
        }
    }
        ///  Progress (0..1.0)

    pub fn progress(&self) -> f32 {
        self.inner.mission_get_progress()
    }
}


pub struct ProgressDataOrMissionOwned {
        pub has_progress: bool,
        pub progress: f32,
        pub has_mission: bool,
        pub mission_plan: MissionPlanOwned,
}

/*  */
pub struct ProgressDataOrMission<'a> {
    inner: &'a crate::mission::mavsdk::Mission_ProgressDataOrMission,
}

impl<'a> ProgressDataOrMission<'a> {
    pub fn new(inner: &'a crate::mission::mavsdk::Mission_ProgressDataOrMission) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> ProgressDataOrMissionOwned {
        ProgressDataOrMissionOwned {
            
            has_progress: self.has_progress(),
            progress: self.progress(),
            has_mission: self.has_mission(),
            mission_plan: self.mission_plan(),
        }
    }
        ///  Whether this ProgressData contains a 'progress' status or not

    pub fn has_progress(&self) -> bool {
        self.inner.mission_get_has_progress()
    }
        ///  Progress (0..1.0)

    pub fn progress(&self) -> f32 {
        self.inner.mission_get_progress()
    }
        ///  Whether this ProgressData contains a 'mission_plan' or not

    pub fn has_mission(&self) -> bool {
        self.inner.mission_get_has_mission()
    }
        ///  Mission plan

    pub fn mission_plan(&self) -> MissionPlanOwned {MissionPlan::new(self.inner.mission_get_mission_plan()).into_owned()
    }
}

    struct MissionInner {
        plugin: cxx::UniquePtr<crate::mission::mavsdk::Mission>,
        
                /// MissionProgress State
                mission_progress_handle: Option<usize>,
                mission_progress_user_data: *mut c_void,
    }

    pub struct MissionClient {
        inner: Mutex<MissionInner>,
        
                // Producer for the latest MissionProgress state; broadcast to all active subscribers.
                mission_progress_tx: watch::Sender<std::option::Option<
        MissionProgressOwned>>,
    }

    impl MissionClient {
        pub fn new(plugin: cxx::UniquePtr<crate::mission::mavsdk::Mission>) -> Self {
            
                    let (mission_progress_tx, _) = watch::channel(None);

            Self {
                inner: Mutex::new(MissionInner {
                    plugin,
                    
                            mission_progress_handle: None,
                            mission_progress_user_data: std::ptr::null_mut(),
                }),
                
                        mission_progress_tx,
            }
        }
        
        pub fn subscribe_mission_progress(&self) -> watch::Receiver<std::option::Option<
        MissionProgressOwned>> {
                    let mut inner = self.inner.lock().unwrap();

                    if inner.mission_progress_handle.is_none() {
                        // Lazy initialization of C++ 'mission_progress' mission stream.
                        let user_data_ptr = Box::into_raw(Box::new(self.mission_progress_tx.clone())) as *mut c_void;

                        let handle = inner.plugin.pin_mut().subscribe_mission_progress(
                            mission_mission_progress_callback_ffi as usize as libc::uintptr_t,
                            user_data_ptr as libc::uintptr_t
                        ) as usize;

                        inner.mission_progress_handle = Some(handle);
                        inner.mission_progress_user_data = user_data_ptr;
                    }
                    
                    self.mission_progress_tx.subscribe()
                }
    }


        #[unsafe(no_mangle)]
        pub extern "C" fn mission_mission_progress_callback_ffi(
            user_data: *mut c_void,
            mission_progress: &crate::mission::mavsdk::Mission_MissionProgress,
        ) {
            if user_data.is_null() { return; }
                let send_mission_progress = MissionProgress::new(mission_progress).into_owned();

            // Cast the pointer to watch::Sender
            let sender = unsafe {
                &*(user_data as *const watch::Sender<std::option::Option<
        MissionProgressOwned>>)
            };

            // Overwrite the current value in the channel with the new update
            let _ = sender.send(Some(send_mission_progress));
        }