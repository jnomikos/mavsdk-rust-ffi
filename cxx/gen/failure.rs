// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use tokio::sync::watch;
use std::sync::Mutex;





    struct FailureInner {
        plugin: cxx::UniquePtr<crate::failure::mavsdk::Failure>,
        
    }

    pub struct FailureClient {
        inner: Mutex<FailureInner>,
        
    }

    impl FailureClient {
        pub fn new(plugin: cxx::UniquePtr<crate::failure::mavsdk::Failure>) -> Self {
            

            Self {
                inner: Mutex::new(FailureInner {
                    plugin,
                    
                }),
                
            }
        }
        
        
    }

