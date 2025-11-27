// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace MavlinkMessage {
  const std::string& get_message_name(const mavsdk::MavlinkDirect::MavlinkMessage& s) { return s.message_name; }
  uint32_t get_system_id(const mavsdk::MavlinkDirect::MavlinkMessage& s) { return s.system_id; }
  uint32_t get_component_id(const mavsdk::MavlinkDirect::MavlinkMessage& s) { return s.component_id; }
  uint32_t get_target_system_id(const mavsdk::MavlinkDirect::MavlinkMessage& s) { return s.target_system_id; }
  uint32_t get_target_component_id(const mavsdk::MavlinkDirect::MavlinkMessage& s) { return s.target_component_id; }
  const std::string& get_fields_json(const mavsdk::MavlinkDirect::MavlinkMessage& s) { return s.fields_json; }
} // namespace mavsdk::MavlinkDirect::MavlinkMessage
