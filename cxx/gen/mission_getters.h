// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace MissionPlanGetters {
  const std::vector<mavsdk::Mission::MissionItem>& get_mission_items(const mavsdk::Mission::MissionPlan& s) { return s.mission_items; }
} // namespace mavsdk::Mission::MissionPlan
namespace ProgressDataOrMissionGetters {
  bool get_has_progress(const mavsdk::Mission::ProgressDataOrMission& s) { return s.has_progress; }
  float get_progress(const mavsdk::Mission::ProgressDataOrMission& s) { return s.progress; }
  bool get_has_mission(const mavsdk::Mission::ProgressDataOrMission& s) { return s.has_mission; }
  const mavsdk::Mission::MissionPlan& get_mission_plan(const mavsdk::Mission::ProgressDataOrMission& s) { return s.mission_plan; }
} // namespace mavsdk::Mission::ProgressDataOrMission
