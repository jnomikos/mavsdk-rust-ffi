// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace PositionBody {
  float get_x_m(const mavsdk::Mocap::PositionBody& s) { return s.x_m; }
  float get_y_m(const mavsdk::Mocap::PositionBody& s) { return s.y_m; }
  float get_z_m(const mavsdk::Mocap::PositionBody& s) { return s.z_m; }
} // namespace mavsdk::Mocap::PositionBody
namespace AngleBody {
  float get_roll_rad(const mavsdk::Mocap::AngleBody& s) { return s.roll_rad; }
  float get_pitch_rad(const mavsdk::Mocap::AngleBody& s) { return s.pitch_rad; }
  float get_yaw_rad(const mavsdk::Mocap::AngleBody& s) { return s.yaw_rad; }
} // namespace mavsdk::Mocap::AngleBody
namespace SpeedBody {
  float get_x_m_s(const mavsdk::Mocap::SpeedBody& s) { return s.x_m_s; }
  float get_y_m_s(const mavsdk::Mocap::SpeedBody& s) { return s.y_m_s; }
  float get_z_m_s(const mavsdk::Mocap::SpeedBody& s) { return s.z_m_s; }
} // namespace mavsdk::Mocap::SpeedBody
namespace SpeedNed {
  float get_north_m_s(const mavsdk::Mocap::SpeedNed& s) { return s.north_m_s; }
  float get_east_m_s(const mavsdk::Mocap::SpeedNed& s) { return s.east_m_s; }
  float get_down_m_s(const mavsdk::Mocap::SpeedNed& s) { return s.down_m_s; }
} // namespace mavsdk::Mocap::SpeedNed
namespace AngularVelocityBody {
  float get_roll_rad_s(const mavsdk::Mocap::AngularVelocityBody& s) { return s.roll_rad_s; }
  float get_pitch_rad_s(const mavsdk::Mocap::AngularVelocityBody& s) { return s.pitch_rad_s; }
  float get_yaw_rad_s(const mavsdk::Mocap::AngularVelocityBody& s) { return s.yaw_rad_s; }
} // namespace mavsdk::Mocap::AngularVelocityBody
namespace Covariance {
  const std::vector<float>& get_covariance_matrix(const mavsdk::Mocap::Covariance& s) { return s.covariance_matrix; }
} // namespace mavsdk::Mocap::Covariance
namespace Quaternion {
  float get_w(const mavsdk::Mocap::Quaternion& s) { return s.w; }
  float get_x(const mavsdk::Mocap::Quaternion& s) { return s.x; }
  float get_y(const mavsdk::Mocap::Quaternion& s) { return s.y; }
  float get_z(const mavsdk::Mocap::Quaternion& s) { return s.z; }
} // namespace mavsdk::Mocap::Quaternion
namespace VisionPositionEstimate {
  uint64_t get_time_usec(const mavsdk::Mocap::VisionPositionEstimate& s) { return s.time_usec; }
  const mavsdk::Mocap::PositionBody& get_position_body(const mavsdk::Mocap::VisionPositionEstimate& s) { return s.position_body; }
  const mavsdk::Mocap::AngleBody& get_angle_body(const mavsdk::Mocap::VisionPositionEstimate& s) { return s.angle_body; }
  const mavsdk::Mocap::Covariance& get_pose_covariance(const mavsdk::Mocap::VisionPositionEstimate& s) { return s.pose_covariance; }
} // namespace mavsdk::Mocap::VisionPositionEstimate
namespace VisionSpeedEstimate {
  uint64_t get_time_usec(const mavsdk::Mocap::VisionSpeedEstimate& s) { return s.time_usec; }
  const mavsdk::Mocap::SpeedNed& get_speed_ned(const mavsdk::Mocap::VisionSpeedEstimate& s) { return s.speed_ned; }
  const mavsdk::Mocap::Covariance& get_speed_covariance(const mavsdk::Mocap::VisionSpeedEstimate& s) { return s.speed_covariance; }
} // namespace mavsdk::Mocap::VisionSpeedEstimate
namespace AttitudePositionMocap {
  uint64_t get_time_usec(const mavsdk::Mocap::AttitudePositionMocap& s) { return s.time_usec; }
  const mavsdk::Mocap::Quaternion& get_q(const mavsdk::Mocap::AttitudePositionMocap& s) { return s.q; }
  const mavsdk::Mocap::PositionBody& get_position_body(const mavsdk::Mocap::AttitudePositionMocap& s) { return s.position_body; }
  const mavsdk::Mocap::Covariance& get_pose_covariance(const mavsdk::Mocap::AttitudePositionMocap& s) { return s.pose_covariance; }
} // namespace mavsdk::Mocap::AttitudePositionMocap
namespace Odometry {
  uint64_t get_time_usec(const mavsdk::Mocap::Odometry& s) { return s.time_usec; }
  mavsdk::Mocap::Odometry::MavFrame get_frame_id(const mavsdk::Mocap::Odometry& s) { return s.frame_id; }
  const mavsdk::Mocap::PositionBody& get_position_body(const mavsdk::Mocap::Odometry& s) { return s.position_body; }
  const mavsdk::Mocap::Quaternion& get_q(const mavsdk::Mocap::Odometry& s) { return s.q; }
  const mavsdk::Mocap::SpeedBody& get_speed_body(const mavsdk::Mocap::Odometry& s) { return s.speed_body; }
  const mavsdk::Mocap::AngularVelocityBody& get_angular_velocity_body(const mavsdk::Mocap::Odometry& s) { return s.angular_velocity_body; }
  const mavsdk::Mocap::Covariance& get_pose_covariance(const mavsdk::Mocap::Odometry& s) { return s.pose_covariance; }
  const mavsdk::Mocap::Covariance& get_velocity_covariance(const mavsdk::Mocap::Odometry& s) { return s.velocity_covariance; }
} // namespace mavsdk::Mocap::Odometry
