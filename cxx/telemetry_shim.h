#pragma once

#include "telemetry.h"
#include "shim_macros.h"

namespace subscriptions {
    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_position, mavsdk::Telemetry::PositionCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_home, mavsdk::Telemetry::HomeCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_in_air, mavsdk::Telemetry::InAirCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_landed_state, mavsdk::Telemetry::LandedStateCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_armed, mavsdk::Telemetry::ArmedCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_vtol_state, mavsdk::Telemetry::VtolStateCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_attitude_quaternion, mavsdk::Telemetry::AttitudeQuaternionCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_attitude_euler, mavsdk::Telemetry::AttitudeEulerCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_attitude_angular_velocity_body, mavsdk::Telemetry::AttitudeAngularVelocityBodyCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_velocity_ned, mavsdk::Telemetry::VelocityNedCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_gps_info, mavsdk::Telemetry::GpsInfoCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_raw_gps, mavsdk::Telemetry::RawGpsCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_battery, mavsdk::Telemetry::BatteryCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_flight_mode, mavsdk::Telemetry::FlightModeCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_health, mavsdk::Telemetry::HealthCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_rc_status, mavsdk::Telemetry::RcStatusCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_status_text, mavsdk::Telemetry::StatusTextCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_actuator_control_target, mavsdk::Telemetry::ActuatorControlTargetCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_actuator_output_status, mavsdk::Telemetry::ActuatorOutputStatusCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_odometry, mavsdk::Telemetry::OdometryCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_position_velocity_ned, mavsdk::Telemetry::PositionVelocityNedCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_ground_truth, mavsdk::Telemetry::GroundTruthCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_fixedwing_metrics, mavsdk::Telemetry::FixedwingMetricsCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_imu, mavsdk::Telemetry::ImuCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_scaled_imu, mavsdk::Telemetry::ScaledImuCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_raw_imu, mavsdk::Telemetry::RawImuCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_health_all_ok, mavsdk::Telemetry::HealthAllOkCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_unix_epoch_time, mavsdk::Telemetry::UnixEpochTimeCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_distance_sensor, mavsdk::Telemetry::DistanceSensorCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_scaled_pressure, mavsdk::Telemetry::ScaledPressureCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_heading, mavsdk::Telemetry::HeadingCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_altitude, mavsdk::Telemetry::AltitudeCallback)

    DECLARE_SUBSCRIBE_SHIM(mavsdk::Telemetry*, subscribe_wind, mavsdk::Telemetry::WindCallback)
}