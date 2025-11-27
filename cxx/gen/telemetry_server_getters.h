// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace Position {
  double get_latitude_deg(const mavsdk::TelemetryServer::Position& s) { return s.latitude_deg; }
  double get_longitude_deg(const mavsdk::TelemetryServer::Position& s) { return s.longitude_deg; }
  float get_absolute_altitude_m(const mavsdk::TelemetryServer::Position& s) { return s.absolute_altitude_m; }
  float get_relative_altitude_m(const mavsdk::TelemetryServer::Position& s) { return s.relative_altitude_m; }
} // namespace mavsdk::TelemetryServer::Position
namespace Heading {
  double get_heading_deg(const mavsdk::TelemetryServer::Heading& s) { return s.heading_deg; }
} // namespace mavsdk::TelemetryServer::Heading
namespace Quaternion {
  float get_w(const mavsdk::TelemetryServer::Quaternion& s) { return s.w; }
  float get_x(const mavsdk::TelemetryServer::Quaternion& s) { return s.x; }
  float get_y(const mavsdk::TelemetryServer::Quaternion& s) { return s.y; }
  float get_z(const mavsdk::TelemetryServer::Quaternion& s) { return s.z; }
  uint64_t get_timestamp_us(const mavsdk::TelemetryServer::Quaternion& s) { return s.timestamp_us; }
} // namespace mavsdk::TelemetryServer::Quaternion
namespace EulerAngle {
  float get_roll_deg(const mavsdk::TelemetryServer::EulerAngle& s) { return s.roll_deg; }
  float get_pitch_deg(const mavsdk::TelemetryServer::EulerAngle& s) { return s.pitch_deg; }
  float get_yaw_deg(const mavsdk::TelemetryServer::EulerAngle& s) { return s.yaw_deg; }
  uint64_t get_timestamp_us(const mavsdk::TelemetryServer::EulerAngle& s) { return s.timestamp_us; }
} // namespace mavsdk::TelemetryServer::EulerAngle
namespace AngularVelocityBody {
  float get_roll_rad_s(const mavsdk::TelemetryServer::AngularVelocityBody& s) { return s.roll_rad_s; }
  float get_pitch_rad_s(const mavsdk::TelemetryServer::AngularVelocityBody& s) { return s.pitch_rad_s; }
  float get_yaw_rad_s(const mavsdk::TelemetryServer::AngularVelocityBody& s) { return s.yaw_rad_s; }
} // namespace mavsdk::TelemetryServer::AngularVelocityBody
namespace GpsInfo {
  int32_t get_num_satellites(const mavsdk::TelemetryServer::GpsInfo& s) { return s.num_satellites; }
  mavsdk::TelemetryServer::FixType get_fix_type(const mavsdk::TelemetryServer::GpsInfo& s) { return s.fix_type; }
} // namespace mavsdk::TelemetryServer::GpsInfo
namespace RawGps {
  uint64_t get_timestamp_us(const mavsdk::TelemetryServer::RawGps& s) { return s.timestamp_us; }
  double get_latitude_deg(const mavsdk::TelemetryServer::RawGps& s) { return s.latitude_deg; }
  double get_longitude_deg(const mavsdk::TelemetryServer::RawGps& s) { return s.longitude_deg; }
  float get_absolute_altitude_m(const mavsdk::TelemetryServer::RawGps& s) { return s.absolute_altitude_m; }
  float get_hdop(const mavsdk::TelemetryServer::RawGps& s) { return s.hdop; }
  float get_vdop(const mavsdk::TelemetryServer::RawGps& s) { return s.vdop; }
  float get_velocity_m_s(const mavsdk::TelemetryServer::RawGps& s) { return s.velocity_m_s; }
  float get_cog_deg(const mavsdk::TelemetryServer::RawGps& s) { return s.cog_deg; }
  float get_altitude_ellipsoid_m(const mavsdk::TelemetryServer::RawGps& s) { return s.altitude_ellipsoid_m; }
  float get_horizontal_uncertainty_m(const mavsdk::TelemetryServer::RawGps& s) { return s.horizontal_uncertainty_m; }
  float get_vertical_uncertainty_m(const mavsdk::TelemetryServer::RawGps& s) { return s.vertical_uncertainty_m; }
  float get_velocity_uncertainty_m_s(const mavsdk::TelemetryServer::RawGps& s) { return s.velocity_uncertainty_m_s; }
  float get_heading_uncertainty_deg(const mavsdk::TelemetryServer::RawGps& s) { return s.heading_uncertainty_deg; }
  float get_yaw_deg(const mavsdk::TelemetryServer::RawGps& s) { return s.yaw_deg; }
} // namespace mavsdk::TelemetryServer::RawGps
namespace Battery {
  float get_voltage_v(const mavsdk::TelemetryServer::Battery& s) { return s.voltage_v; }
  float get_remaining_percent(const mavsdk::TelemetryServer::Battery& s) { return s.remaining_percent; }
} // namespace mavsdk::TelemetryServer::Battery
namespace RcStatus {
  bool get_was_available_once(const mavsdk::TelemetryServer::RcStatus& s) { return s.was_available_once; }
  bool get_is_available(const mavsdk::TelemetryServer::RcStatus& s) { return s.is_available; }
  float get_signal_strength_percent(const mavsdk::TelemetryServer::RcStatus& s) { return s.signal_strength_percent; }
} // namespace mavsdk::TelemetryServer::RcStatus
namespace StatusText {
  mavsdk::TelemetryServer::StatusTextType get_type(const mavsdk::TelemetryServer::StatusText& s) { return s.type; }
  const std::string& get_text(const mavsdk::TelemetryServer::StatusText& s) { return s.text; }
} // namespace mavsdk::TelemetryServer::StatusText
namespace ActuatorControlTarget {
  int32_t get_group(const mavsdk::TelemetryServer::ActuatorControlTarget& s) { return s.group; }
  const std::vector<float>& get_controls(const mavsdk::TelemetryServer::ActuatorControlTarget& s) { return s.controls; }
} // namespace mavsdk::TelemetryServer::ActuatorControlTarget
namespace ActuatorOutputStatus {
  uint32_t get_active(const mavsdk::TelemetryServer::ActuatorOutputStatus& s) { return s.active; }
  const std::vector<float>& get_actuator(const mavsdk::TelemetryServer::ActuatorOutputStatus& s) { return s.actuator; }
} // namespace mavsdk::TelemetryServer::ActuatorOutputStatus
namespace Covariance {
  const std::vector<float>& get_covariance_matrix(const mavsdk::TelemetryServer::Covariance& s) { return s.covariance_matrix; }
} // namespace mavsdk::TelemetryServer::Covariance
namespace VelocityBody {
  float get_x_m_s(const mavsdk::TelemetryServer::VelocityBody& s) { return s.x_m_s; }
  float get_y_m_s(const mavsdk::TelemetryServer::VelocityBody& s) { return s.y_m_s; }
  float get_z_m_s(const mavsdk::TelemetryServer::VelocityBody& s) { return s.z_m_s; }
} // namespace mavsdk::TelemetryServer::VelocityBody
namespace PositionBody {
  float get_x_m(const mavsdk::TelemetryServer::PositionBody& s) { return s.x_m; }
  float get_y_m(const mavsdk::TelemetryServer::PositionBody& s) { return s.y_m; }
  float get_z_m(const mavsdk::TelemetryServer::PositionBody& s) { return s.z_m; }
} // namespace mavsdk::TelemetryServer::PositionBody
namespace Odometry {
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
namespace DistanceSensor {
  float get_minimum_distance_m(const mavsdk::TelemetryServer::DistanceSensor& s) { return s.minimum_distance_m; }
  float get_maximum_distance_m(const mavsdk::TelemetryServer::DistanceSensor& s) { return s.maximum_distance_m; }
  float get_current_distance_m(const mavsdk::TelemetryServer::DistanceSensor& s) { return s.current_distance_m; }
} // namespace mavsdk::TelemetryServer::DistanceSensor
namespace ScaledPressure {
  uint64_t get_timestamp_us(const mavsdk::TelemetryServer::ScaledPressure& s) { return s.timestamp_us; }
  float get_absolute_pressure_hpa(const mavsdk::TelemetryServer::ScaledPressure& s) { return s.absolute_pressure_hpa; }
  float get_differential_pressure_hpa(const mavsdk::TelemetryServer::ScaledPressure& s) { return s.differential_pressure_hpa; }
  float get_temperature_deg(const mavsdk::TelemetryServer::ScaledPressure& s) { return s.temperature_deg; }
  float get_differential_pressure_temperature_deg(const mavsdk::TelemetryServer::ScaledPressure& s) { return s.differential_pressure_temperature_deg; }
} // namespace mavsdk::TelemetryServer::ScaledPressure
namespace PositionNed {
  float get_north_m(const mavsdk::TelemetryServer::PositionNed& s) { return s.north_m; }
  float get_east_m(const mavsdk::TelemetryServer::PositionNed& s) { return s.east_m; }
  float get_down_m(const mavsdk::TelemetryServer::PositionNed& s) { return s.down_m; }
} // namespace mavsdk::TelemetryServer::PositionNed
namespace VelocityNed {
  float get_north_m_s(const mavsdk::TelemetryServer::VelocityNed& s) { return s.north_m_s; }
  float get_east_m_s(const mavsdk::TelemetryServer::VelocityNed& s) { return s.east_m_s; }
  float get_down_m_s(const mavsdk::TelemetryServer::VelocityNed& s) { return s.down_m_s; }
} // namespace mavsdk::TelemetryServer::VelocityNed
namespace PositionVelocityNed {
  const mavsdk::TelemetryServer::PositionNed& get_position(const mavsdk::TelemetryServer::PositionVelocityNed& s) { return s.position; }
  const mavsdk::TelemetryServer::VelocityNed& get_velocity(const mavsdk::TelemetryServer::PositionVelocityNed& s) { return s.velocity; }
} // namespace mavsdk::TelemetryServer::PositionVelocityNed
namespace GroundTruth {
  double get_latitude_deg(const mavsdk::TelemetryServer::GroundTruth& s) { return s.latitude_deg; }
  double get_longitude_deg(const mavsdk::TelemetryServer::GroundTruth& s) { return s.longitude_deg; }
  float get_absolute_altitude_m(const mavsdk::TelemetryServer::GroundTruth& s) { return s.absolute_altitude_m; }
} // namespace mavsdk::TelemetryServer::GroundTruth
namespace FixedwingMetrics {
  float get_airspeed_m_s(const mavsdk::TelemetryServer::FixedwingMetrics& s) { return s.airspeed_m_s; }
  float get_throttle_percentage(const mavsdk::TelemetryServer::FixedwingMetrics& s) { return s.throttle_percentage; }
  float get_climb_rate_m_s(const mavsdk::TelemetryServer::FixedwingMetrics& s) { return s.climb_rate_m_s; }
  float get_groundspeed_m_s(const mavsdk::TelemetryServer::FixedwingMetrics& s) { return s.groundspeed_m_s; }
  float get_heading_deg(const mavsdk::TelemetryServer::FixedwingMetrics& s) { return s.heading_deg; }
  float get_absolute_altitude_m(const mavsdk::TelemetryServer::FixedwingMetrics& s) { return s.absolute_altitude_m; }
} // namespace mavsdk::TelemetryServer::FixedwingMetrics
namespace AccelerationFrd {
  float get_forward_m_s2(const mavsdk::TelemetryServer::AccelerationFrd& s) { return s.forward_m_s2; }
  float get_right_m_s2(const mavsdk::TelemetryServer::AccelerationFrd& s) { return s.right_m_s2; }
  float get_down_m_s2(const mavsdk::TelemetryServer::AccelerationFrd& s) { return s.down_m_s2; }
} // namespace mavsdk::TelemetryServer::AccelerationFrd
namespace AngularVelocityFrd {
  float get_forward_rad_s(const mavsdk::TelemetryServer::AngularVelocityFrd& s) { return s.forward_rad_s; }
  float get_right_rad_s(const mavsdk::TelemetryServer::AngularVelocityFrd& s) { return s.right_rad_s; }
  float get_down_rad_s(const mavsdk::TelemetryServer::AngularVelocityFrd& s) { return s.down_rad_s; }
} // namespace mavsdk::TelemetryServer::AngularVelocityFrd
namespace MagneticFieldFrd {
  float get_forward_gauss(const mavsdk::TelemetryServer::MagneticFieldFrd& s) { return s.forward_gauss; }
  float get_right_gauss(const mavsdk::TelemetryServer::MagneticFieldFrd& s) { return s.right_gauss; }
  float get_down_gauss(const mavsdk::TelemetryServer::MagneticFieldFrd& s) { return s.down_gauss; }
} // namespace mavsdk::TelemetryServer::MagneticFieldFrd
namespace Imu {
  const mavsdk::TelemetryServer::AccelerationFrd& get_acceleration_frd(const mavsdk::TelemetryServer::Imu& s) { return s.acceleration_frd; }
  const mavsdk::TelemetryServer::AngularVelocityFrd& get_angular_velocity_frd(const mavsdk::TelemetryServer::Imu& s) { return s.angular_velocity_frd; }
  const mavsdk::TelemetryServer::MagneticFieldFrd& get_magnetic_field_frd(const mavsdk::TelemetryServer::Imu& s) { return s.magnetic_field_frd; }
  float get_temperature_degc(const mavsdk::TelemetryServer::Imu& s) { return s.temperature_degc; }
  uint64_t get_timestamp_us(const mavsdk::TelemetryServer::Imu& s) { return s.timestamp_us; }
} // namespace mavsdk::TelemetryServer::Imu
