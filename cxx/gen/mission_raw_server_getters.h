// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace MissionItem {
  uint32_t get_seq(const mavsdk::MissionRawServer::MissionItem& s) { return s.seq; }
  uint32_t get_frame(const mavsdk::MissionRawServer::MissionItem& s) { return s.frame; }
  uint32_t get_command(const mavsdk::MissionRawServer::MissionItem& s) { return s.command; }
  uint32_t get_current(const mavsdk::MissionRawServer::MissionItem& s) { return s.current; }
  uint32_t get_autocontinue(const mavsdk::MissionRawServer::MissionItem& s) { return s.autocontinue; }
  float get_param1(const mavsdk::MissionRawServer::MissionItem& s) { return s.param1; }
  float get_param2(const mavsdk::MissionRawServer::MissionItem& s) { return s.param2; }
  float get_param3(const mavsdk::MissionRawServer::MissionItem& s) { return s.param3; }
  float get_param4(const mavsdk::MissionRawServer::MissionItem& s) { return s.param4; }
  int32_t get_x(const mavsdk::MissionRawServer::MissionItem& s) { return s.x; }
  int32_t get_y(const mavsdk::MissionRawServer::MissionItem& s) { return s.y; }
  float get_z(const mavsdk::MissionRawServer::MissionItem& s) { return s.z; }
  uint32_t get_mission_type(const mavsdk::MissionRawServer::MissionItem& s) { return s.mission_type; }
} // namespace mavsdk::MissionRawServer::MissionItem
namespace MissionPlan {
  const std::vector<MissionItem>& get_mission_items(const mavsdk::MissionRawServer::MissionPlan& s) { return s.mission_items; }
} // namespace mavsdk::MissionRawServer::MissionPlan
namespace MissionProgress {
  int32_t get_current(const mavsdk::MissionRawServer::MissionProgress& s) { return s.current; }
  int32_t get_total(const mavsdk::MissionRawServer::MissionProgress& s) { return s.total; }
} // namespace mavsdk::MissionRawServer::MissionProgress
