// THIS FILE IS AUTO-GENERATED. DO NOT EDIT MANUALLY!
#pragma once
namespace AdsbVehicle {
  uint32_t get_icao_address(const mavsdk::Transponder::AdsbVehicle& s) { return s.icao_address; }
  double get_latitude_deg(const mavsdk::Transponder::AdsbVehicle& s) { return s.latitude_deg; }
  double get_longitude_deg(const mavsdk::Transponder::AdsbVehicle& s) { return s.longitude_deg; }
  mavsdk::Transponder::AdsbAltitudeType get_altitude_type(const mavsdk::Transponder::AdsbVehicle& s) { return s.altitude_type; }
  float get_absolute_altitude_m(const mavsdk::Transponder::AdsbVehicle& s) { return s.absolute_altitude_m; }
  float get_heading_deg(const mavsdk::Transponder::AdsbVehicle& s) { return s.heading_deg; }
  float get_horizontal_velocity_m_s(const mavsdk::Transponder::AdsbVehicle& s) { return s.horizontal_velocity_m_s; }
  float get_vertical_velocity_m_s(const mavsdk::Transponder::AdsbVehicle& s) { return s.vertical_velocity_m_s; }
  const std::string& get_callsign(const mavsdk::Transponder::AdsbVehicle& s) { return s.callsign; }
  mavsdk::Transponder::AdsbEmitterType get_emitter_type(const mavsdk::Transponder::AdsbVehicle& s) { return s.emitter_type; }
  uint32_t get_squawk(const mavsdk::Transponder::AdsbVehicle& s) { return s.squawk; }
  uint32_t get_tslc_s(const mavsdk::Transponder::AdsbVehicle& s) { return s.tslc_s; }
} // namespace mavsdk::Transponder::AdsbVehicle
