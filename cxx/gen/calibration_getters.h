// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace ProgressDataGetters {
  bool get_has_progress(const mavsdk::Calibration::ProgressData& s) { return s.has_progress; }
  float get_progress(const mavsdk::Calibration::ProgressData& s) { return s.progress; }
  bool get_has_status_text(const mavsdk::Calibration::ProgressData& s) { return s.has_status_text; }
  const std::string& get_status_text(const mavsdk::Calibration::ProgressData& s) { return s.status_text; }
} // namespace mavsdk::Calibration::ProgressData
