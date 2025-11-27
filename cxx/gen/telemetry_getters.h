// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace Position {
  double get_latitude_deg(const mavsdk::Telemetry::Position& s) { return s.latitude_deg; }
  double get_longitude_deg(const mavsdk::Telemetry::Position& s) { return s.longitude_deg; }
  float get_absolute_altitude_m(const mavsdk::Telemetry::Position& s) { return s.absolute_altitude_m; }
  float get_relative_altitude_m(const mavsdk::Telemetry::Position& s) { return s.relative_altitude_m; }
} // namespace mavsdk::Telemetry::Position
namespace Heading {
  double get_heading_deg(const mavsdk::Telemetry::Heading& s) { return s.heading_deg; }
} // namespace mavsdk::Telemetry::Heading
namespace Quaternion {
  float get_w(const mavsdk::Telemetry::Quaternion& s) { return s.w; }
  float get_x(const mavsdk::Telemetry::Quaternion& s) { return s.x; }
  float get_y(const mavsdk::Telemetry::Quaternion& s) { return s.y; }
  float get_z(const mavsdk::Telemetry::Quaternion& s) { return s.z; }
  uint64_t get_timestamp_us(const mavsdk::Telemetry::Quaternion& s) { return s.timestamp_us; }
} // namespace mavsdk::Telemetry::Quaternion
namespace EulerAngle {
  float get_roll_deg(const mavsdk::Telemetry::EulerAngle& s) { return s.roll_deg; }
  float get_pitch_deg(const mavsdk::Telemetry::EulerAngle& s) { return s.pitch_deg; }
  float get_yaw_deg(const mavsdk::Telemetry::EulerAngle& s) { return s.yaw_deg; }
  uint64_t get_timestamp_us(const mavsdk::Telemetry::EulerAngle& s) { return s.timestamp_us; }
} // namespace mavsdk::Telemetry::EulerAngle
namespace AngularVelocityBody {
  float get_roll_rad_s(const mavsdk::Telemetry::AngularVelocityBody& s) { return s.roll_rad_s; }
  float get_pitch_rad_s(const mavsdk::Telemetry::AngularVelocityBody& s) { return s.pitch_rad_s; }
  float get_yaw_rad_s(const mavsdk::Telemetry::AngularVelocityBody& s) { return s.yaw_rad_s; }
} // namespace mavsdk::Telemetry::AngularVelocityBody
namespace GpsInfo {
  int32_t get_num_satellites(const mavsdk::Telemetry::GpsInfo& s) { return s.num_satellites; }
  mavsdk::Telemetry::FixType get_fix_type(const mavsdk::Telemetry::GpsInfo& s) { return s.fix_type; }
} // namespace mavsdk::Telemetry::GpsInfo
namespace RawGps {
  uint64_t get_timestamp_us(const mavsdk::Telemetry::RawGps& s) { return s.timestamp_us; }
  double get_latitude_deg(const mavsdk::Telemetry::RawGps& s) { return s.latitude_deg; }
  double get_longitude_deg(const mavsdk::Telemetry::RawGps& s) { return s.longitude_deg; }
  float get_absolute_altitude_m(const mavsdk::Telemetry::RawGps& s) { return s.absolute_altitude_m; }
  float get_hdop(const mavsdk::Telemetry::RawGps& s) { return s.hdop; }
  float get_vdop(const mavsdk::Telemetry::RawGps& s) { return s.vdop; }
  float get_velocity_m_s(const mavsdk::Telemetry::RawGps& s) { return s.velocity_m_s; }
  float get_cog_deg(const mavsdk::Telemetry::RawGps& s) { return s.cog_deg; }
  float get_altitude_ellipsoid_m(const mavsdk::Telemetry::RawGps& s) { return s.altitude_ellipsoid_m; }
  float get_horizontal_uncertainty_m(const mavsdk::Telemetry::RawGps& s) { return s.horizontal_uncertainty_m; }
  float get_vertical_uncertainty_m(const mavsdk::Telemetry::RawGps& s) { return s.vertical_uncertainty_m; }
  float get_velocity_uncertainty_m_s(const mavsdk::Telemetry::RawGps& s) { return s.velocity_uncertainty_m_s; }
  float get_heading_uncertainty_deg(const mavsdk::Telemetry::RawGps& s) { return s.heading_uncertainty_deg; }
  float get_yaw_deg(const mavsdk::Telemetry::RawGps& s) { return s.yaw_deg; }
} // namespace mavsdk::Telemetry::RawGps
namespace Battery {
  uint32_t get_id(const mavsdk::Telemetry::Battery& s) { return s.id; }
  float get_temperature_degc(const mavsdk::Telemetry::Battery& s) { return s.temperature_degc; }
  float get_voltage_v(const mavsdk::Telemetry::Battery& s) { return s.voltage_v; }
  float get_current_battery_a(const mavsdk::Telemetry::Battery& s) { return s.current_battery_a; }
  float get_capacity_consumed_ah(const mavsdk::Telemetry::Battery& s) { return s.capacity_consumed_ah; }
  float get_remaining_percent(const mavsdk::Telemetry::Battery& s) { return s.remaining_percent; }
  float get_time_remaining_s(const mavsdk::Telemetry::Battery& s) { return s.time_remaining_s; }
  mavsdk::Telemetry::BatteryFunction get_battery_function(const mavsdk::Telemetry::Battery& s) { return s.battery_function; }
} // namespace mavsdk::Telemetry::Battery
namespace Health {
  bool get_is_gyrometer_calibration_ok(const mavsdk::Telemetry::Health& s) { return s.is_gyrometer_calibration_ok; }
  bool get_is_accelerometer_calibration_ok(const mavsdk::Telemetry::Health& s) { return s.is_accelerometer_calibration_ok; }
  bool get_is_magnetometer_calibration_ok(const mavsdk::Telemetry::Health& s) { return s.is_magnetometer_calibration_ok; }
  bool get_is_local_position_ok(const mavsdk::Telemetry::Health& s) { return s.is_local_position_ok; }
  bool get_is_global_position_ok(const mavsdk::Telemetry::Health& s) { return s.is_global_position_ok; }
  bool get_is_home_position_ok(const mavsdk::Telemetry::Health& s) { return s.is_home_position_ok; }
  bool get_is_armable(const mavsdk::Telemetry::Health& s) { return s.is_armable; }
} // namespace mavsdk::Telemetry::Health
namespace RcStatus {
  bool get_was_available_once(const mavsdk::Telemetry::RcStatus& s) { return s.was_available_once; }
  bool get_is_available(const mavsdk::Telemetry::RcStatus& s) { return s.is_available; }
  float get_signal_strength_percent(const mavsdk::Telemetry::RcStatus& s) { return s.signal_strength_percent; }
} // namespace mavsdk::Telemetry::RcStatus
namespace StatusText {
  mavsdk::Telemetry::StatusTextType get_type(const mavsdk::Telemetry::StatusText& s) { return s.type; }
  const std::string& get_text(const mavsdk::Telemetry::StatusText& s) { return s.text; }
} // namespace mavsdk::Telemetry::StatusText
namespace ActuatorControlTarget {
  int32_t get_group(const mavsdk::Telemetry::ActuatorControlTarget& s) { return s.group; }
  const std::vector<float>& get_controls(const mavsdk::Telemetry::ActuatorControlTarget& s) { return s.controls; }
} // namespace mavsdk::Telemetry::ActuatorControlTarget
namespace ActuatorOutputStatus {
  uint32_t get_active(const mavsdk::Telemetry::ActuatorOutputStatus& s) { return s.active; }
  const std::vector<float>& get_actuator(const mavsdk::Telemetry::ActuatorOutputStatus& s) { return s.actuator; }
} // namespace mavsdk::Telemetry::ActuatorOutputStatus
namespace Covariance {
  const std::vector<float>& get_covariance_matrix(const mavsdk::Telemetry::Covariance& s) { return s.covariance_matrix; }
} // namespace mavsdk::Telemetry::Covariance
namespace VelocityBody {
  float get_x_m_s(const mavsdk::Telemetry::VelocityBody& s) { return s.x_m_s; }
  float get_y_m_s(const mavsdk::Telemetry::VelocityBody& s) { return s.y_m_s; }
  float get_z_m_s(const mavsdk::Telemetry::VelocityBody& s) { return s.z_m_s; }
} // namespace mavsdk::Telemetry::VelocityBody
namespace PositionBody {
  float get_x_m(const mavsdk::Telemetry::PositionBody& s) { return s.x_m; }
  float get_y_m(const mavsdk::Telemetry::PositionBody& s) { return s.y_m; }
  float get_z_m(const mavsdk::Telemetry::PositionBody& s) { return s.z_m; }
} // namespace mavsdk::Telemetry::PositionBody
namespace Odometry {
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
namespace DistanceSensor {
  float get_minimum_distance_m(const mavsdk::Telemetry::DistanceSensor& s) { return s.minimum_distance_m; }
  float get_maximum_distance_m(const mavsdk::Telemetry::DistanceSensor& s) { return s.maximum_distance_m; }
  float get_current_distance_m(const mavsdk::Telemetry::DistanceSensor& s) { return s.current_distance_m; }
  const mavsdk::Telemetry::EulerAngle& get_orientation(const mavsdk::Telemetry::DistanceSensor& s) { return s.orientation; }
} // namespace mavsdk::Telemetry::DistanceSensor
namespace ScaledPressure {
  uint64_t get_timestamp_us(const mavsdk::Telemetry::ScaledPressure& s) { return s.timestamp_us; }
  float get_absolute_pressure_hpa(const mavsdk::Telemetry::ScaledPressure& s) { return s.absolute_pressure_hpa; }
  float get_differential_pressure_hpa(const mavsdk::Telemetry::ScaledPressure& s) { return s.differential_pressure_hpa; }
  float get_temperature_deg(const mavsdk::Telemetry::ScaledPressure& s) { return s.temperature_deg; }
  float get_differential_pressure_temperature_deg(const mavsdk::Telemetry::ScaledPressure& s) { return s.differential_pressure_temperature_deg; }
} // namespace mavsdk::Telemetry::ScaledPressure
namespace PositionNed {
  float get_north_m(const mavsdk::Telemetry::PositionNed& s) { return s.north_m; }
  float get_east_m(const mavsdk::Telemetry::PositionNed& s) { return s.east_m; }
  float get_down_m(const mavsdk::Telemetry::PositionNed& s) { return s.down_m; }
} // namespace mavsdk::Telemetry::PositionNed
namespace VelocityNed {
  float get_north_m_s(const mavsdk::Telemetry::VelocityNed& s) { return s.north_m_s; }
  float get_east_m_s(const mavsdk::Telemetry::VelocityNed& s) { return s.east_m_s; }
  float get_down_m_s(const mavsdk::Telemetry::VelocityNed& s) { return s.down_m_s; }
} // namespace mavsdk::Telemetry::VelocityNed
namespace PositionVelocityNed {
  const mavsdk::Telemetry::PositionNed& get_position(const mavsdk::Telemetry::PositionVelocityNed& s) { return s.position; }
  const mavsdk::Telemetry::VelocityNed& get_velocity(const mavsdk::Telemetry::PositionVelocityNed& s) { return s.velocity; }
} // namespace mavsdk::Telemetry::PositionVelocityNed
namespace GroundTruth {
  double get_latitude_deg(const mavsdk::Telemetry::GroundTruth& s) { return s.latitude_deg; }
  double get_longitude_deg(const mavsdk::Telemetry::GroundTruth& s) { return s.longitude_deg; }
  float get_absolute_altitude_m(const mavsdk::Telemetry::GroundTruth& s) { return s.absolute_altitude_m; }
} // namespace mavsdk::Telemetry::GroundTruth
namespace FixedwingMetrics {
  float get_airspeed_m_s(const mavsdk::Telemetry::FixedwingMetrics& s) { return s.airspeed_m_s; }
  float get_throttle_percentage(const mavsdk::Telemetry::FixedwingMetrics& s) { return s.throttle_percentage; }
  float get_climb_rate_m_s(const mavsdk::Telemetry::FixedwingMetrics& s) { return s.climb_rate_m_s; }
  float get_groundspeed_m_s(const mavsdk::Telemetry::FixedwingMetrics& s) { return s.groundspeed_m_s; }
  float get_heading_deg(const mavsdk::Telemetry::FixedwingMetrics& s) { return s.heading_deg; }
  float get_absolute_altitude_m(const mavsdk::Telemetry::FixedwingMetrics& s) { return s.absolute_altitude_m; }
} // namespace mavsdk::Telemetry::FixedwingMetrics
namespace AccelerationFrd {
  float get_forward_m_s2(const mavsdk::Telemetry::AccelerationFrd& s) { return s.forward_m_s2; }
  float get_right_m_s2(const mavsdk::Telemetry::AccelerationFrd& s) { return s.right_m_s2; }
  float get_down_m_s2(const mavsdk::Telemetry::AccelerationFrd& s) { return s.down_m_s2; }
} // namespace mavsdk::Telemetry::AccelerationFrd
namespace AngularVelocityFrd {
  float get_forward_rad_s(const mavsdk::Telemetry::AngularVelocityFrd& s) { return s.forward_rad_s; }
  float get_right_rad_s(const mavsdk::Telemetry::AngularVelocityFrd& s) { return s.right_rad_s; }
  float get_down_rad_s(const mavsdk::Telemetry::AngularVelocityFrd& s) { return s.down_rad_s; }
} // namespace mavsdk::Telemetry::AngularVelocityFrd
namespace MagneticFieldFrd {
  float get_forward_gauss(const mavsdk::Telemetry::MagneticFieldFrd& s) { return s.forward_gauss; }
  float get_right_gauss(const mavsdk::Telemetry::MagneticFieldFrd& s) { return s.right_gauss; }
  float get_down_gauss(const mavsdk::Telemetry::MagneticFieldFrd& s) { return s.down_gauss; }
} // namespace mavsdk::Telemetry::MagneticFieldFrd
namespace Imu {
  const mavsdk::Telemetry::AccelerationFrd& get_acceleration_frd(const mavsdk::Telemetry::Imu& s) { return s.acceleration_frd; }
  const mavsdk::Telemetry::AngularVelocityFrd& get_angular_velocity_frd(const mavsdk::Telemetry::Imu& s) { return s.angular_velocity_frd; }
  const mavsdk::Telemetry::MagneticFieldFrd& get_magnetic_field_frd(const mavsdk::Telemetry::Imu& s) { return s.magnetic_field_frd; }
  float get_temperature_degc(const mavsdk::Telemetry::Imu& s) { return s.temperature_degc; }
  uint64_t get_timestamp_us(const mavsdk::Telemetry::Imu& s) { return s.timestamp_us; }
} // namespace mavsdk::Telemetry::Imu
namespace GpsGlobalOrigin {
  double get_latitude_deg(const mavsdk::Telemetry::GpsGlobalOrigin& s) { return s.latitude_deg; }
  double get_longitude_deg(const mavsdk::Telemetry::GpsGlobalOrigin& s) { return s.longitude_deg; }
  float get_altitude_m(const mavsdk::Telemetry::GpsGlobalOrigin& s) { return s.altitude_m; }
} // namespace mavsdk::Telemetry::GpsGlobalOrigin
namespace Altitude {
  float get_altitude_monotonic_m(const mavsdk::Telemetry::Altitude& s) { return s.altitude_monotonic_m; }
  float get_altitude_amsl_m(const mavsdk::Telemetry::Altitude& s) { return s.altitude_amsl_m; }
  float get_altitude_local_m(const mavsdk::Telemetry::Altitude& s) { return s.altitude_local_m; }
  float get_altitude_relative_m(const mavsdk::Telemetry::Altitude& s) { return s.altitude_relative_m; }
  float get_altitude_terrain_m(const mavsdk::Telemetry::Altitude& s) { return s.altitude_terrain_m; }
  float get_bottom_clearance_m(const mavsdk::Telemetry::Altitude& s) { return s.bottom_clearance_m; }
} // namespace mavsdk::Telemetry::Altitude
namespace Wind {
  float get_wind_x_ned_m_s(const mavsdk::Telemetry::Wind& s) { return s.wind_x_ned_m_s; }
  float get_wind_y_ned_m_s(const mavsdk::Telemetry::Wind& s) { return s.wind_y_ned_m_s; }
  float get_wind_z_ned_m_s(const mavsdk::Telemetry::Wind& s) { return s.wind_z_ned_m_s; }
  float get_horizontal_variability_stddev_m_s(const mavsdk::Telemetry::Wind& s) { return s.horizontal_variability_stddev_m_s; }
  float get_vertical_variability_stddev_m_s(const mavsdk::Telemetry::Wind& s) { return s.vertical_variability_stddev_m_s; }
  float get_wind_altitude_msl_m(const mavsdk::Telemetry::Wind& s) { return s.wind_altitude_msl_m; }
  float get_horizontal_wind_speed_accuracy_m_s(const mavsdk::Telemetry::Wind& s) { return s.horizontal_wind_speed_accuracy_m_s; }
  float get_vertical_wind_speed_accuracy_m_s(const mavsdk::Telemetry::Wind& s) { return s.vertical_wind_speed_accuracy_m_s; }
} // namespace mavsdk::Telemetry::Wind
