// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct AttitudeOwned {
    pub roll_deg: f32,
    pub pitch_deg: f32,
    pub yaw_deg: f32,
    pub thrust_value: f32,
}

/*  */
pub struct Attitude<'a> {
    inner: &'a crate::offboard::mavsdk::Offboard_Attitude,
}

impl<'a> Attitude<'a> {
    pub fn new(inner: &'a crate::offboard::mavsdk::Offboard_Attitude) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> AttitudeOwned {
        AttitudeOwned {
            roll_deg: self.roll_deg(),
            pitch_deg: self.pitch_deg(),
            yaw_deg: self.yaw_deg(),
            thrust_value: self.thrust_value(),
        }
    }
    ///  Roll angle (in degrees, positive is right side down)

    pub fn roll_deg(&self) -> f32 {
        self.inner.offboard_get_roll_deg()
    }
    ///  Pitch angle (in degrees, positive is nose up)

    pub fn pitch_deg(&self) -> f32 {
        self.inner.offboard_get_pitch_deg()
    }
    ///  Yaw angle (in degrees, positive is move nose to the right)

    pub fn yaw_deg(&self) -> f32 {
        self.inner.offboard_get_yaw_deg()
    }
    ///  Thrust (range: 0 to 1)

    pub fn thrust_value(&self) -> f32 {
        self.inner.offboard_get_thrust_value()
    }
}

pub struct ActuatorControlGroupOwned {
    pub controls: std::vec::Vec<f32>,
}

/*  */
pub struct ActuatorControlGroup<'a> {
    inner: &'a crate::offboard::mavsdk::Offboard_ActuatorControlGroup,
}

impl<'a> ActuatorControlGroup<'a> {
    pub fn new(inner: &'a crate::offboard::mavsdk::Offboard_ActuatorControlGroup) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> ActuatorControlGroupOwned {
        ActuatorControlGroupOwned {
            controls: self.controls(),
        }
    }
    ///  Controls in the group

    pub fn controls(&self) -> std::vec::Vec<f32> {
        self.inner.offboard_get_controls().as_slice().to_vec()
    }
}

pub struct ActuatorControlOwned {
    pub groups: std::vec::Vec<ActuatorControlGroupOwned>,
}

/*  */
pub struct ActuatorControl<'a> {
    inner: &'a crate::offboard::mavsdk::Offboard_ActuatorControl,
}

impl<'a> ActuatorControl<'a> {
    pub fn new(inner: &'a crate::offboard::mavsdk::Offboard_ActuatorControl) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> ActuatorControlOwned {
        ActuatorControlOwned {
            groups: self.groups(),
        }
    }
    ///  Control groups.

    pub fn groups(&self) -> std::vec::Vec<ActuatorControlGroupOwned> {
        self.inner
            .offboard_get_groups()
            .iter()
            .map(|item| ActuatorControlGroup::new(item).into_owned())
            .collect()
    }
}

pub struct AttitudeRateOwned {
    pub roll_deg_s: f32,
    pub pitch_deg_s: f32,
    pub yaw_deg_s: f32,
    pub thrust_value: f32,
}

/*  */
pub struct AttitudeRate<'a> {
    inner: &'a crate::offboard::mavsdk::Offboard_AttitudeRate,
}

impl<'a> AttitudeRate<'a> {
    pub fn new(inner: &'a crate::offboard::mavsdk::Offboard_AttitudeRate) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> AttitudeRateOwned {
        AttitudeRateOwned {
            roll_deg_s: self.roll_deg_s(),
            pitch_deg_s: self.pitch_deg_s(),
            yaw_deg_s: self.yaw_deg_s(),
            thrust_value: self.thrust_value(),
        }
    }
    ///  Roll angular rate (in degrees/second, positive for clock-wise looking from front)

    pub fn roll_deg_s(&self) -> f32 {
        self.inner.offboard_get_roll_deg_s()
    }
    ///  Pitch angular rate (in degrees/second, positive for head/front moving up)

    pub fn pitch_deg_s(&self) -> f32 {
        self.inner.offboard_get_pitch_deg_s()
    }
    ///  Yaw angular rate (in degrees/second, positive for clock-wise looking from above)

    pub fn yaw_deg_s(&self) -> f32 {
        self.inner.offboard_get_yaw_deg_s()
    }
    ///  Thrust (range: 0 to 1)

    pub fn thrust_value(&self) -> f32 {
        self.inner.offboard_get_thrust_value()
    }
}

pub struct PositionNedYawOwned {
    pub north_m: f32,
    pub east_m: f32,
    pub down_m: f32,
    pub yaw_deg: f32,
}

/*  */
pub struct PositionNedYaw<'a> {
    inner: &'a crate::offboard::mavsdk::Offboard_PositionNedYaw,
}

impl<'a> PositionNedYaw<'a> {
    pub fn new(inner: &'a crate::offboard::mavsdk::Offboard_PositionNedYaw) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> PositionNedYawOwned {
        PositionNedYawOwned {
            north_m: self.north_m(),
            east_m: self.east_m(),
            down_m: self.down_m(),
            yaw_deg: self.yaw_deg(),
        }
    }
    ///  Position North (in metres)

    pub fn north_m(&self) -> f32 {
        self.inner.offboard_get_north_m()
    }
    ///  Position East (in metres)

    pub fn east_m(&self) -> f32 {
        self.inner.offboard_get_east_m()
    }
    ///  Position Down (in metres)

    pub fn down_m(&self) -> f32 {
        self.inner.offboard_get_down_m()
    }
    ///  Yaw in degrees (0 North, positive is clock-wise looking from above)

    pub fn yaw_deg(&self) -> f32 {
        self.inner.offboard_get_yaw_deg()
    }
}

pub struct PositionGlobalYawOwned {
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub alt_m: f32,
    pub yaw_deg: f32,
    pub altitude_type: crate::offboard::mavsdk::Offboard_PositionGlobalYaw_AltitudeType,
}

/*  */
pub struct PositionGlobalYaw<'a> {
    inner: &'a crate::offboard::mavsdk::Offboard_PositionGlobalYaw,
}

impl<'a> PositionGlobalYaw<'a> {
    pub fn new(inner: &'a crate::offboard::mavsdk::Offboard_PositionGlobalYaw) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> PositionGlobalYawOwned {
        PositionGlobalYawOwned {
            lat_deg: self.lat_deg(),
            lon_deg: self.lon_deg(),
            alt_m: self.alt_m(),
            yaw_deg: self.yaw_deg(),
            altitude_type: self.altitude_type(),
        }
    }
    ///  Latitude (in degrees)

    pub fn lat_deg(&self) -> f64 {
        self.inner.offboard_get_lat_deg()
    }
    ///  Longitude (in degrees)

    pub fn lon_deg(&self) -> f64 {
        self.inner.offboard_get_lon_deg()
    }
    ///  altitude (in metres)

    pub fn alt_m(&self) -> f32 {
        self.inner.offboard_get_alt_m()
    }
    ///  Yaw in degrees (0 North, positive is clock-wise looking from above)

    pub fn yaw_deg(&self) -> f32 {
        self.inner.offboard_get_yaw_deg()
    }
    ///  altitude type for this position

    pub fn altitude_type(
        &self,
    ) -> crate::offboard::mavsdk::Offboard_PositionGlobalYaw_AltitudeType {
        self.inner.offboard_get_altitude_type().clone()
    }
}

pub struct VelocityBodyYawspeedOwned {
    pub forward_m_s: f32,
    pub right_m_s: f32,
    pub down_m_s: f32,
    pub yawspeed_deg_s: f32,
}

/*  */
pub struct VelocityBodyYawspeed<'a> {
    inner: &'a crate::offboard::mavsdk::Offboard_VelocityBodyYawspeed,
}

impl<'a> VelocityBodyYawspeed<'a> {
    pub fn new(inner: &'a crate::offboard::mavsdk::Offboard_VelocityBodyYawspeed) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> VelocityBodyYawspeedOwned {
        VelocityBodyYawspeedOwned {
            forward_m_s: self.forward_m_s(),
            right_m_s: self.right_m_s(),
            down_m_s: self.down_m_s(),
            yawspeed_deg_s: self.yawspeed_deg_s(),
        }
    }
    ///  Velocity forward (in metres/second)

    pub fn forward_m_s(&self) -> f32 {
        self.inner.offboard_get_forward_m_s()
    }
    ///  Velocity right (in metres/second)

    pub fn right_m_s(&self) -> f32 {
        self.inner.offboard_get_right_m_s()
    }
    ///  Velocity down (in metres/second)

    pub fn down_m_s(&self) -> f32 {
        self.inner.offboard_get_down_m_s()
    }
    ///  Yaw angular rate (in degrees/second, positive for clock-wise looking from above)

    pub fn yawspeed_deg_s(&self) -> f32 {
        self.inner.offboard_get_yawspeed_deg_s()
    }
}

pub struct VelocityNedYawOwned {
    pub north_m_s: f32,
    pub east_m_s: f32,
    pub down_m_s: f32,
    pub yaw_deg: f32,
}

/*  */
pub struct VelocityNedYaw<'a> {
    inner: &'a crate::offboard::mavsdk::Offboard_VelocityNedYaw,
}

impl<'a> VelocityNedYaw<'a> {
    pub fn new(inner: &'a crate::offboard::mavsdk::Offboard_VelocityNedYaw) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> VelocityNedYawOwned {
        VelocityNedYawOwned {
            north_m_s: self.north_m_s(),
            east_m_s: self.east_m_s(),
            down_m_s: self.down_m_s(),
            yaw_deg: self.yaw_deg(),
        }
    }
    ///  Velocity North (in metres/second)

    pub fn north_m_s(&self) -> f32 {
        self.inner.offboard_get_north_m_s()
    }
    ///  Velocity East (in metres/second)

    pub fn east_m_s(&self) -> f32 {
        self.inner.offboard_get_east_m_s()
    }
    ///  Velocity Down (in metres/second)

    pub fn down_m_s(&self) -> f32 {
        self.inner.offboard_get_down_m_s()
    }
    ///  Yaw in degrees (0 North, positive is clock-wise looking from above)

    pub fn yaw_deg(&self) -> f32 {
        self.inner.offboard_get_yaw_deg()
    }
}

pub struct AccelerationNedOwned {
    pub north_m_s2: f32,
    pub east_m_s2: f32,
    pub down_m_s2: f32,
}

/*  */
pub struct AccelerationNed<'a> {
    inner: &'a crate::offboard::mavsdk::Offboard_AccelerationNed,
}

impl<'a> AccelerationNed<'a> {
    pub fn new(inner: &'a crate::offboard::mavsdk::Offboard_AccelerationNed) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> AccelerationNedOwned {
        AccelerationNedOwned {
            north_m_s2: self.north_m_s2(),
            east_m_s2: self.east_m_s2(),
            down_m_s2: self.down_m_s2(),
        }
    }
    ///  Acceleration North (in metres/second^2)

    pub fn north_m_s2(&self) -> f32 {
        self.inner.offboard_get_north_m_s2()
    }
    ///  Acceleration East (in metres/second^2)

    pub fn east_m_s2(&self) -> f32 {
        self.inner.offboard_get_east_m_s2()
    }
    ///  Acceleration Down (in metres/second^2)

    pub fn down_m_s2(&self) -> f32 {
        self.inner.offboard_get_down_m_s2()
    }
}

struct OffboardInner {
    plugin: cxx::UniquePtr<crate::offboard::mavsdk::Offboard>,
}

pub struct OffboardClient {
    inner: Mutex<OffboardInner>,
}

impl OffboardClient {
    pub fn new(plugin: cxx::UniquePtr<crate::offboard::mavsdk::Offboard>) -> Self {
        Self {
            inner: Mutex::new(OffboardInner { plugin }),
        }
    }
}
