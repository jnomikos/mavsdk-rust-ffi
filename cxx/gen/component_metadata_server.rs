// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use tokio::sync::watch;
use std::sync::Mutex;






pub struct MetadataOwned {
        pub r#type: crate::component_metadata_server::mavsdk::ComponentMetadataServer_MetadataType,
        pub json_metadata: String,
}

/*  */
pub struct Metadata<'a> {
    inner: &'a crate::component_metadata_server::mavsdk::ComponentMetadataServer_Metadata,
}

impl<'a> Metadata<'a> {
    pub fn new(inner: &'a crate::component_metadata_server::mavsdk::ComponentMetadataServer_Metadata) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> MetadataOwned {
        MetadataOwned {
            
            r#type: self.r#type(),
            json_metadata: self.json_metadata(),
        }
    }
        ///  The metadata type

    pub fn r#type(&self) -> crate::component_metadata_server::mavsdk::ComponentMetadataServer_MetadataType {self.inner.component_metadata_server_get_type().clone()
    }
        ///  The JSON metadata

    pub fn json_metadata(&self) -> String {
        self.inner.component_metadata_server_get_json_metadata().to_string_lossy().into_owned()
    }
}

    struct ComponentMetadataServerInner {
        plugin: cxx::UniquePtr<crate::component_metadata_server::mavsdk::ComponentMetadataServer>,
        
    }

    pub struct ComponentMetadataServerClient {
        inner: Mutex<ComponentMetadataServerInner>,
        
    }

    impl ComponentMetadataServerClient {
        pub fn new(plugin: cxx::UniquePtr<crate::component_metadata_server::mavsdk::ComponentMetadataServer>) -> Self {
            

            Self {
                inner: Mutex::new(ComponentMetadataServerInner {
                    plugin,
                    
                }),
                
            }
        }
        
        
    }

