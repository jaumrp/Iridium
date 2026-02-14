use iridium::{
    entity::EntityType,
    server::{
        ServerContext,
        iridium_server::{IridiumServer, async_trait},
    },
};

pub struct TestServer;

#[async_trait]
impl IridiumServer for TestServer {
    async fn on_enable(&mut self, _: &mut ServerContext) {
        let test = EntityType::Player;
        test.ident();
        let test = EntityType::Zombie;
        test.ident();
    }
}

#[iridium::main]
async fn main() -> TestServer {
    TestServer
}
