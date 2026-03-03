// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use tokio::sync::watch;
use std::sync::Mutex;






pub struct PositionOwned {
        pub latitude_deg: f64,
        pub longitude_deg: f64,
        pub absolute_altitude_m: f32,
        pub relative_altitude_m: f32,
}

/*  */
pub struct Position<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_Position,
}

impl<'a> Position<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_Position) -> Self {
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
        self.inner.telemetry_server_get_latitude_deg()
    }
        ///  Longitude in degrees (range: -180 to +180)

    pub fn longitude_deg(&self) -> f64 {
        self.inner.telemetry_server_get_longitude_deg()
    }
        ///  Altitude AMSL (above mean sea level) in metres

    pub fn absolute_altitude_m(&self) -> f32 {
        self.inner.telemetry_server_get_absolute_altitude_m()
    }
        ///  Altitude relative to takeoff altitude in metres

    pub fn relative_altitude_m(&self) -> f32 {
        self.inner.telemetry_server_get_relative_altitude_m()
    }
}


pub struct HeadingOwned {
        pub heading_deg: f64,
}

/*  */
pub struct Heading<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_Heading,
}

impl<'a> Heading<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_Heading) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> HeadingOwned {
        HeadingOwned {
            
            heading_deg: self.heading_deg(),
        }
    }
        ///  Heading in degrees (range: 0 to +360)

    pub fn heading_deg(&self) -> f64 {
        self.inner.telemetry_server_get_heading_deg()
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
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_Quaternion,
}

impl<'a> Quaternion<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_Quaternion) -> Self {
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
        self.inner.telemetry_server_get_w()
    }
        ///  Quaternion entry 1, also denoted as b

    pub fn x(&self) -> f32 {
        self.inner.telemetry_server_get_x()
    }
        ///  Quaternion entry 2, also denoted as c

    pub fn y(&self) -> f32 {
        self.inner.telemetry_server_get_y()
    }
        ///  Quaternion entry 3, also denoted as d

    pub fn z(&self) -> f32 {
        self.inner.telemetry_server_get_z()
    }
        ///  Timestamp in microseconds

    pub fn timestamp_us(&self) -> u64 {
        self.inner.telemetry_server_get_timestamp_us()
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
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_EulerAngle,
}

impl<'a> EulerAngle<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_EulerAngle) -> Self {
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
        self.inner.telemetry_server_get_roll_deg()
    }
        ///  Pitch angle in degrees, positive is pitching nose up

    pub fn pitch_deg(&self) -> f32 {
        self.inner.telemetry_server_get_pitch_deg()
    }
        ///  Yaw angle in degrees, positive is clock-wise seen from above

    pub fn yaw_deg(&self) -> f32 {
        self.inner.telemetry_server_get_yaw_deg()
    }
        ///  Timestamp in microseconds

    pub fn timestamp_us(&self) -> u64 {
        self.inner.telemetry_server_get_timestamp_us()
    }
}


pub struct AngularVelocityBodyOwned {
        pub roll_rad_s: f32,
        pub pitch_rad_s: f32,
        pub yaw_rad_s: f32,
}

/*  */
pub struct AngularVelocityBody<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_AngularVelocityBody,
}

impl<'a> AngularVelocityBody<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_AngularVelocityBody) -> Self {
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
        self.inner.telemetry_server_get_roll_rad_s()
    }
        ///  Pitch angular velocity

    pub fn pitch_rad_s(&self) -> f32 {
        self.inner.telemetry_server_get_pitch_rad_s()
    }
        ///  Yaw angular velocity

    pub fn yaw_rad_s(&self) -> f32 {
        self.inner.telemetry_server_get_yaw_rad_s()
    }
}


pub struct GpsInfoOwned {
        pub num_satellites: i32,
        pub fix_type: crate::telemetry_server::mavsdk::TelemetryServer_FixType,
}

/*  */
pub struct GpsInfo<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_GpsInfo,
}

impl<'a> GpsInfo<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_GpsInfo) -> Self {
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
        self.inner.telemetry_server_get_num_satellites()
    }
        ///  Fix type

    pub fn fix_type(&self) -> crate::telemetry_server::mavsdk::TelemetryServer_FixType {self.inner.telemetry_server_get_fix_type().clone()
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
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_RawGps,
}

impl<'a> RawGps<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_RawGps) -> Self {
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
        self.inner.telemetry_server_get_timestamp_us()
    }
        ///  Latitude in degrees (WGS84, EGM96 ellipsoid)

    pub fn latitude_deg(&self) -> f64 {
        self.inner.telemetry_server_get_latitude_deg()
    }
        ///  Longitude in degrees (WGS84, EGM96 ellipsoid)

    pub fn longitude_deg(&self) -> f64 {
        self.inner.telemetry_server_get_longitude_deg()
    }
        ///  Altitude AMSL (above mean sea level) in metres

    pub fn absolute_altitude_m(&self) -> f32 {
        self.inner.telemetry_server_get_absolute_altitude_m()
    }
        ///  GPS HDOP horizontal dilution of position (unitless). If unknown, set to NaN

    pub fn hdop(&self) -> f32 {
        self.inner.telemetry_server_get_hdop()
    }
        ///  GPS VDOP vertical dilution of position (unitless). If unknown, set to NaN

    pub fn vdop(&self) -> f32 {
        self.inner.telemetry_server_get_vdop()
    }
        ///  Ground velocity in metres per second

    pub fn velocity_m_s(&self) -> f32 {
        self.inner.telemetry_server_get_velocity_m_s()
    }
        ///  Course over ground (NOT heading, but direction of movement) in degrees. If unknown, set to NaN

    pub fn cog_deg(&self) -> f32 {
        self.inner.telemetry_server_get_cog_deg()
    }
        ///  Altitude in metres (above WGS84, EGM96 ellipsoid)

    pub fn altitude_ellipsoid_m(&self) -> f32 {
        self.inner.telemetry_server_get_altitude_ellipsoid_m()
    }
        ///  Position uncertainty in metres

    pub fn horizontal_uncertainty_m(&self) -> f32 {
        self.inner.telemetry_server_get_horizontal_uncertainty_m()
    }
        ///  Altitude uncertainty in metres

    pub fn vertical_uncertainty_m(&self) -> f32 {
        self.inner.telemetry_server_get_vertical_uncertainty_m()
    }
        ///  Velocity uncertainty in metres per second

    pub fn velocity_uncertainty_m_s(&self) -> f32 {
        self.inner.telemetry_server_get_velocity_uncertainty_m_s()
    }
        ///  Heading uncertainty in degrees

    pub fn heading_uncertainty_deg(&self) -> f32 {
        self.inner.telemetry_server_get_heading_uncertainty_deg()
    }
        ///  Yaw in earth frame from north.

    pub fn yaw_deg(&self) -> f32 {
        self.inner.telemetry_server_get_yaw_deg()
    }
}


pub struct BatteryOwned {
        pub voltage_v: f32,
        pub remaining_percent: f32,
}

/*  */
pub struct Battery<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_Battery,
}

impl<'a> Battery<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_Battery) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> BatteryOwned {
        BatteryOwned {
            
            voltage_v: self.voltage_v(),
            remaining_percent: self.remaining_percent(),
        }
    }
        ///  Voltage in volts

    pub fn voltage_v(&self) -> f32 {
        self.inner.telemetry_server_get_voltage_v()
    }
        ///  Estimated battery remaining (range: 0.0 to 1.0)

    pub fn remaining_percent(&self) -> f32 {
        self.inner.telemetry_server_get_remaining_percent()
    }
}


pub struct RcStatusOwned {
        pub was_available_once: bool,
        pub is_available: bool,
        pub signal_strength_percent: f32,
}

/*  */
pub struct RcStatus<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_RcStatus,
}

impl<'a> RcStatus<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_RcStatus) -> Self {
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
        self.inner.telemetry_server_get_was_available_once()
    }
        ///  True if the RC signal is available now

    pub fn is_available(&self) -> bool {
        self.inner.telemetry_server_get_is_available()
    }
        ///  Signal strength (range: 0 to 100, NaN if unknown)

    pub fn signal_strength_percent(&self) -> f32 {
        self.inner.telemetry_server_get_signal_strength_percent()
    }
}


pub struct StatusTextOwned {
        pub r#type: crate::telemetry_server::mavsdk::TelemetryServer_StatusTextType,
        pub text: String,
}

/*  */
pub struct StatusText<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_StatusText,
}

impl<'a> StatusText<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_StatusText) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> StatusTextOwned {
        StatusTextOwned {
            
            r#type: self.r#type(),
            text: self.text(),
        }
    }
        ///  Message type

    pub fn r#type(&self) -> crate::telemetry_server::mavsdk::TelemetryServer_StatusTextType {self.inner.telemetry_server_get_type().clone()
    }
        ///  MAVLink status message

    pub fn text(&self) -> String {
        self.inner.telemetry_server_get_text().to_string_lossy().into_owned()
    }
}


pub struct ActuatorControlTargetOwned {
        pub group: i32,
        pub controls: 
            std::vec::Vec<f32>,
}

/*  */
pub struct ActuatorControlTarget<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_ActuatorControlTarget,
}

impl<'a> ActuatorControlTarget<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_ActuatorControlTarget) -> Self {
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
        self.inner.telemetry_server_get_group()
    }
        ///  Controls normed from -1 to 1, where 0 is neutral position.

    pub fn controls(&self) -> 
            std::vec::Vec<f32> {
            self.inner.telemetry_server_get_controls().as_slice().to_vec()
    }
}


pub struct ActuatorOutputStatusOwned {
        pub active: u32,
        pub actuator: 
            std::vec::Vec<f32>,
}

/*  */
pub struct ActuatorOutputStatus<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_ActuatorOutputStatus,
}

impl<'a> ActuatorOutputStatus<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_ActuatorOutputStatus) -> Self {
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
        self.inner.telemetry_server_get_active()
    }
        ///  Servo/motor output values

    pub fn actuator(&self) -> 
            std::vec::Vec<f32> {
            self.inner.telemetry_server_get_actuator().as_slice().to_vec()
    }
}


pub struct CovarianceOwned {
        pub covariance_matrix: 
            std::vec::Vec<f32>,
}

/*  */
pub struct Covariance<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_Covariance,
}

impl<'a> Covariance<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_Covariance) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> CovarianceOwned {
        CovarianceOwned {
            
            covariance_matrix: self.covariance_matrix(),
        }
    }
        ///  Representation of a covariance matrix.

    pub fn covariance_matrix(&self) -> 
            std::vec::Vec<f32> {
            self.inner.telemetry_server_get_covariance_matrix().as_slice().to_vec()
    }
}


pub struct VelocityBodyOwned {
        pub x_m_s: f32,
        pub y_m_s: f32,
        pub z_m_s: f32,
}

/*  */
pub struct VelocityBody<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_VelocityBody,
}

impl<'a> VelocityBody<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_VelocityBody) -> Self {
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
        self.inner.telemetry_server_get_x_m_s()
    }
        ///  Velocity in Y in metres/second

    pub fn y_m_s(&self) -> f32 {
        self.inner.telemetry_server_get_y_m_s()
    }
        ///  Velocity in Z in metres/second

    pub fn z_m_s(&self) -> f32 {
        self.inner.telemetry_server_get_z_m_s()
    }
}


pub struct PositionBodyOwned {
        pub x_m: f32,
        pub y_m: f32,
        pub z_m: f32,
}

/*  */
pub struct PositionBody<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_PositionBody,
}

impl<'a> PositionBody<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_PositionBody) -> Self {
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
        self.inner.telemetry_server_get_x_m()
    }
        ///  Y Position in metres.

    pub fn y_m(&self) -> f32 {
        self.inner.telemetry_server_get_y_m()
    }
        ///  Z Position in metres.

    pub fn z_m(&self) -> f32 {
        self.inner.telemetry_server_get_z_m()
    }
}


pub struct OdometryOwned {
        pub time_usec: u64,
        pub frame_id: crate::telemetry_server::mavsdk::TelemetryServer_Odometry_MavFrame,
        pub child_frame_id: crate::telemetry_server::mavsdk::TelemetryServer_Odometry_MavFrame,
        pub position_body: PositionBodyOwned,
        pub q: QuaternionOwned,
        pub velocity_body: VelocityBodyOwned,
        pub angular_velocity_body: AngularVelocityBodyOwned,
        pub pose_covariance: CovarianceOwned,
        pub velocity_covariance: CovarianceOwned,
}

/*  */
pub struct Odometry<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_Odometry,
}

impl<'a> Odometry<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_Odometry) -> Self {
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
        self.inner.telemetry_server_get_time_usec()
    }
        ///  Coordinate frame of reference for the pose data.

    pub fn frame_id(&self) -> crate::telemetry_server::mavsdk::TelemetryServer_Odometry_MavFrame {
        self.inner.telemetry_server_get_frame_id().clone()
    }
        ///  Coordinate frame of reference for the velocity in free space (twist) data.

    pub fn child_frame_id(&self) -> crate::telemetry_server::mavsdk::TelemetryServer_Odometry_MavFrame {
        self.inner.telemetry_server_get_child_frame_id().clone()
    }
        ///  Position.

    pub fn position_body(&self) -> PositionBodyOwned {
        PositionBody::new(self.inner.telemetry_server_get_position_body()).into_owned()
    }
        ///  Quaternion components, w, x, y, z (1 0 0 0 is the null-rotation).

    pub fn q(&self) -> QuaternionOwned {
        Quaternion::new(self.inner.telemetry_server_get_q()).into_owned()
    }
        ///  Linear velocity (m/s).

    pub fn velocity_body(&self) -> VelocityBodyOwned {
        VelocityBody::new(self.inner.telemetry_server_get_velocity_body()).into_owned()
    }
        ///  Angular velocity (rad/s).

    pub fn angular_velocity_body(&self) -> AngularVelocityBodyOwned {
        AngularVelocityBody::new(self.inner.telemetry_server_get_angular_velocity_body()).into_owned()
    }
        ///  Pose cross-covariance matrix.

    pub fn pose_covariance(&self) -> CovarianceOwned {Covariance::new(self.inner.telemetry_server_get_pose_covariance()).into_owned()
    }
        ///  Velocity cross-covariance matrix.

    pub fn velocity_covariance(&self) -> CovarianceOwned {Covariance::new(self.inner.telemetry_server_get_velocity_covariance()).into_owned()
    }
}


pub struct DistanceSensorOwned {
        pub minimum_distance_m: f32,
        pub maximum_distance_m: f32,
        pub current_distance_m: f32,
}

/*  */
pub struct DistanceSensor<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_DistanceSensor,
}

impl<'a> DistanceSensor<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_DistanceSensor) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> DistanceSensorOwned {
        DistanceSensorOwned {
            
            minimum_distance_m: self.minimum_distance_m(),
            maximum_distance_m: self.maximum_distance_m(),
            current_distance_m: self.current_distance_m(),
        }
    }
        ///  Minimum distance the sensor can measure, NaN if unknown.

    pub fn minimum_distance_m(&self) -> f32 {
        self.inner.telemetry_server_get_minimum_distance_m()
    }
        ///  Maximum distance the sensor can measure, NaN if unknown.

    pub fn maximum_distance_m(&self) -> f32 {
        self.inner.telemetry_server_get_maximum_distance_m()
    }
        ///  Current distance reading, NaN if unknown.

    pub fn current_distance_m(&self) -> f32 {
        self.inner.telemetry_server_get_current_distance_m()
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
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_ScaledPressure,
}

impl<'a> ScaledPressure<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_ScaledPressure) -> Self {
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
        self.inner.telemetry_server_get_timestamp_us()
    }
        ///  Absolute pressure in hPa

    pub fn absolute_pressure_hpa(&self) -> f32 {
        self.inner.telemetry_server_get_absolute_pressure_hpa()
    }
        ///  Differential pressure 1 in hPa

    pub fn differential_pressure_hpa(&self) -> f32 {
        self.inner.telemetry_server_get_differential_pressure_hpa()
    }
        ///  Absolute pressure temperature (in celsius)

    pub fn temperature_deg(&self) -> f32 {
        self.inner.telemetry_server_get_temperature_deg()
    }
        ///  Differential pressure temperature (in celsius, 0 if not available)

    pub fn differential_pressure_temperature_deg(&self) -> f32 {
        self.inner.telemetry_server_get_differential_pressure_temperature_deg()
    }
}


pub struct PositionNedOwned {
        pub north_m: f32,
        pub east_m: f32,
        pub down_m: f32,
}

/*  */
pub struct PositionNed<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_PositionNed,
}

impl<'a> PositionNed<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_PositionNed) -> Self {
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
        self.inner.telemetry_server_get_north_m()
    }
        ///  Position along east direction in metres

    pub fn east_m(&self) -> f32 {
        self.inner.telemetry_server_get_east_m()
    }
        ///  Position along down direction in metres

    pub fn down_m(&self) -> f32 {
        self.inner.telemetry_server_get_down_m()
    }
}


pub struct VelocityNedOwned {
        pub north_m_s: f32,
        pub east_m_s: f32,
        pub down_m_s: f32,
}

/*  */
pub struct VelocityNed<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_VelocityNed,
}

impl<'a> VelocityNed<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_VelocityNed) -> Self {
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
        self.inner.telemetry_server_get_north_m_s()
    }
        ///  Velocity along east direction in metres per second

    pub fn east_m_s(&self) -> f32 {
        self.inner.telemetry_server_get_east_m_s()
    }
        ///  Velocity along down direction in metres per second

    pub fn down_m_s(&self) -> f32 {
        self.inner.telemetry_server_get_down_m_s()
    }
}


pub struct PositionVelocityNedOwned {
        pub position: PositionNedOwned,
        pub velocity: VelocityNedOwned,
}

/*  */
pub struct PositionVelocityNed<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_PositionVelocityNed,
}

impl<'a> PositionVelocityNed<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_PositionVelocityNed) -> Self {
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
        PositionNed::new(self.inner.telemetry_server_get_position()).into_owned()
    }
        ///  Velocity (NED)

    pub fn velocity(&self) -> VelocityNedOwned {
        VelocityNed::new(self.inner.telemetry_server_get_velocity()).into_owned()
    }
}


pub struct GroundTruthOwned {
        pub latitude_deg: f64,
        pub longitude_deg: f64,
        pub absolute_altitude_m: f32,
}

/*  */
pub struct GroundTruth<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_GroundTruth,
}

impl<'a> GroundTruth<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_GroundTruth) -> Self {
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
        self.inner.telemetry_server_get_latitude_deg()
    }
        ///  Longitude in degrees (range: -180 to 180)

    pub fn longitude_deg(&self) -> f64 {
        self.inner.telemetry_server_get_longitude_deg()
    }
        ///  Altitude AMSL (above mean sea level) in metres

    pub fn absolute_altitude_m(&self) -> f32 {
        self.inner.telemetry_server_get_absolute_altitude_m()
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
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_FixedwingMetrics,
}

impl<'a> FixedwingMetrics<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_FixedwingMetrics) -> Self {
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
        self.inner.telemetry_server_get_airspeed_m_s()
    }
        ///  Current throttle setting (0 to 100)

    pub fn throttle_percentage(&self) -> f32 {
        self.inner.telemetry_server_get_throttle_percentage()
    }
        ///  Current climb rate in metres per second

    pub fn climb_rate_m_s(&self) -> f32 {
        self.inner.telemetry_server_get_climb_rate_m_s()
    }
        ///  Current groundspeed metres per second

    pub fn groundspeed_m_s(&self) -> f32 {
        self.inner.telemetry_server_get_groundspeed_m_s()
    }
        ///  Current heading in compass units (0-360, 0=north)

    pub fn heading_deg(&self) -> f32 {
        self.inner.telemetry_server_get_heading_deg()
    }
        ///  Current altitude in metres (MSL)

    pub fn absolute_altitude_m(&self) -> f32 {
        self.inner.telemetry_server_get_absolute_altitude_m()
    }
}


pub struct AccelerationFrdOwned {
        pub forward_m_s2: f32,
        pub right_m_s2: f32,
        pub down_m_s2: f32,
}

/*  */
pub struct AccelerationFrd<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_AccelerationFrd,
}

impl<'a> AccelerationFrd<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_AccelerationFrd) -> Self {
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
        self.inner.telemetry_server_get_forward_m_s2()
    }
        ///  Acceleration in right direction in metres per second^2

    pub fn right_m_s2(&self) -> f32 {
        self.inner.telemetry_server_get_right_m_s2()
    }
        ///  Acceleration in down direction in metres per second^2

    pub fn down_m_s2(&self) -> f32 {
        self.inner.telemetry_server_get_down_m_s2()
    }
}


pub struct AngularVelocityFrdOwned {
        pub forward_rad_s: f32,
        pub right_rad_s: f32,
        pub down_rad_s: f32,
}

/*  */
pub struct AngularVelocityFrd<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_AngularVelocityFrd,
}

impl<'a> AngularVelocityFrd<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_AngularVelocityFrd) -> Self {
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
        self.inner.telemetry_server_get_forward_rad_s()
    }
        ///  Angular velocity in right direction in radians per second

    pub fn right_rad_s(&self) -> f32 {
        self.inner.telemetry_server_get_right_rad_s()
    }
        ///  Angular velocity in Down direction in radians per second

    pub fn down_rad_s(&self) -> f32 {
        self.inner.telemetry_server_get_down_rad_s()
    }
}


pub struct MagneticFieldFrdOwned {
        pub forward_gauss: f32,
        pub right_gauss: f32,
        pub down_gauss: f32,
}

/*  */
pub struct MagneticFieldFrd<'a> {
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_MagneticFieldFrd,
}

impl<'a> MagneticFieldFrd<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_MagneticFieldFrd) -> Self {
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
        self.inner.telemetry_server_get_forward_gauss()
    }
        ///  Magnetic field in East direction measured in Gauss

    pub fn right_gauss(&self) -> f32 {
        self.inner.telemetry_server_get_right_gauss()
    }
        ///  Magnetic field in Down direction measured in Gauss

    pub fn down_gauss(&self) -> f32 {
        self.inner.telemetry_server_get_down_gauss()
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
    inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_Imu,
}

impl<'a> Imu<'a> {
    pub fn new(inner: &'a crate::telemetry_server::mavsdk::TelemetryServer_Imu) -> Self {
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
        AccelerationFrd::new(self.inner.telemetry_server_get_acceleration_frd()).into_owned()
    }
        ///  Angular velocity

    pub fn angular_velocity_frd(&self) -> AngularVelocityFrdOwned {
        AngularVelocityFrd::new(self.inner.telemetry_server_get_angular_velocity_frd()).into_owned()
    }
        ///  Magnetic field

    pub fn magnetic_field_frd(&self) -> MagneticFieldFrdOwned {
        MagneticFieldFrd::new(self.inner.telemetry_server_get_magnetic_field_frd()).into_owned()
    }
        ///  Temperature

    pub fn temperature_degc(&self) -> f32 {
        self.inner.telemetry_server_get_temperature_degc()
    }
        ///  Timestamp in microseconds

    pub fn timestamp_us(&self) -> u64 {
        self.inner.telemetry_server_get_timestamp_us()
    }
}

    struct TelemetryServerInner {
        plugin: cxx::UniquePtr<crate::telemetry_server::mavsdk::TelemetryServer>,
        
    }

    pub struct TelemetryServerClient {
        inner: Mutex<TelemetryServerInner>,
        
    }

    impl TelemetryServerClient {
        pub fn new(plugin: cxx::UniquePtr<crate::telemetry_server::mavsdk::TelemetryServer>) -> Self {
            

            Self {
                inner: Mutex::new(TelemetryServerInner {
                    plugin,
                    
                }),
                
            }
        }
        
        
    }

