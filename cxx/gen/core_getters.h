// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace ConnectionErrorGetters {
  const std::string& get_error_description(const mavsdk::Mavsdk::ConnectionError& s) { return s.error_description; }
  const mavsdk::Mavsdk::ConnectionHandle& get_connection_handle(const mavsdk::Mavsdk::ConnectionError& s) { return s.connection_handle; }
} // namespace mavsdk::Mavsdk::ConnectionError
namespace MavlinkMessageGetters {
  const std::string& get_message_name(const mavsdk::Mavsdk::MavlinkMessage& s) { return s.message_name; }
  uint32_t get_system_id(const mavsdk::Mavsdk::MavlinkMessage& s) { return s.system_id; }
  uint32_t get_component_id(const mavsdk::Mavsdk::MavlinkMessage& s) { return s.component_id; }
  uint32_t get_target_system_id(const mavsdk::Mavsdk::MavlinkMessage& s) { return s.target_system_id; }
  uint32_t get_target_component_id(const mavsdk::Mavsdk::MavlinkMessage& s) { return s.target_component_id; }
  const std::string& get_fields_json(const mavsdk::Mavsdk::MavlinkMessage& s) { return s.fields_json; }
} // namespace mavsdk::Mavsdk::MavlinkMessage
