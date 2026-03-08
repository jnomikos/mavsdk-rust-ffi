// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

struct ShellInner {
    plugin: cxx::UniquePtr<crate::shell::mavsdk::Shell>,

    /// Receive State
    receive_handle: Option<usize>,
    receive_user_data: *mut c_void,
}

pub struct ShellClient {
    inner: Mutex<ShellInner>,

    // Producer for the latest Receive state; broadcast to all active subscribers.
    receive_tx: watch::Sender<std::option::Option<String>>,
}

impl ShellClient {
    pub fn new(plugin: cxx::UniquePtr<crate::shell::mavsdk::Shell>) -> Self {
        let (receive_tx, _) = watch::channel(None);

        Self {
            inner: Mutex::new(ShellInner {
                plugin,

                receive_handle: None,
                receive_user_data: std::ptr::null_mut(),
            }),

            receive_tx,
        }
    }

    pub fn subscribe_receive(&self) -> watch::Receiver<std::option::Option<String>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.receive_handle.is_none() {
            // Lazy initialization of C++ 'receive' shell stream.
            let user_data_ptr = Box::into_raw(Box::new(self.receive_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_receive(
                shell_receive_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.receive_handle = Some(handle);
            inner.receive_user_data = user_data_ptr;
        }

        self.receive_tx.subscribe()
    }
}

impl Drop for ShellClient {
    fn drop(&mut self) {
        if let Ok(mut inner) = self.inner.lock() {
            if let Some(handle) = inner.receive_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_receive(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.receive_user_data as *mut watch::Sender<std::option::Option<String>>,
                    );
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn shell_receive_callback_ffi(user_data: *mut c_void, receive: cxx::CxxString) {
    if user_data.is_null() {
        return;
    }
    let send_receive = receive.to_string_lossy().into_owned();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<String>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_receive));
}
