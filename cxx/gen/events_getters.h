// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace Event {
  uint32_t get_compid(const mavsdk::Events::Event& s) { return s.compid; }
  const std::string& get_message(const mavsdk::Events::Event& s) { return s.message; }
  const std::string& get_description(const mavsdk::Events::Event& s) { return s.description; }
  mavsdk::Events::LogLevel get_log_level(const mavsdk::Events::Event& s) { return s.log_level; }
  const std::string& get_event_namespace(const mavsdk::Events::Event& s) { return s.event_namespace; }
  const std::string& get_event_name(const mavsdk::Events::Event& s) { return s.event_name; }
} // namespace mavsdk::Events::Event
namespace HealthAndArmingCheckProblem {
  const std::string& get_message(const mavsdk::Events::HealthAndArmingCheckProblem& s) { return s.message; }
  const std::string& get_description(const mavsdk::Events::HealthAndArmingCheckProblem& s) { return s.description; }
  mavsdk::Events::LogLevel get_log_level(const mavsdk::Events::HealthAndArmingCheckProblem& s) { return s.log_level; }
  const std::string& get_health_component(const mavsdk::Events::HealthAndArmingCheckProblem& s) { return s.health_component; }
} // namespace mavsdk::Events::HealthAndArmingCheckProblem
namespace HealthAndArmingCheckMode {
  const std::string& get_mode_name(const mavsdk::Events::HealthAndArmingCheckMode& s) { return s.mode_name; }
  bool get_can_arm_or_run(const mavsdk::Events::HealthAndArmingCheckMode& s) { return s.can_arm_or_run; }
  const std::vector<HealthAndArmingCheckProblem>& get_problems(const mavsdk::Events::HealthAndArmingCheckMode& s) { return s.problems; }
} // namespace mavsdk::Events::HealthAndArmingCheckMode
namespace HealthComponentReport {
  const std::string& get_name(const mavsdk::Events::HealthComponentReport& s) { return s.name; }
  const std::string& get_label(const mavsdk::Events::HealthComponentReport& s) { return s.label; }
  bool get_is_present(const mavsdk::Events::HealthComponentReport& s) { return s.is_present; }
  bool get_has_error(const mavsdk::Events::HealthComponentReport& s) { return s.has_error; }
  bool get_has_warning(const mavsdk::Events::HealthComponentReport& s) { return s.has_warning; }
} // namespace mavsdk::Events::HealthComponentReport
namespace HealthAndArmingCheckReport {
  const mavsdk::Events::HealthAndArmingCheckMode& get_current_mode_intention(const mavsdk::Events::HealthAndArmingCheckReport& s) { return s.current_mode_intention; }
  const std::vector<HealthComponentReport>& get_health_components(const mavsdk::Events::HealthAndArmingCheckReport& s) { return s.health_components; }
  const std::vector<HealthAndArmingCheckProblem>& get_all_problems(const mavsdk::Events::HealthAndArmingCheckReport& s) { return s.all_problems; }
} // namespace mavsdk::Events::HealthAndArmingCheckReport
