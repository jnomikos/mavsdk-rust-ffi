// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace CovarianceGetters {
  const std::vector<float>& get_covariance_matrix(const mavsdk::Mocap::Covariance& s) { return s.covariance_matrix; }
} // namespace mavsdk::Mocap::Covariance
namespace VisionPositionEstimateGetters {
  uint64_t get_time_usec(const mavsdk::Mocap::VisionPositionEstimate& s) { return s.time_usec; }
  const mavsdk::Mocap::PositionBody& get_position_body(const mavsdk::Mocap::VisionPositionEstimate& s) { return s.position_body; }
  const mavsdk::Mocap::AngleBody& get_angle_body(const mavsdk::Mocap::VisionPositionEstimate& s) { return s.angle_body; }
  const mavsdk::Mocap::Covariance& get_pose_covariance(const mavsdk::Mocap::VisionPositionEstimate& s) { return s.pose_covariance; }
} // namespace mavsdk::Mocap::VisionPositionEstimate
namespace VisionSpeedEstimateGetters {
  uint64_t get_time_usec(const mavsdk::Mocap::VisionSpeedEstimate& s) { return s.time_usec; }
  const mavsdk::Mocap::SpeedNed& get_speed_ned(const mavsdk::Mocap::VisionSpeedEstimate& s) { return s.speed_ned; }
  const mavsdk::Mocap::Covariance& get_speed_covariance(const mavsdk::Mocap::VisionSpeedEstimate& s) { return s.speed_covariance; }
} // namespace mavsdk::Mocap::VisionSpeedEstimate
namespace AttitudePositionMocapGetters {
  uint64_t get_time_usec(const mavsdk::Mocap::AttitudePositionMocap& s) { return s.time_usec; }
  const mavsdk::Mocap::Quaternion& get_q(const mavsdk::Mocap::AttitudePositionMocap& s) { return s.q; }
  const mavsdk::Mocap::PositionBody& get_position_body(const mavsdk::Mocap::AttitudePositionMocap& s) { return s.position_body; }
  const mavsdk::Mocap::Covariance& get_pose_covariance(const mavsdk::Mocap::AttitudePositionMocap& s) { return s.pose_covariance; }
} // namespace mavsdk::Mocap::AttitudePositionMocap
namespace OdometryGetters {
  uint64_t get_time_usec(const mavsdk::Mocap::Odometry& s) { return s.time_usec; }
  mavsdk::Mocap::Odometry::MavFrame get_frame_id(const mavsdk::Mocap::Odometry& s) { return s.frame_id; }
  const mavsdk::Mocap::PositionBody& get_position_body(const mavsdk::Mocap::Odometry& s) { return s.position_body; }
  const mavsdk::Mocap::Quaternion& get_q(const mavsdk::Mocap::Odometry& s) { return s.q; }
  const mavsdk::Mocap::SpeedBody& get_speed_body(const mavsdk::Mocap::Odometry& s) { return s.speed_body; }
  const mavsdk::Mocap::AngularVelocityBody& get_angular_velocity_body(const mavsdk::Mocap::Odometry& s) { return s.angular_velocity_body; }
  const mavsdk::Mocap::Covariance& get_pose_covariance(const mavsdk::Mocap::Odometry& s) { return s.pose_covariance; }
  const mavsdk::Mocap::Covariance& get_velocity_covariance(const mavsdk::Mocap::Odometry& s) { return s.velocity_covariance; }
} // namespace mavsdk::Mocap::Odometry
