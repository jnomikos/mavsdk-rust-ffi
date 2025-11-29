impl core::mavsdk::Mavsdk_MavlinkMessage {
    pub fn message_name(&self) -> &str {
        let msg_name = core::MavlinkMessageGetters::get_message_name(self);
        msg_name.to_str().unwrap_or("")
    }
}