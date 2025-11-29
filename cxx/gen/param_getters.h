// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace IntParamGetters {
  const std::string& get_name(const mavsdk::Param::IntParam& s) { return s.name; }
  int32_t get_value(const mavsdk::Param::IntParam& s) { return s.value; }
} // namespace mavsdk::Param::IntParam
namespace FloatParamGetters {
  const std::string& get_name(const mavsdk::Param::FloatParam& s) { return s.name; }
  float get_value(const mavsdk::Param::FloatParam& s) { return s.value; }
} // namespace mavsdk::Param::FloatParam
namespace CustomParamGetters {
  const std::string& get_name(const mavsdk::Param::CustomParam& s) { return s.name; }
  const std::string& get_value(const mavsdk::Param::CustomParam& s) { return s.value; }
} // namespace mavsdk::Param::CustomParam
namespace AllParamsGetters {
  const std::vector<mavsdk::Param::IntParam>& get_int_params(const mavsdk::Param::AllParams& s) { return s.int_params; }
  const std::vector<mavsdk::Param::FloatParam>& get_float_params(const mavsdk::Param::AllParams& s) { return s.float_params; }
  const std::vector<mavsdk::Param::CustomParam>& get_custom_params(const mavsdk::Param::AllParams& s) { return s.custom_params; }
} // namespace mavsdk::Param::AllParams
