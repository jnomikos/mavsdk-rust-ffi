// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace Information {
  const std::string& get_vendor_name(const mavsdk::CameraServer::Information& s) { return s.vendor_name; }
  const std::string& get_model_name(const mavsdk::CameraServer::Information& s) { return s.model_name; }
  const std::string& get_firmware_version(const mavsdk::CameraServer::Information& s) { return s.firmware_version; }
  float get_focal_length_mm(const mavsdk::CameraServer::Information& s) { return s.focal_length_mm; }
  float get_horizontal_sensor_size_mm(const mavsdk::CameraServer::Information& s) { return s.horizontal_sensor_size_mm; }
  float get_vertical_sensor_size_mm(const mavsdk::CameraServer::Information& s) { return s.vertical_sensor_size_mm; }
  uint32_t get_horizontal_resolution_px(const mavsdk::CameraServer::Information& s) { return s.horizontal_resolution_px; }
  uint32_t get_vertical_resolution_px(const mavsdk::CameraServer::Information& s) { return s.vertical_resolution_px; }
  uint32_t get_lens_id(const mavsdk::CameraServer::Information& s) { return s.lens_id; }
  uint32_t get_definition_file_version(const mavsdk::CameraServer::Information& s) { return s.definition_file_version; }
  const std::string& get_definition_file_uri(const mavsdk::CameraServer::Information& s) { return s.definition_file_uri; }
  bool get_image_in_video_mode_supported(const mavsdk::CameraServer::Information& s) { return s.image_in_video_mode_supported; }
  bool get_video_in_image_mode_supported(const mavsdk::CameraServer::Information& s) { return s.video_in_image_mode_supported; }
} // namespace mavsdk::CameraServer::Information
namespace VideoStreaming {
  bool get_has_rtsp_server(const mavsdk::CameraServer::VideoStreaming& s) { return s.has_rtsp_server; }
  const std::string& get_rtsp_uri(const mavsdk::CameraServer::VideoStreaming& s) { return s.rtsp_uri; }
} // namespace mavsdk::CameraServer::VideoStreaming
namespace Position {
  double get_latitude_deg(const mavsdk::CameraServer::Position& s) { return s.latitude_deg; }
  double get_longitude_deg(const mavsdk::CameraServer::Position& s) { return s.longitude_deg; }
  float get_absolute_altitude_m(const mavsdk::CameraServer::Position& s) { return s.absolute_altitude_m; }
  float get_relative_altitude_m(const mavsdk::CameraServer::Position& s) { return s.relative_altitude_m; }
} // namespace mavsdk::CameraServer::Position
namespace Quaternion {
  float get_w(const mavsdk::CameraServer::Quaternion& s) { return s.w; }
  float get_x(const mavsdk::CameraServer::Quaternion& s) { return s.x; }
  float get_y(const mavsdk::CameraServer::Quaternion& s) { return s.y; }
  float get_z(const mavsdk::CameraServer::Quaternion& s) { return s.z; }
} // namespace mavsdk::CameraServer::Quaternion
namespace CaptureInfo {
  const mavsdk::CameraServer::Position& get_position(const mavsdk::CameraServer::CaptureInfo& s) { return s.position; }
  const mavsdk::CameraServer::Quaternion& get_attitude_quaternion(const mavsdk::CameraServer::CaptureInfo& s) { return s.attitude_quaternion; }
  uint64_t get_time_utc_us(const mavsdk::CameraServer::CaptureInfo& s) { return s.time_utc_us; }
  bool get_is_success(const mavsdk::CameraServer::CaptureInfo& s) { return s.is_success; }
  int32_t get_index(const mavsdk::CameraServer::CaptureInfo& s) { return s.index; }
  const std::string& get_file_url(const mavsdk::CameraServer::CaptureInfo& s) { return s.file_url; }
} // namespace mavsdk::CameraServer::CaptureInfo
namespace StorageInformation {
  float get_used_storage_mib(const mavsdk::CameraServer::StorageInformation& s) { return s.used_storage_mib; }
  float get_available_storage_mib(const mavsdk::CameraServer::StorageInformation& s) { return s.available_storage_mib; }
  float get_total_storage_mib(const mavsdk::CameraServer::StorageInformation& s) { return s.total_storage_mib; }
  mavsdk::CameraServer::StorageInformation::StorageStatus get_storage_status(const mavsdk::CameraServer::StorageInformation& s) { return s.storage_status; }
  uint32_t get_storage_id(const mavsdk::CameraServer::StorageInformation& s) { return s.storage_id; }
  mavsdk::CameraServer::StorageInformation::StorageType get_storage_type(const mavsdk::CameraServer::StorageInformation& s) { return s.storage_type; }
  float get_read_speed_mib_s(const mavsdk::CameraServer::StorageInformation& s) { return s.read_speed_mib_s; }
  float get_write_speed_mib_s(const mavsdk::CameraServer::StorageInformation& s) { return s.write_speed_mib_s; }
} // namespace mavsdk::CameraServer::StorageInformation
namespace CaptureStatus {
  float get_image_interval_s(const mavsdk::CameraServer::CaptureStatus& s) { return s.image_interval_s; }
  float get_recording_time_s(const mavsdk::CameraServer::CaptureStatus& s) { return s.recording_time_s; }
  float get_available_capacity_mib(const mavsdk::CameraServer::CaptureStatus& s) { return s.available_capacity_mib; }
  mavsdk::CameraServer::CaptureStatus::ImageStatus get_image_status(const mavsdk::CameraServer::CaptureStatus& s) { return s.image_status; }
  mavsdk::CameraServer::CaptureStatus::VideoStatus get_video_status(const mavsdk::CameraServer::CaptureStatus& s) { return s.video_status; }
  int32_t get_image_count(const mavsdk::CameraServer::CaptureStatus& s) { return s.image_count; }
} // namespace mavsdk::CameraServer::CaptureStatus
namespace TrackPoint {
  float get_point_x(const mavsdk::CameraServer::TrackPoint& s) { return s.point_x; }
  float get_point_y(const mavsdk::CameraServer::TrackPoint& s) { return s.point_y; }
  float get_radius(const mavsdk::CameraServer::TrackPoint& s) { return s.radius; }
} // namespace mavsdk::CameraServer::TrackPoint
namespace TrackRectangle {
  float get_top_left_corner_x(const mavsdk::CameraServer::TrackRectangle& s) { return s.top_left_corner_x; }
  float get_top_left_corner_y(const mavsdk::CameraServer::TrackRectangle& s) { return s.top_left_corner_y; }
  float get_bottom_right_corner_x(const mavsdk::CameraServer::TrackRectangle& s) { return s.bottom_right_corner_x; }
  float get_bottom_right_corner_y(const mavsdk::CameraServer::TrackRectangle& s) { return s.bottom_right_corner_y; }
} // namespace mavsdk::CameraServer::TrackRectangle
