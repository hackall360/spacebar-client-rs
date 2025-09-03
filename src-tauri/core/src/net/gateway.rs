use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::{protocol::frame::coding::CloseCode, Message};
use url::Url;

use log::{debug, info, warn};

/// Simple gateway client using a WebSocket connection.
/// Handles authentication and basic reconnection logic.
pub struct Gateway {
    url: Url,
    token: String,
    reconnect_delay: Duration,
}

impl Gateway {
    pub fn new(url: Url, token: String) -> Self {
        Self {
            url,
            token,
            reconnect_delay: Duration::from_secs(10),
        }
    }

    /// Start the gateway connection. This will keep trying to reconnect
    /// until an authentication failure occurs.
    pub async fn start(&mut self) {
        loop {
            match self.connect_once().await {
                Ok(reconnect) => {
                    if !reconnect {
                        break;
                    }
                    warn!("gateway closed; reconnecting in {:?}", self.reconnect_delay);
                    sleep(self.reconnect_delay).await;
                    self.reconnect_delay *= 2;
                }
                Err(e) => {
                    warn!(
                        "gateway error: {:?}; reconnecting in {:?}",
                        e, self.reconnect_delay
                    );
                    sleep(self.reconnect_delay).await;
                    self.reconnect_delay *= 2;
                }
            }
        }
    }

    /// Connects once to the gateway. Returns Ok(true) if the connection
    /// should be retried, Ok(false) if it should stop (e.g. auth failure).
    async fn connect_once(&self) -> Result<bool, tokio_tungstenite::tungstenite::Error> {
        let url = self.url.clone();
        info!("connecting to {}", url);
        let (ws_stream, _) = connect_async(url).await?;
        let (mut write, mut read) = ws_stream.split();

        // send identify payload
        let identify = json!({
            "op": 2,
            "d": {
                "token": self.token,
                "capabilities": 16381,
                "properties": {
                    "browser": "Spacebar Tauri",
                    "client_build_number": 0,
                    "release_channel": "dev",
                    "browser_user_agent": "",
                },
                "compress": false,
                "presence": {
                    "status": "online",
                    "since": 0,
                    "activities": [],
                    "afk": false,
                }
            }
        });
        write.send(Message::Text(identify.to_string())).await?;

        let mut should_reconnect = true;
        while let Some(msg) = read.next().await {
            match msg? {
                Message::Text(text) => {
                    debug!("gateway -> {}", text);
                }
                Message::Close(frame) => {
                    if let Some(frame) = frame {
                        if frame.code == CloseCode::Library(4004) {
                            // authentication failed, do not reconnect
                            warn!("gateway authentication failed");
                            should_reconnect = false;
                        }
                    }
                    break;
                }
                _ => {}
            }
        }

        Ok(should_reconnect)
    }
}
