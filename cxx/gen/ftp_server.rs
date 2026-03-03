// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use tokio::sync::watch;
use std::sync::Mutex;





    struct FtpServerInner {
        plugin: cxx::UniquePtr<crate::ftp_server::mavsdk::FtpServer>,
        
    }

    pub struct FtpServerClient {
        inner: Mutex<FtpServerInner>,
        
    }

    impl FtpServerClient {
        pub fn new(plugin: cxx::UniquePtr<crate::ftp_server::mavsdk::FtpServer>) -> Self {
            

            Self {
                inner: Mutex::new(FtpServerInner {
                    plugin,
                    
                }),
                
            }
        }
        
        
    }

