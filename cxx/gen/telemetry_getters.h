// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace StatusTextGetters {
  mavsdk::Telemetry::StatusTextType get_type(const mavsdk::Telemetry::StatusText& s) { return s.type; }
  const std::string& get_text(const mavsdk::Telemetry::StatusText& s) { return s.text; }
} // namespace mavsdk::Telemetry::StatusText
namespace ActuatorControlTargetGetters {
  int32_t get_group(const mavsdk::Telemetry::ActuatorControlTarget& s) { return s.group; }
  const std::vector<float>& get_controls(const mavsdk::Telemetry::ActuatorControlTarget& s) { return s.controls; }
} // namespace mavsdk::Telemetry::ActuatorControlTarget
namespace ActuatorOutputStatusGetters {
  uint32_t get_active(const mavsdk::Telemetry::ActuatorOutputStatus& s) { return s.active; }
  const std::vector<float>& get_actuator(const mavsdk::Telemetry::ActuatorOutputStatus& s) { return s.actuator; }
} // namespace mavsdk::Telemetry::ActuatorOutputStatus
namespace CovarianceGetters {
  const std::vector<float>& get_covariance_matrix(const mavsdk::Telemetry::Covariance& s) { return s.covariance_matrix; }
} // namespace mavsdk::Telemetry::Covariance
namespace OdometryGetters {
  uint64_t get_time_usec(const mavsdk::Telemetry::Odometry& s) { return s.time_usec; }
  mavsdk::Telemetry::Odometry::MavFrame get_frame_id(const mavsdk::Telemetry::Odometry& s) { return s.frame_id; }
  mavsdk::Telemetry::Odometry::MavFrame get_child_frame_id(const mavsdk::Telemetry::Odometry& s) { return s.child_frame_id; }
  const mavsdk::Telemetry::PositionBody& get_position_body(const mavsdk::Telemetry::Odometry& s) { return s.position_body; }
  const mavsdk::Telemetry::Quaternion& get_q(const mavsdk::Telemetry::Odometry& s) { return s.q; }
  const mavsdk::Telemetry::VelocityBody& get_velocity_body(const mavsdk::Telemetry::Odometry& s) { return s.velocity_body; }
  const mavsdk::Telemetry::AngularVelocityBody& get_angular_velocity_body(const mavsdk::Telemetry::Odometry& s) { return s.angular_velocity_body; }
  const mavsdk::Telemetry::Covariance& get_pose_covariance(const mavsdk::Telemetry::Odometry& s) { return s.pose_covariance; }
  const mavsdk::Telemetry::Covariance& get_velocity_covariance(const mavsdk::Telemetry::Odometry& s) { return s.velocity_covariance; }
} // namespace mavsdk::Telemetry::Odometry
