// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace MissionImportDataGetters {
  const std::vector<mavsdk::MissionRaw::MissionItem>& get_mission_items(const mavsdk::MissionRaw::MissionImportData& s) { return s.mission_items; }
  const std::vector<mavsdk::MissionRaw::MissionItem>& get_geofence_items(const mavsdk::MissionRaw::MissionImportData& s) { return s.geofence_items; }
  const std::vector<mavsdk::MissionRaw::MissionItem>& get_rally_items(const mavsdk::MissionRaw::MissionImportData& s) { return s.rally_items; }
} // namespace mavsdk::MissionRaw::MissionImportData
