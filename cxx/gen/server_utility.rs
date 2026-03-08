// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

struct ServerUtilityInner {
    plugin: cxx::UniquePtr<crate::server_utility::mavsdk::ServerUtility>,
}

pub struct ServerUtilityClient {
    inner: Mutex<ServerUtilityInner>,
}

impl ServerUtilityClient {
    pub fn new(plugin: cxx::UniquePtr<crate::server_utility::mavsdk::ServerUtility>) -> Self {
        Self {
            inner: Mutex::new(ServerUtilityInner { plugin }),
        }
    }
}
