// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct LogStreamingRawOwned {
    pub data_base64: String,
}

/*  */
pub struct LogStreamingRaw<'a> {
    inner: &'a crate::log_streaming::mavsdk::LogStreaming_LogStreamingRaw,
}

impl<'a> LogStreamingRaw<'a> {
    pub fn new(inner: &'a crate::log_streaming::mavsdk::LogStreaming_LogStreamingRaw) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> LogStreamingRawOwned {
        LogStreamingRawOwned {
            data_base64: self.data_base64(),
        }
    }
    ///  Ulog file stream data encoded as base64

    pub fn data_base64(&self) -> String {
        self.inner
            .log_streaming_get_data_base64()
            .to_string_lossy()
            .into_owned()
    }
}

struct LogStreamingInner {
    plugin: cxx::UniquePtr<crate::log_streaming::mavsdk::LogStreaming>,

    /// LogStreamingRaw State
    log_streaming_raw_handle: Option<usize>,
    log_streaming_raw_user_data: *mut c_void,
}

pub struct LogStreamingClient {
    inner: Mutex<LogStreamingInner>,

    // Producer for the latest LogStreamingRaw state; broadcast to all active subscribers.
    log_streaming_raw_tx: watch::Sender<std::option::Option<LogStreamingRawOwned>>,
}

impl LogStreamingClient {
    pub fn new(plugin: cxx::UniquePtr<crate::log_streaming::mavsdk::LogStreaming>) -> Self {
        let (log_streaming_raw_tx, _) = watch::channel(None);

        Self {
            inner: Mutex::new(LogStreamingInner {
                plugin,

                log_streaming_raw_handle: None,
                log_streaming_raw_user_data: std::ptr::null_mut(),
            }),

            log_streaming_raw_tx,
        }
    }

    pub fn subscribe_log_streaming_raw(
        &self,
    ) -> watch::Receiver<std::option::Option<LogStreamingRawOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.log_streaming_raw_handle.is_none() {
            // Lazy initialization of C++ 'log_streaming_raw' log_streaming stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.log_streaming_raw_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_log_streaming_raw(
                log_streaming_log_streaming_raw_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.log_streaming_raw_handle = Some(handle);
            inner.log_streaming_raw_user_data = user_data_ptr;
        }

        self.log_streaming_raw_tx.subscribe()
    }
}

impl Drop for LogStreamingClient {
    fn drop(&mut self) {
        if let Ok(mut inner) = self.inner.lock() {
            if let Some(handle) = inner.log_streaming_raw_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_log_streaming_raw(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.log_streaming_raw_user_data
                            as *mut watch::Sender<std::option::Option<LogStreamingRawOwned>>,
                    );
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn log_streaming_log_streaming_raw_callback_ffi(
    user_data: *mut c_void,
    log_streaming_raw: &crate::log_streaming::mavsdk::LogStreaming_LogStreamingRaw,
) {
    if user_data.is_null() {
        return;
    }
    let send_log_streaming_raw = LogStreamingRaw::new(log_streaming_raw).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<LogStreamingRawOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_log_streaming_raw));
}
