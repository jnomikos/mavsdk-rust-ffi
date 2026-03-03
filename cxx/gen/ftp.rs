// Auto-generated wrapper. Do not edit.
use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct ListDirectoryDataOwned {
    pub dirs: std::vec::Vec<String>,
    pub files: std::vec::Vec<String>,
}

/*  */
pub struct ListDirectoryData<'a> {
    inner: &'a crate::ftp::mavsdk::Ftp_ListDirectoryData,
}

impl<'a> ListDirectoryData<'a> {
    pub fn new(inner: &'a crate::ftp::mavsdk::Ftp_ListDirectoryData) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> ListDirectoryDataOwned {
        ListDirectoryDataOwned {
            dirs: self.dirs(),
            files: self.files(),
        }
    }
    ///  The found directories.

    pub fn dirs(&self) -> std::vec::Vec<String> {
        // CxxString convert to Rust String in vector
        self.inner
            .ftp_get_dirs()
            .iter()
            .map(|s| s.to_string_lossy().into_owned())
            .collect()
    }
    ///  The found files.

    pub fn files(&self) -> std::vec::Vec<String> {
        // CxxString convert to Rust String in vector
        self.inner
            .ftp_get_files()
            .iter()
            .map(|s| s.to_string_lossy().into_owned())
            .collect()
    }
}

pub struct ProgressDataOwned {
    pub bytes_transferred: u32,
    pub total_bytes: u32,
}

/*  */
pub struct ProgressData<'a> {
    inner: &'a crate::ftp::mavsdk::Ftp_ProgressData,
}

impl<'a> ProgressData<'a> {
    pub fn new(inner: &'a crate::ftp::mavsdk::Ftp_ProgressData) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> ProgressDataOwned {
        ProgressDataOwned {
            bytes_transferred: self.bytes_transferred(),
            total_bytes: self.total_bytes(),
        }
    }
    ///  The number of bytes already transferred.

    pub fn bytes_transferred(&self) -> u32 {
        self.inner.ftp_get_bytes_transferred()
    }
    ///  The total bytes to transfer.

    pub fn total_bytes(&self) -> u32 {
        self.inner.ftp_get_total_bytes()
    }
}

struct FtpInner {
    plugin: cxx::UniquePtr<crate::ftp::mavsdk::Ftp>,
}

pub struct FtpClient {
    inner: Mutex<FtpInner>,
}

impl FtpClient {
    pub fn new(plugin: cxx::UniquePtr<crate::ftp::mavsdk::Ftp>) -> Self {
        Self {
            inner: Mutex::new(FtpInner { plugin }),
        }
    }
}
