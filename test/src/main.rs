use iridium::server::iridium_server::{IridiumServer, async_trait};

pub struct TestServer {
    // Add fields here
}

impl TestServer {
    pub fn new() -> Self {
        TestServer {}
    }
}

#[async_trait]
impl IridiumServer for TestServer {}

#[iridium::main]
async fn main() -> TestServer {
    let my_server = TestServer::new();
    my_server
}
