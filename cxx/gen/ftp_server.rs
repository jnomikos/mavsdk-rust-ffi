// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

struct FtpServerInner {
    plugin: cxx::UniquePtr<crate::ftp_server::mavsdk::FtpServer>,
}

pub struct FtpServerClient {
    inner: Mutex<FtpServerInner>,
}

impl FtpServerClient {
    pub fn new(plugin: cxx::UniquePtr<crate::ftp_server::mavsdk::FtpServer>) -> Self {
        Self {
            inner: Mutex::new(FtpServerInner { plugin }),
        }
    }
}
