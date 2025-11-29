// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace StatusTextGetters {
  mavsdk::TelemetryServer::StatusTextType get_type(const mavsdk::TelemetryServer::StatusText& s) { return s.type; }
  const std::string& get_text(const mavsdk::TelemetryServer::StatusText& s) { return s.text; }
} // namespace mavsdk::TelemetryServer::StatusText
namespace ActuatorControlTargetGetters {
  int32_t get_group(const mavsdk::TelemetryServer::ActuatorControlTarget& s) { return s.group; }
  const std::vector<float>& get_controls(const mavsdk::TelemetryServer::ActuatorControlTarget& s) { return s.controls; }
} // namespace mavsdk::TelemetryServer::ActuatorControlTarget
namespace ActuatorOutputStatusGetters {
  uint32_t get_active(const mavsdk::TelemetryServer::ActuatorOutputStatus& s) { return s.active; }
  const std::vector<float>& get_actuator(const mavsdk::TelemetryServer::ActuatorOutputStatus& s) { return s.actuator; }
} // namespace mavsdk::TelemetryServer::ActuatorOutputStatus
namespace CovarianceGetters {
  const std::vector<float>& get_covariance_matrix(const mavsdk::TelemetryServer::Covariance& s) { return s.covariance_matrix; }
} // namespace mavsdk::TelemetryServer::Covariance
namespace OdometryGetters {
  uint64_t get_time_usec(const mavsdk::TelemetryServer::Odometry& s) { return s.time_usec; }
  mavsdk::TelemetryServer::Odometry::MavFrame get_frame_id(const mavsdk::TelemetryServer::Odometry& s) { return s.frame_id; }
  mavsdk::TelemetryServer::Odometry::MavFrame get_child_frame_id(const mavsdk::TelemetryServer::Odometry& s) { return s.child_frame_id; }
  const mavsdk::TelemetryServer::PositionBody& get_position_body(const mavsdk::TelemetryServer::Odometry& s) { return s.position_body; }
  const mavsdk::TelemetryServer::Quaternion& get_q(const mavsdk::TelemetryServer::Odometry& s) { return s.q; }
  const mavsdk::TelemetryServer::VelocityBody& get_velocity_body(const mavsdk::TelemetryServer::Odometry& s) { return s.velocity_body; }
  const mavsdk::TelemetryServer::AngularVelocityBody& get_angular_velocity_body(const mavsdk::TelemetryServer::Odometry& s) { return s.angular_velocity_body; }
  const mavsdk::TelemetryServer::Covariance& get_pose_covariance(const mavsdk::TelemetryServer::Odometry& s) { return s.pose_covariance; }
  const mavsdk::TelemetryServer::Covariance& get_velocity_covariance(const mavsdk::TelemetryServer::Odometry& s) { return s.velocity_covariance; }
} // namespace mavsdk::TelemetryServer::Odometry
