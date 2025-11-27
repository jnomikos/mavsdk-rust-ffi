// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace FlightInfo {
  uint32_t get_time_boot_ms(const mavsdk::Info::FlightInfo& s) { return s.time_boot_ms; }
  uint64_t get_flight_uid(const mavsdk::Info::FlightInfo& s) { return s.flight_uid; }
  uint32_t get_duration_since_arming_ms(const mavsdk::Info::FlightInfo& s) { return s.duration_since_arming_ms; }
  uint32_t get_duration_since_takeoff_ms(const mavsdk::Info::FlightInfo& s) { return s.duration_since_takeoff_ms; }
} // namespace mavsdk::Info::FlightInfo
namespace Identification {
  const std::string& get_hardware_uid(const mavsdk::Info::Identification& s) { return s.hardware_uid; }
  uint64_t get_legacy_uid(const mavsdk::Info::Identification& s) { return s.legacy_uid; }
} // namespace mavsdk::Info::Identification
namespace Product {
  int32_t get_vendor_id(const mavsdk::Info::Product& s) { return s.vendor_id; }
  const std::string& get_vendor_name(const mavsdk::Info::Product& s) { return s.vendor_name; }
  int32_t get_product_id(const mavsdk::Info::Product& s) { return s.product_id; }
  const std::string& get_product_name(const mavsdk::Info::Product& s) { return s.product_name; }
} // namespace mavsdk::Info::Product
namespace Version {
  int32_t get_flight_sw_major(const mavsdk::Info::Version& s) { return s.flight_sw_major; }
  int32_t get_flight_sw_minor(const mavsdk::Info::Version& s) { return s.flight_sw_minor; }
  int32_t get_flight_sw_patch(const mavsdk::Info::Version& s) { return s.flight_sw_patch; }
  int32_t get_flight_sw_vendor_major(const mavsdk::Info::Version& s) { return s.flight_sw_vendor_major; }
  int32_t get_flight_sw_vendor_minor(const mavsdk::Info::Version& s) { return s.flight_sw_vendor_minor; }
  int32_t get_flight_sw_vendor_patch(const mavsdk::Info::Version& s) { return s.flight_sw_vendor_patch; }
  int32_t get_os_sw_major(const mavsdk::Info::Version& s) { return s.os_sw_major; }
  int32_t get_os_sw_minor(const mavsdk::Info::Version& s) { return s.os_sw_minor; }
  int32_t get_os_sw_patch(const mavsdk::Info::Version& s) { return s.os_sw_patch; }
  const std::string& get_flight_sw_git_hash(const mavsdk::Info::Version& s) { return s.flight_sw_git_hash; }
  const std::string& get_os_sw_git_hash(const mavsdk::Info::Version& s) { return s.os_sw_git_hash; }
  mavsdk::Info::Version::FlightSoftwareVersionType get_flight_sw_version_type(const mavsdk::Info::Version& s) { return s.flight_sw_version_type; }
} // namespace mavsdk::Info::Version
