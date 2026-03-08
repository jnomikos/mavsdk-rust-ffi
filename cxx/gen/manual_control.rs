// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

struct ManualControlInner {
    plugin: cxx::UniquePtr<crate::manual_control::mavsdk::ManualControl>,
}

pub struct ManualControlClient {
    inner: Mutex<ManualControlInner>,
}

impl ManualControlClient {
    pub fn new(plugin: cxx::UniquePtr<crate::manual_control::mavsdk::ManualControl>) -> Self {
        Self {
            inner: Mutex::new(ManualControlInner { plugin }),
        }
    }
}
