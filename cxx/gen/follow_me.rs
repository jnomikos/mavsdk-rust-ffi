// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct ConfigOwned {
    pub follow_height_m: f32,
    pub follow_distance_m: f32,
    pub responsiveness: f32,
    pub altitude_mode: crate::follow_me::mavsdk::FollowMe_Config_FollowAltitudeMode,
    pub max_tangential_vel_m_s: f32,
    pub follow_angle_deg: f32,
}

/*  */
pub struct Config<'a> {
    inner: &'a crate::follow_me::mavsdk::FollowMe_Config,
}

impl<'a> Config<'a> {
    pub fn new(inner: &'a crate::follow_me::mavsdk::FollowMe_Config) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> ConfigOwned {
        ConfigOwned {
            follow_height_m: self.follow_height_m(),
            follow_distance_m: self.follow_distance_m(),
            responsiveness: self.responsiveness(),
            altitude_mode: self.altitude_mode(),
            max_tangential_vel_m_s: self.max_tangential_vel_m_s(),
            follow_angle_deg: self.follow_angle_deg(),
        }
    }
    ///  [m] Follow height in meters (recommended minimum 8 meters)

    pub fn follow_height_m(&self) -> f32 {
        self.inner.follow_me_get_follow_height_m()
    }
    ///  [m] Follow distance to target in meters (recommended minimum 4 meter)

    pub fn follow_distance_m(&self) -> f32 {
        self.inner.follow_me_get_follow_distance_m()
    }
    ///  How responsive the vehicle is to the motion of the target, Lower value = More responsive (range 0.0 to 1.0)

    pub fn responsiveness(&self) -> f32 {
        self.inner.follow_me_get_responsiveness()
    }
    ///  Follow Altitude control mode

    pub fn altitude_mode(&self) -> crate::follow_me::mavsdk::FollowMe_Config_FollowAltitudeMode {
        self.inner.follow_me_get_altitude_mode().clone()
    }
    ///  [m/s] Maximum orbit tangential velocity relative to the target, in meters per second. Higher value = More aggressive follow angle tracking.

    pub fn max_tangential_vel_m_s(&self) -> f32 {
        self.inner.follow_me_get_max_tangential_vel_m_s()
    }
    ///  [deg] Follow Angle relative to the target. 0 equals following in front of the target's direction. Angle increases in Clockwise direction, so following from right would be 90 degrees, from the left is -90 degrees, and so on.

    pub fn follow_angle_deg(&self) -> f32 {
        self.inner.follow_me_get_follow_angle_deg()
    }
}

pub struct TargetLocationOwned {
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub absolute_altitude_m: f32,
    pub velocity_x_m_s: f32,
    pub velocity_y_m_s: f32,
    pub velocity_z_m_s: f32,
}

/*  */
pub struct TargetLocation<'a> {
    inner: &'a crate::follow_me::mavsdk::FollowMe_TargetLocation,
}

impl<'a> TargetLocation<'a> {
    pub fn new(inner: &'a crate::follow_me::mavsdk::FollowMe_TargetLocation) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> TargetLocationOwned {
        TargetLocationOwned {
            latitude_deg: self.latitude_deg(),
            longitude_deg: self.longitude_deg(),
            absolute_altitude_m: self.absolute_altitude_m(),
            velocity_x_m_s: self.velocity_x_m_s(),
            velocity_y_m_s: self.velocity_y_m_s(),
            velocity_z_m_s: self.velocity_z_m_s(),
        }
    }
    ///  Target latitude in degrees

    pub fn latitude_deg(&self) -> f64 {
        self.inner.follow_me_get_latitude_deg()
    }
    ///  Target longitude in degrees

    pub fn longitude_deg(&self) -> f64 {
        self.inner.follow_me_get_longitude_deg()
    }
    ///  Target altitude in meters above MSL

    pub fn absolute_altitude_m(&self) -> f32 {
        self.inner.follow_me_get_absolute_altitude_m()
    }
    ///  Target velocity in X axis, in meters per second

    pub fn velocity_x_m_s(&self) -> f32 {
        self.inner.follow_me_get_velocity_x_m_s()
    }
    ///  Target velocity in Y axis, in meters per second

    pub fn velocity_y_m_s(&self) -> f32 {
        self.inner.follow_me_get_velocity_y_m_s()
    }
    ///  Target velocity in Z axis, in meters per second

    pub fn velocity_z_m_s(&self) -> f32 {
        self.inner.follow_me_get_velocity_z_m_s()
    }
}

struct FollowMeInner {
    plugin: cxx::UniquePtr<crate::follow_me::mavsdk::FollowMe>,
}

pub struct FollowMeClient {
    inner: Mutex<FollowMeInner>,
}

impl FollowMeClient {
    pub fn new(plugin: cxx::UniquePtr<crate::follow_me::mavsdk::FollowMe>) -> Self {
        Self {
            inner: Mutex::new(FollowMeInner { plugin }),
        }
    }
}
