// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace AllowableFlightModes {
  bool get_can_auto_mode(const mavsdk::ActionServer::AllowableFlightModes& s) { return s.can_auto_mode; }
  bool get_can_guided_mode(const mavsdk::ActionServer::AllowableFlightModes& s) { return s.can_guided_mode; }
  bool get_can_stabilize_mode(const mavsdk::ActionServer::AllowableFlightModes& s) { return s.can_stabilize_mode; }
} // namespace mavsdk::ActionServer::AllowableFlightModes
namespace ArmDisarm {
  bool get_arm(const mavsdk::ActionServer::ArmDisarm& s) { return s.arm; }
  bool get_force(const mavsdk::ActionServer::ArmDisarm& s) { return s.force; }
} // namespace mavsdk::ActionServer::ArmDisarm
