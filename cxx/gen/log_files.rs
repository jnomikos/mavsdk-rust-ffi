// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct ProgressDataOwned {
    pub progress: f32,
}

/*  */
pub struct ProgressData<'a> {
    inner: &'a crate::log_files::mavsdk::LogFiles_ProgressData,
}

impl<'a> ProgressData<'a> {
    pub fn new(inner: &'a crate::log_files::mavsdk::LogFiles_ProgressData) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> ProgressDataOwned {
        ProgressDataOwned {
            progress: self.progress(),
        }
    }
    ///  Progress from 0 to 1

    pub fn progress(&self) -> f32 {
        self.inner.log_files_get_progress()
    }
}

pub struct EntryOwned {
    pub id: u32,
    pub date: String,
    pub size_bytes: u32,
}

/*  */
pub struct Entry<'a> {
    inner: &'a crate::log_files::mavsdk::LogFiles_Entry,
}

impl<'a> Entry<'a> {
    pub fn new(inner: &'a crate::log_files::mavsdk::LogFiles_Entry) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> EntryOwned {
        EntryOwned {
            id: self.id(),
            date: self.date(),
            size_bytes: self.size_bytes(),
        }
    }
    ///  ID of the log file, to specify a file to be downloaded

    pub fn id(&self) -> u32 {
        self.inner.log_files_get_id()
    }
    ///  Date of the log file in UTC in ISO 8601 format "yyyy-mm-ddThh:mm:ssZ"

    pub fn date(&self) -> String {
        self.inner
            .log_files_get_date()
            .to_string_lossy()
            .into_owned()
    }
    ///  Size of file in bytes

    pub fn size_bytes(&self) -> u32 {
        self.inner.log_files_get_size_bytes()
    }
}

struct LogFilesInner {
    plugin: cxx::UniquePtr<crate::log_files::mavsdk::LogFiles>,
}

pub struct LogFilesClient {
    inner: Mutex<LogFilesInner>,
}

impl LogFilesClient {
    pub fn new(plugin: cxx::UniquePtr<crate::log_files::mavsdk::LogFiles>) -> Self {
        Self {
            inner: Mutex::new(LogFilesInner { plugin }),
        }
    }
}
