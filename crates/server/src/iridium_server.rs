pub use async_trait::async_trait;
use log::{error, info, warn};

use network::handle_connection;

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

use crate::{ServerConfig, ServerContext};

#[async_trait]
pub trait IridiumServer {
    async fn on_enable(&mut self, _ctx: &mut ServerContext) {}
    async fn on_reload(&mut self, _ctx: &mut ServerContext) {}
    async fn on_disable(&mut self, _ctx: &mut ServerContext) {}
}

pub async fn bootstrap<Server: IridiumServer + Send + Sync + 'static>(mut server: Server) {
    let mut ctx = ServerContext::new();

    if let Err(e) = ctx.config.load_config().await {
        warn!("could not load server.yml: {}", e);
    }

    let (add, port) = assert_config(&ctx.config).unwrap_or_else(|| {
        return ("0.0.0.0", 25565);
    });

    let address = format!("{}:{}", add, port);

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

    log::info!("╭──────────────────────────────────────────╮");
    log::info!("│ 💎 Iridium Server                        │");
    log::info!("│ 🌍 Address: {:<28} │", add);
    log::info!("│ 🔌 Port:    {:<28} │", port);
    log::info!("╰──────────────────────────────────────────╯");

    server.on_enable(&mut ctx).await;

    info!("server is running. Type 'stop' to exit.");

    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            accept_result = listener.accept() => {
                match accept_result {
                    Ok((socket, _address)) => {
                        let rx = shutdown_tex.subscribe();
                        let event_bus = ctx.event_bus.clone();
                        tokio::spawn(async move {
                            handle_connection(socket, rx, event_bus).await;
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
                        server.on_reload(&mut ctx).await;
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
    server.on_disable(&mut ctx).await;
}

pub fn assert_config(config: &ServerConfig) -> Option<(&str, i64)> {
    let address = config.get_str("server.host").unwrap_or_else(|| "0.0.0.0");
    let port = config.get_int("server.port").unwrap_or_else(|| 25565);

    if address.trim().is_empty() {
        return None;
    }

    if port < 1024 || port >= 65535 {
        return None;
    }
    return Some((address, port));
}
