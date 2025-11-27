// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace Attitude {
  float get_roll_deg(const mavsdk::Offboard::Attitude& s) { return s.roll_deg; }
  float get_pitch_deg(const mavsdk::Offboard::Attitude& s) { return s.pitch_deg; }
  float get_yaw_deg(const mavsdk::Offboard::Attitude& s) { return s.yaw_deg; }
  float get_thrust_value(const mavsdk::Offboard::Attitude& s) { return s.thrust_value; }
} // namespace mavsdk::Offboard::Attitude
namespace ActuatorControlGroup {
  const std::vector<float>& get_controls(const mavsdk::Offboard::ActuatorControlGroup& s) { return s.controls; }
} // namespace mavsdk::Offboard::ActuatorControlGroup
namespace ActuatorControl {
  const std::vector<ActuatorControlGroup>& get_groups(const mavsdk::Offboard::ActuatorControl& s) { return s.groups; }
} // namespace mavsdk::Offboard::ActuatorControl
namespace AttitudeRate {
  float get_roll_deg_s(const mavsdk::Offboard::AttitudeRate& s) { return s.roll_deg_s; }
  float get_pitch_deg_s(const mavsdk::Offboard::AttitudeRate& s) { return s.pitch_deg_s; }
  float get_yaw_deg_s(const mavsdk::Offboard::AttitudeRate& s) { return s.yaw_deg_s; }
  float get_thrust_value(const mavsdk::Offboard::AttitudeRate& s) { return s.thrust_value; }
} // namespace mavsdk::Offboard::AttitudeRate
namespace PositionNedYaw {
  float get_north_m(const mavsdk::Offboard::PositionNedYaw& s) { return s.north_m; }
  float get_east_m(const mavsdk::Offboard::PositionNedYaw& s) { return s.east_m; }
  float get_down_m(const mavsdk::Offboard::PositionNedYaw& s) { return s.down_m; }
  float get_yaw_deg(const mavsdk::Offboard::PositionNedYaw& s) { return s.yaw_deg; }
} // namespace mavsdk::Offboard::PositionNedYaw
namespace PositionGlobalYaw {
  double get_lat_deg(const mavsdk::Offboard::PositionGlobalYaw& s) { return s.lat_deg; }
  double get_lon_deg(const mavsdk::Offboard::PositionGlobalYaw& s) { return s.lon_deg; }
  float get_alt_m(const mavsdk::Offboard::PositionGlobalYaw& s) { return s.alt_m; }
  float get_yaw_deg(const mavsdk::Offboard::PositionGlobalYaw& s) { return s.yaw_deg; }
  mavsdk::Offboard::PositionGlobalYaw::AltitudeType get_altitude_type(const mavsdk::Offboard::PositionGlobalYaw& s) { return s.altitude_type; }
} // namespace mavsdk::Offboard::PositionGlobalYaw
namespace VelocityBodyYawspeed {
  float get_forward_m_s(const mavsdk::Offboard::VelocityBodyYawspeed& s) { return s.forward_m_s; }
  float get_right_m_s(const mavsdk::Offboard::VelocityBodyYawspeed& s) { return s.right_m_s; }
  float get_down_m_s(const mavsdk::Offboard::VelocityBodyYawspeed& s) { return s.down_m_s; }
  float get_yawspeed_deg_s(const mavsdk::Offboard::VelocityBodyYawspeed& s) { return s.yawspeed_deg_s; }
} // namespace mavsdk::Offboard::VelocityBodyYawspeed
namespace VelocityNedYaw {
  float get_north_m_s(const mavsdk::Offboard::VelocityNedYaw& s) { return s.north_m_s; }
  float get_east_m_s(const mavsdk::Offboard::VelocityNedYaw& s) { return s.east_m_s; }
  float get_down_m_s(const mavsdk::Offboard::VelocityNedYaw& s) { return s.down_m_s; }
  float get_yaw_deg(const mavsdk::Offboard::VelocityNedYaw& s) { return s.yaw_deg; }
} // namespace mavsdk::Offboard::VelocityNedYaw
namespace AccelerationNed {
  float get_north_m_s2(const mavsdk::Offboard::AccelerationNed& s) { return s.north_m_s2; }
  float get_east_m_s2(const mavsdk::Offboard::AccelerationNed& s) { return s.east_m_s2; }
  float get_down_m_s2(const mavsdk::Offboard::AccelerationNed& s) { return s.down_m_s2; }
} // namespace mavsdk::Offboard::AccelerationNed
