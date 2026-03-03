// Auto-generated wrapper. Do not edit.
use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct PositionOwned {
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub absolute_altitude_m: f32,
    pub relative_altitude_m: f32,
}

/*  */
pub struct Position<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_Position,
}

impl<'a> Position<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_Position) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> PositionOwned {
        PositionOwned {
            latitude_deg: self.latitude_deg(),
            longitude_deg: self.longitude_deg(),
            absolute_altitude_m: self.absolute_altitude_m(),
            relative_altitude_m: self.relative_altitude_m(),
        }
    }
    ///  Latitude in degrees (range: -90 to +90)

    pub fn latitude_deg(&self) -> f64 {
        self.inner.telemetry_get_latitude_deg()
    }
    ///  Longitude in degrees (range: -180 to +180)

    pub fn longitude_deg(&self) -> f64 {
        self.inner.telemetry_get_longitude_deg()
    }
    ///  Altitude AMSL (above mean sea level) in metres

    pub fn absolute_altitude_m(&self) -> f32 {
        self.inner.telemetry_get_absolute_altitude_m()
    }
    ///  Altitude relative to takeoff altitude in metres

    pub fn relative_altitude_m(&self) -> f32 {
        self.inner.telemetry_get_relative_altitude_m()
    }
}

pub struct HeadingOwned {
    pub heading_deg: f64,
}

/*  */
pub struct Heading<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_Heading,
}

impl<'a> Heading<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_Heading) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> HeadingOwned {
        HeadingOwned {
            heading_deg: self.heading_deg(),
        }
    }
    ///  Heading in degrees (range: 0 to +360)

    pub fn heading_deg(&self) -> f64 {
        self.inner.telemetry_get_heading_deg()
    }
}

pub struct QuaternionOwned {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub timestamp_us: u64,
}

/*  */
pub struct Quaternion<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_Quaternion,
}

impl<'a> Quaternion<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_Quaternion) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> QuaternionOwned {
        QuaternionOwned {
            w: self.w(),
            x: self.x(),
            y: self.y(),
            z: self.z(),
            timestamp_us: self.timestamp_us(),
        }
    }
    ///  Quaternion entry 0, also denoted as a

    pub fn w(&self) -> f32 {
        self.inner.telemetry_get_w()
    }
    ///  Quaternion entry 1, also denoted as b

    pub fn x(&self) -> f32 {
        self.inner.telemetry_get_x()
    }
    ///  Quaternion entry 2, also denoted as c

    pub fn y(&self) -> f32 {
        self.inner.telemetry_get_y()
    }
    ///  Quaternion entry 3, also denoted as d

    pub fn z(&self) -> f32 {
        self.inner.telemetry_get_z()
    }
    ///  Timestamp in microseconds

    pub fn timestamp_us(&self) -> u64 {
        self.inner.telemetry_get_timestamp_us()
    }
}

pub struct EulerAngleOwned {
    pub roll_deg: f32,
    pub pitch_deg: f32,
    pub yaw_deg: f32,
    pub timestamp_us: u64,
}

/*  */
pub struct EulerAngle<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_EulerAngle,
}

impl<'a> EulerAngle<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_EulerAngle) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> EulerAngleOwned {
        EulerAngleOwned {
            roll_deg: self.roll_deg(),
            pitch_deg: self.pitch_deg(),
            yaw_deg: self.yaw_deg(),
            timestamp_us: self.timestamp_us(),
        }
    }
    ///  Roll angle in degrees, positive is banking to the right

    pub fn roll_deg(&self) -> f32 {
        self.inner.telemetry_get_roll_deg()
    }
    ///  Pitch angle in degrees, positive is pitching nose up

    pub fn pitch_deg(&self) -> f32 {
        self.inner.telemetry_get_pitch_deg()
    }
    ///  Yaw angle in degrees, positive is clock-wise seen from above

    pub fn yaw_deg(&self) -> f32 {
        self.inner.telemetry_get_yaw_deg()
    }
    ///  Timestamp in microseconds

    pub fn timestamp_us(&self) -> u64 {
        self.inner.telemetry_get_timestamp_us()
    }
}

pub struct AngularVelocityBodyOwned {
    pub roll_rad_s: f32,
    pub pitch_rad_s: f32,
    pub yaw_rad_s: f32,
}

/*  */
pub struct AngularVelocityBody<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_AngularVelocityBody,
}

impl<'a> AngularVelocityBody<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_AngularVelocityBody) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> AngularVelocityBodyOwned {
        AngularVelocityBodyOwned {
            roll_rad_s: self.roll_rad_s(),
            pitch_rad_s: self.pitch_rad_s(),
            yaw_rad_s: self.yaw_rad_s(),
        }
    }
    ///  Roll angular velocity

    pub fn roll_rad_s(&self) -> f32 {
        self.inner.telemetry_get_roll_rad_s()
    }
    ///  Pitch angular velocity

    pub fn pitch_rad_s(&self) -> f32 {
        self.inner.telemetry_get_pitch_rad_s()
    }
    ///  Yaw angular velocity

    pub fn yaw_rad_s(&self) -> f32 {
        self.inner.telemetry_get_yaw_rad_s()
    }
}

pub struct GpsInfoOwned {
    pub num_satellites: i32,
    pub fix_type: crate::telemetry::mavsdk::Telemetry_FixType,
}

/*  */
pub struct GpsInfo<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_GpsInfo,
}

impl<'a> GpsInfo<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_GpsInfo) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> GpsInfoOwned {
        GpsInfoOwned {
            num_satellites: self.num_satellites(),
            fix_type: self.fix_type(),
        }
    }
    ///  Number of visible satellites in use

    pub fn num_satellites(&self) -> i32 {
        self.inner.telemetry_get_num_satellites()
    }
    ///  Fix type

    pub fn fix_type(&self) -> crate::telemetry::mavsdk::Telemetry_FixType {
        self.inner.telemetry_get_fix_type().clone()
    }
}

pub struct RawGpsOwned {
    pub timestamp_us: u64,
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub absolute_altitude_m: f32,
    pub hdop: f32,
    pub vdop: f32,
    pub velocity_m_s: f32,
    pub cog_deg: f32,
    pub altitude_ellipsoid_m: f32,
    pub horizontal_uncertainty_m: f32,
    pub vertical_uncertainty_m: f32,
    pub velocity_uncertainty_m_s: f32,
    pub heading_uncertainty_deg: f32,
    pub yaw_deg: f32,
}

/*  */
pub struct RawGps<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_RawGps,
}

impl<'a> RawGps<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_RawGps) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> RawGpsOwned {
        RawGpsOwned {
            timestamp_us: self.timestamp_us(),
            latitude_deg: self.latitude_deg(),
            longitude_deg: self.longitude_deg(),
            absolute_altitude_m: self.absolute_altitude_m(),
            hdop: self.hdop(),
            vdop: self.vdop(),
            velocity_m_s: self.velocity_m_s(),
            cog_deg: self.cog_deg(),
            altitude_ellipsoid_m: self.altitude_ellipsoid_m(),
            horizontal_uncertainty_m: self.horizontal_uncertainty_m(),
            vertical_uncertainty_m: self.vertical_uncertainty_m(),
            velocity_uncertainty_m_s: self.velocity_uncertainty_m_s(),
            heading_uncertainty_deg: self.heading_uncertainty_deg(),
            yaw_deg: self.yaw_deg(),
        }
    }
    ///  Timestamp in microseconds (UNIX Epoch time or time since system boot, to be inferred)

    pub fn timestamp_us(&self) -> u64 {
        self.inner.telemetry_get_timestamp_us()
    }
    ///  Latitude in degrees (WGS84, EGM96 ellipsoid)

    pub fn latitude_deg(&self) -> f64 {
        self.inner.telemetry_get_latitude_deg()
    }
    ///  Longitude in degrees (WGS84, EGM96 ellipsoid)

    pub fn longitude_deg(&self) -> f64 {
        self.inner.telemetry_get_longitude_deg()
    }
    ///  Altitude AMSL (above mean sea level) in metres

    pub fn absolute_altitude_m(&self) -> f32 {
        self.inner.telemetry_get_absolute_altitude_m()
    }
    ///  GPS HDOP horizontal dilution of position (unitless). If unknown, set to NaN

    pub fn hdop(&self) -> f32 {
        self.inner.telemetry_get_hdop()
    }
    ///  GPS VDOP vertical dilution of position (unitless). If unknown, set to NaN

    pub fn vdop(&self) -> f32 {
        self.inner.telemetry_get_vdop()
    }
    ///  Ground velocity in metres per second

    pub fn velocity_m_s(&self) -> f32 {
        self.inner.telemetry_get_velocity_m_s()
    }
    ///  Course over ground (NOT heading, but direction of movement) in degrees. If unknown, set to NaN

    pub fn cog_deg(&self) -> f32 {
        self.inner.telemetry_get_cog_deg()
    }
    ///  Altitude in metres (above WGS84, EGM96 ellipsoid)

    pub fn altitude_ellipsoid_m(&self) -> f32 {
        self.inner.telemetry_get_altitude_ellipsoid_m()
    }
    ///  Position uncertainty in metres

    pub fn horizontal_uncertainty_m(&self) -> f32 {
        self.inner.telemetry_get_horizontal_uncertainty_m()
    }
    ///  Altitude uncertainty in metres

    pub fn vertical_uncertainty_m(&self) -> f32 {
        self.inner.telemetry_get_vertical_uncertainty_m()
    }
    ///  Velocity uncertainty in metres per second

    pub fn velocity_uncertainty_m_s(&self) -> f32 {
        self.inner.telemetry_get_velocity_uncertainty_m_s()
    }
    ///  Heading uncertainty in degrees

    pub fn heading_uncertainty_deg(&self) -> f32 {
        self.inner.telemetry_get_heading_uncertainty_deg()
    }
    ///  Yaw in earth frame from north.

    pub fn yaw_deg(&self) -> f32 {
        self.inner.telemetry_get_yaw_deg()
    }
}

pub struct BatteryOwned {
    pub id: u32,
    pub temperature_degc: f32,
    pub voltage_v: f32,
    pub current_battery_a: f32,
    pub capacity_consumed_ah: f32,
    pub remaining_percent: f32,
    pub time_remaining_s: f32,
    pub battery_function: crate::telemetry::mavsdk::Telemetry_BatteryFunction,
}

/*  */
pub struct Battery<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_Battery,
}

impl<'a> Battery<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_Battery) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> BatteryOwned {
        BatteryOwned {
            id: self.id(),
            temperature_degc: self.temperature_degc(),
            voltage_v: self.voltage_v(),
            current_battery_a: self.current_battery_a(),
            capacity_consumed_ah: self.capacity_consumed_ah(),
            remaining_percent: self.remaining_percent(),
            time_remaining_s: self.time_remaining_s(),
            battery_function: self.battery_function(),
        }
    }
    ///  Battery ID, for systems with multiple batteries

    pub fn id(&self) -> u32 {
        self.inner.telemetry_get_id()
    }
    ///  Temperature of the battery in degrees Celsius. NAN for unknown temperature

    pub fn temperature_degc(&self) -> f32 {
        self.inner.telemetry_get_temperature_degc()
    }
    ///  Voltage in volts

    pub fn voltage_v(&self) -> f32 {
        self.inner.telemetry_get_voltage_v()
    }
    ///  Battery current in Amps, NAN if autopilot does not measure the current

    pub fn current_battery_a(&self) -> f32 {
        self.inner.telemetry_get_current_battery_a()
    }
    ///  Consumed charge in Amp hours, NAN if autopilot does not provide consumption estimate

    pub fn capacity_consumed_ah(&self) -> f32 {
        self.inner.telemetry_get_capacity_consumed_ah()
    }
    ///  Estimated battery remaining (range: 0 to 100)

    pub fn remaining_percent(&self) -> f32 {
        self.inner.telemetry_get_remaining_percent()
    }
    ///  Estimated battery usage time remaining

    pub fn time_remaining_s(&self) -> f32 {
        self.inner.telemetry_get_time_remaining_s()
    }
    ///  Function of the battery

    pub fn battery_function(&self) -> crate::telemetry::mavsdk::Telemetry_BatteryFunction {
        self.inner.telemetry_get_battery_function().clone()
    }
}

pub struct HealthOwned {
    pub is_gyrometer_calibration_ok: bool,
    pub is_accelerometer_calibration_ok: bool,
    pub is_magnetometer_calibration_ok: bool,
    pub is_local_position_ok: bool,
    pub is_global_position_ok: bool,
    pub is_home_position_ok: bool,
    pub is_armable: bool,
}

/*  */
pub struct Health<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_Health,
}

impl<'a> Health<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_Health) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> HealthOwned {
        HealthOwned {
            is_gyrometer_calibration_ok: self.is_gyrometer_calibration_ok(),
            is_accelerometer_calibration_ok: self.is_accelerometer_calibration_ok(),
            is_magnetometer_calibration_ok: self.is_magnetometer_calibration_ok(),
            is_local_position_ok: self.is_local_position_ok(),
            is_global_position_ok: self.is_global_position_ok(),
            is_home_position_ok: self.is_home_position_ok(),
            is_armable: self.is_armable(),
        }
    }
    ///  True if the gyrometer is calibrated

    pub fn is_gyrometer_calibration_ok(&self) -> bool {
        self.inner.telemetry_get_is_gyrometer_calibration_ok()
    }
    ///  True if the accelerometer is calibrated

    pub fn is_accelerometer_calibration_ok(&self) -> bool {
        self.inner.telemetry_get_is_accelerometer_calibration_ok()
    }
    ///  True if the magnetometer is calibrated

    pub fn is_magnetometer_calibration_ok(&self) -> bool {
        self.inner.telemetry_get_is_magnetometer_calibration_ok()
    }
    ///  True if the local position estimate is good enough to fly in 'position control' mode

    pub fn is_local_position_ok(&self) -> bool {
        self.inner.telemetry_get_is_local_position_ok()
    }
    ///  True if the global position estimate is good enough to fly in 'position control' mode

    pub fn is_global_position_ok(&self) -> bool {
        self.inner.telemetry_get_is_global_position_ok()
    }
    ///  True if the home position has been initialized properly

    pub fn is_home_position_ok(&self) -> bool {
        self.inner.telemetry_get_is_home_position_ok()
    }
    ///  True if system can be armed

    pub fn is_armable(&self) -> bool {
        self.inner.telemetry_get_is_armable()
    }
}

pub struct RcStatusOwned {
    pub was_available_once: bool,
    pub is_available: bool,
    pub signal_strength_percent: f32,
}

/*  */
pub struct RcStatus<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_RcStatus,
}

impl<'a> RcStatus<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_RcStatus) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> RcStatusOwned {
        RcStatusOwned {
            was_available_once: self.was_available_once(),
            is_available: self.is_available(),
            signal_strength_percent: self.signal_strength_percent(),
        }
    }
    ///  True if an RC signal has been available once

    pub fn was_available_once(&self) -> bool {
        self.inner.telemetry_get_was_available_once()
    }
    ///  True if the RC signal is available now

    pub fn is_available(&self) -> bool {
        self.inner.telemetry_get_is_available()
    }
    ///  Signal strength (range: 0 to 100, NaN if unknown)

    pub fn signal_strength_percent(&self) -> f32 {
        self.inner.telemetry_get_signal_strength_percent()
    }
}

pub struct StatusTextOwned {
    pub r#type: crate::telemetry::mavsdk::Telemetry_StatusTextType,
    pub text: String,
}

/*  */
pub struct StatusText<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_StatusText,
}

impl<'a> StatusText<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_StatusText) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> StatusTextOwned {
        StatusTextOwned {
            r#type: self.r#type(),
            text: self.text(),
        }
    }
    ///  Message type

    pub fn r#type(&self) -> crate::telemetry::mavsdk::Telemetry_StatusTextType {
        self.inner.telemetry_get_type().clone()
    }
    ///  MAVLink status message

    pub fn text(&self) -> String {
        self.inner
            .telemetry_get_text()
            .to_string_lossy()
            .into_owned()
    }
}

pub struct ActuatorControlTargetOwned {
    pub group: i32,
    pub controls: std::vec::Vec<f32>,
}

/*  */
pub struct ActuatorControlTarget<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_ActuatorControlTarget,
}

impl<'a> ActuatorControlTarget<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_ActuatorControlTarget) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> ActuatorControlTargetOwned {
        ActuatorControlTargetOwned {
            group: self.group(),
            controls: self.controls(),
        }
    }
    ///  An actuator control group is e.g. 'attitude' for the core flight controls, or 'gimbal' for a payload.

    pub fn group(&self) -> i32 {
        self.inner.telemetry_get_group()
    }
    ///  Controls normed from -1 to 1, where 0 is neutral position.

    pub fn controls(&self) -> std::vec::Vec<f32> {
        self.inner.telemetry_get_controls().as_slice().to_vec()
    }
}

pub struct ActuatorOutputStatusOwned {
    pub active: u32,
    pub actuator: std::vec::Vec<f32>,
}

/*  */
pub struct ActuatorOutputStatus<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_ActuatorOutputStatus,
}

impl<'a> ActuatorOutputStatus<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_ActuatorOutputStatus) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> ActuatorOutputStatusOwned {
        ActuatorOutputStatusOwned {
            active: self.active(),
            actuator: self.actuator(),
        }
    }
    ///  Active outputs

    pub fn active(&self) -> u32 {
        self.inner.telemetry_get_active()
    }
    ///  Servo/motor output values

    pub fn actuator(&self) -> std::vec::Vec<f32> {
        self.inner.telemetry_get_actuator().as_slice().to_vec()
    }
}

pub struct CovarianceOwned {
    pub covariance_matrix: std::vec::Vec<f32>,
}

/*  */
pub struct Covariance<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_Covariance,
}

impl<'a> Covariance<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_Covariance) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> CovarianceOwned {
        CovarianceOwned {
            covariance_matrix: self.covariance_matrix(),
        }
    }
    ///  Representation of a covariance matrix.

    pub fn covariance_matrix(&self) -> std::vec::Vec<f32> {
        self.inner
            .telemetry_get_covariance_matrix()
            .as_slice()
            .to_vec()
    }
}

pub struct VelocityBodyOwned {
    pub x_m_s: f32,
    pub y_m_s: f32,
    pub z_m_s: f32,
}

/*  */
pub struct VelocityBody<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_VelocityBody,
}

impl<'a> VelocityBody<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_VelocityBody) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> VelocityBodyOwned {
        VelocityBodyOwned {
            x_m_s: self.x_m_s(),
            y_m_s: self.y_m_s(),
            z_m_s: self.z_m_s(),
        }
    }
    ///  Velocity in X in metres/second

    pub fn x_m_s(&self) -> f32 {
        self.inner.telemetry_get_x_m_s()
    }
    ///  Velocity in Y in metres/second

    pub fn y_m_s(&self) -> f32 {
        self.inner.telemetry_get_y_m_s()
    }
    ///  Velocity in Z in metres/second

    pub fn z_m_s(&self) -> f32 {
        self.inner.telemetry_get_z_m_s()
    }
}

pub struct PositionBodyOwned {
    pub x_m: f32,
    pub y_m: f32,
    pub z_m: f32,
}

/*  */
pub struct PositionBody<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_PositionBody,
}

impl<'a> PositionBody<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_PositionBody) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> PositionBodyOwned {
        PositionBodyOwned {
            x_m: self.x_m(),
            y_m: self.y_m(),
            z_m: self.z_m(),
        }
    }
    ///  X Position in metres.

    pub fn x_m(&self) -> f32 {
        self.inner.telemetry_get_x_m()
    }
    ///  Y Position in metres.

    pub fn y_m(&self) -> f32 {
        self.inner.telemetry_get_y_m()
    }
    ///  Z Position in metres.

    pub fn z_m(&self) -> f32 {
        self.inner.telemetry_get_z_m()
    }
}

pub struct OdometryOwned {
    pub time_usec: u64,
    pub frame_id: crate::telemetry::mavsdk::Telemetry_Odometry_MavFrame,
    pub child_frame_id: crate::telemetry::mavsdk::Telemetry_Odometry_MavFrame,
    pub position_body: PositionBodyOwned,
    pub q: QuaternionOwned,
    pub velocity_body: VelocityBodyOwned,
    pub angular_velocity_body: AngularVelocityBodyOwned,
    pub pose_covariance: CovarianceOwned,
    pub velocity_covariance: CovarianceOwned,
}

/*  */
pub struct Odometry<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_Odometry,
}

impl<'a> Odometry<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_Odometry) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> OdometryOwned {
        OdometryOwned {
            time_usec: self.time_usec(),
            frame_id: self.frame_id(),
            child_frame_id: self.child_frame_id(),
            position_body: self.position_body(),
            q: self.q(),
            velocity_body: self.velocity_body(),
            angular_velocity_body: self.angular_velocity_body(),
            pose_covariance: self.pose_covariance(),
            velocity_covariance: self.velocity_covariance(),
        }
    }
    ///  Timestamp (0 to use Backend timestamp).

    pub fn time_usec(&self) -> u64 {
        self.inner.telemetry_get_time_usec()
    }
    ///  Coordinate frame of reference for the pose data.

    pub fn frame_id(&self) -> crate::telemetry::mavsdk::Telemetry_Odometry_MavFrame {
        self.inner.telemetry_get_frame_id().clone()
    }
    ///  Coordinate frame of reference for the velocity in free space (twist) data.

    pub fn child_frame_id(&self) -> crate::telemetry::mavsdk::Telemetry_Odometry_MavFrame {
        self.inner.telemetry_get_child_frame_id().clone()
    }
    ///  Position.

    pub fn position_body(&self) -> PositionBodyOwned {
        PositionBody::new(self.inner.telemetry_get_position_body()).into_owned()
    }
    ///  Quaternion components, w, x, y, z (1 0 0 0 is the null-rotation).

    pub fn q(&self) -> QuaternionOwned {
        Quaternion::new(self.inner.telemetry_get_q()).into_owned()
    }
    ///  Linear velocity (m/s).

    pub fn velocity_body(&self) -> VelocityBodyOwned {
        VelocityBody::new(self.inner.telemetry_get_velocity_body()).into_owned()
    }
    ///  Angular velocity (rad/s).

    pub fn angular_velocity_body(&self) -> AngularVelocityBodyOwned {
        AngularVelocityBody::new(self.inner.telemetry_get_angular_velocity_body()).into_owned()
    }
    ///  Pose cross-covariance matrix.

    pub fn pose_covariance(&self) -> CovarianceOwned {
        Covariance::new(self.inner.telemetry_get_pose_covariance()).into_owned()
    }
    ///  Velocity cross-covariance matrix.

    pub fn velocity_covariance(&self) -> CovarianceOwned {
        Covariance::new(self.inner.telemetry_get_velocity_covariance()).into_owned()
    }
}

pub struct DistanceSensorOwned {
    pub minimum_distance_m: f32,
    pub maximum_distance_m: f32,
    pub current_distance_m: f32,
    pub orientation: EulerAngleOwned,
}

/*  */
pub struct DistanceSensor<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_DistanceSensor,
}

impl<'a> DistanceSensor<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_DistanceSensor) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> DistanceSensorOwned {
        DistanceSensorOwned {
            minimum_distance_m: self.minimum_distance_m(),
            maximum_distance_m: self.maximum_distance_m(),
            current_distance_m: self.current_distance_m(),
            orientation: self.orientation(),
        }
    }
    ///  Minimum distance the sensor can measure, NaN if unknown.

    pub fn minimum_distance_m(&self) -> f32 {
        self.inner.telemetry_get_minimum_distance_m()
    }
    ///  Maximum distance the sensor can measure, NaN if unknown.

    pub fn maximum_distance_m(&self) -> f32 {
        self.inner.telemetry_get_maximum_distance_m()
    }
    ///  Current distance reading, NaN if unknown.

    pub fn current_distance_m(&self) -> f32 {
        self.inner.telemetry_get_current_distance_m()
    }
    ///  Sensor Orientation reading.

    pub fn orientation(&self) -> EulerAngleOwned {
        EulerAngle::new(self.inner.telemetry_get_orientation()).into_owned()
    }
}

pub struct ScaledPressureOwned {
    pub timestamp_us: u64,
    pub absolute_pressure_hpa: f32,
    pub differential_pressure_hpa: f32,
    pub temperature_deg: f32,
    pub differential_pressure_temperature_deg: f32,
}

/*  */
pub struct ScaledPressure<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_ScaledPressure,
}

impl<'a> ScaledPressure<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_ScaledPressure) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> ScaledPressureOwned {
        ScaledPressureOwned {
            timestamp_us: self.timestamp_us(),
            absolute_pressure_hpa: self.absolute_pressure_hpa(),
            differential_pressure_hpa: self.differential_pressure_hpa(),
            temperature_deg: self.temperature_deg(),
            differential_pressure_temperature_deg: self.differential_pressure_temperature_deg(),
        }
    }
    ///  Timestamp (time since system boot)

    pub fn timestamp_us(&self) -> u64 {
        self.inner.telemetry_get_timestamp_us()
    }
    ///  Absolute pressure in hPa

    pub fn absolute_pressure_hpa(&self) -> f32 {
        self.inner.telemetry_get_absolute_pressure_hpa()
    }
    ///  Differential pressure 1 in hPa

    pub fn differential_pressure_hpa(&self) -> f32 {
        self.inner.telemetry_get_differential_pressure_hpa()
    }
    ///  Absolute pressure temperature (in celsius)

    pub fn temperature_deg(&self) -> f32 {
        self.inner.telemetry_get_temperature_deg()
    }
    ///  Differential pressure temperature (in celsius, 0 if not available)

    pub fn differential_pressure_temperature_deg(&self) -> f32 {
        self.inner
            .telemetry_get_differential_pressure_temperature_deg()
    }
}

pub struct PositionNedOwned {
    pub north_m: f32,
    pub east_m: f32,
    pub down_m: f32,
}

/*  */
pub struct PositionNed<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_PositionNed,
}

impl<'a> PositionNed<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_PositionNed) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> PositionNedOwned {
        PositionNedOwned {
            north_m: self.north_m(),
            east_m: self.east_m(),
            down_m: self.down_m(),
        }
    }
    ///  Position along north direction in metres

    pub fn north_m(&self) -> f32 {
        self.inner.telemetry_get_north_m()
    }
    ///  Position along east direction in metres

    pub fn east_m(&self) -> f32 {
        self.inner.telemetry_get_east_m()
    }
    ///  Position along down direction in metres

    pub fn down_m(&self) -> f32 {
        self.inner.telemetry_get_down_m()
    }
}

pub struct VelocityNedOwned {
    pub north_m_s: f32,
    pub east_m_s: f32,
    pub down_m_s: f32,
}

/*  */
pub struct VelocityNed<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_VelocityNed,
}

impl<'a> VelocityNed<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_VelocityNed) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> VelocityNedOwned {
        VelocityNedOwned {
            north_m_s: self.north_m_s(),
            east_m_s: self.east_m_s(),
            down_m_s: self.down_m_s(),
        }
    }
    ///  Velocity along north direction in metres per second

    pub fn north_m_s(&self) -> f32 {
        self.inner.telemetry_get_north_m_s()
    }
    ///  Velocity along east direction in metres per second

    pub fn east_m_s(&self) -> f32 {
        self.inner.telemetry_get_east_m_s()
    }
    ///  Velocity along down direction in metres per second

    pub fn down_m_s(&self) -> f32 {
        self.inner.telemetry_get_down_m_s()
    }
}

pub struct PositionVelocityNedOwned {
    pub position: PositionNedOwned,
    pub velocity: VelocityNedOwned,
}

/*  */
pub struct PositionVelocityNed<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_PositionVelocityNed,
}

impl<'a> PositionVelocityNed<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_PositionVelocityNed) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> PositionVelocityNedOwned {
        PositionVelocityNedOwned {
            position: self.position(),
            velocity: self.velocity(),
        }
    }
    ///  Position (NED)

    pub fn position(&self) -> PositionNedOwned {
        PositionNed::new(self.inner.telemetry_get_position()).into_owned()
    }
    ///  Velocity (NED)

    pub fn velocity(&self) -> VelocityNedOwned {
        VelocityNed::new(self.inner.telemetry_get_velocity()).into_owned()
    }
}

pub struct GroundTruthOwned {
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub absolute_altitude_m: f32,
}

/*  */
pub struct GroundTruth<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_GroundTruth,
}

impl<'a> GroundTruth<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_GroundTruth) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> GroundTruthOwned {
        GroundTruthOwned {
            latitude_deg: self.latitude_deg(),
            longitude_deg: self.longitude_deg(),
            absolute_altitude_m: self.absolute_altitude_m(),
        }
    }
    ///  Latitude in degrees (range: -90 to +90)

    pub fn latitude_deg(&self) -> f64 {
        self.inner.telemetry_get_latitude_deg()
    }
    ///  Longitude in degrees (range: -180 to 180)

    pub fn longitude_deg(&self) -> f64 {
        self.inner.telemetry_get_longitude_deg()
    }
    ///  Altitude AMSL (above mean sea level) in metres

    pub fn absolute_altitude_m(&self) -> f32 {
        self.inner.telemetry_get_absolute_altitude_m()
    }
}

pub struct FixedwingMetricsOwned {
    pub airspeed_m_s: f32,
    pub throttle_percentage: f32,
    pub climb_rate_m_s: f32,
    pub groundspeed_m_s: f32,
    pub heading_deg: f32,
    pub absolute_altitude_m: f32,
}

/*  */
pub struct FixedwingMetrics<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_FixedwingMetrics,
}

impl<'a> FixedwingMetrics<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_FixedwingMetrics) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> FixedwingMetricsOwned {
        FixedwingMetricsOwned {
            airspeed_m_s: self.airspeed_m_s(),
            throttle_percentage: self.throttle_percentage(),
            climb_rate_m_s: self.climb_rate_m_s(),
            groundspeed_m_s: self.groundspeed_m_s(),
            heading_deg: self.heading_deg(),
            absolute_altitude_m: self.absolute_altitude_m(),
        }
    }
    ///  Current indicated airspeed (IAS) in metres per second

    pub fn airspeed_m_s(&self) -> f32 {
        self.inner.telemetry_get_airspeed_m_s()
    }
    ///  Current throttle setting (0 to 100)

    pub fn throttle_percentage(&self) -> f32 {
        self.inner.telemetry_get_throttle_percentage()
    }
    ///  Current climb rate in metres per second

    pub fn climb_rate_m_s(&self) -> f32 {
        self.inner.telemetry_get_climb_rate_m_s()
    }
    ///  Current groundspeed metres per second

    pub fn groundspeed_m_s(&self) -> f32 {
        self.inner.telemetry_get_groundspeed_m_s()
    }
    ///  Current heading in compass units (0-360, 0=north)

    pub fn heading_deg(&self) -> f32 {
        self.inner.telemetry_get_heading_deg()
    }
    ///  Current altitude in metres (MSL)

    pub fn absolute_altitude_m(&self) -> f32 {
        self.inner.telemetry_get_absolute_altitude_m()
    }
}

pub struct AccelerationFrdOwned {
    pub forward_m_s2: f32,
    pub right_m_s2: f32,
    pub down_m_s2: f32,
}

/*  */
pub struct AccelerationFrd<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_AccelerationFrd,
}

impl<'a> AccelerationFrd<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_AccelerationFrd) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> AccelerationFrdOwned {
        AccelerationFrdOwned {
            forward_m_s2: self.forward_m_s2(),
            right_m_s2: self.right_m_s2(),
            down_m_s2: self.down_m_s2(),
        }
    }
    ///  Acceleration in forward direction in metres per second^2

    pub fn forward_m_s2(&self) -> f32 {
        self.inner.telemetry_get_forward_m_s2()
    }
    ///  Acceleration in right direction in metres per second^2

    pub fn right_m_s2(&self) -> f32 {
        self.inner.telemetry_get_right_m_s2()
    }
    ///  Acceleration in down direction in metres per second^2

    pub fn down_m_s2(&self) -> f32 {
        self.inner.telemetry_get_down_m_s2()
    }
}

pub struct AngularVelocityFrdOwned {
    pub forward_rad_s: f32,
    pub right_rad_s: f32,
    pub down_rad_s: f32,
}

/*  */
pub struct AngularVelocityFrd<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_AngularVelocityFrd,
}

impl<'a> AngularVelocityFrd<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_AngularVelocityFrd) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> AngularVelocityFrdOwned {
        AngularVelocityFrdOwned {
            forward_rad_s: self.forward_rad_s(),
            right_rad_s: self.right_rad_s(),
            down_rad_s: self.down_rad_s(),
        }
    }
    ///  Angular velocity in forward direction in radians per second

    pub fn forward_rad_s(&self) -> f32 {
        self.inner.telemetry_get_forward_rad_s()
    }
    ///  Angular velocity in right direction in radians per second

    pub fn right_rad_s(&self) -> f32 {
        self.inner.telemetry_get_right_rad_s()
    }
    ///  Angular velocity in Down direction in radians per second

    pub fn down_rad_s(&self) -> f32 {
        self.inner.telemetry_get_down_rad_s()
    }
}

pub struct MagneticFieldFrdOwned {
    pub forward_gauss: f32,
    pub right_gauss: f32,
    pub down_gauss: f32,
}

/*  */
pub struct MagneticFieldFrd<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_MagneticFieldFrd,
}

impl<'a> MagneticFieldFrd<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_MagneticFieldFrd) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> MagneticFieldFrdOwned {
        MagneticFieldFrdOwned {
            forward_gauss: self.forward_gauss(),
            right_gauss: self.right_gauss(),
            down_gauss: self.down_gauss(),
        }
    }
    ///  Magnetic field in forward direction measured in Gauss

    pub fn forward_gauss(&self) -> f32 {
        self.inner.telemetry_get_forward_gauss()
    }
    ///  Magnetic field in East direction measured in Gauss

    pub fn right_gauss(&self) -> f32 {
        self.inner.telemetry_get_right_gauss()
    }
    ///  Magnetic field in Down direction measured in Gauss

    pub fn down_gauss(&self) -> f32 {
        self.inner.telemetry_get_down_gauss()
    }
}

pub struct ImuOwned {
    pub acceleration_frd: AccelerationFrdOwned,
    pub angular_velocity_frd: AngularVelocityFrdOwned,
    pub magnetic_field_frd: MagneticFieldFrdOwned,
    pub temperature_degc: f32,
    pub timestamp_us: u64,
}

/*  */
pub struct Imu<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_Imu,
}

impl<'a> Imu<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_Imu) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> ImuOwned {
        ImuOwned {
            acceleration_frd: self.acceleration_frd(),
            angular_velocity_frd: self.angular_velocity_frd(),
            magnetic_field_frd: self.magnetic_field_frd(),
            temperature_degc: self.temperature_degc(),
            timestamp_us: self.timestamp_us(),
        }
    }
    ///  Acceleration

    pub fn acceleration_frd(&self) -> AccelerationFrdOwned {
        AccelerationFrd::new(self.inner.telemetry_get_acceleration_frd()).into_owned()
    }
    ///  Angular velocity

    pub fn angular_velocity_frd(&self) -> AngularVelocityFrdOwned {
        AngularVelocityFrd::new(self.inner.telemetry_get_angular_velocity_frd()).into_owned()
    }
    ///  Magnetic field

    pub fn magnetic_field_frd(&self) -> MagneticFieldFrdOwned {
        MagneticFieldFrd::new(self.inner.telemetry_get_magnetic_field_frd()).into_owned()
    }
    ///  Temperature

    pub fn temperature_degc(&self) -> f32 {
        self.inner.telemetry_get_temperature_degc()
    }
    ///  Timestamp in microseconds

    pub fn timestamp_us(&self) -> u64 {
        self.inner.telemetry_get_timestamp_us()
    }
}

pub struct GpsGlobalOriginOwned {
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub altitude_m: f32,
}

/*  */
pub struct GpsGlobalOrigin<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_GpsGlobalOrigin,
}

impl<'a> GpsGlobalOrigin<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_GpsGlobalOrigin) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> GpsGlobalOriginOwned {
        GpsGlobalOriginOwned {
            latitude_deg: self.latitude_deg(),
            longitude_deg: self.longitude_deg(),
            altitude_m: self.altitude_m(),
        }
    }
    ///  Latitude of the origin

    pub fn latitude_deg(&self) -> f64 {
        self.inner.telemetry_get_latitude_deg()
    }
    ///  Longitude of the origin

    pub fn longitude_deg(&self) -> f64 {
        self.inner.telemetry_get_longitude_deg()
    }
    ///  Altitude AMSL (above mean sea level) in metres

    pub fn altitude_m(&self) -> f32 {
        self.inner.telemetry_get_altitude_m()
    }
}

pub struct AltitudeOwned {
    pub altitude_monotonic_m: f32,
    pub altitude_amsl_m: f32,
    pub altitude_local_m: f32,
    pub altitude_relative_m: f32,
    pub altitude_terrain_m: f32,
    pub bottom_clearance_m: f32,
}

/*  */
pub struct Altitude<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_Altitude,
}

impl<'a> Altitude<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_Altitude) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> AltitudeOwned {
        AltitudeOwned {
            altitude_monotonic_m: self.altitude_monotonic_m(),
            altitude_amsl_m: self.altitude_amsl_m(),
            altitude_local_m: self.altitude_local_m(),
            altitude_relative_m: self.altitude_relative_m(),
            altitude_terrain_m: self.altitude_terrain_m(),
            bottom_clearance_m: self.bottom_clearance_m(),
        }
    }
    ///  Altitude in meters is initialized on system boot and monotonic

    pub fn altitude_monotonic_m(&self) -> f32 {
        self.inner.telemetry_get_altitude_monotonic_m()
    }
    ///   Altitude AMSL (above mean sea level) in meters

    pub fn altitude_amsl_m(&self) -> f32 {
        self.inner.telemetry_get_altitude_amsl_m()
    }
    ///  Local altitude in meters

    pub fn altitude_local_m(&self) -> f32 {
        self.inner.telemetry_get_altitude_local_m()
    }
    ///  Altitude above home position in meters

    pub fn altitude_relative_m(&self) -> f32 {
        self.inner.telemetry_get_altitude_relative_m()
    }
    ///  Altitude above terrain in meters

    pub fn altitude_terrain_m(&self) -> f32 {
        self.inner.telemetry_get_altitude_terrain_m()
    }
    ///  This is not the altitude, but the clear space below the system according to the fused clearance estimate in meters.

    pub fn bottom_clearance_m(&self) -> f32 {
        self.inner.telemetry_get_bottom_clearance_m()
    }
}

pub struct WindOwned {
    pub wind_x_ned_m_s: f32,
    pub wind_y_ned_m_s: f32,
    pub wind_z_ned_m_s: f32,
    pub horizontal_variability_stddev_m_s: f32,
    pub vertical_variability_stddev_m_s: f32,
    pub wind_altitude_msl_m: f32,
    pub horizontal_wind_speed_accuracy_m_s: f32,
    pub vertical_wind_speed_accuracy_m_s: f32,
}

/*  */
pub struct Wind<'a> {
    inner: &'a crate::telemetry::mavsdk::Telemetry_Wind,
}

impl<'a> Wind<'a> {
    pub fn new(inner: &'a crate::telemetry::mavsdk::Telemetry_Wind) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> WindOwned {
        WindOwned {
            wind_x_ned_m_s: self.wind_x_ned_m_s(),
            wind_y_ned_m_s: self.wind_y_ned_m_s(),
            wind_z_ned_m_s: self.wind_z_ned_m_s(),
            horizontal_variability_stddev_m_s: self.horizontal_variability_stddev_m_s(),
            vertical_variability_stddev_m_s: self.vertical_variability_stddev_m_s(),
            wind_altitude_msl_m: self.wind_altitude_msl_m(),
            horizontal_wind_speed_accuracy_m_s: self.horizontal_wind_speed_accuracy_m_s(),
            vertical_wind_speed_accuracy_m_s: self.vertical_wind_speed_accuracy_m_s(),
        }
    }
    ///  Wind in North (NED) direction

    pub fn wind_x_ned_m_s(&self) -> f32 {
        self.inner.telemetry_get_wind_x_ned_m_s()
    }
    ///   Wind in East (NED) direction

    pub fn wind_y_ned_m_s(&self) -> f32 {
        self.inner.telemetry_get_wind_y_ned_m_s()
    }
    ///  Wind in down (NED) direction

    pub fn wind_z_ned_m_s(&self) -> f32 {
        self.inner.telemetry_get_wind_z_ned_m_s()
    }
    ///  Variability of wind in XY, 1-STD estimated from a 1 Hz lowpassed wind estimate

    pub fn horizontal_variability_stddev_m_s(&self) -> f32 {
        self.inner.telemetry_get_horizontal_variability_stddev_m_s()
    }
    ///  Variability of wind in Z, 1-STD estimated from a 1 Hz lowpassed wind estimate

    pub fn vertical_variability_stddev_m_s(&self) -> f32 {
        self.inner.telemetry_get_vertical_variability_stddev_m_s()
    }
    ///  Altitude (MSL) that this measurement was taken at

    pub fn wind_altitude_msl_m(&self) -> f32 {
        self.inner.telemetry_get_wind_altitude_msl_m()
    }
    ///  Horizontal speed 1-STD accuracy

    pub fn horizontal_wind_speed_accuracy_m_s(&self) -> f32 {
        self.inner
            .telemetry_get_horizontal_wind_speed_accuracy_m_s()
    }
    ///  Vertical speed 1-STD accuracy

    pub fn vertical_wind_speed_accuracy_m_s(&self) -> f32 {
        self.inner.telemetry_get_vertical_wind_speed_accuracy_m_s()
    }
}

struct TelemetryInner {
    plugin: cxx::UniquePtr<crate::telemetry::mavsdk::Telemetry>,

    /// Position State
    position_handle: Option<usize>,
    position_user_data: *mut c_void,
    /// Home State
    home_handle: Option<usize>,
    home_user_data: *mut c_void,
    /// InAir State
    in_air_handle: Option<usize>,
    in_air_user_data: *mut c_void,
    /// LandedState State
    landed_state_handle: Option<usize>,
    landed_state_user_data: *mut c_void,
    /// Armed State
    armed_handle: Option<usize>,
    armed_user_data: *mut c_void,
    /// VtolState State
    vtol_state_handle: Option<usize>,
    vtol_state_user_data: *mut c_void,
    /// AttitudeQuaternion State
    attitude_quaternion_handle: Option<usize>,
    attitude_quaternion_user_data: *mut c_void,
    /// AttitudeEuler State
    attitude_euler_handle: Option<usize>,
    attitude_euler_user_data: *mut c_void,
    /// AttitudeAngularVelocityBody State
    attitude_angular_velocity_body_handle: Option<usize>,
    attitude_angular_velocity_body_user_data: *mut c_void,
    /// VelocityNed State
    velocity_ned_handle: Option<usize>,
    velocity_ned_user_data: *mut c_void,
    /// GpsInfo State
    gps_info_handle: Option<usize>,
    gps_info_user_data: *mut c_void,
    /// RawGps State
    raw_gps_handle: Option<usize>,
    raw_gps_user_data: *mut c_void,
    /// Battery State
    battery_handle: Option<usize>,
    battery_user_data: *mut c_void,
    /// FlightMode State
    flight_mode_handle: Option<usize>,
    flight_mode_user_data: *mut c_void,
    /// Health State
    health_handle: Option<usize>,
    health_user_data: *mut c_void,
    /// RcStatus State
    rc_status_handle: Option<usize>,
    rc_status_user_data: *mut c_void,
    /// StatusText State
    status_text_handle: Option<usize>,
    status_text_user_data: *mut c_void,
    /// ActuatorControlTarget State
    actuator_control_target_handle: Option<usize>,
    actuator_control_target_user_data: *mut c_void,
    /// ActuatorOutputStatus State
    actuator_output_status_handle: Option<usize>,
    actuator_output_status_user_data: *mut c_void,
    /// Odometry State
    odometry_handle: Option<usize>,
    odometry_user_data: *mut c_void,
    /// PositionVelocityNed State
    position_velocity_ned_handle: Option<usize>,
    position_velocity_ned_user_data: *mut c_void,
    /// GroundTruth State
    ground_truth_handle: Option<usize>,
    ground_truth_user_data: *mut c_void,
    /// FixedwingMetrics State
    fixedwing_metrics_handle: Option<usize>,
    fixedwing_metrics_user_data: *mut c_void,
    /// Imu State
    imu_handle: Option<usize>,
    imu_user_data: *mut c_void,
    /// ScaledImu State
    scaled_imu_handle: Option<usize>,
    scaled_imu_user_data: *mut c_void,
    /// RawImu State
    raw_imu_handle: Option<usize>,
    raw_imu_user_data: *mut c_void,
    /// HealthAllOk State
    health_all_ok_handle: Option<usize>,
    health_all_ok_user_data: *mut c_void,
    /// UnixEpochTime State
    unix_epoch_time_handle: Option<usize>,
    unix_epoch_time_user_data: *mut c_void,
    /// DistanceSensor State
    distance_sensor_handle: Option<usize>,
    distance_sensor_user_data: *mut c_void,
    /// ScaledPressure State
    scaled_pressure_handle: Option<usize>,
    scaled_pressure_user_data: *mut c_void,
    /// Heading State
    heading_handle: Option<usize>,
    heading_user_data: *mut c_void,
    /// Altitude State
    altitude_handle: Option<usize>,
    altitude_user_data: *mut c_void,
    /// Wind State
    wind_handle: Option<usize>,
    wind_user_data: *mut c_void,
}

pub struct TelemetryClient {
    inner: Mutex<TelemetryInner>,

    // Producer for the latest Position state; broadcast to all active subscribers.
    position_tx: watch::Sender<std::option::Option<PositionOwned>>,
    // Producer for the latest Home state; broadcast to all active subscribers.
    home_tx: watch::Sender<std::option::Option<PositionOwned>>,
    // Producer for the latest InAir state; broadcast to all active subscribers.
    in_air_tx: watch::Sender<std::option::Option<bool>>,
    // Producer for the latest LandedState state; broadcast to all active subscribers.
    landed_state_tx:
        watch::Sender<std::option::Option<crate::telemetry::mavsdk::Telemetry_LandedState>>,
    // Producer for the latest Armed state; broadcast to all active subscribers.
    armed_tx: watch::Sender<std::option::Option<bool>>,
    // Producer for the latest VtolState state; broadcast to all active subscribers.
    vtol_state_tx:
        watch::Sender<std::option::Option<crate::telemetry::mavsdk::Telemetry_VtolState>>,
    // Producer for the latest AttitudeQuaternion state; broadcast to all active subscribers.
    attitude_quaternion_tx: watch::Sender<std::option::Option<QuaternionOwned>>,
    // Producer for the latest AttitudeEuler state; broadcast to all active subscribers.
    attitude_euler_tx: watch::Sender<std::option::Option<EulerAngleOwned>>,
    // Producer for the latest AttitudeAngularVelocityBody state; broadcast to all active subscribers.
    attitude_angular_velocity_body_tx: watch::Sender<std::option::Option<AngularVelocityBodyOwned>>,
    // Producer for the latest VelocityNed state; broadcast to all active subscribers.
    velocity_ned_tx: watch::Sender<std::option::Option<VelocityNedOwned>>,
    // Producer for the latest GpsInfo state; broadcast to all active subscribers.
    gps_info_tx: watch::Sender<std::option::Option<GpsInfoOwned>>,
    // Producer for the latest RawGps state; broadcast to all active subscribers.
    raw_gps_tx: watch::Sender<std::option::Option<RawGpsOwned>>,
    // Producer for the latest Battery state; broadcast to all active subscribers.
    battery_tx: watch::Sender<std::option::Option<BatteryOwned>>,
    // Producer for the latest FlightMode state; broadcast to all active subscribers.
    flight_mode_tx:
        watch::Sender<std::option::Option<crate::telemetry::mavsdk::Telemetry_FlightMode>>,
    // Producer for the latest Health state; broadcast to all active subscribers.
    health_tx: watch::Sender<std::option::Option<HealthOwned>>,
    // Producer for the latest RcStatus state; broadcast to all active subscribers.
    rc_status_tx: watch::Sender<std::option::Option<RcStatusOwned>>,
    // Producer for the latest StatusText state; broadcast to all active subscribers.
    status_text_tx: watch::Sender<std::option::Option<StatusTextOwned>>,
    // Producer for the latest ActuatorControlTarget state; broadcast to all active subscribers.
    actuator_control_target_tx: watch::Sender<std::option::Option<ActuatorControlTargetOwned>>,
    // Producer for the latest ActuatorOutputStatus state; broadcast to all active subscribers.
    actuator_output_status_tx: watch::Sender<std::option::Option<ActuatorOutputStatusOwned>>,
    // Producer for the latest Odometry state; broadcast to all active subscribers.
    odometry_tx: watch::Sender<std::option::Option<OdometryOwned>>,
    // Producer for the latest PositionVelocityNed state; broadcast to all active subscribers.
    position_velocity_ned_tx: watch::Sender<std::option::Option<PositionVelocityNedOwned>>,
    // Producer for the latest GroundTruth state; broadcast to all active subscribers.
    ground_truth_tx: watch::Sender<std::option::Option<GroundTruthOwned>>,
    // Producer for the latest FixedwingMetrics state; broadcast to all active subscribers.
    fixedwing_metrics_tx: watch::Sender<std::option::Option<FixedwingMetricsOwned>>,
    // Producer for the latest Imu state; broadcast to all active subscribers.
    imu_tx: watch::Sender<std::option::Option<ImuOwned>>,
    // Producer for the latest ScaledImu state; broadcast to all active subscribers.
    scaled_imu_tx: watch::Sender<std::option::Option<ImuOwned>>,
    // Producer for the latest RawImu state; broadcast to all active subscribers.
    raw_imu_tx: watch::Sender<std::option::Option<ImuOwned>>,
    // Producer for the latest HealthAllOk state; broadcast to all active subscribers.
    health_all_ok_tx: watch::Sender<std::option::Option<bool>>,
    // Producer for the latest UnixEpochTime state; broadcast to all active subscribers.
    unix_epoch_time_tx: watch::Sender<std::option::Option<u64>>,
    // Producer for the latest DistanceSensor state; broadcast to all active subscribers.
    distance_sensor_tx: watch::Sender<std::option::Option<DistanceSensorOwned>>,
    // Producer for the latest ScaledPressure state; broadcast to all active subscribers.
    scaled_pressure_tx: watch::Sender<std::option::Option<ScaledPressureOwned>>,
    // Producer for the latest Heading state; broadcast to all active subscribers.
    heading_tx: watch::Sender<std::option::Option<HeadingOwned>>,
    // Producer for the latest Altitude state; broadcast to all active subscribers.
    altitude_tx: watch::Sender<std::option::Option<AltitudeOwned>>,
    // Producer for the latest Wind state; broadcast to all active subscribers.
    wind_tx: watch::Sender<std::option::Option<WindOwned>>,
}

impl TelemetryClient {
    pub fn new(plugin: cxx::UniquePtr<crate::telemetry::mavsdk::Telemetry>) -> Self {
        let (position_tx, _) = watch::channel(None);
        let (home_tx, _) = watch::channel(None);
        let (in_air_tx, _) = watch::channel(None);
        let (landed_state_tx, _) = watch::channel(None);
        let (armed_tx, _) = watch::channel(None);
        let (vtol_state_tx, _) = watch::channel(None);
        let (attitude_quaternion_tx, _) = watch::channel(None);
        let (attitude_euler_tx, _) = watch::channel(None);
        let (attitude_angular_velocity_body_tx, _) = watch::channel(None);
        let (velocity_ned_tx, _) = watch::channel(None);
        let (gps_info_tx, _) = watch::channel(None);
        let (raw_gps_tx, _) = watch::channel(None);
        let (battery_tx, _) = watch::channel(None);
        let (flight_mode_tx, _) = watch::channel(None);
        let (health_tx, _) = watch::channel(None);
        let (rc_status_tx, _) = watch::channel(None);
        let (status_text_tx, _) = watch::channel(None);
        let (actuator_control_target_tx, _) = watch::channel(None);
        let (actuator_output_status_tx, _) = watch::channel(None);
        let (odometry_tx, _) = watch::channel(None);
        let (position_velocity_ned_tx, _) = watch::channel(None);
        let (ground_truth_tx, _) = watch::channel(None);
        let (fixedwing_metrics_tx, _) = watch::channel(None);
        let (imu_tx, _) = watch::channel(None);
        let (scaled_imu_tx, _) = watch::channel(None);
        let (raw_imu_tx, _) = watch::channel(None);
        let (health_all_ok_tx, _) = watch::channel(None);
        let (unix_epoch_time_tx, _) = watch::channel(None);
        let (distance_sensor_tx, _) = watch::channel(None);
        let (scaled_pressure_tx, _) = watch::channel(None);
        let (heading_tx, _) = watch::channel(None);
        let (altitude_tx, _) = watch::channel(None);
        let (wind_tx, _) = watch::channel(None);

        Self {
            inner: Mutex::new(TelemetryInner {
                plugin,

                position_handle: None,
                position_user_data: std::ptr::null_mut(),
                home_handle: None,
                home_user_data: std::ptr::null_mut(),
                in_air_handle: None,
                in_air_user_data: std::ptr::null_mut(),
                landed_state_handle: None,
                landed_state_user_data: std::ptr::null_mut(),
                armed_handle: None,
                armed_user_data: std::ptr::null_mut(),
                vtol_state_handle: None,
                vtol_state_user_data: std::ptr::null_mut(),
                attitude_quaternion_handle: None,
                attitude_quaternion_user_data: std::ptr::null_mut(),
                attitude_euler_handle: None,
                attitude_euler_user_data: std::ptr::null_mut(),
                attitude_angular_velocity_body_handle: None,
                attitude_angular_velocity_body_user_data: std::ptr::null_mut(),
                velocity_ned_handle: None,
                velocity_ned_user_data: std::ptr::null_mut(),
                gps_info_handle: None,
                gps_info_user_data: std::ptr::null_mut(),
                raw_gps_handle: None,
                raw_gps_user_data: std::ptr::null_mut(),
                battery_handle: None,
                battery_user_data: std::ptr::null_mut(),
                flight_mode_handle: None,
                flight_mode_user_data: std::ptr::null_mut(),
                health_handle: None,
                health_user_data: std::ptr::null_mut(),
                rc_status_handle: None,
                rc_status_user_data: std::ptr::null_mut(),
                status_text_handle: None,
                status_text_user_data: std::ptr::null_mut(),
                actuator_control_target_handle: None,
                actuator_control_target_user_data: std::ptr::null_mut(),
                actuator_output_status_handle: None,
                actuator_output_status_user_data: std::ptr::null_mut(),
                odometry_handle: None,
                odometry_user_data: std::ptr::null_mut(),
                position_velocity_ned_handle: None,
                position_velocity_ned_user_data: std::ptr::null_mut(),
                ground_truth_handle: None,
                ground_truth_user_data: std::ptr::null_mut(),
                fixedwing_metrics_handle: None,
                fixedwing_metrics_user_data: std::ptr::null_mut(),
                imu_handle: None,
                imu_user_data: std::ptr::null_mut(),
                scaled_imu_handle: None,
                scaled_imu_user_data: std::ptr::null_mut(),
                raw_imu_handle: None,
                raw_imu_user_data: std::ptr::null_mut(),
                health_all_ok_handle: None,
                health_all_ok_user_data: std::ptr::null_mut(),
                unix_epoch_time_handle: None,
                unix_epoch_time_user_data: std::ptr::null_mut(),
                distance_sensor_handle: None,
                distance_sensor_user_data: std::ptr::null_mut(),
                scaled_pressure_handle: None,
                scaled_pressure_user_data: std::ptr::null_mut(),
                heading_handle: None,
                heading_user_data: std::ptr::null_mut(),
                altitude_handle: None,
                altitude_user_data: std::ptr::null_mut(),
                wind_handle: None,
                wind_user_data: std::ptr::null_mut(),
            }),

            position_tx,
            home_tx,
            in_air_tx,
            landed_state_tx,
            armed_tx,
            vtol_state_tx,
            attitude_quaternion_tx,
            attitude_euler_tx,
            attitude_angular_velocity_body_tx,
            velocity_ned_tx,
            gps_info_tx,
            raw_gps_tx,
            battery_tx,
            flight_mode_tx,
            health_tx,
            rc_status_tx,
            status_text_tx,
            actuator_control_target_tx,
            actuator_output_status_tx,
            odometry_tx,
            position_velocity_ned_tx,
            ground_truth_tx,
            fixedwing_metrics_tx,
            imu_tx,
            scaled_imu_tx,
            raw_imu_tx,
            health_all_ok_tx,
            unix_epoch_time_tx,
            distance_sensor_tx,
            scaled_pressure_tx,
            heading_tx,
            altitude_tx,
            wind_tx,
        }
    }

    pub fn subscribe_position(&self) -> watch::Receiver<std::option::Option<PositionOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.position_handle.is_none() {
            // Lazy initialization of C++ 'position' telemetry stream.
            let user_data_ptr = Box::into_raw(Box::new(self.position_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_position(
                telemetry_position_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.position_handle = Some(handle);
            inner.position_user_data = user_data_ptr;
        }

        self.position_tx.subscribe()
    }
    pub fn subscribe_home(&self) -> watch::Receiver<std::option::Option<PositionOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.home_handle.is_none() {
            // Lazy initialization of C++ 'home' telemetry stream.
            let user_data_ptr = Box::into_raw(Box::new(self.home_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_home(
                telemetry_home_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.home_handle = Some(handle);
            inner.home_user_data = user_data_ptr;
        }

        self.home_tx.subscribe()
    }
    pub fn subscribe_in_air(&self) -> watch::Receiver<std::option::Option<bool>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.in_air_handle.is_none() {
            // Lazy initialization of C++ 'in_air' telemetry stream.
            let user_data_ptr = Box::into_raw(Box::new(self.in_air_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_in_air(
                telemetry_in_air_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.in_air_handle = Some(handle);
            inner.in_air_user_data = user_data_ptr;
        }

        self.in_air_tx.subscribe()
    }
    pub fn subscribe_landed_state(
        &self,
    ) -> watch::Receiver<std::option::Option<crate::telemetry::mavsdk::Telemetry_LandedState>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.landed_state_handle.is_none() {
            // Lazy initialization of C++ 'landed_state' telemetry stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.landed_state_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_landed_state(
                telemetry_landed_state_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.landed_state_handle = Some(handle);
            inner.landed_state_user_data = user_data_ptr;
        }

        self.landed_state_tx.subscribe()
    }
    pub fn subscribe_armed(&self) -> watch::Receiver<std::option::Option<bool>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.armed_handle.is_none() {
            // Lazy initialization of C++ 'armed' telemetry stream.
            let user_data_ptr = Box::into_raw(Box::new(self.armed_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_armed(
                telemetry_armed_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.armed_handle = Some(handle);
            inner.armed_user_data = user_data_ptr;
        }

        self.armed_tx.subscribe()
    }
    pub fn subscribe_vtol_state(
        &self,
    ) -> watch::Receiver<std::option::Option<crate::telemetry::mavsdk::Telemetry_VtolState>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.vtol_state_handle.is_none() {
            // Lazy initialization of C++ 'vtol_state' telemetry stream.
            let user_data_ptr = Box::into_raw(Box::new(self.vtol_state_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_vtol_state(
                telemetry_vtol_state_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.vtol_state_handle = Some(handle);
            inner.vtol_state_user_data = user_data_ptr;
        }

        self.vtol_state_tx.subscribe()
    }
    pub fn subscribe_attitude_quaternion(
        &self,
    ) -> watch::Receiver<std::option::Option<QuaternionOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.attitude_quaternion_handle.is_none() {
            // Lazy initialization of C++ 'attitude_quaternion' telemetry stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.attitude_quaternion_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_attitude_quaternion(
                telemetry_attitude_quaternion_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.attitude_quaternion_handle = Some(handle);
            inner.attitude_quaternion_user_data = user_data_ptr;
        }

        self.attitude_quaternion_tx.subscribe()
    }
    pub fn subscribe_attitude_euler(
        &self,
    ) -> watch::Receiver<std::option::Option<EulerAngleOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.attitude_euler_handle.is_none() {
            // Lazy initialization of C++ 'attitude_euler' telemetry stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.attitude_euler_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_attitude_euler(
                telemetry_attitude_euler_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.attitude_euler_handle = Some(handle);
            inner.attitude_euler_user_data = user_data_ptr;
        }

        self.attitude_euler_tx.subscribe()
    }
    pub fn subscribe_attitude_angular_velocity_body(
        &self,
    ) -> watch::Receiver<std::option::Option<AngularVelocityBodyOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.attitude_angular_velocity_body_handle.is_none() {
            // Lazy initialization of C++ 'attitude_angular_velocity_body' telemetry stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.attitude_angular_velocity_body_tx.clone()))
                    as *mut c_void;

            let handle = inner
                .plugin
                .pin_mut()
                .subscribe_attitude_angular_velocity_body(
                    telemetry_attitude_angular_velocity_body_callback_ffi as usize
                        as libc::uintptr_t,
                    user_data_ptr as libc::uintptr_t,
                ) as usize;

            inner.attitude_angular_velocity_body_handle = Some(handle);
            inner.attitude_angular_velocity_body_user_data = user_data_ptr;
        }

        self.attitude_angular_velocity_body_tx.subscribe()
    }
    pub fn subscribe_velocity_ned(&self) -> watch::Receiver<std::option::Option<VelocityNedOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.velocity_ned_handle.is_none() {
            // Lazy initialization of C++ 'velocity_ned' telemetry stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.velocity_ned_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_velocity_ned(
                telemetry_velocity_ned_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.velocity_ned_handle = Some(handle);
            inner.velocity_ned_user_data = user_data_ptr;
        }

        self.velocity_ned_tx.subscribe()
    }
    pub fn subscribe_gps_info(&self) -> watch::Receiver<std::option::Option<GpsInfoOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.gps_info_handle.is_none() {
            // Lazy initialization of C++ 'gps_info' telemetry stream.
            let user_data_ptr = Box::into_raw(Box::new(self.gps_info_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_gps_info(
                telemetry_gps_info_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.gps_info_handle = Some(handle);
            inner.gps_info_user_data = user_data_ptr;
        }

        self.gps_info_tx.subscribe()
    }
    pub fn subscribe_raw_gps(&self) -> watch::Receiver<std::option::Option<RawGpsOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.raw_gps_handle.is_none() {
            // Lazy initialization of C++ 'raw_gps' telemetry stream.
            let user_data_ptr = Box::into_raw(Box::new(self.raw_gps_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_raw_gps(
                telemetry_raw_gps_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.raw_gps_handle = Some(handle);
            inner.raw_gps_user_data = user_data_ptr;
        }

        self.raw_gps_tx.subscribe()
    }
    pub fn subscribe_battery(&self) -> watch::Receiver<std::option::Option<BatteryOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.battery_handle.is_none() {
            // Lazy initialization of C++ 'battery' telemetry stream.
            let user_data_ptr = Box::into_raw(Box::new(self.battery_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_battery(
                telemetry_battery_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.battery_handle = Some(handle);
            inner.battery_user_data = user_data_ptr;
        }

        self.battery_tx.subscribe()
    }
    pub fn subscribe_flight_mode(
        &self,
    ) -> watch::Receiver<std::option::Option<crate::telemetry::mavsdk::Telemetry_FlightMode>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.flight_mode_handle.is_none() {
            // Lazy initialization of C++ 'flight_mode' telemetry stream.
            let user_data_ptr = Box::into_raw(Box::new(self.flight_mode_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_flight_mode(
                telemetry_flight_mode_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.flight_mode_handle = Some(handle);
            inner.flight_mode_user_data = user_data_ptr;
        }

        self.flight_mode_tx.subscribe()
    }
    pub fn subscribe_health(&self) -> watch::Receiver<std::option::Option<HealthOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.health_handle.is_none() {
            // Lazy initialization of C++ 'health' telemetry stream.
            let user_data_ptr = Box::into_raw(Box::new(self.health_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_health(
                telemetry_health_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.health_handle = Some(handle);
            inner.health_user_data = user_data_ptr;
        }

        self.health_tx.subscribe()
    }
    pub fn subscribe_rc_status(&self) -> watch::Receiver<std::option::Option<RcStatusOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.rc_status_handle.is_none() {
            // Lazy initialization of C++ 'rc_status' telemetry stream.
            let user_data_ptr = Box::into_raw(Box::new(self.rc_status_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_rc_status(
                telemetry_rc_status_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.rc_status_handle = Some(handle);
            inner.rc_status_user_data = user_data_ptr;
        }

        self.rc_status_tx.subscribe()
    }
    pub fn subscribe_status_text(&self) -> watch::Receiver<std::option::Option<StatusTextOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.status_text_handle.is_none() {
            // Lazy initialization of C++ 'status_text' telemetry stream.
            let user_data_ptr = Box::into_raw(Box::new(self.status_text_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_status_text(
                telemetry_status_text_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.status_text_handle = Some(handle);
            inner.status_text_user_data = user_data_ptr;
        }

        self.status_text_tx.subscribe()
    }
    pub fn subscribe_actuator_control_target(
        &self,
    ) -> watch::Receiver<std::option::Option<ActuatorControlTargetOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.actuator_control_target_handle.is_none() {
            // Lazy initialization of C++ 'actuator_control_target' telemetry stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.actuator_control_target_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_actuator_control_target(
                telemetry_actuator_control_target_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.actuator_control_target_handle = Some(handle);
            inner.actuator_control_target_user_data = user_data_ptr;
        }

        self.actuator_control_target_tx.subscribe()
    }
    pub fn subscribe_actuator_output_status(
        &self,
    ) -> watch::Receiver<std::option::Option<ActuatorOutputStatusOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.actuator_output_status_handle.is_none() {
            // Lazy initialization of C++ 'actuator_output_status' telemetry stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.actuator_output_status_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_actuator_output_status(
                telemetry_actuator_output_status_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.actuator_output_status_handle = Some(handle);
            inner.actuator_output_status_user_data = user_data_ptr;
        }

        self.actuator_output_status_tx.subscribe()
    }
    pub fn subscribe_odometry(&self) -> watch::Receiver<std::option::Option<OdometryOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.odometry_handle.is_none() {
            // Lazy initialization of C++ 'odometry' telemetry stream.
            let user_data_ptr = Box::into_raw(Box::new(self.odometry_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_odometry(
                telemetry_odometry_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.odometry_handle = Some(handle);
            inner.odometry_user_data = user_data_ptr;
        }

        self.odometry_tx.subscribe()
    }
    pub fn subscribe_position_velocity_ned(
        &self,
    ) -> watch::Receiver<std::option::Option<PositionVelocityNedOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.position_velocity_ned_handle.is_none() {
            // Lazy initialization of C++ 'position_velocity_ned' telemetry stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.position_velocity_ned_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_position_velocity_ned(
                telemetry_position_velocity_ned_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.position_velocity_ned_handle = Some(handle);
            inner.position_velocity_ned_user_data = user_data_ptr;
        }

        self.position_velocity_ned_tx.subscribe()
    }
    pub fn subscribe_ground_truth(&self) -> watch::Receiver<std::option::Option<GroundTruthOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.ground_truth_handle.is_none() {
            // Lazy initialization of C++ 'ground_truth' telemetry stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.ground_truth_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_ground_truth(
                telemetry_ground_truth_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.ground_truth_handle = Some(handle);
            inner.ground_truth_user_data = user_data_ptr;
        }

        self.ground_truth_tx.subscribe()
    }
    pub fn subscribe_fixedwing_metrics(
        &self,
    ) -> watch::Receiver<std::option::Option<FixedwingMetricsOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.fixedwing_metrics_handle.is_none() {
            // Lazy initialization of C++ 'fixedwing_metrics' telemetry stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.fixedwing_metrics_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_fixedwing_metrics(
                telemetry_fixedwing_metrics_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.fixedwing_metrics_handle = Some(handle);
            inner.fixedwing_metrics_user_data = user_data_ptr;
        }

        self.fixedwing_metrics_tx.subscribe()
    }
    pub fn subscribe_imu(&self) -> watch::Receiver<std::option::Option<ImuOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.imu_handle.is_none() {
            // Lazy initialization of C++ 'imu' telemetry stream.
            let user_data_ptr = Box::into_raw(Box::new(self.imu_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_imu(
                telemetry_imu_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.imu_handle = Some(handle);
            inner.imu_user_data = user_data_ptr;
        }

        self.imu_tx.subscribe()
    }
    pub fn subscribe_scaled_imu(&self) -> watch::Receiver<std::option::Option<ImuOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.scaled_imu_handle.is_none() {
            // Lazy initialization of C++ 'scaled_imu' telemetry stream.
            let user_data_ptr = Box::into_raw(Box::new(self.scaled_imu_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_scaled_imu(
                telemetry_scaled_imu_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.scaled_imu_handle = Some(handle);
            inner.scaled_imu_user_data = user_data_ptr;
        }

        self.scaled_imu_tx.subscribe()
    }
    pub fn subscribe_raw_imu(&self) -> watch::Receiver<std::option::Option<ImuOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.raw_imu_handle.is_none() {
            // Lazy initialization of C++ 'raw_imu' telemetry stream.
            let user_data_ptr = Box::into_raw(Box::new(self.raw_imu_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_raw_imu(
                telemetry_raw_imu_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.raw_imu_handle = Some(handle);
            inner.raw_imu_user_data = user_data_ptr;
        }

        self.raw_imu_tx.subscribe()
    }
    pub fn subscribe_health_all_ok(&self) -> watch::Receiver<std::option::Option<bool>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.health_all_ok_handle.is_none() {
            // Lazy initialization of C++ 'health_all_ok' telemetry stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.health_all_ok_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_health_all_ok(
                telemetry_health_all_ok_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.health_all_ok_handle = Some(handle);
            inner.health_all_ok_user_data = user_data_ptr;
        }

        self.health_all_ok_tx.subscribe()
    }
    pub fn subscribe_unix_epoch_time(&self) -> watch::Receiver<std::option::Option<u64>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.unix_epoch_time_handle.is_none() {
            // Lazy initialization of C++ 'unix_epoch_time' telemetry stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.unix_epoch_time_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_unix_epoch_time(
                telemetry_unix_epoch_time_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.unix_epoch_time_handle = Some(handle);
            inner.unix_epoch_time_user_data = user_data_ptr;
        }

        self.unix_epoch_time_tx.subscribe()
    }
    pub fn subscribe_distance_sensor(
        &self,
    ) -> watch::Receiver<std::option::Option<DistanceSensorOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.distance_sensor_handle.is_none() {
            // Lazy initialization of C++ 'distance_sensor' telemetry stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.distance_sensor_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_distance_sensor(
                telemetry_distance_sensor_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.distance_sensor_handle = Some(handle);
            inner.distance_sensor_user_data = user_data_ptr;
        }

        self.distance_sensor_tx.subscribe()
    }
    pub fn subscribe_scaled_pressure(
        &self,
    ) -> watch::Receiver<std::option::Option<ScaledPressureOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.scaled_pressure_handle.is_none() {
            // Lazy initialization of C++ 'scaled_pressure' telemetry stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.scaled_pressure_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_scaled_pressure(
                telemetry_scaled_pressure_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.scaled_pressure_handle = Some(handle);
            inner.scaled_pressure_user_data = user_data_ptr;
        }

        self.scaled_pressure_tx.subscribe()
    }
    pub fn subscribe_heading(&self) -> watch::Receiver<std::option::Option<HeadingOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.heading_handle.is_none() {
            // Lazy initialization of C++ 'heading' telemetry stream.
            let user_data_ptr = Box::into_raw(Box::new(self.heading_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_heading(
                telemetry_heading_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.heading_handle = Some(handle);
            inner.heading_user_data = user_data_ptr;
        }

        self.heading_tx.subscribe()
    }
    pub fn subscribe_altitude(&self) -> watch::Receiver<std::option::Option<AltitudeOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.altitude_handle.is_none() {
            // Lazy initialization of C++ 'altitude' telemetry stream.
            let user_data_ptr = Box::into_raw(Box::new(self.altitude_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_altitude(
                telemetry_altitude_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.altitude_handle = Some(handle);
            inner.altitude_user_data = user_data_ptr;
        }

        self.altitude_tx.subscribe()
    }
    pub fn subscribe_wind(&self) -> watch::Receiver<std::option::Option<WindOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.wind_handle.is_none() {
            // Lazy initialization of C++ 'wind' telemetry stream.
            let user_data_ptr = Box::into_raw(Box::new(self.wind_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_wind(
                telemetry_wind_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.wind_handle = Some(handle);
            inner.wind_user_data = user_data_ptr;
        }

        self.wind_tx.subscribe()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn telemetry_position_callback_ffi(
    user_data: *mut c_void,
    position: &crate::telemetry::mavsdk::Telemetry_Position,
) {
    if user_data.is_null() {
        return;
    }
    let send_position = Position::new(position).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<PositionOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_position));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_home_callback_ffi(
    user_data: *mut c_void,
    home: &crate::telemetry::mavsdk::Telemetry_Position,
) {
    if user_data.is_null() {
        return;
    }
    let send_home = Position::new(home).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<PositionOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_home));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_in_air_callback_ffi(user_data: *mut c_void, in_air: bool) {
    if user_data.is_null() {
        return;
    }
    let send_in_air = in_air.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<bool>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_in_air));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_landed_state_callback_ffi(
    user_data: *mut c_void,
    landed_state: &crate::telemetry::mavsdk::Telemetry_LandedState,
) {
    if user_data.is_null() {
        return;
    }
    let send_landed_state = landed_state.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe {
        &*(user_data
            as *const watch::Sender<
                std::option::Option<crate::telemetry::mavsdk::Telemetry_LandedState>,
            >)
    };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_landed_state));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_armed_callback_ffi(user_data: *mut c_void, armed: bool) {
    if user_data.is_null() {
        return;
    }
    let send_armed = armed.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<bool>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_armed));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_vtol_state_callback_ffi(
    user_data: *mut c_void,
    vtol_state: &crate::telemetry::mavsdk::Telemetry_VtolState,
) {
    if user_data.is_null() {
        return;
    }
    let send_vtol_state = vtol_state.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe {
        &*(user_data
            as *const watch::Sender<
                std::option::Option<crate::telemetry::mavsdk::Telemetry_VtolState>,
            >)
    };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_vtol_state));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_attitude_quaternion_callback_ffi(
    user_data: *mut c_void,
    attitude_quaternion: &crate::telemetry::mavsdk::Telemetry_Quaternion,
) {
    if user_data.is_null() {
        return;
    }
    let send_attitude_quaternion = Quaternion::new(attitude_quaternion).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<QuaternionOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_attitude_quaternion));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_attitude_euler_callback_ffi(
    user_data: *mut c_void,
    attitude_euler: &crate::telemetry::mavsdk::Telemetry_EulerAngle,
) {
    if user_data.is_null() {
        return;
    }
    let send_attitude_euler = EulerAngle::new(attitude_euler).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<EulerAngleOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_attitude_euler));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_attitude_angular_velocity_body_callback_ffi(
    user_data: *mut c_void,
    attitude_angular_velocity_body: &crate::telemetry::mavsdk::Telemetry_AngularVelocityBody,
) {
    if user_data.is_null() {
        return;
    }
    let send_attitude_angular_velocity_body =
        AngularVelocityBody::new(attitude_angular_velocity_body).into_owned();

    // Cast the pointer to watch::Sender
    let sender = unsafe {
        &*(user_data as *const watch::Sender<std::option::Option<AngularVelocityBodyOwned>>)
    };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_attitude_angular_velocity_body));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_velocity_ned_callback_ffi(
    user_data: *mut c_void,
    velocity_ned: &crate::telemetry::mavsdk::Telemetry_VelocityNed,
) {
    if user_data.is_null() {
        return;
    }
    let send_velocity_ned = VelocityNed::new(velocity_ned).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<VelocityNedOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_velocity_ned));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_gps_info_callback_ffi(
    user_data: *mut c_void,
    gps_info: &crate::telemetry::mavsdk::Telemetry_GpsInfo,
) {
    if user_data.is_null() {
        return;
    }
    let send_gps_info = GpsInfo::new(gps_info).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<GpsInfoOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_gps_info));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_raw_gps_callback_ffi(
    user_data: *mut c_void,
    raw_gps: &crate::telemetry::mavsdk::Telemetry_RawGps,
) {
    if user_data.is_null() {
        return;
    }
    let send_raw_gps = RawGps::new(raw_gps).into_owned();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<RawGpsOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_raw_gps));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_battery_callback_ffi(
    user_data: *mut c_void,
    battery: &crate::telemetry::mavsdk::Telemetry_Battery,
) {
    if user_data.is_null() {
        return;
    }
    let send_battery = Battery::new(battery).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<BatteryOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_battery));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_flight_mode_callback_ffi(
    user_data: *mut c_void,
    flight_mode: &crate::telemetry::mavsdk::Telemetry_FlightMode,
) {
    if user_data.is_null() {
        return;
    }
    let send_flight_mode = flight_mode.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe {
        &*(user_data
            as *const watch::Sender<
                std::option::Option<crate::telemetry::mavsdk::Telemetry_FlightMode>,
            >)
    };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_flight_mode));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_health_callback_ffi(
    user_data: *mut c_void,
    health: &crate::telemetry::mavsdk::Telemetry_Health,
) {
    if user_data.is_null() {
        return;
    }
    let send_health = Health::new(health).into_owned();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<HealthOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_health));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_rc_status_callback_ffi(
    user_data: *mut c_void,
    rc_status: &crate::telemetry::mavsdk::Telemetry_RcStatus,
) {
    if user_data.is_null() {
        return;
    }
    let send_rc_status = RcStatus::new(rc_status).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<RcStatusOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_rc_status));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_status_text_callback_ffi(
    user_data: *mut c_void,
    status_text: &crate::telemetry::mavsdk::Telemetry_StatusText,
) {
    if user_data.is_null() {
        return;
    }
    let send_status_text = StatusText::new(status_text).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<StatusTextOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_status_text));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_actuator_control_target_callback_ffi(
    user_data: *mut c_void,
    actuator_control_target: &crate::telemetry::mavsdk::Telemetry_ActuatorControlTarget,
) {
    if user_data.is_null() {
        return;
    }
    let send_actuator_control_target =
        ActuatorControlTarget::new(actuator_control_target).into_owned();

    // Cast the pointer to watch::Sender
    let sender = unsafe {
        &*(user_data as *const watch::Sender<std::option::Option<ActuatorControlTargetOwned>>)
    };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_actuator_control_target));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_actuator_output_status_callback_ffi(
    user_data: *mut c_void,
    actuator_output_status: &crate::telemetry::mavsdk::Telemetry_ActuatorOutputStatus,
) {
    if user_data.is_null() {
        return;
    }
    let send_actuator_output_status =
        ActuatorOutputStatus::new(actuator_output_status).into_owned();

    // Cast the pointer to watch::Sender
    let sender = unsafe {
        &*(user_data as *const watch::Sender<std::option::Option<ActuatorOutputStatusOwned>>)
    };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_actuator_output_status));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_odometry_callback_ffi(
    user_data: *mut c_void,
    odometry: &crate::telemetry::mavsdk::Telemetry_Odometry,
) {
    if user_data.is_null() {
        return;
    }
    let send_odometry = Odometry::new(odometry).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<OdometryOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_odometry));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_position_velocity_ned_callback_ffi(
    user_data: *mut c_void,
    position_velocity_ned: &crate::telemetry::mavsdk::Telemetry_PositionVelocityNed,
) {
    if user_data.is_null() {
        return;
    }
    let send_position_velocity_ned = PositionVelocityNed::new(position_velocity_ned).into_owned();

    // Cast the pointer to watch::Sender
    let sender = unsafe {
        &*(user_data as *const watch::Sender<std::option::Option<PositionVelocityNedOwned>>)
    };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_position_velocity_ned));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_ground_truth_callback_ffi(
    user_data: *mut c_void,
    ground_truth: &crate::telemetry::mavsdk::Telemetry_GroundTruth,
) {
    if user_data.is_null() {
        return;
    }
    let send_ground_truth = GroundTruth::new(ground_truth).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<GroundTruthOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_ground_truth));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_fixedwing_metrics_callback_ffi(
    user_data: *mut c_void,
    fixedwing_metrics: &crate::telemetry::mavsdk::Telemetry_FixedwingMetrics,
) {
    if user_data.is_null() {
        return;
    }
    let send_fixedwing_metrics = FixedwingMetrics::new(fixedwing_metrics).into_owned();

    // Cast the pointer to watch::Sender
    let sender = unsafe {
        &*(user_data as *const watch::Sender<std::option::Option<FixedwingMetricsOwned>>)
    };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_fixedwing_metrics));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_imu_callback_ffi(
    user_data: *mut c_void,
    imu: &crate::telemetry::mavsdk::Telemetry_Imu,
) {
    if user_data.is_null() {
        return;
    }
    let send_imu = Imu::new(imu).into_owned();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<ImuOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_imu));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_scaled_imu_callback_ffi(
    user_data: *mut c_void,
    scaled_imu: &crate::telemetry::mavsdk::Telemetry_Imu,
) {
    if user_data.is_null() {
        return;
    }
    let send_scaled_imu = Imu::new(scaled_imu).into_owned();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<ImuOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_scaled_imu));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_raw_imu_callback_ffi(
    user_data: *mut c_void,
    raw_imu: &crate::telemetry::mavsdk::Telemetry_Imu,
) {
    if user_data.is_null() {
        return;
    }
    let send_raw_imu = Imu::new(raw_imu).into_owned();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<ImuOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_raw_imu));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_health_all_ok_callback_ffi(
    user_data: *mut c_void,
    health_all_ok: bool,
) {
    if user_data.is_null() {
        return;
    }
    let send_health_all_ok = health_all_ok.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<bool>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_health_all_ok));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_unix_epoch_time_callback_ffi(
    user_data: *mut c_void,
    unix_epoch_time: u64,
) {
    if user_data.is_null() {
        return;
    }
    let send_unix_epoch_time = unix_epoch_time.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<u64>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_unix_epoch_time));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_distance_sensor_callback_ffi(
    user_data: *mut c_void,
    distance_sensor: &crate::telemetry::mavsdk::Telemetry_DistanceSensor,
) {
    if user_data.is_null() {
        return;
    }
    let send_distance_sensor = DistanceSensor::new(distance_sensor).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<DistanceSensorOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_distance_sensor));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_scaled_pressure_callback_ffi(
    user_data: *mut c_void,
    scaled_pressure: &crate::telemetry::mavsdk::Telemetry_ScaledPressure,
) {
    if user_data.is_null() {
        return;
    }
    let send_scaled_pressure = ScaledPressure::new(scaled_pressure).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<ScaledPressureOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_scaled_pressure));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_heading_callback_ffi(
    user_data: *mut c_void,
    heading: &crate::telemetry::mavsdk::Telemetry_Heading,
) {
    if user_data.is_null() {
        return;
    }
    let send_heading = Heading::new(heading).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<HeadingOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_heading));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_altitude_callback_ffi(
    user_data: *mut c_void,
    altitude: &crate::telemetry::mavsdk::Telemetry_Altitude,
) {
    if user_data.is_null() {
        return;
    }
    let send_altitude = Altitude::new(altitude).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<AltitudeOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_altitude));
}
#[unsafe(no_mangle)]
pub extern "C" fn telemetry_wind_callback_ffi(
    user_data: *mut c_void,
    wind: &crate::telemetry::mavsdk::Telemetry_Wind,
) {
    if user_data.is_null() {
        return;
    }
    let send_wind = Wind::new(wind).into_owned();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<WindOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_wind));
}
