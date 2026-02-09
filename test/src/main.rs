use iridium::server::{
    ServerContext,
    iridium_server::{IridiumServer, async_trait},
};

pub struct TestServer {
    // Add fields here
}

impl TestServer {
    pub fn new() -> Self {
        TestServer {}
    }
}

#[async_trait]
impl IridiumServer for TestServer {
    async fn on_enable(&mut self, _: &mut ServerContext) {}
}

#[iridium::main]
async fn main() -> TestServer {
    let my_server = TestServer::new();
    my_server
}
