// Auto-generated wrapper. Do not edit.
use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct PositionBodyOwned {
    pub x_m: f32,
    pub y_m: f32,
    pub z_m: f32,
}

/*  */
pub struct PositionBody<'a> {
    inner: &'a crate::mocap::mavsdk::Mocap_PositionBody,
}

impl<'a> PositionBody<'a> {
    pub fn new(inner: &'a crate::mocap::mavsdk::Mocap_PositionBody) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> PositionBodyOwned {
        PositionBodyOwned {
            x_m: self.x_m(),
            y_m: self.y_m(),
            z_m: self.z_m(),
        }
    }
    ///  X position in metres.

    pub fn x_m(&self) -> f32 {
        self.inner.mocap_get_x_m()
    }
    ///  Y position in metres.

    pub fn y_m(&self) -> f32 {
        self.inner.mocap_get_y_m()
    }
    ///  Z position in metres.

    pub fn z_m(&self) -> f32 {
        self.inner.mocap_get_z_m()
    }
}

pub struct AngleBodyOwned {
    pub roll_rad: f32,
    pub pitch_rad: f32,
    pub yaw_rad: f32,
}

/*  */
pub struct AngleBody<'a> {
    inner: &'a crate::mocap::mavsdk::Mocap_AngleBody,
}

impl<'a> AngleBody<'a> {
    pub fn new(inner: &'a crate::mocap::mavsdk::Mocap_AngleBody) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> AngleBodyOwned {
        AngleBodyOwned {
            roll_rad: self.roll_rad(),
            pitch_rad: self.pitch_rad(),
            yaw_rad: self.yaw_rad(),
        }
    }
    ///  Roll angle in radians.

    pub fn roll_rad(&self) -> f32 {
        self.inner.mocap_get_roll_rad()
    }
    ///  Pitch angle in radians.

    pub fn pitch_rad(&self) -> f32 {
        self.inner.mocap_get_pitch_rad()
    }
    ///  Yaw angle in radians.

    pub fn yaw_rad(&self) -> f32 {
        self.inner.mocap_get_yaw_rad()
    }
}

pub struct SpeedBodyOwned {
    pub x_m_s: f32,
    pub y_m_s: f32,
    pub z_m_s: f32,
}

/*  */
pub struct SpeedBody<'a> {
    inner: &'a crate::mocap::mavsdk::Mocap_SpeedBody,
}

impl<'a> SpeedBody<'a> {
    pub fn new(inner: &'a crate::mocap::mavsdk::Mocap_SpeedBody) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> SpeedBodyOwned {
        SpeedBodyOwned {
            x_m_s: self.x_m_s(),
            y_m_s: self.y_m_s(),
            z_m_s: self.z_m_s(),
        }
    }
    ///  Velocity in X in metres/second.

    pub fn x_m_s(&self) -> f32 {
        self.inner.mocap_get_x_m_s()
    }
    ///  Velocity in Y in metres/second.

    pub fn y_m_s(&self) -> f32 {
        self.inner.mocap_get_y_m_s()
    }
    ///  Velocity in Z in metres/second.

    pub fn z_m_s(&self) -> f32 {
        self.inner.mocap_get_z_m_s()
    }
}

pub struct SpeedNedOwned {
    pub north_m_s: f32,
    pub east_m_s: f32,
    pub down_m_s: f32,
}

/*  */
pub struct SpeedNed<'a> {
    inner: &'a crate::mocap::mavsdk::Mocap_SpeedNed,
}

impl<'a> SpeedNed<'a> {
    pub fn new(inner: &'a crate::mocap::mavsdk::Mocap_SpeedNed) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> SpeedNedOwned {
        SpeedNedOwned {
            north_m_s: self.north_m_s(),
            east_m_s: self.east_m_s(),
            down_m_s: self.down_m_s(),
        }
    }
    ///  Velocity North in metres/second.

    pub fn north_m_s(&self) -> f32 {
        self.inner.mocap_get_north_m_s()
    }
    ///  Velocity East in metres/second.

    pub fn east_m_s(&self) -> f32 {
        self.inner.mocap_get_east_m_s()
    }
    ///  Velocity Down in metres/second.

    pub fn down_m_s(&self) -> f32 {
        self.inner.mocap_get_down_m_s()
    }
}

pub struct AngularVelocityBodyOwned {
    pub roll_rad_s: f32,
    pub pitch_rad_s: f32,
    pub yaw_rad_s: f32,
}

/*  */
pub struct AngularVelocityBody<'a> {
    inner: &'a crate::mocap::mavsdk::Mocap_AngularVelocityBody,
}

impl<'a> AngularVelocityBody<'a> {
    pub fn new(inner: &'a crate::mocap::mavsdk::Mocap_AngularVelocityBody) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> AngularVelocityBodyOwned {
        AngularVelocityBodyOwned {
            roll_rad_s: self.roll_rad_s(),
            pitch_rad_s: self.pitch_rad_s(),
            yaw_rad_s: self.yaw_rad_s(),
        }
    }
    ///  Roll angular velocity in radians/second.

    pub fn roll_rad_s(&self) -> f32 {
        self.inner.mocap_get_roll_rad_s()
    }
    ///  Pitch angular velocity in radians/second.

    pub fn pitch_rad_s(&self) -> f32 {
        self.inner.mocap_get_pitch_rad_s()
    }
    ///  Yaw angular velocity in radians/second.

    pub fn yaw_rad_s(&self) -> f32 {
        self.inner.mocap_get_yaw_rad_s()
    }
}

pub struct CovarianceOwned {
    pub covariance_matrix: std::vec::Vec<f32>,
}

/*  */
pub struct Covariance<'a> {
    inner: &'a crate::mocap::mavsdk::Mocap_Covariance,
}

impl<'a> Covariance<'a> {
    pub fn new(inner: &'a crate::mocap::mavsdk::Mocap_Covariance) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> CovarianceOwned {
        CovarianceOwned {
            covariance_matrix: self.covariance_matrix(),
        }
    }
    ///  The covariance matrix

    pub fn covariance_matrix(&self) -> std::vec::Vec<f32> {
        self.inner.mocap_get_covariance_matrix().as_slice().to_vec()
    }
}

pub struct QuaternionOwned {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/*  */
pub struct Quaternion<'a> {
    inner: &'a crate::mocap::mavsdk::Mocap_Quaternion,
}

impl<'a> Quaternion<'a> {
    pub fn new(inner: &'a crate::mocap::mavsdk::Mocap_Quaternion) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> QuaternionOwned {
        QuaternionOwned {
            w: self.w(),
            x: self.x(),
            y: self.y(),
            z: self.z(),
        }
    }
    ///  Quaternion entry 0, also denoted as a

    pub fn w(&self) -> f32 {
        self.inner.mocap_get_w()
    }
    ///  Quaternion entry 1, also denoted as b

    pub fn x(&self) -> f32 {
        self.inner.mocap_get_x()
    }
    ///  Quaternion entry 2, also denoted as c

    pub fn y(&self) -> f32 {
        self.inner.mocap_get_y()
    }
    ///  Quaternion entry 3, also denoted as d

    pub fn z(&self) -> f32 {
        self.inner.mocap_get_z()
    }
}

pub struct VisionPositionEstimateOwned {
    pub time_usec: u64,
    pub position_body: PositionBodyOwned,
    pub angle_body: AngleBodyOwned,
    pub pose_covariance: CovarianceOwned,
}

/*  */
pub struct VisionPositionEstimate<'a> {
    inner: &'a crate::mocap::mavsdk::Mocap_VisionPositionEstimate,
}

impl<'a> VisionPositionEstimate<'a> {
    pub fn new(inner: &'a crate::mocap::mavsdk::Mocap_VisionPositionEstimate) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> VisionPositionEstimateOwned {
        VisionPositionEstimateOwned {
            time_usec: self.time_usec(),
            position_body: self.position_body(),
            angle_body: self.angle_body(),
            pose_covariance: self.pose_covariance(),
        }
    }
    ///  PositionBody frame timestamp UNIX Epoch time (0 to use Backend timestamp)

    pub fn time_usec(&self) -> u64 {
        self.inner.mocap_get_time_usec()
    }
    ///  Global position (m)

    pub fn position_body(&self) -> PositionBodyOwned {
        PositionBody::new(self.inner.mocap_get_position_body()).into_owned()
    }
    ///  Body angle (rad).

    pub fn angle_body(&self) -> AngleBodyOwned {
        AngleBody::new(self.inner.mocap_get_angle_body()).into_owned()
    }
    ///  Pose cross-covariance matrix.

    pub fn pose_covariance(&self) -> CovarianceOwned {
        Covariance::new(self.inner.mocap_get_pose_covariance()).into_owned()
    }
}

pub struct VisionSpeedEstimateOwned {
    pub time_usec: u64,
    pub speed_ned: SpeedNedOwned,
    pub speed_covariance: CovarianceOwned,
}

/*  */
pub struct VisionSpeedEstimate<'a> {
    inner: &'a crate::mocap::mavsdk::Mocap_VisionSpeedEstimate,
}

impl<'a> VisionSpeedEstimate<'a> {
    pub fn new(inner: &'a crate::mocap::mavsdk::Mocap_VisionSpeedEstimate) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> VisionSpeedEstimateOwned {
        VisionSpeedEstimateOwned {
            time_usec: self.time_usec(),
            speed_ned: self.speed_ned(),
            speed_covariance: self.speed_covariance(),
        }
    }
    ///  Timestamp UNIX Epoch time (0 to use Backend timestamp)

    pub fn time_usec(&self) -> u64 {
        self.inner.mocap_get_time_usec()
    }
    ///  Global speed (m/s)

    pub fn speed_ned(&self) -> SpeedNedOwned {
        SpeedNed::new(self.inner.mocap_get_speed_ned()).into_owned()
    }
    ///  Linear velocity cross-covariance matrix.

    pub fn speed_covariance(&self) -> CovarianceOwned {
        Covariance::new(self.inner.mocap_get_speed_covariance()).into_owned()
    }
}

pub struct AttitudePositionMocapOwned {
    pub time_usec: u64,
    pub q: QuaternionOwned,
    pub position_body: PositionBodyOwned,
    pub pose_covariance: CovarianceOwned,
}

/*  */
pub struct AttitudePositionMocap<'a> {
    inner: &'a crate::mocap::mavsdk::Mocap_AttitudePositionMocap,
}

impl<'a> AttitudePositionMocap<'a> {
    pub fn new(inner: &'a crate::mocap::mavsdk::Mocap_AttitudePositionMocap) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> AttitudePositionMocapOwned {
        AttitudePositionMocapOwned {
            time_usec: self.time_usec(),
            q: self.q(),
            position_body: self.position_body(),
            pose_covariance: self.pose_covariance(),
        }
    }
    ///  PositionBody frame timestamp UNIX Epoch time (0 to use Backend timestamp)

    pub fn time_usec(&self) -> u64 {
        self.inner.mocap_get_time_usec()
    }
    ///  Attitude quaternion (w, x, y, z order, zero-rotation is 1, 0, 0, 0)

    pub fn q(&self) -> QuaternionOwned {
        Quaternion::new(self.inner.mocap_get_q()).into_owned()
    }
    ///  Body Position (NED)

    pub fn position_body(&self) -> PositionBodyOwned {
        PositionBody::new(self.inner.mocap_get_position_body()).into_owned()
    }
    ///  Pose cross-covariance matrix.

    pub fn pose_covariance(&self) -> CovarianceOwned {
        Covariance::new(self.inner.mocap_get_pose_covariance()).into_owned()
    }
}

pub struct OdometryOwned {
    pub time_usec: u64,
    pub frame_id: crate::mocap::mavsdk::Mocap_Odometry_MavFrame,
    pub position_body: PositionBodyOwned,
    pub q: QuaternionOwned,
    pub speed_body: SpeedBodyOwned,
    pub angular_velocity_body: AngularVelocityBodyOwned,
    pub pose_covariance: CovarianceOwned,
    pub velocity_covariance: CovarianceOwned,
}

/*  */
pub struct Odometry<'a> {
    inner: &'a crate::mocap::mavsdk::Mocap_Odometry,
}

impl<'a> Odometry<'a> {
    pub fn new(inner: &'a crate::mocap::mavsdk::Mocap_Odometry) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> OdometryOwned {
        OdometryOwned {
            time_usec: self.time_usec(),
            frame_id: self.frame_id(),
            position_body: self.position_body(),
            q: self.q(),
            speed_body: self.speed_body(),
            angular_velocity_body: self.angular_velocity_body(),
            pose_covariance: self.pose_covariance(),
            velocity_covariance: self.velocity_covariance(),
        }
    }
    ///  Timestamp (0 to use Backend timestamp).

    pub fn time_usec(&self) -> u64 {
        self.inner.mocap_get_time_usec()
    }
    ///  Coordinate frame of reference for the pose data.

    pub fn frame_id(&self) -> crate::mocap::mavsdk::Mocap_Odometry_MavFrame {
        self.inner.mocap_get_frame_id().clone()
    }
    ///  Body Position.

    pub fn position_body(&self) -> PositionBodyOwned {
        PositionBody::new(self.inner.mocap_get_position_body()).into_owned()
    }
    ///  Quaternion components, w, x, y, z (1 0 0 0 is the null-rotation).

    pub fn q(&self) -> QuaternionOwned {
        Quaternion::new(self.inner.mocap_get_q()).into_owned()
    }
    ///  Linear speed (m/s).

    pub fn speed_body(&self) -> SpeedBodyOwned {
        SpeedBody::new(self.inner.mocap_get_speed_body()).into_owned()
    }
    ///  Angular speed (rad/s).

    pub fn angular_velocity_body(&self) -> AngularVelocityBodyOwned {
        AngularVelocityBody::new(self.inner.mocap_get_angular_velocity_body()).into_owned()
    }
    ///  Pose cross-covariance matrix.

    pub fn pose_covariance(&self) -> CovarianceOwned {
        Covariance::new(self.inner.mocap_get_pose_covariance()).into_owned()
    }
    ///  Velocity cross-covariance matrix.

    pub fn velocity_covariance(&self) -> CovarianceOwned {
        Covariance::new(self.inner.mocap_get_velocity_covariance()).into_owned()
    }
}

struct MocapInner {
    plugin: cxx::UniquePtr<crate::mocap::mavsdk::Mocap>,
}

pub struct MocapClient {
    inner: Mutex<MocapInner>,
}

impl MocapClient {
    pub fn new(plugin: cxx::UniquePtr<crate::mocap::mavsdk::Mocap>) -> Self {
        Self {
            inner: Mutex::new(MocapInner { plugin }),
        }
    }
}
