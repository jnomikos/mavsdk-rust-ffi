// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct InformationOwned {
    pub vendor_name: String,
    pub model_name: String,
    pub firmware_version: String,
    pub focal_length_mm: f32,
    pub horizontal_sensor_size_mm: f32,
    pub vertical_sensor_size_mm: f32,
    pub horizontal_resolution_px: u32,
    pub vertical_resolution_px: u32,
    pub lens_id: u32,
    pub definition_file_version: u32,
    pub definition_file_uri: String,
    pub image_in_video_mode_supported: bool,
    pub video_in_image_mode_supported: bool,
}

/*  */
pub struct Information<'a> {
    inner: &'a crate::camera_server::mavsdk::CameraServer_Information,
}

impl<'a> Information<'a> {
    pub fn new(inner: &'a crate::camera_server::mavsdk::CameraServer_Information) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> InformationOwned {
        InformationOwned {
            vendor_name: self.vendor_name(),
            model_name: self.model_name(),
            firmware_version: self.firmware_version(),
            focal_length_mm: self.focal_length_mm(),
            horizontal_sensor_size_mm: self.horizontal_sensor_size_mm(),
            vertical_sensor_size_mm: self.vertical_sensor_size_mm(),
            horizontal_resolution_px: self.horizontal_resolution_px(),
            vertical_resolution_px: self.vertical_resolution_px(),
            lens_id: self.lens_id(),
            definition_file_version: self.definition_file_version(),
            definition_file_uri: self.definition_file_uri(),
            image_in_video_mode_supported: self.image_in_video_mode_supported(),
            video_in_image_mode_supported: self.video_in_image_mode_supported(),
        }
    }
    ///  Name of the camera vendor

    pub fn vendor_name(&self) -> String {
        self.inner
            .camera_server_get_vendor_name()
            .to_string_lossy()
            .into_owned()
    }
    ///  Name of the camera model

    pub fn model_name(&self) -> String {
        self.inner
            .camera_server_get_model_name()
            .to_string_lossy()
            .into_owned()
    }
    ///  Camera firmware version in major[.minor[.patch[.dev]]] format

    pub fn firmware_version(&self) -> String {
        self.inner
            .camera_server_get_firmware_version()
            .to_string_lossy()
            .into_owned()
    }
    ///  Focal length

    pub fn focal_length_mm(&self) -> f32 {
        self.inner.camera_server_get_focal_length_mm()
    }
    ///  Horizontal sensor size

    pub fn horizontal_sensor_size_mm(&self) -> f32 {
        self.inner.camera_server_get_horizontal_sensor_size_mm()
    }
    ///  Vertical sensor size

    pub fn vertical_sensor_size_mm(&self) -> f32 {
        self.inner.camera_server_get_vertical_sensor_size_mm()
    }
    ///  Horizontal image resolution in pixels

    pub fn horizontal_resolution_px(&self) -> u32 {
        self.inner.camera_server_get_horizontal_resolution_px()
    }
    ///  Vertical image resolution in pixels

    pub fn vertical_resolution_px(&self) -> u32 {
        self.inner.camera_server_get_vertical_resolution_px()
    }
    ///  Lens ID

    pub fn lens_id(&self) -> u32 {
        self.inner.camera_server_get_lens_id()
    }
    ///  Camera definition file version (iteration)

    pub fn definition_file_version(&self) -> u32 {
        self.inner.camera_server_get_definition_file_version()
    }
    ///  Camera definition URI (http or mavlink ftp)

    pub fn definition_file_uri(&self) -> String {
        self.inner
            .camera_server_get_definition_file_uri()
            .to_string_lossy()
            .into_owned()
    }
    ///  Camera supports taking images while in video mode

    pub fn image_in_video_mode_supported(&self) -> bool {
        self.inner.camera_server_get_image_in_video_mode_supported()
    }
    ///  Camera supports recording video while in image mode

    pub fn video_in_image_mode_supported(&self) -> bool {
        self.inner.camera_server_get_video_in_image_mode_supported()
    }
}

pub struct VideoStreamingOwned {
    pub has_rtsp_server: bool,
    pub rtsp_uri: String,
}

/*  */
pub struct VideoStreaming<'a> {
    inner: &'a crate::camera_server::mavsdk::CameraServer_VideoStreaming,
}

impl<'a> VideoStreaming<'a> {
    pub fn new(inner: &'a crate::camera_server::mavsdk::CameraServer_VideoStreaming) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> VideoStreamingOwned {
        VideoStreamingOwned {
            has_rtsp_server: self.has_rtsp_server(),
            rtsp_uri: self.rtsp_uri(),
        }
    }
    ///  True if the capture was successful

    pub fn has_rtsp_server(&self) -> bool {
        self.inner.camera_server_get_has_rtsp_server()
    }
    ///  RTSP URI (e.g. rtsp://192.168.1.42:8554/live)

    pub fn rtsp_uri(&self) -> String {
        self.inner
            .camera_server_get_rtsp_uri()
            .to_string_lossy()
            .into_owned()
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
    inner: &'a crate::camera_server::mavsdk::CameraServer_Position,
}

impl<'a> Position<'a> {
    pub fn new(inner: &'a crate::camera_server::mavsdk::CameraServer_Position) -> Self {
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
        self.inner.camera_server_get_latitude_deg()
    }
    ///  Longitude in degrees (range: -180 to +180)

    pub fn longitude_deg(&self) -> f64 {
        self.inner.camera_server_get_longitude_deg()
    }
    ///  Altitude AMSL (above mean sea level) in metres

    pub fn absolute_altitude_m(&self) -> f32 {
        self.inner.camera_server_get_absolute_altitude_m()
    }
    ///  Altitude relative to takeoff altitude in metres

    pub fn relative_altitude_m(&self) -> f32 {
        self.inner.camera_server_get_relative_altitude_m()
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
    inner: &'a crate::camera_server::mavsdk::CameraServer_Quaternion,
}

impl<'a> Quaternion<'a> {
    pub fn new(inner: &'a crate::camera_server::mavsdk::CameraServer_Quaternion) -> Self {
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
        self.inner.camera_server_get_w()
    }
    ///  Quaternion entry 1, also denoted as b

    pub fn x(&self) -> f32 {
        self.inner.camera_server_get_x()
    }
    ///  Quaternion entry 2, also denoted as c

    pub fn y(&self) -> f32 {
        self.inner.camera_server_get_y()
    }
    ///  Quaternion entry 3, also denoted as d

    pub fn z(&self) -> f32 {
        self.inner.camera_server_get_z()
    }
}

pub struct CaptureInfoOwned {
    pub position: PositionOwned,
    pub attitude_quaternion: QuaternionOwned,
    pub time_utc_us: u64,
    pub is_success: bool,
    pub index: i32,
    pub file_url: String,
}

/*  */
pub struct CaptureInfo<'a> {
    inner: &'a crate::camera_server::mavsdk::CameraServer_CaptureInfo,
}

impl<'a> CaptureInfo<'a> {
    pub fn new(inner: &'a crate::camera_server::mavsdk::CameraServer_CaptureInfo) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> CaptureInfoOwned {
        CaptureInfoOwned {
            position: self.position(),
            attitude_quaternion: self.attitude_quaternion(),
            time_utc_us: self.time_utc_us(),
            is_success: self.is_success(),
            index: self.index(),
            file_url: self.file_url(),
        }
    }
    ///  Location where the picture was taken

    pub fn position(&self) -> PositionOwned {
        Position::new(self.inner.camera_server_get_position()).into_owned()
    }
    ///  Attitude of the camera when the picture was taken (quaternion)

    pub fn attitude_quaternion(&self) -> QuaternionOwned {
        Quaternion::new(self.inner.camera_server_get_attitude_quaternion()).into_owned()
    }
    ///  Timestamp in UTC (since UNIX epoch) in microseconds

    pub fn time_utc_us(&self) -> u64 {
        self.inner.camera_server_get_time_utc_us()
    }
    ///  True if the capture was successful

    pub fn is_success(&self) -> bool {
        self.inner.camera_server_get_is_success()
    }
    ///  Index from TakePhotoResponse

    pub fn index(&self) -> i32 {
        self.inner.camera_server_get_index()
    }
    ///  Download URL of this image

    pub fn file_url(&self) -> String {
        self.inner
            .camera_server_get_file_url()
            .to_string_lossy()
            .into_owned()
    }
}

pub struct StorageInformationOwned {
    pub used_storage_mib: f32,
    pub available_storage_mib: f32,
    pub total_storage_mib: f32,
    pub storage_status: crate::camera_server::mavsdk::CameraServer_StorageInformation_StorageStatus,
    pub storage_id: u32,
    pub storage_type: crate::camera_server::mavsdk::CameraServer_StorageInformation_StorageType,
    pub read_speed_mib_s: f32,
    pub write_speed_mib_s: f32,
}

/*  */
pub struct StorageInformation<'a> {
    inner: &'a crate::camera_server::mavsdk::CameraServer_StorageInformation,
}

impl<'a> StorageInformation<'a> {
    pub fn new(inner: &'a crate::camera_server::mavsdk::CameraServer_StorageInformation) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> StorageInformationOwned {
        StorageInformationOwned {
            used_storage_mib: self.used_storage_mib(),
            available_storage_mib: self.available_storage_mib(),
            total_storage_mib: self.total_storage_mib(),
            storage_status: self.storage_status(),
            storage_id: self.storage_id(),
            storage_type: self.storage_type(),
            read_speed_mib_s: self.read_speed_mib_s(),
            write_speed_mib_s: self.write_speed_mib_s(),
        }
    }
    ///  Used storage (in MiB)

    pub fn used_storage_mib(&self) -> f32 {
        self.inner.camera_server_get_used_storage_mib()
    }
    ///  Available storage (in MiB)

    pub fn available_storage_mib(&self) -> f32 {
        self.inner.camera_server_get_available_storage_mib()
    }
    ///  Total storage (in MiB)

    pub fn total_storage_mib(&self) -> f32 {
        self.inner.camera_server_get_total_storage_mib()
    }
    ///  Storage status

    pub fn storage_status(
        &self,
    ) -> crate::camera_server::mavsdk::CameraServer_StorageInformation_StorageStatus {
        self.inner.camera_server_get_storage_status().clone()
    }
    ///  Storage ID starting at 1

    pub fn storage_id(&self) -> u32 {
        self.inner.camera_server_get_storage_id()
    }
    ///  Storage type

    pub fn storage_type(
        &self,
    ) -> crate::camera_server::mavsdk::CameraServer_StorageInformation_StorageType {
        self.inner.camera_server_get_storage_type().clone()
    }
    ///  Read speed [MiB/s]

    pub fn read_speed_mib_s(&self) -> f32 {
        self.inner.camera_server_get_read_speed_mib_s()
    }
    ///  Write speed [MiB/s]

    pub fn write_speed_mib_s(&self) -> f32 {
        self.inner.camera_server_get_write_speed_mib_s()
    }
}

pub struct CaptureStatusOwned {
    pub image_interval_s: f32,
    pub recording_time_s: f32,
    pub available_capacity_mib: f32,
    pub image_status: crate::camera_server::mavsdk::CameraServer_CaptureStatus_ImageStatus,
    pub video_status: crate::camera_server::mavsdk::CameraServer_CaptureStatus_VideoStatus,
    pub image_count: i32,
}

/*  */
pub struct CaptureStatus<'a> {
    inner: &'a crate::camera_server::mavsdk::CameraServer_CaptureStatus,
}

impl<'a> CaptureStatus<'a> {
    pub fn new(inner: &'a crate::camera_server::mavsdk::CameraServer_CaptureStatus) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> CaptureStatusOwned {
        CaptureStatusOwned {
            image_interval_s: self.image_interval_s(),
            recording_time_s: self.recording_time_s(),
            available_capacity_mib: self.available_capacity_mib(),
            image_status: self.image_status(),
            video_status: self.video_status(),
            image_count: self.image_count(),
        }
    }
    ///  Image capture interval (in s)

    pub fn image_interval_s(&self) -> f32 {
        self.inner.camera_server_get_image_interval_s()
    }
    ///  Elapsed time since recording started (in s)

    pub fn recording_time_s(&self) -> f32 {
        self.inner.camera_server_get_recording_time_s()
    }
    ///  Available storage capacity. (in MiB)

    pub fn available_capacity_mib(&self) -> f32 {
        self.inner.camera_server_get_available_capacity_mib()
    }
    ///  Current status of image capturing

    pub fn image_status(
        &self,
    ) -> crate::camera_server::mavsdk::CameraServer_CaptureStatus_ImageStatus {
        self.inner.camera_server_get_image_status().clone()
    }
    ///  Current status of video capturing

    pub fn video_status(
        &self,
    ) -> crate::camera_server::mavsdk::CameraServer_CaptureStatus_VideoStatus {
        self.inner.camera_server_get_video_status().clone()
    }
    ///  Total number of images captured ('forever', or until reset using MAV_CMD_STORAGE_FORMAT)

    pub fn image_count(&self) -> i32 {
        self.inner.camera_server_get_image_count()
    }
}

pub struct TrackPointOwned {
    pub point_x: f32,
    pub point_y: f32,
    pub radius: f32,
}

/*  */
pub struct TrackPoint<'a> {
    inner: &'a crate::camera_server::mavsdk::CameraServer_TrackPoint,
}

impl<'a> TrackPoint<'a> {
    pub fn new(inner: &'a crate::camera_server::mavsdk::CameraServer_TrackPoint) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> TrackPointOwned {
        TrackPointOwned {
            point_x: self.point_x(),
            point_y: self.point_y(),
            radius: self.radius(),
        }
    }
    ///  Point to track x value (normalized 0..1, 0 is left, 1 is right).

    pub fn point_x(&self) -> f32 {
        self.inner.camera_server_get_point_x()
    }
    ///  Point to track y value (normalized 0..1, 0 is top, 1 is bottom).

    pub fn point_y(&self) -> f32 {
        self.inner.camera_server_get_point_y()
    }
    ///  Point to track y value (normalized 0..1, 0 is top, 1 is bottom).

    pub fn radius(&self) -> f32 {
        self.inner.camera_server_get_radius()
    }
}

pub struct TrackRectangleOwned {
    pub top_left_corner_x: f32,
    pub top_left_corner_y: f32,
    pub bottom_right_corner_x: f32,
    pub bottom_right_corner_y: f32,
}

/*  */
pub struct TrackRectangle<'a> {
    inner: &'a crate::camera_server::mavsdk::CameraServer_TrackRectangle,
}

impl<'a> TrackRectangle<'a> {
    pub fn new(inner: &'a crate::camera_server::mavsdk::CameraServer_TrackRectangle) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> TrackRectangleOwned {
        TrackRectangleOwned {
            top_left_corner_x: self.top_left_corner_x(),
            top_left_corner_y: self.top_left_corner_y(),
            bottom_right_corner_x: self.bottom_right_corner_x(),
            bottom_right_corner_y: self.bottom_right_corner_y(),
        }
    }
    ///  Top left corner of rectangle x value (normalized 0..1, 0 is left, 1 is right).

    pub fn top_left_corner_x(&self) -> f32 {
        self.inner.camera_server_get_top_left_corner_x()
    }
    ///  Top left corner of rectangle y value (normalized 0..1, 0 is top, 1 is bottom).

    pub fn top_left_corner_y(&self) -> f32 {
        self.inner.camera_server_get_top_left_corner_y()
    }
    ///  Bottom right corner of rectangle x value (normalized 0..1, 0 is left, 1 is right).

    pub fn bottom_right_corner_x(&self) -> f32 {
        self.inner.camera_server_get_bottom_right_corner_x()
    }
    ///  Bottom right corner of rectangle y value (normalized 0..1, 0 is top, 1 is bottom).

    pub fn bottom_right_corner_y(&self) -> f32 {
        self.inner.camera_server_get_bottom_right_corner_y()
    }
}

struct CameraServerInner {
    plugin: cxx::UniquePtr<crate::camera_server::mavsdk::CameraServer>,

    /// TakePhoto State
    take_photo_handle: Option<usize>,
    take_photo_user_data: *mut c_void,
    /// StartVideo State
    start_video_handle: Option<usize>,
    start_video_user_data: *mut c_void,
    /// StopVideo State
    stop_video_handle: Option<usize>,
    stop_video_user_data: *mut c_void,
    /// StartVideoStreaming State
    start_video_streaming_handle: Option<usize>,
    start_video_streaming_user_data: *mut c_void,
    /// StopVideoStreaming State
    stop_video_streaming_handle: Option<usize>,
    stop_video_streaming_user_data: *mut c_void,
    /// SetMode State
    set_mode_handle: Option<usize>,
    set_mode_user_data: *mut c_void,
    /// StorageInformation State
    storage_information_handle: Option<usize>,
    storage_information_user_data: *mut c_void,
    /// CaptureStatus State
    capture_status_handle: Option<usize>,
    capture_status_user_data: *mut c_void,
    /// FormatStorage State
    format_storage_handle: Option<usize>,
    format_storage_user_data: *mut c_void,
    /// ResetSettings State
    reset_settings_handle: Option<usize>,
    reset_settings_user_data: *mut c_void,
    /// ZoomInStart State
    zoom_in_start_handle: Option<usize>,
    zoom_in_start_user_data: *mut c_void,
    /// ZoomOutStart State
    zoom_out_start_handle: Option<usize>,
    zoom_out_start_user_data: *mut c_void,
    /// ZoomStop State
    zoom_stop_handle: Option<usize>,
    zoom_stop_user_data: *mut c_void,
    /// ZoomRange State
    zoom_range_handle: Option<usize>,
    zoom_range_user_data: *mut c_void,
    /// TrackingPointCommand State
    tracking_point_command_handle: Option<usize>,
    tracking_point_command_user_data: *mut c_void,
    /// TrackingRectangleCommand State
    tracking_rectangle_command_handle: Option<usize>,
    tracking_rectangle_command_user_data: *mut c_void,
    /// TrackingOffCommand State
    tracking_off_command_handle: Option<usize>,
    tracking_off_command_user_data: *mut c_void,
}

pub struct CameraServerClient {
    inner: Mutex<CameraServerInner>,

    // Producer for the latest TakePhoto state; broadcast to all active subscribers.
    take_photo_tx: watch::Sender<std::option::Option<i32>>,
    // Producer for the latest StartVideo state; broadcast to all active subscribers.
    start_video_tx: watch::Sender<std::option::Option<i32>>,
    // Producer for the latest StopVideo state; broadcast to all active subscribers.
    stop_video_tx: watch::Sender<std::option::Option<i32>>,
    // Producer for the latest StartVideoStreaming state; broadcast to all active subscribers.
    start_video_streaming_tx: watch::Sender<std::option::Option<i32>>,
    // Producer for the latest StopVideoStreaming state; broadcast to all active subscribers.
    stop_video_streaming_tx: watch::Sender<std::option::Option<i32>>,
    // Producer for the latest SetMode state; broadcast to all active subscribers.
    set_mode_tx:
        watch::Sender<std::option::Option<crate::camera_server::mavsdk::CameraServer_Mode>>,
    // Producer for the latest StorageInformation state; broadcast to all active subscribers.
    storage_information_tx: watch::Sender<std::option::Option<i32>>,
    // Producer for the latest CaptureStatus state; broadcast to all active subscribers.
    capture_status_tx: watch::Sender<std::option::Option<i32>>,
    // Producer for the latest FormatStorage state; broadcast to all active subscribers.
    format_storage_tx: watch::Sender<std::option::Option<i32>>,
    // Producer for the latest ResetSettings state; broadcast to all active subscribers.
    reset_settings_tx: watch::Sender<std::option::Option<i32>>,
    // Producer for the latest ZoomInStart state; broadcast to all active subscribers.
    zoom_in_start_tx: watch::Sender<std::option::Option<i32>>,
    // Producer for the latest ZoomOutStart state; broadcast to all active subscribers.
    zoom_out_start_tx: watch::Sender<std::option::Option<i32>>,
    // Producer for the latest ZoomStop state; broadcast to all active subscribers.
    zoom_stop_tx: watch::Sender<std::option::Option<i32>>,
    // Producer for the latest ZoomRange state; broadcast to all active subscribers.
    zoom_range_tx: watch::Sender<std::option::Option<f32>>,
    // Producer for the latest TrackingPointCommand state; broadcast to all active subscribers.
    tracking_point_command_tx: watch::Sender<std::option::Option<TrackPointOwned>>,
    // Producer for the latest TrackingRectangleCommand state; broadcast to all active subscribers.
    tracking_rectangle_command_tx: watch::Sender<std::option::Option<TrackRectangleOwned>>,
    // Producer for the latest TrackingOffCommand state; broadcast to all active subscribers.
    tracking_off_command_tx: watch::Sender<std::option::Option<i32>>,
}

impl CameraServerClient {
    pub fn new(plugin: cxx::UniquePtr<crate::camera_server::mavsdk::CameraServer>) -> Self {
        let (take_photo_tx, _) = watch::channel(None);
        let (start_video_tx, _) = watch::channel(None);
        let (stop_video_tx, _) = watch::channel(None);
        let (start_video_streaming_tx, _) = watch::channel(None);
        let (stop_video_streaming_tx, _) = watch::channel(None);
        let (set_mode_tx, _) = watch::channel(None);
        let (storage_information_tx, _) = watch::channel(None);
        let (capture_status_tx, _) = watch::channel(None);
        let (format_storage_tx, _) = watch::channel(None);
        let (reset_settings_tx, _) = watch::channel(None);
        let (zoom_in_start_tx, _) = watch::channel(None);
        let (zoom_out_start_tx, _) = watch::channel(None);
        let (zoom_stop_tx, _) = watch::channel(None);
        let (zoom_range_tx, _) = watch::channel(None);
        let (tracking_point_command_tx, _) = watch::channel(None);
        let (tracking_rectangle_command_tx, _) = watch::channel(None);
        let (tracking_off_command_tx, _) = watch::channel(None);

        Self {
            inner: Mutex::new(CameraServerInner {
                plugin,

                take_photo_handle: None,
                take_photo_user_data: std::ptr::null_mut(),
                start_video_handle: None,
                start_video_user_data: std::ptr::null_mut(),
                stop_video_handle: None,
                stop_video_user_data: std::ptr::null_mut(),
                start_video_streaming_handle: None,
                start_video_streaming_user_data: std::ptr::null_mut(),
                stop_video_streaming_handle: None,
                stop_video_streaming_user_data: std::ptr::null_mut(),
                set_mode_handle: None,
                set_mode_user_data: std::ptr::null_mut(),
                storage_information_handle: None,
                storage_information_user_data: std::ptr::null_mut(),
                capture_status_handle: None,
                capture_status_user_data: std::ptr::null_mut(),
                format_storage_handle: None,
                format_storage_user_data: std::ptr::null_mut(),
                reset_settings_handle: None,
                reset_settings_user_data: std::ptr::null_mut(),
                zoom_in_start_handle: None,
                zoom_in_start_user_data: std::ptr::null_mut(),
                zoom_out_start_handle: None,
                zoom_out_start_user_data: std::ptr::null_mut(),
                zoom_stop_handle: None,
                zoom_stop_user_data: std::ptr::null_mut(),
                zoom_range_handle: None,
                zoom_range_user_data: std::ptr::null_mut(),
                tracking_point_command_handle: None,
                tracking_point_command_user_data: std::ptr::null_mut(),
                tracking_rectangle_command_handle: None,
                tracking_rectangle_command_user_data: std::ptr::null_mut(),
                tracking_off_command_handle: None,
                tracking_off_command_user_data: std::ptr::null_mut(),
            }),

            take_photo_tx,
            start_video_tx,
            stop_video_tx,
            start_video_streaming_tx,
            stop_video_streaming_tx,
            set_mode_tx,
            storage_information_tx,
            capture_status_tx,
            format_storage_tx,
            reset_settings_tx,
            zoom_in_start_tx,
            zoom_out_start_tx,
            zoom_stop_tx,
            zoom_range_tx,
            tracking_point_command_tx,
            tracking_rectangle_command_tx,
            tracking_off_command_tx,
        }
    }

    pub fn subscribe_take_photo(&self) -> watch::Receiver<std::option::Option<i32>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.take_photo_handle.is_none() {
            // Lazy initialization of C++ 'take_photo' camera_server stream.
            let user_data_ptr = Box::into_raw(Box::new(self.take_photo_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_take_photo(
                camera_server_take_photo_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.take_photo_handle = Some(handle);
            inner.take_photo_user_data = user_data_ptr;
        }

        self.take_photo_tx.subscribe()
    }
    pub fn subscribe_start_video(&self) -> watch::Receiver<std::option::Option<i32>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.start_video_handle.is_none() {
            // Lazy initialization of C++ 'start_video' camera_server stream.
            let user_data_ptr = Box::into_raw(Box::new(self.start_video_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_start_video(
                camera_server_start_video_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.start_video_handle = Some(handle);
            inner.start_video_user_data = user_data_ptr;
        }

        self.start_video_tx.subscribe()
    }
    pub fn subscribe_stop_video(&self) -> watch::Receiver<std::option::Option<i32>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.stop_video_handle.is_none() {
            // Lazy initialization of C++ 'stop_video' camera_server stream.
            let user_data_ptr = Box::into_raw(Box::new(self.stop_video_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_stop_video(
                camera_server_stop_video_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.stop_video_handle = Some(handle);
            inner.stop_video_user_data = user_data_ptr;
        }

        self.stop_video_tx.subscribe()
    }
    pub fn subscribe_start_video_streaming(&self) -> watch::Receiver<std::option::Option<i32>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.start_video_streaming_handle.is_none() {
            // Lazy initialization of C++ 'start_video_streaming' camera_server stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.start_video_streaming_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_start_video_streaming(
                camera_server_start_video_streaming_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.start_video_streaming_handle = Some(handle);
            inner.start_video_streaming_user_data = user_data_ptr;
        }

        self.start_video_streaming_tx.subscribe()
    }
    pub fn subscribe_stop_video_streaming(&self) -> watch::Receiver<std::option::Option<i32>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.stop_video_streaming_handle.is_none() {
            // Lazy initialization of C++ 'stop_video_streaming' camera_server stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.stop_video_streaming_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_stop_video_streaming(
                camera_server_stop_video_streaming_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.stop_video_streaming_handle = Some(handle);
            inner.stop_video_streaming_user_data = user_data_ptr;
        }

        self.stop_video_streaming_tx.subscribe()
    }
    pub fn subscribe_set_mode(
        &self,
    ) -> watch::Receiver<std::option::Option<crate::camera_server::mavsdk::CameraServer_Mode>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.set_mode_handle.is_none() {
            // Lazy initialization of C++ 'set_mode' camera_server stream.
            let user_data_ptr = Box::into_raw(Box::new(self.set_mode_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_set_mode(
                camera_server_set_mode_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.set_mode_handle = Some(handle);
            inner.set_mode_user_data = user_data_ptr;
        }

        self.set_mode_tx.subscribe()
    }
    pub fn subscribe_storage_information(&self) -> watch::Receiver<std::option::Option<i32>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.storage_information_handle.is_none() {
            // Lazy initialization of C++ 'storage_information' camera_server stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.storage_information_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_storage_information(
                camera_server_storage_information_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.storage_information_handle = Some(handle);
            inner.storage_information_user_data = user_data_ptr;
        }

        self.storage_information_tx.subscribe()
    }
    pub fn subscribe_capture_status(&self) -> watch::Receiver<std::option::Option<i32>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.capture_status_handle.is_none() {
            // Lazy initialization of C++ 'capture_status' camera_server stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.capture_status_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_capture_status(
                camera_server_capture_status_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.capture_status_handle = Some(handle);
            inner.capture_status_user_data = user_data_ptr;
        }

        self.capture_status_tx.subscribe()
    }
    pub fn subscribe_format_storage(&self) -> watch::Receiver<std::option::Option<i32>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.format_storage_handle.is_none() {
            // Lazy initialization of C++ 'format_storage' camera_server stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.format_storage_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_format_storage(
                camera_server_format_storage_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.format_storage_handle = Some(handle);
            inner.format_storage_user_data = user_data_ptr;
        }

        self.format_storage_tx.subscribe()
    }
    pub fn subscribe_reset_settings(&self) -> watch::Receiver<std::option::Option<i32>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.reset_settings_handle.is_none() {
            // Lazy initialization of C++ 'reset_settings' camera_server stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.reset_settings_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_reset_settings(
                camera_server_reset_settings_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.reset_settings_handle = Some(handle);
            inner.reset_settings_user_data = user_data_ptr;
        }

        self.reset_settings_tx.subscribe()
    }
    pub fn subscribe_zoom_in_start(&self) -> watch::Receiver<std::option::Option<i32>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.zoom_in_start_handle.is_none() {
            // Lazy initialization of C++ 'zoom_in_start' camera_server stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.zoom_in_start_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_zoom_in_start(
                camera_server_zoom_in_start_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.zoom_in_start_handle = Some(handle);
            inner.zoom_in_start_user_data = user_data_ptr;
        }

        self.zoom_in_start_tx.subscribe()
    }
    pub fn subscribe_zoom_out_start(&self) -> watch::Receiver<std::option::Option<i32>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.zoom_out_start_handle.is_none() {
            // Lazy initialization of C++ 'zoom_out_start' camera_server stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.zoom_out_start_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_zoom_out_start(
                camera_server_zoom_out_start_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.zoom_out_start_handle = Some(handle);
            inner.zoom_out_start_user_data = user_data_ptr;
        }

        self.zoom_out_start_tx.subscribe()
    }
    pub fn subscribe_zoom_stop(&self) -> watch::Receiver<std::option::Option<i32>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.zoom_stop_handle.is_none() {
            // Lazy initialization of C++ 'zoom_stop' camera_server stream.
            let user_data_ptr = Box::into_raw(Box::new(self.zoom_stop_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_zoom_stop(
                camera_server_zoom_stop_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.zoom_stop_handle = Some(handle);
            inner.zoom_stop_user_data = user_data_ptr;
        }

        self.zoom_stop_tx.subscribe()
    }
    pub fn subscribe_zoom_range(&self) -> watch::Receiver<std::option::Option<f32>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.zoom_range_handle.is_none() {
            // Lazy initialization of C++ 'zoom_range' camera_server stream.
            let user_data_ptr = Box::into_raw(Box::new(self.zoom_range_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_zoom_range(
                camera_server_zoom_range_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.zoom_range_handle = Some(handle);
            inner.zoom_range_user_data = user_data_ptr;
        }

        self.zoom_range_tx.subscribe()
    }
    pub fn subscribe_tracking_point_command(
        &self,
    ) -> watch::Receiver<std::option::Option<TrackPointOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.tracking_point_command_handle.is_none() {
            // Lazy initialization of C++ 'tracking_point_command' camera_server stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.tracking_point_command_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_tracking_point_command(
                camera_server_tracking_point_command_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.tracking_point_command_handle = Some(handle);
            inner.tracking_point_command_user_data = user_data_ptr;
        }

        self.tracking_point_command_tx.subscribe()
    }
    pub fn subscribe_tracking_rectangle_command(
        &self,
    ) -> watch::Receiver<std::option::Option<TrackRectangleOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.tracking_rectangle_command_handle.is_none() {
            // Lazy initialization of C++ 'tracking_rectangle_command' camera_server stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.tracking_rectangle_command_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_tracking_rectangle_command(
                camera_server_tracking_rectangle_command_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.tracking_rectangle_command_handle = Some(handle);
            inner.tracking_rectangle_command_user_data = user_data_ptr;
        }

        self.tracking_rectangle_command_tx.subscribe()
    }
    pub fn subscribe_tracking_off_command(&self) -> watch::Receiver<std::option::Option<i32>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.tracking_off_command_handle.is_none() {
            // Lazy initialization of C++ 'tracking_off_command' camera_server stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.tracking_off_command_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_tracking_off_command(
                camera_server_tracking_off_command_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.tracking_off_command_handle = Some(handle);
            inner.tracking_off_command_user_data = user_data_ptr;
        }

        self.tracking_off_command_tx.subscribe()
    }
}

impl Drop for CameraServerClient {
    fn drop(&mut self) {
        if let Ok(mut inner) = self.inner.lock() {
            if let Some(handle) = inner.take_photo_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_take_photo(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.take_photo_user_data as *mut watch::Sender<std::option::Option<i32>>,
                    );
                }
            }
            if let Some(handle) = inner.start_video_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_start_video(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.start_video_user_data as *mut watch::Sender<std::option::Option<i32>>,
                    );
                }
            }
            if let Some(handle) = inner.stop_video_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_stop_video(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.stop_video_user_data as *mut watch::Sender<std::option::Option<i32>>,
                    );
                }
            }
            if let Some(handle) = inner.start_video_streaming_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_start_video_streaming(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.start_video_streaming_user_data
                            as *mut watch::Sender<std::option::Option<i32>>,
                    );
                }
            }
            if let Some(handle) = inner.stop_video_streaming_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_stop_video_streaming(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.stop_video_streaming_user_data
                            as *mut watch::Sender<std::option::Option<i32>>,
                    );
                }
            }
            if let Some(handle) = inner.set_mode_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_set_mode(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.set_mode_user_data
                            as *mut watch::Sender<
                                std::option::Option<
                                    crate::camera_server::mavsdk::CameraServer_Mode,
                                >,
                            >,
                    );
                }
            }
            if let Some(handle) = inner.storage_information_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_storage_information(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.storage_information_user_data
                            as *mut watch::Sender<std::option::Option<i32>>,
                    );
                }
            }
            if let Some(handle) = inner.capture_status_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_capture_status(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.capture_status_user_data
                            as *mut watch::Sender<std::option::Option<i32>>,
                    );
                }
            }
            if let Some(handle) = inner.format_storage_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_format_storage(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.format_storage_user_data
                            as *mut watch::Sender<std::option::Option<i32>>,
                    );
                }
            }
            if let Some(handle) = inner.reset_settings_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_reset_settings(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.reset_settings_user_data
                            as *mut watch::Sender<std::option::Option<i32>>,
                    );
                }
            }
            if let Some(handle) = inner.zoom_in_start_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_zoom_in_start(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.zoom_in_start_user_data
                            as *mut watch::Sender<std::option::Option<i32>>,
                    );
                }
            }
            if let Some(handle) = inner.zoom_out_start_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_zoom_out_start(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.zoom_out_start_user_data
                            as *mut watch::Sender<std::option::Option<i32>>,
                    );
                }
            }
            if let Some(handle) = inner.zoom_stop_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_zoom_stop(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.zoom_stop_user_data as *mut watch::Sender<std::option::Option<i32>>,
                    );
                }
            }
            if let Some(handle) = inner.zoom_range_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_zoom_range(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.zoom_range_user_data as *mut watch::Sender<std::option::Option<f32>>,
                    );
                }
            }
            if let Some(handle) = inner.tracking_point_command_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_tracking_point_command(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.tracking_point_command_user_data
                            as *mut watch::Sender<std::option::Option<TrackPointOwned>>,
                    );
                }
            }
            if let Some(handle) = inner.tracking_rectangle_command_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_tracking_rectangle_command(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.tracking_rectangle_command_user_data
                            as *mut watch::Sender<std::option::Option<TrackRectangleOwned>>,
                    );
                }
            }
            if let Some(handle) = inner.tracking_off_command_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_tracking_off_command(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.tracking_off_command_user_data
                            as *mut watch::Sender<std::option::Option<i32>>,
                    );
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn camera_server_take_photo_callback_ffi(user_data: *mut c_void, take_photo: i32) {
    if user_data.is_null() {
        return;
    }
    let send_take_photo = take_photo.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<i32>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_take_photo));
}
#[unsafe(no_mangle)]
pub extern "C" fn camera_server_start_video_callback_ffi(user_data: *mut c_void, start_video: i32) {
    if user_data.is_null() {
        return;
    }
    let send_start_video = start_video.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<i32>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_start_video));
}
#[unsafe(no_mangle)]
pub extern "C" fn camera_server_stop_video_callback_ffi(user_data: *mut c_void, stop_video: i32) {
    if user_data.is_null() {
        return;
    }
    let send_stop_video = stop_video.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<i32>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_stop_video));
}
#[unsafe(no_mangle)]
pub extern "C" fn camera_server_start_video_streaming_callback_ffi(
    user_data: *mut c_void,
    start_video_streaming: i32,
) {
    if user_data.is_null() {
        return;
    }
    let send_start_video_streaming = start_video_streaming.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<i32>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_start_video_streaming));
}
#[unsafe(no_mangle)]
pub extern "C" fn camera_server_stop_video_streaming_callback_ffi(
    user_data: *mut c_void,
    stop_video_streaming: i32,
) {
    if user_data.is_null() {
        return;
    }
    let send_stop_video_streaming = stop_video_streaming.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<i32>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_stop_video_streaming));
}
#[unsafe(no_mangle)]
pub extern "C" fn camera_server_set_mode_callback_ffi(
    user_data: *mut c_void,
    set_mode: &crate::camera_server::mavsdk::CameraServer_Mode,
) {
    if user_data.is_null() {
        return;
    }
    let send_set_mode = set_mode.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe {
        &*(user_data
            as *const watch::Sender<
                std::option::Option<crate::camera_server::mavsdk::CameraServer_Mode>,
            >)
    };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_set_mode));
}
#[unsafe(no_mangle)]
pub extern "C" fn camera_server_storage_information_callback_ffi(
    user_data: *mut c_void,
    storage_information: i32,
) {
    if user_data.is_null() {
        return;
    }
    let send_storage_information = storage_information.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<i32>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_storage_information));
}
#[unsafe(no_mangle)]
pub extern "C" fn camera_server_capture_status_callback_ffi(
    user_data: *mut c_void,
    capture_status: i32,
) {
    if user_data.is_null() {
        return;
    }
    let send_capture_status = capture_status.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<i32>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_capture_status));
}
#[unsafe(no_mangle)]
pub extern "C" fn camera_server_format_storage_callback_ffi(
    user_data: *mut c_void,
    format_storage: i32,
) {
    if user_data.is_null() {
        return;
    }
    let send_format_storage = format_storage.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<i32>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_format_storage));
}
#[unsafe(no_mangle)]
pub extern "C" fn camera_server_reset_settings_callback_ffi(
    user_data: *mut c_void,
    reset_settings: i32,
) {
    if user_data.is_null() {
        return;
    }
    let send_reset_settings = reset_settings.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<i32>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_reset_settings));
}
#[unsafe(no_mangle)]
pub extern "C" fn camera_server_zoom_in_start_callback_ffi(
    user_data: *mut c_void,
    zoom_in_start: i32,
) {
    if user_data.is_null() {
        return;
    }
    let send_zoom_in_start = zoom_in_start.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<i32>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_zoom_in_start));
}
#[unsafe(no_mangle)]
pub extern "C" fn camera_server_zoom_out_start_callback_ffi(
    user_data: *mut c_void,
    zoom_out_start: i32,
) {
    if user_data.is_null() {
        return;
    }
    let send_zoom_out_start = zoom_out_start.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<i32>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_zoom_out_start));
}
#[unsafe(no_mangle)]
pub extern "C" fn camera_server_zoom_stop_callback_ffi(user_data: *mut c_void, zoom_stop: i32) {
    if user_data.is_null() {
        return;
    }
    let send_zoom_stop = zoom_stop.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<i32>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_zoom_stop));
}
#[unsafe(no_mangle)]
pub extern "C" fn camera_server_zoom_range_callback_ffi(user_data: *mut c_void, zoom_range: f32) {
    if user_data.is_null() {
        return;
    }
    let send_zoom_range = zoom_range.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<f32>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_zoom_range));
}
#[unsafe(no_mangle)]
pub extern "C" fn camera_server_tracking_point_command_callback_ffi(
    user_data: *mut c_void,
    tracking_point_command: &crate::camera_server::mavsdk::CameraServer_TrackPoint,
) {
    if user_data.is_null() {
        return;
    }
    let send_tracking_point_command = TrackPoint::new(tracking_point_command).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<TrackPointOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_tracking_point_command));
}
#[unsafe(no_mangle)]
pub extern "C" fn camera_server_tracking_rectangle_command_callback_ffi(
    user_data: *mut c_void,
    tracking_rectangle_command: &crate::camera_server::mavsdk::CameraServer_TrackRectangle,
) {
    if user_data.is_null() {
        return;
    }
    let send_tracking_rectangle_command =
        TrackRectangle::new(tracking_rectangle_command).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<TrackRectangleOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_tracking_rectangle_command));
}
#[unsafe(no_mangle)]
pub extern "C" fn camera_server_tracking_off_command_callback_ffi(
    user_data: *mut c_void,
    tracking_off_command: i32,
) {
    if user_data.is_null() {
        return;
    }
    let send_tracking_off_command = tracking_off_command.clone();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<i32>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_tracking_off_command));
}
