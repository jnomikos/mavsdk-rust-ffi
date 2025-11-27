// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace Config {
  float get_follow_height_m(const mavsdk::FollowMe::Config& s) { return s.follow_height_m; }
  float get_follow_distance_m(const mavsdk::FollowMe::Config& s) { return s.follow_distance_m; }
  float get_responsiveness(const mavsdk::FollowMe::Config& s) { return s.responsiveness; }
  mavsdk::FollowMe::Config::FollowAltitudeMode get_altitude_mode(const mavsdk::FollowMe::Config& s) { return s.altitude_mode; }
  float get_max_tangential_vel_m_s(const mavsdk::FollowMe::Config& s) { return s.max_tangential_vel_m_s; }
  float get_follow_angle_deg(const mavsdk::FollowMe::Config& s) { return s.follow_angle_deg; }
} // namespace mavsdk::FollowMe::Config
namespace TargetLocation {
  double get_latitude_deg(const mavsdk::FollowMe::TargetLocation& s) { return s.latitude_deg; }
  double get_longitude_deg(const mavsdk::FollowMe::TargetLocation& s) { return s.longitude_deg; }
  float get_absolute_altitude_m(const mavsdk::FollowMe::TargetLocation& s) { return s.absolute_altitude_m; }
  float get_velocity_x_m_s(const mavsdk::FollowMe::TargetLocation& s) { return s.velocity_x_m_s; }
  float get_velocity_y_m_s(const mavsdk::FollowMe::TargetLocation& s) { return s.velocity_y_m_s; }
  float get_velocity_z_m_s(const mavsdk::FollowMe::TargetLocation& s) { return s.velocity_z_m_s; }
} // namespace mavsdk::FollowMe::TargetLocation
