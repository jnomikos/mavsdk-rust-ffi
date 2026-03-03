// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use tokio::sync::watch;
use std::sync::Mutex;





    struct ActionInner {
        plugin: cxx::UniquePtr<crate::action::mavsdk::Action>,
        
    }

    pub struct ActionClient {
        inner: Mutex<ActionInner>,
        
    }

    impl ActionClient {
        pub fn new(plugin: cxx::UniquePtr<crate::action::mavsdk::Action>) -> Self {
            

            Self {
                inner: Mutex::new(ActionInner {
                    plugin,
                    
                }),
                
            }
        }
        
        
    }

