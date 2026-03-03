// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use tokio::sync::watch;
use std::sync::Mutex;






pub struct AdsbVehicleOwned {
        pub icao_address: u32,
        pub latitude_deg: f64,
        pub longitude_deg: f64,
        pub altitude_type: crate::transponder::mavsdk::Transponder_AdsbAltitudeType,
        pub absolute_altitude_m: f32,
        pub heading_deg: f32,
        pub horizontal_velocity_m_s: f32,
        pub vertical_velocity_m_s: f32,
        pub callsign: String,
        pub emitter_type: crate::transponder::mavsdk::Transponder_AdsbEmitterType,
        pub squawk: u32,
        pub tslc_s: u32,
}

/*  */
pub struct AdsbVehicle<'a> {
    inner: &'a crate::transponder::mavsdk::Transponder_AdsbVehicle,
}

impl<'a> AdsbVehicle<'a> {
    pub fn new(inner: &'a crate::transponder::mavsdk::Transponder_AdsbVehicle) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> AdsbVehicleOwned {
        AdsbVehicleOwned {
            
            icao_address: self.icao_address(),
            latitude_deg: self.latitude_deg(),
            longitude_deg: self.longitude_deg(),
            altitude_type: self.altitude_type(),
            absolute_altitude_m: self.absolute_altitude_m(),
            heading_deg: self.heading_deg(),
            horizontal_velocity_m_s: self.horizontal_velocity_m_s(),
            vertical_velocity_m_s: self.vertical_velocity_m_s(),
            callsign: self.callsign(),
            emitter_type: self.emitter_type(),
            squawk: self.squawk(),
            tslc_s: self.tslc_s(),
        }
    }
        ///  ICAO (International Civil Aviation Organization) unique worldwide identifier

    pub fn icao_address(&self) -> u32 {
        self.inner.transponder_get_icao_address()
    }
        ///  Latitude in degrees (range: -90 to +90)

    pub fn latitude_deg(&self) -> f64 {
        self.inner.transponder_get_latitude_deg()
    }
        ///  Longitude in degrees (range: -180 to +180).

    pub fn longitude_deg(&self) -> f64 {
        self.inner.transponder_get_longitude_deg()
    }
        ///  ADSB altitude type.

    pub fn altitude_type(&self) -> crate::transponder::mavsdk::Transponder_AdsbAltitudeType {self.inner.transponder_get_altitude_type().clone()
    }
        ///  Altitude in metres according to altitude_type 

    pub fn absolute_altitude_m(&self) -> f32 {
        self.inner.transponder_get_absolute_altitude_m()
    }
        ///  Course over ground, in degrees

    pub fn heading_deg(&self) -> f32 {
        self.inner.transponder_get_heading_deg()
    }
        ///  The horizontal velocity in metres/second

    pub fn horizontal_velocity_m_s(&self) -> f32 {
        self.inner.transponder_get_horizontal_velocity_m_s()
    }
        ///  The vertical velocity in metres/second. Positive is up.

    pub fn vertical_velocity_m_s(&self) -> f32 {
        self.inner.transponder_get_vertical_velocity_m_s()
    }
        ///  The callsign

    pub fn callsign(&self) -> String {
        self.inner.transponder_get_callsign().to_string_lossy().into_owned()
    }
        ///  ADSB emitter type.

    pub fn emitter_type(&self) -> crate::transponder::mavsdk::Transponder_AdsbEmitterType {self.inner.transponder_get_emitter_type().clone()
    }
        ///  Squawk code.

    pub fn squawk(&self) -> u32 {
        self.inner.transponder_get_squawk()
    }
        ///  Time Since Last Communication in seconds.

    pub fn tslc_s(&self) -> u32 {
        self.inner.transponder_get_tslc_s()
    }
}

    struct TransponderInner {
        plugin: cxx::UniquePtr<crate::transponder::mavsdk::Transponder>,
        
                /// Transponder State
                transponder_handle: Option<usize>,
                transponder_user_data: *mut c_void,
    }

    pub struct TransponderClient {
        inner: Mutex<TransponderInner>,
        
                // Producer for the latest Transponder state; broadcast to all active subscribers.
                transponder_tx: watch::Sender<std::option::Option<
        AdsbVehicleOwned>>,
    }

    impl TransponderClient {
        pub fn new(plugin: cxx::UniquePtr<crate::transponder::mavsdk::Transponder>) -> Self {
            
                    let (transponder_tx, _) = watch::channel(None);

            Self {
                inner: Mutex::new(TransponderInner {
                    plugin,
                    
                            transponder_handle: None,
                            transponder_user_data: std::ptr::null_mut(),
                }),
                
                        transponder_tx,
            }
        }
        
        pub fn subscribe_transponder(&self) -> watch::Receiver<std::option::Option<
        AdsbVehicleOwned>> {
                    let mut inner = self.inner.lock().unwrap();

                    if inner.transponder_handle.is_none() {
                        // Lazy initialization of C++ 'transponder' transponder stream.
                        let user_data_ptr = Box::into_raw(Box::new(self.transponder_tx.clone())) as *mut c_void;

                        let handle = inner.plugin.pin_mut().subscribe_transponder(
                            transponder_transponder_callback_ffi as usize as libc::uintptr_t,
                            user_data_ptr as libc::uintptr_t
                        ) as usize;

                        inner.transponder_handle = Some(handle);
                        inner.transponder_user_data = user_data_ptr;
                    }
                    
                    self.transponder_tx.subscribe()
                }
    }


        #[unsafe(no_mangle)]
        pub extern "C" fn transponder_transponder_callback_ffi(
            user_data: *mut c_void,
            transponder: &crate::transponder::mavsdk::Transponder_AdsbVehicle,
        ) {
            if user_data.is_null() { return; }
                let send_transponder = AdsbVehicle::new(transponder).into_owned();

            // Cast the pointer to watch::Sender
            let sender = unsafe {
                &*(user_data as *const watch::Sender<std::option::Option<
        AdsbVehicleOwned>>)
            };

            // Overwrite the current value in the channel with the new update
            let _ = sender.send(Some(send_transponder));
        }