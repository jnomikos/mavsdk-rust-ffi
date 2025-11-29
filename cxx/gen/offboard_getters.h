// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace ActuatorControlGroupGetters {
  const std::vector<float>& get_controls(const mavsdk::Offboard::ActuatorControlGroup& s) { return s.controls; }
} // namespace mavsdk::Offboard::ActuatorControlGroup
namespace ActuatorControlGetters {
  const std::vector<mavsdk::Offboard::ActuatorControlGroup>& get_groups(const mavsdk::Offboard::ActuatorControl& s) { return s.groups; }
} // namespace mavsdk::Offboard::ActuatorControl
