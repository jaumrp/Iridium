use iridium::server::iridium_server::{IridiumServer, async_trait};

pub struct TestServer {
    // Add fields here
    _max_connections: usize,
}

impl TestServer {
    pub fn new() -> Self {
        TestServer {
            _max_connections: 26,
        }
    }
}

#[async_trait]
impl IridiumServer for TestServer {
    async fn on_enable(&mut self) {}
}

#[iridium::main]
async fn main() -> TestServer {
    let my_server = TestServer::new();
    my_server
}
