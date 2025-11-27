// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace MissionItem {
  double get_latitude_deg(const mavsdk::Mission::MissionItem& s) { return s.latitude_deg; }
  double get_longitude_deg(const mavsdk::Mission::MissionItem& s) { return s.longitude_deg; }
  float get_relative_altitude_m(const mavsdk::Mission::MissionItem& s) { return s.relative_altitude_m; }
  float get_speed_m_s(const mavsdk::Mission::MissionItem& s) { return s.speed_m_s; }
  bool get_is_fly_through(const mavsdk::Mission::MissionItem& s) { return s.is_fly_through; }
  float get_gimbal_pitch_deg(const mavsdk::Mission::MissionItem& s) { return s.gimbal_pitch_deg; }
  float get_gimbal_yaw_deg(const mavsdk::Mission::MissionItem& s) { return s.gimbal_yaw_deg; }
  mavsdk::Mission::MissionItem::CameraAction get_camera_action(const mavsdk::Mission::MissionItem& s) { return s.camera_action; }
  float get_loiter_time_s(const mavsdk::Mission::MissionItem& s) { return s.loiter_time_s; }
  double get_camera_photo_interval_s(const mavsdk::Mission::MissionItem& s) { return s.camera_photo_interval_s; }
  float get_acceptance_radius_m(const mavsdk::Mission::MissionItem& s) { return s.acceptance_radius_m; }
  float get_yaw_deg(const mavsdk::Mission::MissionItem& s) { return s.yaw_deg; }
  float get_camera_photo_distance_m(const mavsdk::Mission::MissionItem& s) { return s.camera_photo_distance_m; }
  mavsdk::Mission::MissionItem::VehicleAction get_vehicle_action(const mavsdk::Mission::MissionItem& s) { return s.vehicle_action; }
} // namespace mavsdk::Mission::MissionItem
namespace MissionPlan {
  const std::vector<MissionItem>& get_mission_items(const mavsdk::Mission::MissionPlan& s) { return s.mission_items; }
} // namespace mavsdk::Mission::MissionPlan
namespace MissionProgress {
  int32_t get_current(const mavsdk::Mission::MissionProgress& s) { return s.current; }
  int32_t get_total(const mavsdk::Mission::MissionProgress& s) { return s.total; }
} // namespace mavsdk::Mission::MissionProgress
namespace ProgressData {
  float get_progress(const mavsdk::Mission::ProgressData& s) { return s.progress; }
} // namespace mavsdk::Mission::ProgressData
namespace ProgressDataOrMission {
  bool get_has_progress(const mavsdk::Mission::ProgressDataOrMission& s) { return s.has_progress; }
  float get_progress(const mavsdk::Mission::ProgressDataOrMission& s) { return s.progress; }
  bool get_has_mission(const mavsdk::Mission::ProgressDataOrMission& s) { return s.has_mission; }
  const mavsdk::Mission::MissionPlan& get_mission_plan(const mavsdk::Mission::ProgressDataOrMission& s) { return s.mission_plan; }
} // namespace mavsdk::Mission::ProgressDataOrMission
