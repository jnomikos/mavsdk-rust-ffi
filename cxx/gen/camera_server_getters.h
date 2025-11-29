// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace InformationGetters {
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
namespace VideoStreamingGetters {
  bool get_has_rtsp_server(const mavsdk::CameraServer::VideoStreaming& s) { return s.has_rtsp_server; }
  const std::string& get_rtsp_uri(const mavsdk::CameraServer::VideoStreaming& s) { return s.rtsp_uri; }
} // namespace mavsdk::CameraServer::VideoStreaming
namespace CaptureInfoGetters {
  const mavsdk::CameraServer::Position& get_position(const mavsdk::CameraServer::CaptureInfo& s) { return s.position; }
  const mavsdk::CameraServer::Quaternion& get_attitude_quaternion(const mavsdk::CameraServer::CaptureInfo& s) { return s.attitude_quaternion; }
  uint64_t get_time_utc_us(const mavsdk::CameraServer::CaptureInfo& s) { return s.time_utc_us; }
  bool get_is_success(const mavsdk::CameraServer::CaptureInfo& s) { return s.is_success; }
  int32_t get_index(const mavsdk::CameraServer::CaptureInfo& s) { return s.index; }
  const std::string& get_file_url(const mavsdk::CameraServer::CaptureInfo& s) { return s.file_url; }
} // namespace mavsdk::CameraServer::CaptureInfo
