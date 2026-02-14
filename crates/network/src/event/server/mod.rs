use macros::event;

use crate::packets::client::status::StatusBuilder;

#[event]
pub struct ServerListPingEvent {
    pub status: StatusBuilder,
}

impl ServerListPingEvent {
    pub fn new(status: StatusBuilder) -> Self {
        Self {
            status: status,
            is_canceled: false,
        }
    }
}
