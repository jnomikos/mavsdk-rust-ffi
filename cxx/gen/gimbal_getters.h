// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace GimbalItemGetters {
  int32_t get_gimbal_id(const mavsdk::Gimbal::GimbalItem& s) { return s.gimbal_id; }
  const std::string& get_vendor_name(const mavsdk::Gimbal::GimbalItem& s) { return s.vendor_name; }
  const std::string& get_model_name(const mavsdk::Gimbal::GimbalItem& s) { return s.model_name; }
  const std::string& get_custom_name(const mavsdk::Gimbal::GimbalItem& s) { return s.custom_name; }
  int32_t get_gimbal_manager_component_id(const mavsdk::Gimbal::GimbalItem& s) { return s.gimbal_manager_component_id; }
  int32_t get_gimbal_device_id(const mavsdk::Gimbal::GimbalItem& s) { return s.gimbal_device_id; }
} // namespace mavsdk::Gimbal::GimbalItem
namespace GimbalListGetters {
  const std::vector<mavsdk::Gimbal::GimbalItem>& get_gimbals(const mavsdk::Gimbal::GimbalList& s) { return s.gimbals; }
} // namespace mavsdk::Gimbal::GimbalList
