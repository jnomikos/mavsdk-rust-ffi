// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

struct GripperInner {
    plugin: cxx::UniquePtr<crate::gripper::mavsdk::Gripper>,
}

pub struct GripperClient {
    inner: Mutex<GripperInner>,
}

impl GripperClient {
    pub fn new(plugin: cxx::UniquePtr<crate::gripper::mavsdk::Gripper>) -> Self {
        Self {
            inner: Mutex::new(GripperInner { plugin }),
        }
    }
}
