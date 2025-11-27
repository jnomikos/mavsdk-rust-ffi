// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace ListDirectoryData {
  const std::vector<std::string>& get_dirs(const mavsdk::Ftp::ListDirectoryData& s) { return s.dirs; }
  const std::vector<std::string>& get_files(const mavsdk::Ftp::ListDirectoryData& s) { return s.files; }
} // namespace mavsdk::Ftp::ListDirectoryData
namespace ProgressData {
  uint32_t get_bytes_transferred(const mavsdk::Ftp::ProgressData& s) { return s.bytes_transferred; }
  uint32_t get_total_bytes(const mavsdk::Ftp::ProgressData& s) { return s.total_bytes; }
} // namespace mavsdk::Ftp::ProgressData
