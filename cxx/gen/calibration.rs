// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct ProgressDataOwned {
    pub has_progress: bool,
    pub progress: f32,
    pub has_status_text: bool,
    pub status_text: String,
}

/*  */
pub struct ProgressData<'a> {
    inner: &'a crate::calibration::mavsdk::Calibration_ProgressData,
}

impl<'a> ProgressData<'a> {
    pub fn new(inner: &'a crate::calibration::mavsdk::Calibration_ProgressData) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> ProgressDataOwned {
        ProgressDataOwned {
            has_progress: self.has_progress(),
            progress: self.progress(),
            has_status_text: self.has_status_text(),
            status_text: self.status_text(),
        }
    }
    ///  Whether this ProgressData contains a 'progress' status or not

    pub fn has_progress(&self) -> bool {
        self.inner.calibration_get_has_progress()
    }
    ///  Progress (percentage)

    pub fn progress(&self) -> f32 {
        self.inner.calibration_get_progress()
    }
    ///  Whether this ProgressData contains a 'status_text' or not

    pub fn has_status_text(&self) -> bool {
        self.inner.calibration_get_has_status_text()
    }
    ///  Instruction text

    pub fn status_text(&self) -> String {
        self.inner
            .calibration_get_status_text()
            .to_string_lossy()
            .into_owned()
    }
}

struct CalibrationInner {
    plugin: cxx::UniquePtr<crate::calibration::mavsdk::Calibration>,
}

pub struct CalibrationClient {
    inner: Mutex<CalibrationInner>,
}

impl CalibrationClient {
    pub fn new(plugin: cxx::UniquePtr<crate::calibration::mavsdk::Calibration>) -> Self {
        Self {
            inner: Mutex::new(CalibrationInner { plugin }),
        }
    }
}
