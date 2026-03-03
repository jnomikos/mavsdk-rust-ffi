// Auto-generated wrapper. Do not edit.
use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct TuneDescriptionOwned {
    pub song_elements: std::vec::Vec<crate::tune::mavsdk::Tune_SongElement>,
    pub tempo: i32,
}

/*  */
pub struct TuneDescription<'a> {
    inner: &'a crate::tune::mavsdk::Tune_TuneDescription,
}

impl<'a> TuneDescription<'a> {
    pub fn new(inner: &'a crate::tune::mavsdk::Tune_TuneDescription) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> TuneDescriptionOwned {
        TuneDescriptionOwned {
            song_elements: self.song_elements(),
            tempo: self.tempo(),
        }
    }
    ///  The list of song elements (notes, pauses, ...) to be played

    pub fn song_elements(&self) -> std::vec::Vec<crate::tune::mavsdk::Tune_SongElement> {
        self.inner
            .tune_get_song_elements()
            .iter()
            .cloned()
            .collect()
    }
    ///  The tempo of the song (range: 32 - 255)

    pub fn tempo(&self) -> i32 {
        self.inner.tune_get_tempo()
    }
}

struct TuneInner {
    plugin: cxx::UniquePtr<crate::tune::mavsdk::Tune>,
}

pub struct TuneClient {
    inner: Mutex<TuneInner>,
}

impl TuneClient {
    pub fn new(plugin: cxx::UniquePtr<crate::tune::mavsdk::Tune>) -> Self {
        Self {
            inner: Mutex::new(TuneInner { plugin }),
        }
    }
}
