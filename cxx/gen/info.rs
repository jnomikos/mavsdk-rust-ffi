// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use tokio::sync::watch;
use std::sync::Mutex;






pub struct FlightInfoOwned {
        pub time_boot_ms: u32,
        pub flight_uid: u64,
        pub duration_since_arming_ms: u32,
        pub duration_since_takeoff_ms: u32,
}

/*  */
pub struct FlightInfo<'a> {
    inner: &'a crate::info::mavsdk::Info_FlightInfo,
}

impl<'a> FlightInfo<'a> {
    pub fn new(inner: &'a crate::info::mavsdk::Info_FlightInfo) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> FlightInfoOwned {
        FlightInfoOwned {
            
            time_boot_ms: self.time_boot_ms(),
            flight_uid: self.flight_uid(),
            duration_since_arming_ms: self.duration_since_arming_ms(),
            duration_since_takeoff_ms: self.duration_since_takeoff_ms(),
        }
    }
        ///  Time since system boot

    pub fn time_boot_ms(&self) -> u32 {
        self.inner.info_get_time_boot_ms()
    }
        ///  Flight counter. Starts from zero, is incremented at every disarm and is never reset (even after reboot)

    pub fn flight_uid(&self) -> u64 {
        self.inner.info_get_flight_uid()
    }
        ///  Duration since arming in milliseconds

    pub fn duration_since_arming_ms(&self) -> u32 {
        self.inner.info_get_duration_since_arming_ms()
    }
        ///  Duration since takeoff in milliseconds

    pub fn duration_since_takeoff_ms(&self) -> u32 {
        self.inner.info_get_duration_since_takeoff_ms()
    }
}


pub struct IdentificationOwned {
        pub hardware_uid: String,
        pub legacy_uid: u64,
}

/*  */
pub struct Identification<'a> {
    inner: &'a crate::info::mavsdk::Info_Identification,
}

impl<'a> Identification<'a> {
    pub fn new(inner: &'a crate::info::mavsdk::Info_Identification) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> IdentificationOwned {
        IdentificationOwned {
            
            hardware_uid: self.hardware_uid(),
            legacy_uid: self.legacy_uid(),
        }
    }
        ///  UID of the hardware. This refers to uid2 of MAVLink. If the system does not support uid2 yet, this is all zeros.

    pub fn hardware_uid(&self) -> String {
        self.inner.info_get_hardware_uid().to_string_lossy().into_owned()
    }
        ///  Legacy UID of the hardware, referred to as uid in MAVLink (formerly exposed during system discovery as UUID).

    pub fn legacy_uid(&self) -> u64 {
        self.inner.info_get_legacy_uid()
    }
}


pub struct ProductOwned {
        pub vendor_id: i32,
        pub vendor_name: String,
        pub product_id: i32,
        pub product_name: String,
}

/*  */
pub struct Product<'a> {
    inner: &'a crate::info::mavsdk::Info_Product,
}

impl<'a> Product<'a> {
    pub fn new(inner: &'a crate::info::mavsdk::Info_Product) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> ProductOwned {
        ProductOwned {
            
            vendor_id: self.vendor_id(),
            vendor_name: self.vendor_name(),
            product_id: self.product_id(),
            product_name: self.product_name(),
        }
    }
        ///  ID of the board vendor

    pub fn vendor_id(&self) -> i32 {
        self.inner.info_get_vendor_id()
    }
        ///  Name of the vendor

    pub fn vendor_name(&self) -> String {
        self.inner.info_get_vendor_name().to_string_lossy().into_owned()
    }
        ///  ID of the product

    pub fn product_id(&self) -> i32 {
        self.inner.info_get_product_id()
    }
        ///  Name of the product

    pub fn product_name(&self) -> String {
        self.inner.info_get_product_name().to_string_lossy().into_owned()
    }
}


pub struct VersionOwned {
        pub flight_sw_major: i32,
        pub flight_sw_minor: i32,
        pub flight_sw_patch: i32,
        pub flight_sw_vendor_major: i32,
        pub flight_sw_vendor_minor: i32,
        pub flight_sw_vendor_patch: i32,
        pub os_sw_major: i32,
        pub os_sw_minor: i32,
        pub os_sw_patch: i32,
        pub flight_sw_git_hash: String,
        pub os_sw_git_hash: String,
        pub flight_sw_version_type: crate::info::mavsdk::Info_Version_FlightSoftwareVersionType,
}

/*  */
pub struct Version<'a> {
    inner: &'a crate::info::mavsdk::Info_Version,
}

impl<'a> Version<'a> {
    pub fn new(inner: &'a crate::info::mavsdk::Info_Version) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> VersionOwned {
        VersionOwned {
            
            flight_sw_major: self.flight_sw_major(),
            flight_sw_minor: self.flight_sw_minor(),
            flight_sw_patch: self.flight_sw_patch(),
            flight_sw_vendor_major: self.flight_sw_vendor_major(),
            flight_sw_vendor_minor: self.flight_sw_vendor_minor(),
            flight_sw_vendor_patch: self.flight_sw_vendor_patch(),
            os_sw_major: self.os_sw_major(),
            os_sw_minor: self.os_sw_minor(),
            os_sw_patch: self.os_sw_patch(),
            flight_sw_git_hash: self.flight_sw_git_hash(),
            os_sw_git_hash: self.os_sw_git_hash(),
            flight_sw_version_type: self.flight_sw_version_type(),
        }
    }
        ///  Flight software major version

    pub fn flight_sw_major(&self) -> i32 {
        self.inner.info_get_flight_sw_major()
    }
        ///  Flight software minor version

    pub fn flight_sw_minor(&self) -> i32 {
        self.inner.info_get_flight_sw_minor()
    }
        ///  Flight software patch version

    pub fn flight_sw_patch(&self) -> i32 {
        self.inner.info_get_flight_sw_patch()
    }
        ///  Flight software vendor major version

    pub fn flight_sw_vendor_major(&self) -> i32 {
        self.inner.info_get_flight_sw_vendor_major()
    }
        ///  Flight software vendor minor version

    pub fn flight_sw_vendor_minor(&self) -> i32 {
        self.inner.info_get_flight_sw_vendor_minor()
    }
        ///  Flight software vendor patch version

    pub fn flight_sw_vendor_patch(&self) -> i32 {
        self.inner.info_get_flight_sw_vendor_patch()
    }
        ///  Operating system software major version

    pub fn os_sw_major(&self) -> i32 {
        self.inner.info_get_os_sw_major()
    }
        ///  Operating system software minor version

    pub fn os_sw_minor(&self) -> i32 {
        self.inner.info_get_os_sw_minor()
    }
        ///  Operating system software patch version

    pub fn os_sw_patch(&self) -> i32 {
        self.inner.info_get_os_sw_patch()
    }
        ///  Flight software git hash

    pub fn flight_sw_git_hash(&self) -> String {
        self.inner.info_get_flight_sw_git_hash().to_string_lossy().into_owned()
    }
        ///  Operating system software git hash

    pub fn os_sw_git_hash(&self) -> String {
        self.inner.info_get_os_sw_git_hash().to_string_lossy().into_owned()
    }
        ///  Flight software version type

    pub fn flight_sw_version_type(&self) -> crate::info::mavsdk::Info_Version_FlightSoftwareVersionType {
        self.inner.info_get_flight_sw_version_type().clone()
    }
}

    struct InfoInner {
        plugin: cxx::UniquePtr<crate::info::mavsdk::Info>,
        
                /// FlightInformation State
                flight_information_handle: Option<usize>,
                flight_information_user_data: *mut c_void,
    }

    pub struct InfoClient {
        inner: Mutex<InfoInner>,
        
                // Producer for the latest FlightInformation state; broadcast to all active subscribers.
                flight_information_tx: watch::Sender<std::option::Option<
        FlightInfoOwned>>,
    }

    impl InfoClient {
        pub fn new(plugin: cxx::UniquePtr<crate::info::mavsdk::Info>) -> Self {
            
                    let (flight_information_tx, _) = watch::channel(None);

            Self {
                inner: Mutex::new(InfoInner {
                    plugin,
                    
                            flight_information_handle: None,
                            flight_information_user_data: std::ptr::null_mut(),
                }),
                
                        flight_information_tx,
            }
        }
        
        pub fn subscribe_flight_information(&self) -> watch::Receiver<std::option::Option<
        FlightInfoOwned>> {
                    let mut inner = self.inner.lock().unwrap();

                    if inner.flight_information_handle.is_none() {
                        // Lazy initialization of C++ 'flight_information' info stream.
                        let user_data_ptr = Box::into_raw(Box::new(self.flight_information_tx.clone())) as *mut c_void;

                        let handle = inner.plugin.pin_mut().subscribe_flight_information(
                            info_flight_information_callback_ffi as usize as libc::uintptr_t,
                            user_data_ptr as libc::uintptr_t
                        ) as usize;

                        inner.flight_information_handle = Some(handle);
                        inner.flight_information_user_data = user_data_ptr;
                    }
                    
                    self.flight_information_tx.subscribe()
                }
    }


        #[unsafe(no_mangle)]
        pub extern "C" fn info_flight_information_callback_ffi(
            user_data: *mut c_void,
            flight_information: &crate::info::mavsdk::Info_FlightInfo,
        ) {
            if user_data.is_null() { return; }
                let send_flight_information = FlightInfo::new(flight_information).into_owned();

            // Cast the pointer to watch::Sender
            let sender = unsafe {
                &*(user_data as *const watch::Sender<std::option::Option<
        FlightInfoOwned>>)
            };

            // Overwrite the current value in the channel with the new update
            let _ = sender.send(Some(send_flight_information));
        }