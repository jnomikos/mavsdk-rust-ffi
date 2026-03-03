// Auto-generated wrapper. Do not edit.
use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct RtcmDataOwned {
    pub data_base64: String,
}

/*  */
pub struct RtcmData<'a> {
    inner: &'a crate::rtk::mavsdk::Rtk_RtcmData,
}

impl<'a> RtcmData<'a> {
    pub fn new(inner: &'a crate::rtk::mavsdk::Rtk_RtcmData) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> RtcmDataOwned {
        RtcmDataOwned {
            data_base64: self.data_base64(),
        }
    }
    ///  The data encoded as a base64 string

    pub fn data_base64(&self) -> String {
        self.inner
            .rtk_get_data_base64()
            .to_string_lossy()
            .into_owned()
    }
}

struct RtkInner {
    plugin: cxx::UniquePtr<crate::rtk::mavsdk::Rtk>,
}

pub struct RtkClient {
    inner: Mutex<RtkInner>,
}

impl RtkClient {
    pub fn new(plugin: cxx::UniquePtr<crate::rtk::mavsdk::Rtk>) -> Self {
        Self {
            inner: Mutex::new(RtkInner { plugin }),
        }
    }
}
