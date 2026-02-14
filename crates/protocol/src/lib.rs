pub mod serial;
pub mod types;

pub enum ConnectionState {
    Handshaking,
    Login,
    Status,
    Play,
    Configuration,
}
