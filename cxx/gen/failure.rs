// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

struct FailureInner {
    plugin: cxx::UniquePtr<crate::failure::mavsdk::Failure>,
}

pub struct FailureClient {
    inner: Mutex<FailureInner>,
}

impl FailureClient {
    pub fn new(plugin: cxx::UniquePtr<crate::failure::mavsdk::Failure>) -> Self {
        Self {
            inner: Mutex::new(FailureInner { plugin }),
        }
    }
}
