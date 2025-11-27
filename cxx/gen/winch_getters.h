// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace StatusFlags {
  bool get_healthy(const mavsdk::Winch::StatusFlags& s) { return s.healthy; }
  bool get_fully_retracted(const mavsdk::Winch::StatusFlags& s) { return s.fully_retracted; }
  bool get_moving(const mavsdk::Winch::StatusFlags& s) { return s.moving; }
  bool get_clutch_engaged(const mavsdk::Winch::StatusFlags& s) { return s.clutch_engaged; }
  bool get_locked(const mavsdk::Winch::StatusFlags& s) { return s.locked; }
  bool get_dropping(const mavsdk::Winch::StatusFlags& s) { return s.dropping; }
  bool get_arresting(const mavsdk::Winch::StatusFlags& s) { return s.arresting; }
  bool get_ground_sense(const mavsdk::Winch::StatusFlags& s) { return s.ground_sense; }
  bool get_retracting(const mavsdk::Winch::StatusFlags& s) { return s.retracting; }
  bool get_redeliver(const mavsdk::Winch::StatusFlags& s) { return s.redeliver; }
  bool get_abandon_line(const mavsdk::Winch::StatusFlags& s) { return s.abandon_line; }
  bool get_locking(const mavsdk::Winch::StatusFlags& s) { return s.locking; }
  bool get_load_line(const mavsdk::Winch::StatusFlags& s) { return s.load_line; }
  bool get_load_payload(const mavsdk::Winch::StatusFlags& s) { return s.load_payload; }
} // namespace mavsdk::Winch::StatusFlags
namespace Status {
  uint64_t get_time_usec(const mavsdk::Winch::Status& s) { return s.time_usec; }
  float get_line_length_m(const mavsdk::Winch::Status& s) { return s.line_length_m; }
  float get_speed_m_s(const mavsdk::Winch::Status& s) { return s.speed_m_s; }
  float get_tension_kg(const mavsdk::Winch::Status& s) { return s.tension_kg; }
  float get_voltage_v(const mavsdk::Winch::Status& s) { return s.voltage_v; }
  float get_current_a(const mavsdk::Winch::Status& s) { return s.current_a; }
  int32_t get_temperature_c(const mavsdk::Winch::Status& s) { return s.temperature_c; }
  const mavsdk::Winch::StatusFlags& get_status_flags(const mavsdk::Winch::Status& s) { return s.status_flags; }
} // namespace mavsdk::Winch::Status
