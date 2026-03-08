// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct MetadataDataOwned {
    pub json_metadata: String,
}

/*  */
pub struct MetadataData<'a> {
    inner: &'a crate::component_metadata::mavsdk::ComponentMetadata_MetadataData,
}

impl<'a> MetadataData<'a> {
    pub fn new(
        inner: &'a crate::component_metadata::mavsdk::ComponentMetadata_MetadataData,
    ) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> MetadataDataOwned {
        MetadataDataOwned {
            json_metadata: self.json_metadata(),
        }
    }
    ///  The JSON metadata

    pub fn json_metadata(&self) -> String {
        self.inner
            .component_metadata_get_json_metadata()
            .to_string_lossy()
            .into_owned()
    }
}

pub struct MetadataUpdateOwned {
    pub compid: u32,
    pub r#type: crate::component_metadata::mavsdk::ComponentMetadata_MetadataType,
    pub json_metadata: String,
}

/*  */
pub struct MetadataUpdate<'a> {
    inner: &'a crate::component_metadata::mavsdk::ComponentMetadata_MetadataUpdate,
}

impl<'a> MetadataUpdate<'a> {
    pub fn new(
        inner: &'a crate::component_metadata::mavsdk::ComponentMetadata_MetadataUpdate,
    ) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> MetadataUpdateOwned {
        MetadataUpdateOwned {
            compid: self.compid(),
            r#type: self.r#type(),
            json_metadata: self.json_metadata(),
        }
    }
    ///  The component ID

    pub fn compid(&self) -> u32 {
        self.inner.component_metadata_get_compid()
    }
    ///  The metadata type

    pub fn r#type(&self) -> crate::component_metadata::mavsdk::ComponentMetadata_MetadataType {
        self.inner.component_metadata_get_type().clone()
    }
    ///  The JSON metadata

    pub fn json_metadata(&self) -> String {
        self.inner
            .component_metadata_get_json_metadata()
            .to_string_lossy()
            .into_owned()
    }
}

struct ComponentMetadataInner {
    plugin: cxx::UniquePtr<crate::component_metadata::mavsdk::ComponentMetadata>,

    /// MetadataAvailable State
    metadata_available_handle: Option<usize>,
    metadata_available_user_data: *mut c_void,
}

pub struct ComponentMetadataClient {
    inner: Mutex<ComponentMetadataInner>,

    // Producer for the latest MetadataAvailable state; broadcast to all active subscribers.
    metadata_available_tx: watch::Sender<std::option::Option<MetadataUpdateOwned>>,
}

impl ComponentMetadataClient {
    pub fn new(
        plugin: cxx::UniquePtr<crate::component_metadata::mavsdk::ComponentMetadata>,
    ) -> Self {
        let (metadata_available_tx, _) = watch::channel(None);

        Self {
            inner: Mutex::new(ComponentMetadataInner {
                plugin,

                metadata_available_handle: None,
                metadata_available_user_data: std::ptr::null_mut(),
            }),

            metadata_available_tx,
        }
    }

    pub fn subscribe_metadata_available(
        &self,
    ) -> watch::Receiver<std::option::Option<MetadataUpdateOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.metadata_available_handle.is_none() {
            // Lazy initialization of C++ 'metadata_available' component_metadata stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.metadata_available_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_metadata_available(
                component_metadata_metadata_available_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.metadata_available_handle = Some(handle);
            inner.metadata_available_user_data = user_data_ptr;
        }

        self.metadata_available_tx.subscribe()
    }
}

impl Drop for ComponentMetadataClient {
    fn drop(&mut self) {
        if let Ok(mut inner) = self.inner.lock() {
            if let Some(handle) = inner.metadata_available_handle {
                inner
                    .plugin
                    .pin_mut()
                    .unsubscribe_metadata_available(handle as libc::uintptr_t);
                unsafe {
                    let _ = Box::from_raw(
                        inner.metadata_available_user_data
                            as *mut watch::Sender<std::option::Option<MetadataUpdateOwned>>,
                    );
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn component_metadata_metadata_available_callback_ffi(
    user_data: *mut c_void,
    metadata_available: &crate::component_metadata::mavsdk::ComponentMetadata_MetadataUpdate,
) {
    if user_data.is_null() {
        return;
    }
    let send_metadata_available = MetadataUpdate::new(metadata_available).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<MetadataUpdateOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_metadata_available));
}
