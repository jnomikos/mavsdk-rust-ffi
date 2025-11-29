// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace IntParamGetters {
  const std::string& get_name(const mavsdk::ParamServer::IntParam& s) { return s.name; }
  int32_t get_value(const mavsdk::ParamServer::IntParam& s) { return s.value; }
} // namespace mavsdk::ParamServer::IntParam
namespace FloatParamGetters {
  const std::string& get_name(const mavsdk::ParamServer::FloatParam& s) { return s.name; }
  float get_value(const mavsdk::ParamServer::FloatParam& s) { return s.value; }
} // namespace mavsdk::ParamServer::FloatParam
namespace CustomParamGetters {
  const std::string& get_name(const mavsdk::ParamServer::CustomParam& s) { return s.name; }
  const std::string& get_value(const mavsdk::ParamServer::CustomParam& s) { return s.value; }
} // namespace mavsdk::ParamServer::CustomParam
namespace AllParamsGetters {
  const std::vector<mavsdk::ParamServer::IntParam>& get_int_params(const mavsdk::ParamServer::AllParams& s) { return s.int_params; }
  const std::vector<mavsdk::ParamServer::FloatParam>& get_float_params(const mavsdk::ParamServer::AllParams& s) { return s.float_params; }
  const std::vector<mavsdk::ParamServer::CustomParam>& get_custom_params(const mavsdk::ParamServer::AllParams& s) { return s.custom_params; }
} // namespace mavsdk::ParamServer::AllParams
