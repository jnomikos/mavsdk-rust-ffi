// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use tokio::sync::watch;
use std::sync::Mutex;





    struct ArmAuthorizerServerInner {
        plugin: cxx::UniquePtr<crate::arm_authorizer_server::mavsdk::ArmAuthorizerServer>,
        
                /// ArmAuthorization State
                arm_authorization_handle: Option<usize>,
                arm_authorization_user_data: *mut c_void,
    }

    pub struct ArmAuthorizerServerClient {
        inner: Mutex<ArmAuthorizerServerInner>,
        
                // Producer for the latest ArmAuthorization state; broadcast to all active subscribers.
                arm_authorization_tx: watch::Sender<std::option::Option<
        u32>>,
    }

    impl ArmAuthorizerServerClient {
        pub fn new(plugin: cxx::UniquePtr<crate::arm_authorizer_server::mavsdk::ArmAuthorizerServer>) -> Self {
            
                    let (arm_authorization_tx, _) = watch::channel(None);

            Self {
                inner: Mutex::new(ArmAuthorizerServerInner {
                    plugin,
                    
                            arm_authorization_handle: None,
                            arm_authorization_user_data: std::ptr::null_mut(),
                }),
                
                        arm_authorization_tx,
            }
        }
        
        pub fn subscribe_arm_authorization(&self) -> watch::Receiver<std::option::Option<
        u32>> {
                    let mut inner = self.inner.lock().unwrap();

                    if inner.arm_authorization_handle.is_none() {
                        // Lazy initialization of C++ 'arm_authorization' arm_authorizer_server stream.
                        let user_data_ptr = Box::into_raw(Box::new(self.arm_authorization_tx.clone())) as *mut c_void;

                        let handle = inner.plugin.pin_mut().subscribe_arm_authorization(
                            arm_authorizer_server_arm_authorization_callback_ffi as usize as libc::uintptr_t,
                            user_data_ptr as libc::uintptr_t
                        ) as usize;

                        inner.arm_authorization_handle = Some(handle);
                        inner.arm_authorization_user_data = user_data_ptr;
                    }
                    
                    self.arm_authorization_tx.subscribe()
                }
    }


        #[unsafe(no_mangle)]
        pub extern "C" fn arm_authorizer_server_arm_authorization_callback_ffi(
            user_data: *mut c_void,
            arm_authorization: u32,
        ) {
            if user_data.is_null() { return; }
                let send_arm_authorization = 
        arm_authorization.clone();

            // Cast the pointer to watch::Sender
            let sender = unsafe {
                &*(user_data as *const watch::Sender<std::option::Option<
        u32>>)
            };

            // Overwrite the current value in the channel with the new update
            let _ = sender.send(Some(send_arm_authorization));
        }