// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use tokio::sync::watch;
use std::sync::Mutex;






pub struct OptionOwned {
        pub option_id: String,
        pub option_description: String,
}

/*  */
pub struct CameraOption<'a> {
    inner: &'a crate::camera::mavsdk::Camera_Option,
}

impl<'a> CameraOption<'a> {
    pub fn new(inner: &'a crate::camera::mavsdk::Camera_Option) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> OptionOwned {
        OptionOwned {
            
            option_id: self.option_id(),
            option_description: self.option_description(),
        }
    }
        ///  Name of the option (machine readable)

    pub fn option_id(&self) -> String {
        self.inner.camera_get_option_id().to_string_lossy().into_owned()
    }
        ///  Description of the option (human readable)

    pub fn option_description(&self) -> String {
        self.inner.camera_get_option_description().to_string_lossy().into_owned()
    }
}


pub struct SettingOwned {
        pub setting_id: String,
        pub setting_description: String,
        pub option: OptionOwned,
        pub is_range: bool,
}

/*  */
pub struct Setting<'a> {
    inner: &'a crate::camera::mavsdk::Camera_Setting,
}

impl<'a> Setting<'a> {
    pub fn new(inner: &'a crate::camera::mavsdk::Camera_Setting) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> SettingOwned {
        SettingOwned {
            
            setting_id: self.setting_id(),
            setting_description: self.setting_description(),
            option: self.option(),
            is_range: self.is_range(),
        }
    }
        ///  Name of a setting (machine readable)

    pub fn setting_id(&self) -> String {
        self.inner.camera_get_setting_id().to_string_lossy().into_owned()
    }
        ///  Description of the setting (human readable). This field is meant to be read from the drone, ignore it when setting.

    pub fn setting_description(&self) -> String {
        self.inner.camera_get_setting_description().to_string_lossy().into_owned()
    }
        ///  Selected option

    pub fn option(&self) -> OptionOwned {CameraOption::new(self.inner.camera_get_option()).into_owned()
    }
        ///  If option is given as a range. This field is meant to be read from the drone, ignore it when setting.

    pub fn is_range(&self) -> bool {
        self.inner.camera_get_is_range()
    }
}


pub struct SettingOptionsOwned {
        pub component_id: i32,
        pub setting_id: String,
        pub setting_description: String,
        pub options: 
            std::vec::Vec<OptionOwned>,
        pub is_range: bool,
}

/*  */
pub struct SettingOptions<'a> {
    inner: &'a crate::camera::mavsdk::Camera_SettingOptions,
}

impl<'a> SettingOptions<'a> {
    pub fn new(inner: &'a crate::camera::mavsdk::Camera_SettingOptions) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> SettingOptionsOwned {
        SettingOptionsOwned {
            
            component_id: self.component_id(),
            setting_id: self.setting_id(),
            setting_description: self.setting_description(),
            options: self.options(),
            is_range: self.is_range(),
        }
    }
        ///  Component ID

    pub fn component_id(&self) -> i32 {
        self.inner.camera_get_component_id()
    }
        ///  Name of the setting (machine readable)

    pub fn setting_id(&self) -> String {
        self.inner.camera_get_setting_id().to_string_lossy().into_owned()
    }
        ///  Description of the setting (human readable)

    pub fn setting_description(&self) -> String {
        self.inner.camera_get_setting_description().to_string_lossy().into_owned()
    }
        ///  List of options or if range [min, max] or [min, max, interval]

    pub fn options(&self) -> 
            std::vec::Vec<OptionOwned> {
            self.inner.camera_get_options()
                .iter()
                .map(|item| CameraOption::new(item).into_owned())
                .collect()
    }
        ///  If option is given as a range

    pub fn is_range(&self) -> bool {
        self.inner.camera_get_is_range()
    }
}


pub struct VideoStreamSettingsOwned {
        pub frame_rate_hz: f32,
        pub horizontal_resolution_pix: u32,
        pub vertical_resolution_pix: u32,
        pub bit_rate_b_s: u32,
        pub rotation_deg: u32,
        pub uri: String,
        pub horizontal_fov_deg: f32,
}

/*  */
pub struct VideoStreamSettings<'a> {
    inner: &'a crate::camera::mavsdk::Camera_VideoStreamSettings,
}

impl<'a> VideoStreamSettings<'a> {
    pub fn new(inner: &'a crate::camera::mavsdk::Camera_VideoStreamSettings) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> VideoStreamSettingsOwned {
        VideoStreamSettingsOwned {
            
            frame_rate_hz: self.frame_rate_hz(),
            horizontal_resolution_pix: self.horizontal_resolution_pix(),
            vertical_resolution_pix: self.vertical_resolution_pix(),
            bit_rate_b_s: self.bit_rate_b_s(),
            rotation_deg: self.rotation_deg(),
            uri: self.uri(),
            horizontal_fov_deg: self.horizontal_fov_deg(),
        }
    }
        ///  Frames per second

    pub fn frame_rate_hz(&self) -> f32 {
        self.inner.camera_get_frame_rate_hz()
    }
        ///  Horizontal resolution (in pixels)

    pub fn horizontal_resolution_pix(&self) -> u32 {
        self.inner.camera_get_horizontal_resolution_pix()
    }
        ///  Vertical resolution (in pixels)

    pub fn vertical_resolution_pix(&self) -> u32 {
        self.inner.camera_get_vertical_resolution_pix()
    }
        ///  Bit rate (in bits per second)

    pub fn bit_rate_b_s(&self) -> u32 {
        self.inner.camera_get_bit_rate_b_s()
    }
        ///  Video image rotation (clockwise, 0-359 degrees)

    pub fn rotation_deg(&self) -> u32 {
        self.inner.camera_get_rotation_deg()
    }
        ///  Video stream URI

    pub fn uri(&self) -> String {
        self.inner.camera_get_uri().to_string_lossy().into_owned()
    }
        ///  Horizontal fov in degrees

    pub fn horizontal_fov_deg(&self) -> f32 {
        self.inner.camera_get_horizontal_fov_deg()
    }
}


pub struct VideoStreamInfoOwned {
        pub stream_id: i32,
        pub settings: VideoStreamSettingsOwned,
        pub status: crate::camera::mavsdk::Camera_VideoStreamInfo_VideoStreamStatus,
        pub spectrum: crate::camera::mavsdk::Camera_VideoStreamInfo_VideoStreamSpectrum,
}

/*  */
pub struct VideoStreamInfo<'a> {
    inner: &'a crate::camera::mavsdk::Camera_VideoStreamInfo,
}

impl<'a> VideoStreamInfo<'a> {
    pub fn new(inner: &'a crate::camera::mavsdk::Camera_VideoStreamInfo) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> VideoStreamInfoOwned {
        VideoStreamInfoOwned {
            
            stream_id: self.stream_id(),
            settings: self.settings(),
            status: self.status(),
            spectrum: self.spectrum(),
        }
    }
        ///  Stream ID

    pub fn stream_id(&self) -> i32 {
        self.inner.camera_get_stream_id()
    }
        ///  Video stream settings

    pub fn settings(&self) -> VideoStreamSettingsOwned {VideoStreamSettings::new(self.inner.camera_get_settings()).into_owned()
    }
        ///  Current status of video streaming

    pub fn status(&self) -> crate::camera::mavsdk::Camera_VideoStreamInfo_VideoStreamStatus {
        self.inner.camera_get_status().clone()
    }
        ///  Light-spectrum of the video stream

    pub fn spectrum(&self) -> crate::camera::mavsdk::Camera_VideoStreamInfo_VideoStreamSpectrum {
        self.inner.camera_get_spectrum().clone()
    }
}


pub struct ModeUpdateOwned {
        pub component_id: i32,
        pub mode: crate::camera::mavsdk::Camera_Mode,
}

/*  */
pub struct ModeUpdate<'a> {
    inner: &'a crate::camera::mavsdk::Camera_ModeUpdate,
}

impl<'a> ModeUpdate<'a> {
    pub fn new(inner: &'a crate::camera::mavsdk::Camera_ModeUpdate) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> ModeUpdateOwned {
        ModeUpdateOwned {
            
            component_id: self.component_id(),
            mode: self.mode(),
        }
    }
        ///  Component ID

    pub fn component_id(&self) -> i32 {
        self.inner.camera_get_component_id()
    }
        ///  Camera mode

    pub fn mode(&self) -> crate::camera::mavsdk::Camera_Mode {self.inner.camera_get_mode().clone()
    }
}


pub struct VideoStreamUpdateOwned {
        pub component_id: i32,
        pub video_stream_info: VideoStreamInfoOwned,
}

/*  */
pub struct VideoStreamUpdate<'a> {
    inner: &'a crate::camera::mavsdk::Camera_VideoStreamUpdate,
}

impl<'a> VideoStreamUpdate<'a> {
    pub fn new(inner: &'a crate::camera::mavsdk::Camera_VideoStreamUpdate) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> VideoStreamUpdateOwned {
        VideoStreamUpdateOwned {
            
            component_id: self.component_id(),
            video_stream_info: self.video_stream_info(),
        }
    }
        ///  Component ID

    pub fn component_id(&self) -> i32 {
        self.inner.camera_get_component_id()
    }
        ///  Video stream info

    pub fn video_stream_info(&self) -> VideoStreamInfoOwned {VideoStreamInfo::new(self.inner.camera_get_video_stream_info()).into_owned()
    }
}


pub struct StorageOwned {
        pub component_id: i32,
        pub video_on: bool,
        pub photo_interval_on: bool,
        pub used_storage_mib: f32,
        pub available_storage_mib: f32,
        pub total_storage_mib: f32,
        pub recording_time_s: f32,
        pub media_folder_name: String,
        pub storage_status: crate::camera::mavsdk::Camera_Storage_StorageStatus,
        pub storage_id: u32,
        pub storage_type: crate::camera::mavsdk::Camera_Storage_StorageType,
}

/*  */
pub struct Storage<'a> {
    inner: &'a crate::camera::mavsdk::Camera_Storage,
}

impl<'a> Storage<'a> {
    pub fn new(inner: &'a crate::camera::mavsdk::Camera_Storage) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> StorageOwned {
        StorageOwned {
            
            component_id: self.component_id(),
            video_on: self.video_on(),
            photo_interval_on: self.photo_interval_on(),
            used_storage_mib: self.used_storage_mib(),
            available_storage_mib: self.available_storage_mib(),
            total_storage_mib: self.total_storage_mib(),
            recording_time_s: self.recording_time_s(),
            media_folder_name: self.media_folder_name(),
            storage_status: self.storage_status(),
            storage_id: self.storage_id(),
            storage_type: self.storage_type(),
        }
    }
        ///  Component ID

    pub fn component_id(&self) -> i32 {
        self.inner.camera_get_component_id()
    }
        ///  Whether video recording is currently in process

    pub fn video_on(&self) -> bool {
        self.inner.camera_get_video_on()
    }
        ///  Whether a photo interval is currently in process

    pub fn photo_interval_on(&self) -> bool {
        self.inner.camera_get_photo_interval_on()
    }
        ///  Used storage (in MiB)

    pub fn used_storage_mib(&self) -> f32 {
        self.inner.camera_get_used_storage_mib()
    }
        ///  Available storage (in MiB)

    pub fn available_storage_mib(&self) -> f32 {
        self.inner.camera_get_available_storage_mib()
    }
        ///  Total storage (in MiB)

    pub fn total_storage_mib(&self) -> f32 {
        self.inner.camera_get_total_storage_mib()
    }
        ///  Elapsed time since starting the video recording (in seconds)

    pub fn recording_time_s(&self) -> f32 {
        self.inner.camera_get_recording_time_s()
    }
        ///  Current folder name where media are saved

    pub fn media_folder_name(&self) -> String {
        self.inner.camera_get_media_folder_name().to_string_lossy().into_owned()
    }
        ///  Storage status

    pub fn storage_status(&self) -> crate::camera::mavsdk::Camera_Storage_StorageStatus {
        self.inner.camera_get_storage_status().clone()
    }
        ///  Storage ID starting at 1

    pub fn storage_id(&self) -> u32 {
        self.inner.camera_get_storage_id()
    }
        ///  Storage type

    pub fn storage_type(&self) -> crate::camera::mavsdk::Camera_Storage_StorageType {
        self.inner.camera_get_storage_type().clone()
    }
}


pub struct StorageUpdateOwned {
        pub component_id: i32,
        pub storage: StorageOwned,
}

/*  */
pub struct StorageUpdate<'a> {
    inner: &'a crate::camera::mavsdk::Camera_StorageUpdate,
}

impl<'a> StorageUpdate<'a> {
    pub fn new(inner: &'a crate::camera::mavsdk::Camera_StorageUpdate) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> StorageUpdateOwned {
        StorageUpdateOwned {
            
            component_id: self.component_id(),
            storage: self.storage(),
        }
    }
        ///  Component ID

    pub fn component_id(&self) -> i32 {
        self.inner.camera_get_component_id()
    }
        ///  Storage

    pub fn storage(&self) -> StorageOwned {Storage::new(self.inner.camera_get_storage()).into_owned()
    }
}


pub struct CurrentSettingsUpdateOwned {
        pub component_id: i32,
        pub current_settings: 
            std::vec::Vec<SettingOwned>,
}

/*  */
pub struct CurrentSettingsUpdate<'a> {
    inner: &'a crate::camera::mavsdk::Camera_CurrentSettingsUpdate,
}

impl<'a> CurrentSettingsUpdate<'a> {
    pub fn new(inner: &'a crate::camera::mavsdk::Camera_CurrentSettingsUpdate) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> CurrentSettingsUpdateOwned {
        CurrentSettingsUpdateOwned {
            
            component_id: self.component_id(),
            current_settings: self.current_settings(),
        }
    }
        ///  Component ID

    pub fn component_id(&self) -> i32 {
        self.inner.camera_get_component_id()
    }
        ///  List of current settings

    pub fn current_settings(&self) -> 
            std::vec::Vec<SettingOwned> {
            self.inner.camera_get_current_settings()
                .iter()
                .map(|item| Setting::new(item).into_owned())
                .collect()
    }
}


pub struct PossibleSettingOptionsUpdateOwned {
        pub component_id: i32,
        pub setting_options: 
            std::vec::Vec<SettingOptionsOwned>,
}

/*  */
pub struct PossibleSettingOptionsUpdate<'a> {
    inner: &'a crate::camera::mavsdk::Camera_PossibleSettingOptionsUpdate,
}

impl<'a> PossibleSettingOptionsUpdate<'a> {
    pub fn new(inner: &'a crate::camera::mavsdk::Camera_PossibleSettingOptionsUpdate) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> PossibleSettingOptionsUpdateOwned {
        PossibleSettingOptionsUpdateOwned {
            
            component_id: self.component_id(),
            setting_options: self.setting_options(),
        }
    }
        ///  Component ID

    pub fn component_id(&self) -> i32 {
        self.inner.camera_get_component_id()
    }
        ///  List of settings that can be changed

    pub fn setting_options(&self) -> 
            std::vec::Vec<SettingOptionsOwned> {
            self.inner.camera_get_setting_options()
                .iter()
                .map(|item| SettingOptions::new(item).into_owned())
                .collect()
    }
}


pub struct PositionOwned {
        pub latitude_deg: f64,
        pub longitude_deg: f64,
        pub absolute_altitude_m: f32,
        pub relative_altitude_m: f32,
}

/*  */
pub struct Position<'a> {
    inner: &'a crate::camera::mavsdk::Camera_Position,
}

impl<'a> Position<'a> {
    pub fn new(inner: &'a crate::camera::mavsdk::Camera_Position) -> Self {
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
        self.inner.camera_get_latitude_deg()
    }
        ///  Longitude in degrees (range: -180 to +180)

    pub fn longitude_deg(&self) -> f64 {
        self.inner.camera_get_longitude_deg()
    }
        ///  Altitude AMSL (above mean sea level) in metres

    pub fn absolute_altitude_m(&self) -> f32 {
        self.inner.camera_get_absolute_altitude_m()
    }
        ///  Altitude relative to takeoff altitude in metres

    pub fn relative_altitude_m(&self) -> f32 {
        self.inner.camera_get_relative_altitude_m()
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
    inner: &'a crate::camera::mavsdk::Camera_Quaternion,
}

impl<'a> Quaternion<'a> {
    pub fn new(inner: &'a crate::camera::mavsdk::Camera_Quaternion) -> Self {
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
        self.inner.camera_get_w()
    }
        ///  Quaternion entry 1, also denoted as b

    pub fn x(&self) -> f32 {
        self.inner.camera_get_x()
    }
        ///  Quaternion entry 2, also denoted as c

    pub fn y(&self) -> f32 {
        self.inner.camera_get_y()
    }
        ///  Quaternion entry 3, also denoted as d

    pub fn z(&self) -> f32 {
        self.inner.camera_get_z()
    }
}


pub struct EulerAngleOwned {
        pub roll_deg: f32,
        pub pitch_deg: f32,
        pub yaw_deg: f32,
}

/*  */
pub struct EulerAngle<'a> {
    inner: &'a crate::camera::mavsdk::Camera_EulerAngle,
}

impl<'a> EulerAngle<'a> {
    pub fn new(inner: &'a crate::camera::mavsdk::Camera_EulerAngle) -> Self {
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
        self.inner.camera_get_roll_deg()
    }
        ///  Pitch angle in degrees, positive is pitching nose up

    pub fn pitch_deg(&self) -> f32 {
        self.inner.camera_get_pitch_deg()
    }
        ///  Yaw angle in degrees, positive is clock-wise seen from above

    pub fn yaw_deg(&self) -> f32 {
        self.inner.camera_get_yaw_deg()
    }
}


pub struct CaptureInfoOwned {
        pub component_id: i32,
        pub position: PositionOwned,
        pub attitude_quaternion: QuaternionOwned,
        pub attitude_euler_angle: EulerAngleOwned,
        pub time_utc_us: u64,
        pub is_success: bool,
        pub index: i32,
        pub file_url: String,
}

/*  */
pub struct CaptureInfo<'a> {
    inner: &'a crate::camera::mavsdk::Camera_CaptureInfo,
}

impl<'a> CaptureInfo<'a> {
    pub fn new(inner: &'a crate::camera::mavsdk::Camera_CaptureInfo) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> CaptureInfoOwned {
        CaptureInfoOwned {
            
            component_id: self.component_id(),
            position: self.position(),
            attitude_quaternion: self.attitude_quaternion(),
            attitude_euler_angle: self.attitude_euler_angle(),
            time_utc_us: self.time_utc_us(),
            is_success: self.is_success(),
            index: self.index(),
            file_url: self.file_url(),
        }
    }
        ///  Component ID

    pub fn component_id(&self) -> i32 {
        self.inner.camera_get_component_id()
    }
        ///  Location where the picture was taken

    pub fn position(&self) -> PositionOwned {
        Position::new(self.inner.camera_get_position()).into_owned()
    }
        ///  Attitude of the camera when the picture was taken (quaternion)

    pub fn attitude_quaternion(&self) -> QuaternionOwned {
        Quaternion::new(self.inner.camera_get_attitude_quaternion()).into_owned()
    }
        ///  Attitude of the camera when the picture was taken (euler angle)

    pub fn attitude_euler_angle(&self) -> EulerAngleOwned {
        EulerAngle::new(self.inner.camera_get_attitude_euler_angle()).into_owned()
    }
        ///  Timestamp in UTC (since UNIX epoch) in microseconds

    pub fn time_utc_us(&self) -> u64 {
        self.inner.camera_get_time_utc_us()
    }
        ///  True if the capture was successful

    pub fn is_success(&self) -> bool {
        self.inner.camera_get_is_success()
    }
        ///  Zero-based index of this image since vehicle was armed

    pub fn index(&self) -> i32 {
        self.inner.camera_get_index()
    }
        ///  Download URL of this image

    pub fn file_url(&self) -> String {
        self.inner.camera_get_file_url().to_string_lossy().into_owned()
    }
}


pub struct InformationOwned {
        pub component_id: i32,
        pub vendor_name: String,
        pub model_name: String,
        pub focal_length_mm: f32,
        pub horizontal_sensor_size_mm: f32,
        pub vertical_sensor_size_mm: f32,
        pub horizontal_resolution_px: u32,
        pub vertical_resolution_px: u32,
}

/*  */
pub struct Information<'a> {
    inner: &'a crate::camera::mavsdk::Camera_Information,
}

impl<'a> Information<'a> {
    pub fn new(inner: &'a crate::camera::mavsdk::Camera_Information) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> InformationOwned {
        InformationOwned {
            
            component_id: self.component_id(),
            vendor_name: self.vendor_name(),
            model_name: self.model_name(),
            focal_length_mm: self.focal_length_mm(),
            horizontal_sensor_size_mm: self.horizontal_sensor_size_mm(),
            vertical_sensor_size_mm: self.vertical_sensor_size_mm(),
            horizontal_resolution_px: self.horizontal_resolution_px(),
            vertical_resolution_px: self.vertical_resolution_px(),
        }
    }
        ///  Component ID

    pub fn component_id(&self) -> i32 {
        self.inner.camera_get_component_id()
    }
        ///  Name of the camera vendor

    pub fn vendor_name(&self) -> String {
        self.inner.camera_get_vendor_name().to_string_lossy().into_owned()
    }
        ///  Name of the camera model

    pub fn model_name(&self) -> String {
        self.inner.camera_get_model_name().to_string_lossy().into_owned()
    }
        ///  Focal length

    pub fn focal_length_mm(&self) -> f32 {
        self.inner.camera_get_focal_length_mm()
    }
        ///  Horizontal sensor size

    pub fn horizontal_sensor_size_mm(&self) -> f32 {
        self.inner.camera_get_horizontal_sensor_size_mm()
    }
        ///  Vertical sensor size

    pub fn vertical_sensor_size_mm(&self) -> f32 {
        self.inner.camera_get_vertical_sensor_size_mm()
    }
        ///  Horizontal image resolution in pixels

    pub fn horizontal_resolution_px(&self) -> u32 {
        self.inner.camera_get_horizontal_resolution_px()
    }
        ///  Vertical image resolution in pixels

    pub fn vertical_resolution_px(&self) -> u32 {
        self.inner.camera_get_vertical_resolution_px()
    }
}


pub struct CameraListOwned {
        pub cameras: 
            std::vec::Vec<InformationOwned>,
}

/*  */
pub struct CameraList<'a> {
    inner: &'a crate::camera::mavsdk::Camera_CameraList,
}

impl<'a> CameraList<'a> {
    pub fn new(inner: &'a crate::camera::mavsdk::Camera_CameraList) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> CameraListOwned {
        CameraListOwned {
            
            cameras: self.cameras(),
        }
    }
        ///  Camera items.

    pub fn cameras(&self) -> 
            std::vec::Vec<InformationOwned> {
            self.inner.camera_get_cameras()
                .iter()
                .map(|item| Information::new(item).into_owned())
                .collect()
    }
}

    struct CameraInner {
        plugin: cxx::UniquePtr<crate::camera::mavsdk::Camera>,
        
                /// CameraList State
                camera_list_handle: Option<usize>,
                camera_list_user_data: *mut c_void,
                /// Mode State
                mode_handle: Option<usize>,
                mode_user_data: *mut c_void,
                /// VideoStreamInfo State
                video_stream_info_handle: Option<usize>,
                video_stream_info_user_data: *mut c_void,
                /// CaptureInfo State
                capture_info_handle: Option<usize>,
                capture_info_user_data: *mut c_void,
                /// Storage State
                storage_handle: Option<usize>,
                storage_user_data: *mut c_void,
                /// CurrentSettings State
                current_settings_handle: Option<usize>,
                current_settings_user_data: *mut c_void,
                /// PossibleSettingOptions State
                possible_setting_options_handle: Option<usize>,
                possible_setting_options_user_data: *mut c_void,
    }

    pub struct CameraClient {
        inner: Mutex<CameraInner>,
        
                // Producer for the latest CameraList state; broadcast to all active subscribers.
                camera_list_tx: watch::Sender<std::option::Option<
        CameraListOwned>>,
                // Producer for the latest Mode state; broadcast to all active subscribers.
                mode_tx: watch::Sender<std::option::Option<
        ModeUpdateOwned>>,
                // Producer for the latest VideoStreamInfo state; broadcast to all active subscribers.
                video_stream_info_tx: watch::Sender<std::option::Option<
        VideoStreamUpdateOwned>>,
                // Producer for the latest CaptureInfo state; broadcast to all active subscribers.
                capture_info_tx: watch::Sender<std::option::Option<
        CaptureInfoOwned>>,
                // Producer for the latest Storage state; broadcast to all active subscribers.
                storage_tx: watch::Sender<std::option::Option<
        StorageUpdateOwned>>,
                // Producer for the latest CurrentSettings state; broadcast to all active subscribers.
                current_settings_tx: watch::Sender<std::option::Option<
        CurrentSettingsUpdateOwned>>,
                // Producer for the latest PossibleSettingOptions state; broadcast to all active subscribers.
                possible_setting_options_tx: watch::Sender<std::option::Option<
        PossibleSettingOptionsUpdateOwned>>,
    }

    impl CameraClient {
        pub fn new(plugin: cxx::UniquePtr<crate::camera::mavsdk::Camera>) -> Self {
            
                    let (camera_list_tx, _) = watch::channel(None);
                    let (mode_tx, _) = watch::channel(None);
                    let (video_stream_info_tx, _) = watch::channel(None);
                    let (capture_info_tx, _) = watch::channel(None);
                    let (storage_tx, _) = watch::channel(None);
                    let (current_settings_tx, _) = watch::channel(None);
                    let (possible_setting_options_tx, _) = watch::channel(None);

            Self {
                inner: Mutex::new(CameraInner {
                    plugin,
                    
                            camera_list_handle: None,
                            camera_list_user_data: std::ptr::null_mut(),
                            mode_handle: None,
                            mode_user_data: std::ptr::null_mut(),
                            video_stream_info_handle: None,
                            video_stream_info_user_data: std::ptr::null_mut(),
                            capture_info_handle: None,
                            capture_info_user_data: std::ptr::null_mut(),
                            storage_handle: None,
                            storage_user_data: std::ptr::null_mut(),
                            current_settings_handle: None,
                            current_settings_user_data: std::ptr::null_mut(),
                            possible_setting_options_handle: None,
                            possible_setting_options_user_data: std::ptr::null_mut(),
                }),
                
                        camera_list_tx,
                        mode_tx,
                        video_stream_info_tx,
                        capture_info_tx,
                        storage_tx,
                        current_settings_tx,
                        possible_setting_options_tx,
            }
        }
        
        pub fn subscribe_camera_list(&self) -> watch::Receiver<std::option::Option<
        CameraListOwned>> {
                    let mut inner = self.inner.lock().unwrap();

                    if inner.camera_list_handle.is_none() {
                        // Lazy initialization of C++ 'camera_list' camera stream.
                        let user_data_ptr = Box::into_raw(Box::new(self.camera_list_tx.clone())) as *mut c_void;

                        let handle = inner.plugin.pin_mut().subscribe_camera_list(
                            camera_camera_list_callback_ffi as usize as libc::uintptr_t,
                            user_data_ptr as libc::uintptr_t
                        ) as usize;

                        inner.camera_list_handle = Some(handle);
                        inner.camera_list_user_data = user_data_ptr;
                    }
                    
                    self.camera_list_tx.subscribe()
                }pub fn subscribe_mode(&self) -> watch::Receiver<std::option::Option<
        ModeUpdateOwned>> {
                    let mut inner = self.inner.lock().unwrap();

                    if inner.mode_handle.is_none() {
                        // Lazy initialization of C++ 'mode' camera stream.
                        let user_data_ptr = Box::into_raw(Box::new(self.mode_tx.clone())) as *mut c_void;

                        let handle = inner.plugin.pin_mut().subscribe_mode(
                            camera_mode_callback_ffi as usize as libc::uintptr_t,
                            user_data_ptr as libc::uintptr_t
                        ) as usize;

                        inner.mode_handle = Some(handle);
                        inner.mode_user_data = user_data_ptr;
                    }
                    
                    self.mode_tx.subscribe()
                }pub fn subscribe_video_stream_info(&self) -> watch::Receiver<std::option::Option<
        VideoStreamUpdateOwned>> {
                    let mut inner = self.inner.lock().unwrap();

                    if inner.video_stream_info_handle.is_none() {
                        // Lazy initialization of C++ 'video_stream_info' camera stream.
                        let user_data_ptr = Box::into_raw(Box::new(self.video_stream_info_tx.clone())) as *mut c_void;

                        let handle = inner.plugin.pin_mut().subscribe_video_stream_info(
                            camera_video_stream_info_callback_ffi as usize as libc::uintptr_t,
                            user_data_ptr as libc::uintptr_t
                        ) as usize;

                        inner.video_stream_info_handle = Some(handle);
                        inner.video_stream_info_user_data = user_data_ptr;
                    }
                    
                    self.video_stream_info_tx.subscribe()
                }pub fn subscribe_capture_info(&self) -> watch::Receiver<std::option::Option<
        CaptureInfoOwned>> {
                    let mut inner = self.inner.lock().unwrap();

                    if inner.capture_info_handle.is_none() {
                        // Lazy initialization of C++ 'capture_info' camera stream.
                        let user_data_ptr = Box::into_raw(Box::new(self.capture_info_tx.clone())) as *mut c_void;

                        let handle = inner.plugin.pin_mut().subscribe_capture_info(
                            camera_capture_info_callback_ffi as usize as libc::uintptr_t,
                            user_data_ptr as libc::uintptr_t
                        ) as usize;

                        inner.capture_info_handle = Some(handle);
                        inner.capture_info_user_data = user_data_ptr;
                    }
                    
                    self.capture_info_tx.subscribe()
                }pub fn subscribe_storage(&self) -> watch::Receiver<std::option::Option<
        StorageUpdateOwned>> {
                    let mut inner = self.inner.lock().unwrap();

                    if inner.storage_handle.is_none() {
                        // Lazy initialization of C++ 'storage' camera stream.
                        let user_data_ptr = Box::into_raw(Box::new(self.storage_tx.clone())) as *mut c_void;

                        let handle = inner.plugin.pin_mut().subscribe_storage(
                            camera_storage_callback_ffi as usize as libc::uintptr_t,
                            user_data_ptr as libc::uintptr_t
                        ) as usize;

                        inner.storage_handle = Some(handle);
                        inner.storage_user_data = user_data_ptr;
                    }
                    
                    self.storage_tx.subscribe()
                }pub fn subscribe_current_settings(&self) -> watch::Receiver<std::option::Option<
        CurrentSettingsUpdateOwned>> {
                    let mut inner = self.inner.lock().unwrap();

                    if inner.current_settings_handle.is_none() {
                        // Lazy initialization of C++ 'current_settings' camera stream.
                        let user_data_ptr = Box::into_raw(Box::new(self.current_settings_tx.clone())) as *mut c_void;

                        let handle = inner.plugin.pin_mut().subscribe_current_settings(
                            camera_current_settings_callback_ffi as usize as libc::uintptr_t,
                            user_data_ptr as libc::uintptr_t
                        ) as usize;

                        inner.current_settings_handle = Some(handle);
                        inner.current_settings_user_data = user_data_ptr;
                    }
                    
                    self.current_settings_tx.subscribe()
                }pub fn subscribe_possible_setting_options(&self) -> watch::Receiver<std::option::Option<
        PossibleSettingOptionsUpdateOwned>> {
                    let mut inner = self.inner.lock().unwrap();

                    if inner.possible_setting_options_handle.is_none() {
                        // Lazy initialization of C++ 'possible_setting_options' camera stream.
                        let user_data_ptr = Box::into_raw(Box::new(self.possible_setting_options_tx.clone())) as *mut c_void;

                        let handle = inner.plugin.pin_mut().subscribe_possible_setting_options(
                            camera_possible_setting_options_callback_ffi as usize as libc::uintptr_t,
                            user_data_ptr as libc::uintptr_t
                        ) as usize;

                        inner.possible_setting_options_handle = Some(handle);
                        inner.possible_setting_options_user_data = user_data_ptr;
                    }
                    
                    self.possible_setting_options_tx.subscribe()
                }
    }


        #[unsafe(no_mangle)]
        pub extern "C" fn camera_camera_list_callback_ffi(
            user_data: *mut c_void,
            camera_list: &crate::camera::mavsdk::Camera_CameraList,
        ) {
            if user_data.is_null() { return; }
                let send_camera_list = CameraList::new(camera_list).into_owned();

            // Cast the pointer to watch::Sender
            let sender = unsafe {
                &*(user_data as *const watch::Sender<std::option::Option<
        CameraListOwned>>)
            };

            // Overwrite the current value in the channel with the new update
            let _ = sender.send(Some(send_camera_list));
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn camera_mode_callback_ffi(
            user_data: *mut c_void,
            mode: &crate::camera::mavsdk::Camera_ModeUpdate,
        ) {
            if user_data.is_null() { return; }
                let send_mode = ModeUpdate::new(mode).into_owned();

            // Cast the pointer to watch::Sender
            let sender = unsafe {
                &*(user_data as *const watch::Sender<std::option::Option<
        ModeUpdateOwned>>)
            };

            // Overwrite the current value in the channel with the new update
            let _ = sender.send(Some(send_mode));
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn camera_video_stream_info_callback_ffi(
            user_data: *mut c_void,
            video_stream_info: &crate::camera::mavsdk::Camera_VideoStreamUpdate,
        ) {
            if user_data.is_null() { return; }
                let send_video_stream_info = VideoStreamUpdate::new(video_stream_info).into_owned();

            // Cast the pointer to watch::Sender
            let sender = unsafe {
                &*(user_data as *const watch::Sender<std::option::Option<
        VideoStreamUpdateOwned>>)
            };

            // Overwrite the current value in the channel with the new update
            let _ = sender.send(Some(send_video_stream_info));
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn camera_capture_info_callback_ffi(
            user_data: *mut c_void,
            capture_info: &crate::camera::mavsdk::Camera_CaptureInfo,
        ) {
            if user_data.is_null() { return; }
                let send_capture_info = CaptureInfo::new(capture_info).into_owned();

            // Cast the pointer to watch::Sender
            let sender = unsafe {
                &*(user_data as *const watch::Sender<std::option::Option<
        CaptureInfoOwned>>)
            };

            // Overwrite the current value in the channel with the new update
            let _ = sender.send(Some(send_capture_info));
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn camera_storage_callback_ffi(
            user_data: *mut c_void,
            storage: &crate::camera::mavsdk::Camera_StorageUpdate,
        ) {
            if user_data.is_null() { return; }
                let send_storage = StorageUpdate::new(storage).into_owned();

            // Cast the pointer to watch::Sender
            let sender = unsafe {
                &*(user_data as *const watch::Sender<std::option::Option<
        StorageUpdateOwned>>)
            };

            // Overwrite the current value in the channel with the new update
            let _ = sender.send(Some(send_storage));
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn camera_current_settings_callback_ffi(
            user_data: *mut c_void,
            current_settings: &crate::camera::mavsdk::Camera_CurrentSettingsUpdate,
        ) {
            if user_data.is_null() { return; }
                let send_current_settings = CurrentSettingsUpdate::new(current_settings).into_owned();

            // Cast the pointer to watch::Sender
            let sender = unsafe {
                &*(user_data as *const watch::Sender<std::option::Option<
        CurrentSettingsUpdateOwned>>)
            };

            // Overwrite the current value in the channel with the new update
            let _ = sender.send(Some(send_current_settings));
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn camera_possible_setting_options_callback_ffi(
            user_data: *mut c_void,
            possible_setting_options: &crate::camera::mavsdk::Camera_PossibleSettingOptionsUpdate,
        ) {
            if user_data.is_null() { return; }
                let send_possible_setting_options = PossibleSettingOptionsUpdate::new(possible_setting_options).into_owned();

            // Cast the pointer to watch::Sender
            let sender = unsafe {
                &*(user_data as *const watch::Sender<std::option::Option<
        PossibleSettingOptionsUpdateOwned>>)
            };

            // Overwrite the current value in the channel with the new update
            let _ = sender.send(Some(send_possible_setting_options));
        }