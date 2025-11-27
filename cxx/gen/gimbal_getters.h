// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace Quaternion {
  float get_w(const mavsdk::Gimbal::Quaternion& s) { return s.w; }
  float get_x(const mavsdk::Gimbal::Quaternion& s) { return s.x; }
  float get_y(const mavsdk::Gimbal::Quaternion& s) { return s.y; }
  float get_z(const mavsdk::Gimbal::Quaternion& s) { return s.z; }
} // namespace mavsdk::Gimbal::Quaternion
namespace EulerAngle {
  float get_roll_deg(const mavsdk::Gimbal::EulerAngle& s) { return s.roll_deg; }
  float get_pitch_deg(const mavsdk::Gimbal::EulerAngle& s) { return s.pitch_deg; }
  float get_yaw_deg(const mavsdk::Gimbal::EulerAngle& s) { return s.yaw_deg; }
} // namespace mavsdk::Gimbal::EulerAngle
namespace AngularVelocityBody {
  float get_roll_rad_s(const mavsdk::Gimbal::AngularVelocityBody& s) { return s.roll_rad_s; }
  float get_pitch_rad_s(const mavsdk::Gimbal::AngularVelocityBody& s) { return s.pitch_rad_s; }
  float get_yaw_rad_s(const mavsdk::Gimbal::AngularVelocityBody& s) { return s.yaw_rad_s; }
} // namespace mavsdk::Gimbal::AngularVelocityBody
namespace Attitude {
  int32_t get_gimbal_id(const mavsdk::Gimbal::Attitude& s) { return s.gimbal_id; }
  const mavsdk::Gimbal::EulerAngle& get_euler_angle_forward(const mavsdk::Gimbal::Attitude& s) { return s.euler_angle_forward; }
  const mavsdk::Gimbal::Quaternion& get_quaternion_forward(const mavsdk::Gimbal::Attitude& s) { return s.quaternion_forward; }
  const mavsdk::Gimbal::EulerAngle& get_euler_angle_north(const mavsdk::Gimbal::Attitude& s) { return s.euler_angle_north; }
  const mavsdk::Gimbal::Quaternion& get_quaternion_north(const mavsdk::Gimbal::Attitude& s) { return s.quaternion_north; }
  const mavsdk::Gimbal::AngularVelocityBody& get_angular_velocity(const mavsdk::Gimbal::Attitude& s) { return s.angular_velocity; }
  uint64_t get_timestamp_us(const mavsdk::Gimbal::Attitude& s) { return s.timestamp_us; }
} // namespace mavsdk::Gimbal::Attitude
namespace GimbalItem {
  int32_t get_gimbal_id(const mavsdk::Gimbal::GimbalItem& s) { return s.gimbal_id; }
  const std::string& get_vendor_name(const mavsdk::Gimbal::GimbalItem& s) { return s.vendor_name; }
  const std::string& get_model_name(const mavsdk::Gimbal::GimbalItem& s) { return s.model_name; }
  const std::string& get_custom_name(const mavsdk::Gimbal::GimbalItem& s) { return s.custom_name; }
  int32_t get_gimbal_manager_component_id(const mavsdk::Gimbal::GimbalItem& s) { return s.gimbal_manager_component_id; }
  int32_t get_gimbal_device_id(const mavsdk::Gimbal::GimbalItem& s) { return s.gimbal_device_id; }
} // namespace mavsdk::Gimbal::GimbalItem
namespace GimbalList {
  const std::vector<GimbalItem>& get_gimbals(const mavsdk::Gimbal::GimbalList& s) { return s.gimbals; }
} // namespace mavsdk::Gimbal::GimbalList
namespace ControlStatus {
  int32_t get_gimbal_id(const mavsdk::Gimbal::ControlStatus& s) { return s.gimbal_id; }
  mavsdk::Gimbal::ControlMode get_control_mode(const mavsdk::Gimbal::ControlStatus& s) { return s.control_mode; }
  int32_t get_sysid_primary_control(const mavsdk::Gimbal::ControlStatus& s) { return s.sysid_primary_control; }
  int32_t get_compid_primary_control(const mavsdk::Gimbal::ControlStatus& s) { return s.compid_primary_control; }
  int32_t get_sysid_secondary_control(const mavsdk::Gimbal::ControlStatus& s) { return s.sysid_secondary_control; }
  int32_t get_compid_secondary_control(const mavsdk::Gimbal::ControlStatus& s) { return s.compid_secondary_control; }
} // namespace mavsdk::Gimbal::ControlStatus
