pub use async_trait::async_trait;
use log::{error, info, warn};

use network::handle_connection;

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

use crate::Config;

#[async_trait]
pub trait IridiumServer {
    async fn on_enable(&mut self) {}
    async fn on_reload(&mut self) {}
    async fn on_disable(&mut self) {
        info!("exiting, bye!")
    }
}

pub async fn bootstrap<Server: IridiumServer + Send + Sync + 'static>(
    mut server: Server,
    config: Config,
) {
    assert_config(&config);

    let address = format!("{}:{}", config.address, config.port);
    let listener = match TcpListener::bind(&address).await {
        Ok(listener) => {
            info!("🚀 iridium server booting on {}", address);
            listener
        }
        Err(error) => {
            error!("failed to bind to {}: {}", address, error);
            return;
        }
    };

    let (shutdown_tex, _) = broadcast::channel::<()>(16);

    server.on_enable().await;
    info!("server is running. Type 'stop' to exit.");

    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            accept_result = listener.accept() => {
                match accept_result {
                    Ok((socket, _address)) => {
                        let rx = shutdown_tex.subscribe();
                        tokio::spawn(async move {
                            handle_connection(socket, rx).await;
                        });
                    },
                    Err(error) => error!("failed to accept connection: {}", error),
                }
            },
            line = reader.next_line() => match line {
                Ok(Some(line)) => {
                    if line == "stop" {
                        break;
                    }
                    if line == "reload" {
                        warn!("Reloading Iridium Server...");
                        server.on_reload().await;
                        info!("reloaded Iridium server");
                    }
                }
                Ok(None) => break,
                Err(error) => error!("failed to read line: {}", error),
            },
            _ = tokio::signal::ctrl_c() => {
                break;
            }
        }
    }

    if !shutdown_tex.is_empty() {
        let _ = shutdown_tex.send(());
    }

    warn!("Shutting down Iridium Server...");
    server.on_disable().await;
}

pub fn assert_config(config: &Config) {
    if config.address.trim().is_empty() {
        panic!("address cannot be empty");
    }

    if config.id.trim().is_empty() {
        panic!("id cannot be empty");
    }

    if config.port < 1024 || config.port >= 65535 {
        panic!("port must be between 1024 and 65535");
    }
}
