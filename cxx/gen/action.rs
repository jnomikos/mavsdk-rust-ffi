// Auto-generated wrapper. Do not edit.
use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

struct ActionInner {
    plugin: cxx::UniquePtr<crate::action::mavsdk::Action>,
}

pub struct ActionClient {
    inner: Mutex<ActionInner>,
}

impl ActionClient {
    pub fn new(plugin: cxx::UniquePtr<crate::action::mavsdk::Action>) -> Self {
        Self {
            inner: Mutex::new(ActionInner { plugin }),
        }
    }
}
