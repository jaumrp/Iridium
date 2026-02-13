use iridium::{
    components::{Component, get_protocol_version},
    events::Cancelable,
    network::event::server::ServerListPingEvent,
    protocol::types::var_int::VarInt,
    server::{
        ServerContext,
        iridium_server::{IridiumServer, async_trait},
    },
};

pub struct TestServer;

#[async_trait]
impl IridiumServer for TestServer {
    async fn on_enable(&mut self, ctx: &mut ServerContext) {
        ctx.event_bus.subscribe::<ServerListPingEvent, _>(|event| {
            let text = if event.status.get_protocol_version() == VarInt(get_protocol_version()) {
                format!(
                    "Protocol version is {}",
                    event.status.get_protocol_version().0
                )
            } else {
                format!(
                    "Protocol version is {}",
                    event.status.get_protocol_version().0
                )
            };
            event.status.online_players(10);
            event
                .status
                .motd(Component::text(format!("hello events?\n{}", text)));

            event.set_canceled(true);

            Ok(())
        });
    }
}

#[iridium::main]
async fn main() -> TestServer {
    TestServer
}
