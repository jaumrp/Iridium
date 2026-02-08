use bytes::BytesMut;
use components::get_protocol_version;

use crate::{
    packets::server::configuration::handshake::HandshakePacket,
    serial::{PacketRead, PacketWrite},
    types::var_int::VarInt,
};

pub mod packets;
pub mod serial;
pub mod types;

pub fn test() {
    let _my_int = VarInt(120);
    let _handshake = HandshakePacket {
        protocol_version: VarInt(get_protocol_version()),
        server_address: "localhost".to_string(),
        server_port: 25565,
        next_state: VarInt(1),
    };

    let mut _buffer = BytesMut::new();
    _handshake.write(&mut _buffer).unwrap();

    let _decode = HandshakePacket::read(&mut _buffer).unwrap();

    println!("decode, {:?} \noriginal {:?}", _decode, _handshake);

    assert_eq!(_handshake.protocol_version, _decode.protocol_version);
    assert_eq!(_handshake.server_address, _decode.server_address);
    assert_eq!(_handshake.server_port, _decode.server_port);
    assert_eq!(_handshake.next_state, _decode.next_state);
}
