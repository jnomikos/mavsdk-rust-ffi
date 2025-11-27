// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace Point {
  double get_latitude_deg(const mavsdk::Geofence::Point& s) { return s.latitude_deg; }
  double get_longitude_deg(const mavsdk::Geofence::Point& s) { return s.longitude_deg; }
} // namespace mavsdk::Geofence::Point
namespace Polygon {
  const std::vector<Point>& get_points(const mavsdk::Geofence::Polygon& s) { return s.points; }
  mavsdk::Geofence::FenceType get_fence_type(const mavsdk::Geofence::Polygon& s) { return s.fence_type; }
} // namespace mavsdk::Geofence::Polygon
namespace Circle {
  const mavsdk::Geofence::Point& get_point(const mavsdk::Geofence::Circle& s) { return s.point; }
  float get_radius(const mavsdk::Geofence::Circle& s) { return s.radius; }
  mavsdk::Geofence::FenceType get_fence_type(const mavsdk::Geofence::Circle& s) { return s.fence_type; }
} // namespace mavsdk::Geofence::Circle
namespace GeofenceData {
  const std::vector<Polygon>& get_polygons(const mavsdk::Geofence::GeofenceData& s) { return s.polygons; }
  const std::vector<Circle>& get_circles(const mavsdk::Geofence::GeofenceData& s) { return s.circles; }
} // namespace mavsdk::Geofence::GeofenceData
