// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace Option {
  const std::string& get_option_id(const mavsdk::Camera::Option& s) { return s.option_id; }
  const std::string& get_option_description(const mavsdk::Camera::Option& s) { return s.option_description; }
} // namespace mavsdk::Camera::Option
namespace Setting {
  const std::string& get_setting_id(const mavsdk::Camera::Setting& s) { return s.setting_id; }
  const std::string& get_setting_description(const mavsdk::Camera::Setting& s) { return s.setting_description; }
  const mavsdk::Camera::Option& get_option(const mavsdk::Camera::Setting& s) { return s.option; }
  bool get_is_range(const mavsdk::Camera::Setting& s) { return s.is_range; }
} // namespace mavsdk::Camera::Setting
namespace SettingOptions {
  int32_t get_component_id(const mavsdk::Camera::SettingOptions& s) { return s.component_id; }
  const std::string& get_setting_id(const mavsdk::Camera::SettingOptions& s) { return s.setting_id; }
  const std::string& get_setting_description(const mavsdk::Camera::SettingOptions& s) { return s.setting_description; }
  const std::vector<Option>& get_options(const mavsdk::Camera::SettingOptions& s) { return s.options; }
  bool get_is_range(const mavsdk::Camera::SettingOptions& s) { return s.is_range; }
} // namespace mavsdk::Camera::SettingOptions
namespace VideoStreamSettings {
  float get_frame_rate_hz(const mavsdk::Camera::VideoStreamSettings& s) { return s.frame_rate_hz; }
  uint32_t get_horizontal_resolution_pix(const mavsdk::Camera::VideoStreamSettings& s) { return s.horizontal_resolution_pix; }
  uint32_t get_vertical_resolution_pix(const mavsdk::Camera::VideoStreamSettings& s) { return s.vertical_resolution_pix; }
  uint32_t get_bit_rate_b_s(const mavsdk::Camera::VideoStreamSettings& s) { return s.bit_rate_b_s; }
  uint32_t get_rotation_deg(const mavsdk::Camera::VideoStreamSettings& s) { return s.rotation_deg; }
  const std::string& get_uri(const mavsdk::Camera::VideoStreamSettings& s) { return s.uri; }
  float get_horizontal_fov_deg(const mavsdk::Camera::VideoStreamSettings& s) { return s.horizontal_fov_deg; }
} // namespace mavsdk::Camera::VideoStreamSettings
namespace VideoStreamInfo {
  int32_t get_stream_id(const mavsdk::Camera::VideoStreamInfo& s) { return s.stream_id; }
  const mavsdk::Camera::VideoStreamSettings& get_settings(const mavsdk::Camera::VideoStreamInfo& s) { return s.settings; }
  mavsdk::Camera::VideoStreamInfo::VideoStreamStatus get_status(const mavsdk::Camera::VideoStreamInfo& s) { return s.status; }
  mavsdk::Camera::VideoStreamInfo::VideoStreamSpectrum get_spectrum(const mavsdk::Camera::VideoStreamInfo& s) { return s.spectrum; }
} // namespace mavsdk::Camera::VideoStreamInfo
namespace ModeUpdate {
  int32_t get_component_id(const mavsdk::Camera::ModeUpdate& s) { return s.component_id; }
  mavsdk::Camera::Mode get_mode(const mavsdk::Camera::ModeUpdate& s) { return s.mode; }
} // namespace mavsdk::Camera::ModeUpdate
namespace VideoStreamUpdate {
  int32_t get_component_id(const mavsdk::Camera::VideoStreamUpdate& s) { return s.component_id; }
  const mavsdk::Camera::VideoStreamInfo& get_video_stream_info(const mavsdk::Camera::VideoStreamUpdate& s) { return s.video_stream_info; }
} // namespace mavsdk::Camera::VideoStreamUpdate
namespace Storage {
  int32_t get_component_id(const mavsdk::Camera::Storage& s) { return s.component_id; }
  bool get_video_on(const mavsdk::Camera::Storage& s) { return s.video_on; }
  bool get_photo_interval_on(const mavsdk::Camera::Storage& s) { return s.photo_interval_on; }
  float get_used_storage_mib(const mavsdk::Camera::Storage& s) { return s.used_storage_mib; }
  float get_available_storage_mib(const mavsdk::Camera::Storage& s) { return s.available_storage_mib; }
  float get_total_storage_mib(const mavsdk::Camera::Storage& s) { return s.total_storage_mib; }
  float get_recording_time_s(const mavsdk::Camera::Storage& s) { return s.recording_time_s; }
  const std::string& get_media_folder_name(const mavsdk::Camera::Storage& s) { return s.media_folder_name; }
  mavsdk::Camera::Storage::StorageStatus get_storage_status(const mavsdk::Camera::Storage& s) { return s.storage_status; }
  uint32_t get_storage_id(const mavsdk::Camera::Storage& s) { return s.storage_id; }
  mavsdk::Camera::Storage::StorageType get_storage_type(const mavsdk::Camera::Storage& s) { return s.storage_type; }
} // namespace mavsdk::Camera::Storage
namespace StorageUpdate {
  int32_t get_component_id(const mavsdk::Camera::StorageUpdate& s) { return s.component_id; }
  const mavsdk::Camera::Storage& get_storage(const mavsdk::Camera::StorageUpdate& s) { return s.storage; }
} // namespace mavsdk::Camera::StorageUpdate
namespace CurrentSettingsUpdate {
  int32_t get_component_id(const mavsdk::Camera::CurrentSettingsUpdate& s) { return s.component_id; }
  const std::vector<Setting>& get_current_settings(const mavsdk::Camera::CurrentSettingsUpdate& s) { return s.current_settings; }
} // namespace mavsdk::Camera::CurrentSettingsUpdate
namespace PossibleSettingOptionsUpdate {
  int32_t get_component_id(const mavsdk::Camera::PossibleSettingOptionsUpdate& s) { return s.component_id; }
  const std::vector<SettingOptions>& get_setting_options(const mavsdk::Camera::PossibleSettingOptionsUpdate& s) { return s.setting_options; }
} // namespace mavsdk::Camera::PossibleSettingOptionsUpdate
namespace Position {
  double get_latitude_deg(const mavsdk::Camera::Position& s) { return s.latitude_deg; }
  double get_longitude_deg(const mavsdk::Camera::Position& s) { return s.longitude_deg; }
  float get_absolute_altitude_m(const mavsdk::Camera::Position& s) { return s.absolute_altitude_m; }
  float get_relative_altitude_m(const mavsdk::Camera::Position& s) { return s.relative_altitude_m; }
} // namespace mavsdk::Camera::Position
namespace Quaternion {
  float get_w(const mavsdk::Camera::Quaternion& s) { return s.w; }
  float get_x(const mavsdk::Camera::Quaternion& s) { return s.x; }
  float get_y(const mavsdk::Camera::Quaternion& s) { return s.y; }
  float get_z(const mavsdk::Camera::Quaternion& s) { return s.z; }
} // namespace mavsdk::Camera::Quaternion
namespace EulerAngle {
  float get_roll_deg(const mavsdk::Camera::EulerAngle& s) { return s.roll_deg; }
  float get_pitch_deg(const mavsdk::Camera::EulerAngle& s) { return s.pitch_deg; }
  float get_yaw_deg(const mavsdk::Camera::EulerAngle& s) { return s.yaw_deg; }
} // namespace mavsdk::Camera::EulerAngle
namespace CaptureInfo {
  int32_t get_component_id(const mavsdk::Camera::CaptureInfo& s) { return s.component_id; }
  const mavsdk::Camera::Position& get_position(const mavsdk::Camera::CaptureInfo& s) { return s.position; }
  const mavsdk::Camera::Quaternion& get_attitude_quaternion(const mavsdk::Camera::CaptureInfo& s) { return s.attitude_quaternion; }
  const mavsdk::Camera::EulerAngle& get_attitude_euler_angle(const mavsdk::Camera::CaptureInfo& s) { return s.attitude_euler_angle; }
  uint64_t get_time_utc_us(const mavsdk::Camera::CaptureInfo& s) { return s.time_utc_us; }
  bool get_is_success(const mavsdk::Camera::CaptureInfo& s) { return s.is_success; }
  int32_t get_index(const mavsdk::Camera::CaptureInfo& s) { return s.index; }
  const std::string& get_file_url(const mavsdk::Camera::CaptureInfo& s) { return s.file_url; }
} // namespace mavsdk::Camera::CaptureInfo
namespace Information {
  int32_t get_component_id(const mavsdk::Camera::Information& s) { return s.component_id; }
  const std::string& get_vendor_name(const mavsdk::Camera::Information& s) { return s.vendor_name; }
  const std::string& get_model_name(const mavsdk::Camera::Information& s) { return s.model_name; }
  float get_focal_length_mm(const mavsdk::Camera::Information& s) { return s.focal_length_mm; }
  float get_horizontal_sensor_size_mm(const mavsdk::Camera::Information& s) { return s.horizontal_sensor_size_mm; }
  float get_vertical_sensor_size_mm(const mavsdk::Camera::Information& s) { return s.vertical_sensor_size_mm; }
  uint32_t get_horizontal_resolution_px(const mavsdk::Camera::Information& s) { return s.horizontal_resolution_px; }
  uint32_t get_vertical_resolution_px(const mavsdk::Camera::Information& s) { return s.vertical_resolution_px; }
} // namespace mavsdk::Camera::Information
namespace CameraList {
  const std::vector<Information>& get_cameras(const mavsdk::Camera::CameraList& s) { return s.cameras; }
} // namespace mavsdk::Camera::CameraList
