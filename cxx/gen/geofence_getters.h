// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace PolygonGetters {
  const std::vector<mavsdk::Geofence::Point>& get_points(const mavsdk::Geofence::Polygon& s) { return s.points; }
  mavsdk::Geofence::FenceType get_fence_type(const mavsdk::Geofence::Polygon& s) { return s.fence_type; }
} // namespace mavsdk::Geofence::Polygon
namespace GeofenceDataGetters {
  const std::vector<mavsdk::Geofence::Polygon>& get_polygons(const mavsdk::Geofence::GeofenceData& s) { return s.polygons; }
  const std::vector<mavsdk::Geofence::Circle>& get_circles(const mavsdk::Geofence::GeofenceData& s) { return s.circles; }
} // namespace mavsdk::Geofence::GeofenceData
