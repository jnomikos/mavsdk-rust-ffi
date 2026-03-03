// Auto-generated wrapper. Do not edit.
use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct MavlinkMessageOwned {
    pub message_name: String,
    pub system_id: u32,
    pub component_id: u32,
    pub target_system_id: u32,
    pub target_component_id: u32,
    pub fields_json: String,
}

/*  */
pub struct MavlinkMessage<'a> {
    inner: &'a crate::mavlink_direct::mavsdk::MavlinkDirect_MavlinkMessage,
}

impl<'a> MavlinkMessage<'a> {
    pub fn new(inner: &'a crate::mavlink_direct::mavsdk::MavlinkDirect_MavlinkMessage) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> MavlinkMessageOwned {
        MavlinkMessageOwned {
            message_name: self.message_name(),
            system_id: self.system_id(),
            component_id: self.component_id(),
            target_system_id: self.target_system_id(),
            target_component_id: self.target_component_id(),
            fields_json: self.fields_json(),
        }
    }
    ///  MAVLink message name (e.g., "HEARTBEAT", "GLOBAL_POSITION_INT")

    pub fn message_name(&self) -> String {
        self.inner
            .mavlink_direct_get_message_name()
            .to_string_lossy()
            .into_owned()
    }
    ///  System ID of the sender (for received messages)

    pub fn system_id(&self) -> u32 {
        self.inner.mavlink_direct_get_system_id()
    }
    ///  Component ID of the sender (for received messages)

    pub fn component_id(&self) -> u32 {
        self.inner.mavlink_direct_get_component_id()
    }
    ///  Target system ID (for sending, 0 for broadcast)

    pub fn target_system_id(&self) -> u32 {
        self.inner.mavlink_direct_get_target_system_id()
    }
    ///  Target component ID (for sending, 0 for broadcast)

    pub fn target_component_id(&self) -> u32 {
        self.inner.mavlink_direct_get_target_component_id()
    }
    ///  All message fields as single JSON object

    pub fn fields_json(&self) -> String {
        self.inner
            .mavlink_direct_get_fields_json()
            .to_string_lossy()
            .into_owned()
    }
}

struct MavlinkDirectInner {
    plugin: cxx::UniquePtr<crate::mavlink_direct::mavsdk::MavlinkDirect>,
}

pub struct MavlinkDirectClient {
    inner: Mutex<MavlinkDirectInner>,
}

impl MavlinkDirectClient {
    pub fn new(plugin: cxx::UniquePtr<crate::mavlink_direct::mavsdk::MavlinkDirect>) -> Self {
        Self {
            inner: Mutex::new(MavlinkDirectInner { plugin }),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn mavlink_direct_message_callback_ffi(
    user_data: *mut c_void,
    message: &crate::mavlink_direct::mavsdk::MavlinkDirect_MavlinkMessage,
) {
    if user_data.is_null() {
        return;
    }
    let send_message = MavlinkMessage::new(message).into_owned();

    // Cast the pointer to watch::Sender
    let sender =
        unsafe { &*(user_data as *const watch::Sender<std::option::Option<MavlinkMessageOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_message));
}
