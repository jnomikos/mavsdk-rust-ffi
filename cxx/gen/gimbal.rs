// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct QuaternionOwned {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/*  */
pub struct Quaternion<'a> {
    inner: &'a crate::gimbal::mavsdk::Gimbal_Quaternion,
}

impl<'a> Quaternion<'a> {
    pub fn new(inner: &'a crate::gimbal::mavsdk::Gimbal_Quaternion) -> Self {
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
        self.inner.gimbal_get_w()
    }
    ///  Quaternion entry 1, also denoted as b

    pub fn x(&self) -> f32 {
        self.inner.gimbal_get_x()
    }
    ///  Quaternion entry 2, also denoted as c

    pub fn y(&self) -> f32 {
        self.inner.gimbal_get_y()
    }
    ///  Quaternion entry 3, also denoted as d

    pub fn z(&self) -> f32 {
        self.inner.gimbal_get_z()
    }
}

pub struct EulerAngleOwned {
    pub roll_deg: f32,
    pub pitch_deg: f32,
    pub yaw_deg: f32,
}

/*  */
pub struct EulerAngle<'a> {
    inner: &'a crate::gimbal::mavsdk::Gimbal_EulerAngle,
}

impl<'a> EulerAngle<'a> {
    pub fn new(inner: &'a crate::gimbal::mavsdk::Gimbal_EulerAngle) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> EulerAngleOwned {
        EulerAngleOwned {
            roll_deg: self.roll_deg(),
            pitch_deg: self.pitch_deg(),
            yaw_deg: self.yaw_deg(),
        }
    }
    ///  Roll angle in degrees, positive is banking to the right

    pub fn roll_deg(&self) -> f32 {
        self.inner.gimbal_get_roll_deg()
    }
    ///  Pitch angle in degrees, positive is pitching nose up

    pub fn pitch_deg(&self) -> f32 {
        self.inner.gimbal_get_pitch_deg()
    }
    ///  Yaw angle in degrees, positive is clock-wise seen from above

    pub fn yaw_deg(&self) -> f32 {
        self.inner.gimbal_get_yaw_deg()
    }
}

pub struct AngularVelocityBodyOwned {
    pub roll_rad_s: f32,
    pub pitch_rad_s: f32,
    pub yaw_rad_s: f32,
}

/*  */
pub struct AngularVelocityBody<'a> {
    inner: &'a crate::gimbal::mavsdk::Gimbal_AngularVelocityBody,
}

impl<'a> AngularVelocityBody<'a> {
    pub fn new(inner: &'a crate::gimbal::mavsdk::Gimbal_AngularVelocityBody) -> Self {
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
        self.inner.gimbal_get_roll_rad_s()
    }
    ///  Pitch angular velocity

    pub fn pitch_rad_s(&self) -> f32 {
        self.inner.gimbal_get_pitch_rad_s()
    }
    ///  Yaw angular velocity

    pub fn yaw_rad_s(&self) -> f32 {
        self.inner.gimbal_get_yaw_rad_s()
    }
}

pub struct AttitudeOwned {
    pub gimbal_id: i32,
    pub euler_angle_forward: EulerAngleOwned,
    pub quaternion_forward: QuaternionOwned,
    pub euler_angle_north: EulerAngleOwned,
    pub quaternion_north: QuaternionOwned,
    pub angular_velocity: AngularVelocityBodyOwned,
    pub timestamp_us: u64,
}

/*  */
pub struct Attitude<'a> {
    inner: &'a crate::gimbal::mavsdk::Gimbal_Attitude,
}

impl<'a> Attitude<'a> {
    pub fn new(inner: &'a crate::gimbal::mavsdk::Gimbal_Attitude) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> AttitudeOwned {
        AttitudeOwned {
            gimbal_id: self.gimbal_id(),
            euler_angle_forward: self.euler_angle_forward(),
            quaternion_forward: self.quaternion_forward(),
            euler_angle_north: self.euler_angle_north(),
            quaternion_north: self.quaternion_north(),
            angular_velocity: self.angular_velocity(),
            timestamp_us: self.timestamp_us(),
        }
    }
    ///  Gimbal ID

    pub fn gimbal_id(&self) -> i32 {
        self.inner.gimbal_get_gimbal_id()
    }
    ///  Euler angle relative to forward

    pub fn euler_angle_forward(&self) -> EulerAngleOwned {
        EulerAngle::new(self.inner.gimbal_get_euler_angle_forward()).into_owned()
    }
    ///  Quaternion relative to forward

    pub fn quaternion_forward(&self) -> QuaternionOwned {
        Quaternion::new(self.inner.gimbal_get_quaternion_forward()).into_owned()
    }
    ///  Euler angle relative to North

    pub fn euler_angle_north(&self) -> EulerAngleOwned {
        EulerAngle::new(self.inner.gimbal_get_euler_angle_north()).into_owned()
    }
    ///  Quaternion relative to North

    pub fn quaternion_north(&self) -> QuaternionOwned {
        Quaternion::new(self.inner.gimbal_get_quaternion_north()).into_owned()
    }
    ///  The angular rate

    pub fn angular_velocity(&self) -> AngularVelocityBodyOwned {
        AngularVelocityBody::new(self.inner.gimbal_get_angular_velocity()).into_owned()
    }
    ///  Timestamp in microseconds

    pub fn timestamp_us(&self) -> u64 {
        self.inner.gimbal_get_timestamp_us()
    }
}

pub struct GimbalItemOwned {
    pub gimbal_id: i32,
    pub vendor_name: String,
    pub model_name: String,
    pub custom_name: String,
    pub gimbal_manager_component_id: i32,
    pub gimbal_device_id: i32,
}

/*  */
pub struct GimbalItem<'a> {
    inner: &'a crate::gimbal::mavsdk::Gimbal_GimbalItem,
}

impl<'a> GimbalItem<'a> {
    pub fn new(inner: &'a crate::gimbal::mavsdk::Gimbal_GimbalItem) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> GimbalItemOwned {
        GimbalItemOwned {
            gimbal_id: self.gimbal_id(),
            vendor_name: self.vendor_name(),
            model_name: self.model_name(),
            custom_name: self.custom_name(),
            gimbal_manager_component_id: self.gimbal_manager_component_id(),
            gimbal_device_id: self.gimbal_device_id(),
        }
    }
    ///  ID to address it, starting at 1 (0 means all gimbals)

    pub fn gimbal_id(&self) -> i32 {
        self.inner.gimbal_get_gimbal_id()
    }
    ///  Vendor name

    pub fn vendor_name(&self) -> String {
        self.inner
            .gimbal_get_vendor_name()
            .to_string_lossy()
            .into_owned()
    }
    ///  Model name

    pub fn model_name(&self) -> String {
        self.inner
            .gimbal_get_model_name()
            .to_string_lossy()
            .into_owned()
    }
    ///  Custom name name

    pub fn custom_name(&self) -> String {
        self.inner
            .gimbal_get_custom_name()
            .to_string_lossy()
            .into_owned()
    }
    ///  MAVLink component of gimbal manager, for debugging purposes

    pub fn gimbal_manager_component_id(&self) -> i32 {
        self.inner.gimbal_get_gimbal_manager_component_id()
    }
    ///  MAVLink component of gimbal device

    pub fn gimbal_device_id(&self) -> i32 {
        self.inner.gimbal_get_gimbal_device_id()
    }
}

pub struct GimbalListOwned {
    pub gimbals: std::vec::Vec<GimbalItemOwned>,
}

/*  */
pub struct GimbalList<'a> {
    inner: &'a crate::gimbal::mavsdk::Gimbal_GimbalList,
}

impl<'a> GimbalList<'a> {
    pub fn new(inner: &'a crate::gimbal::mavsdk::Gimbal_GimbalList) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> GimbalListOwned {
        GimbalListOwned {
            gimbals: self.gimbals(),
        }
    }
    ///  Gimbal items.

    pub fn gimbals(&self) -> std::vec::Vec<GimbalItemOwned> {
        self.inner
            .gimbal_get_gimbals()
            .iter()
            .map(|item| GimbalItem::new(item).into_owned())
            .collect()
    }
}

pub struct ControlStatusOwned {
    pub gimbal_id: i32,
    pub control_mode: crate::gimbal::mavsdk::Gimbal_ControlMode,
    pub sysid_primary_control: i32,
    pub compid_primary_control: i32,
    pub sysid_secondary_control: i32,
    pub compid_secondary_control: i32,
}

/*  */
pub struct ControlStatus<'a> {
    inner: &'a crate::gimbal::mavsdk::Gimbal_ControlStatus,
}

impl<'a> ControlStatus<'a> {
    pub fn new(inner: &'a crate::gimbal::mavsdk::Gimbal_ControlStatus) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> ControlStatusOwned {
        ControlStatusOwned {
            gimbal_id: self.gimbal_id(),
            control_mode: self.control_mode(),
            sysid_primary_control: self.sysid_primary_control(),
            compid_primary_control: self.compid_primary_control(),
            sysid_secondary_control: self.sysid_secondary_control(),
            compid_secondary_control: self.compid_secondary_control(),
        }
    }
    ///  Gimbal ID

    pub fn gimbal_id(&self) -> i32 {
        self.inner.gimbal_get_gimbal_id()
    }
    ///  Control mode (none, primary or secondary)

    pub fn control_mode(&self) -> crate::gimbal::mavsdk::Gimbal_ControlMode {
        self.inner.gimbal_get_control_mode().clone()
    }
    ///  Sysid of the component that has primary control over the gimbal (0 if no one is in control)

    pub fn sysid_primary_control(&self) -> i32 {
        self.inner.gimbal_get_sysid_primary_control()
    }
    ///  Compid of the component that has primary control over the gimbal (0 if no one is in control)

    pub fn compid_primary_control(&self) -> i32 {
        self.inner.gimbal_get_compid_primary_control()
    }
    ///  Sysid of the component that has secondary control over the gimbal (0 if no one is in control)

    pub fn sysid_secondary_control(&self) -> i32 {
        self.inner.gimbal_get_sysid_secondary_control()
    }
    ///  Compid of the component that has secondary control over the gimbal (0 if no one is in control)

    pub fn compid_secondary_control(&self) -> i32 {
        self.inner.gimbal_get_compid_secondary_control()
    }
}

struct GimbalInner {
    plugin: cxx::UniquePtr<crate::gimbal::mavsdk::Gimbal>,

    /// GimbalList State
    gimbal_list_handle: Option<usize>,
    gimbal_list_user_data: *mut c_void,
    /// ControlStatus State
    control_status_handle: Option<usize>,
    control_status_user_data: *mut c_void,
    /// Attitude State
    attitude_handle: Option<usize>,
    attitude_user_data: *mut c_void,
}

pub struct GimbalClient {
    inner: Mutex<GimbalInner>,

    // Producer for the latest GimbalList state; broadcast to all active subscribers.
    gimbal_list_tx: watch::Sender<std::option::Option<GimbalListOwned>>,
    // Producer for the latest ControlStatus state; broadcast to all active subscribers.
    control_status_tx: watch::Sender<std::option::Option<ControlStatusOwned>>,
    // Producer for the latest Attitude state; broadcast to all active subscribers.
    attitude_tx: watch::Sender<std::option::Option<AttitudeOwned>>,
}

impl GimbalClient {
    pub fn new(plugin: cxx::UniquePtr<crate::gimbal::mavsdk::Gimbal>) -> Self {
        let (gimbal_list_tx, _) = watch::channel(None);
        let (control_status_tx, _) = watch::channel(None);
        let (attitude_tx, _) = watch::channel(None);

        Self {
            inner: Mutex::new(GimbalInner {
                plugin,

                gimbal_list_handle: None,
                gimbal_list_user_data: std::ptr::null_mut(),
                control_status_handle: None,
                control_status_user_data: std::ptr::null_mut(),
                attitude_handle: None,
                attitude_user_data: std::ptr::null_mut(),
            }),

            gimbal_list_tx,
            control_status_tx,
            attitude_tx,
        }
    }

    pub fn subscribe_gimbal_list(&self) -> watch::Receiver<std::option::Option<GimbalListOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.gimbal_list_handle.is_none() {
            // Lazy initialization of C++ 'gimbal_list' gimbal stream.
            let user_data_ptr = Box::into_raw(Box::new(self.gimbal_list_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_gimbal_list(
                gimbal_gimbal_list_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.gimbal_list_handle = Some(handle);
            inner.gimbal_list_user_data = user_data_ptr;
        }

        self.gimbal_list_tx.subscribe()
    }
    pub fn subscribe_control_status(
        &self,
    ) -> watch::Receiver<std::option::Option<ControlStatusOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.control_status_handle.is_none() {
            // Lazy initialization of C++ 'control_status' gimbal stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.control_status_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_control_status(
                gimbal_control_status_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.control_status_handle = Some(handle);
            inner.control_status_user_data = user_data_ptr;
        }

        self.control_status_tx.subscribe()
    }
    pub fn subscribe_attitude(&self) -> watch::Receiver<std::option::Option<AttitudeOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.attitude_handle.is_none() {
            // Lazy initialization of C++ 'attitude' gimbal stream.
            let user_data_ptr = Box::into_raw(Box::new(self.attitude_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_attitude(
                gimbal_attitude_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.attitude_handle = Some(handle);
            inner.attitude_user_data = user_data_ptr;
        }

        self.attitude_tx.subscribe()
    }
}

impl Drop for GimbalClient {
    fn drop(&mut self) {
        if let Ok(mut inner) = self.inner.lock() {
            if let Some(handle) = inner.gimbal_list_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_gimbal_list(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.gimbal_list_user_data
                            as *mut watch::Sender<std::option::Option<GimbalListOwned>>,
                    );
                }
            }
            if let Some(handle) = inner.control_status_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_control_status(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.control_status_user_data
                            as *mut watch::Sender<std::option::Option<ControlStatusOwned>>,
                    );
                }
            }
            if let Some(handle) = inner.attitude_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_attitude(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.attitude_user_data
                            as *mut watch::Sender<std::option::Option<AttitudeOwned>>,
                    );
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn gimbal_gimbal_list_callback_ffi(
    user_data: *mut c_void,
    gimbal_list: &crate::gimbal::mavsdk::Gimbal_GimbalList,
) {
    if user_data.is_null() {
        return;
    }
    let send_gimbal_list = GimbalList::new(gimbal_list).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<GimbalListOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_gimbal_list));
}
#[unsafe(no_mangle)]
pub extern "C" fn gimbal_control_status_callback_ffi(
    user_data: *mut c_void,
    control_status: &crate::gimbal::mavsdk::Gimbal_ControlStatus,
) {
    if user_data.is_null() {
        return;
    }
    let send_control_status = ControlStatus::new(control_status).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<ControlStatusOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_control_status));
}
#[unsafe(no_mangle)]
pub extern "C" fn gimbal_attitude_callback_ffi(
    user_data: *mut c_void,
    attitude: &crate::gimbal::mavsdk::Gimbal_Attitude,
) {
    if user_data.is_null() {
        return;
    }
    let send_attitude = Attitude::new(attitude).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<AttitudeOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_attitude));
}
