use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio::sync::{mpsc, Mutex};
use tokio::time::{sleep, Duration};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::{protocol::frame::coding::CloseCode, Message};
use url::Url;

use log::{debug, info, warn};
use std::sync::Arc;

/// Simple gateway client using a WebSocket connection.
/// Handles authentication and basic reconnection logic.
pub struct Gateway {
    url: Url,
    token: String,
    reconnect_delay: Duration,
    event_tx: mpsc::UnboundedSender<GatewayEvent>,
    seq: Arc<Mutex<Option<u64>>>,
}

#[derive(Debug, Clone)]
pub enum GatewayEvent {
    Dispatch { event: String, data: Value },
    HeartbeatAck,
}

impl Gateway {
    pub fn new(url: Url, token: String, event_tx: mpsc::UnboundedSender<GatewayEvent>) -> Self {
        Self {
            url,
            token,
            reconnect_delay: Duration::from_secs(1),
            event_tx,
            seq: Arc::new(Mutex::new(None)),
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
                    let delay = self.reconnect_delay;
                    sleep(delay).await;
                    self.reconnect_delay = (self.reconnect_delay * 2).min(Duration::from_secs(60));
                }
                Err(e) => {
                    warn!(
                        "gateway error: {:?}; reconnecting in {:?}",
                        e, self.reconnect_delay
                    );
                    let delay = self.reconnect_delay;
                    sleep(delay).await;
                    self.reconnect_delay = (self.reconnect_delay * 2).min(Duration::from_secs(60));
                }
            }
        }
    }

    /// Connects once to the gateway. Returns Ok(true) if the connection
    /// should be retried, Ok(false) if it should stop (e.g. auth failure).
    async fn connect_once(&mut self) -> Result<bool, tokio_tungstenite::tungstenite::Error> {
        let url = self.url.clone();
        info!("connecting to {}", url);
        let (ws_stream, _) = connect_async(url).await?;
        let (mut write, mut read) = ws_stream.split();

        // reset backoff since connection succeeded
        self.reconnect_delay = Duration::from_secs(1);

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
        let write = Arc::new(Mutex::new(write));

        let seq = self.seq.clone();
        let mut heartbeat_task: Option<tokio::task::JoinHandle<()>> = None;

        let mut should_reconnect = true;
        while let Some(msg) = read.next().await {
            match msg? {
                Message::Text(text) => {
                    debug!("gateway -> {}", text);
                    let v: Value = serde_json::from_str(&text).unwrap_or_default();
                    match v["op"].as_i64() {
                        Some(10) => {
                            if let Some(interval_ms) = v["d"]["heartbeat_interval"].as_u64() {
                                let write_clone = write.clone();
                                let seq_clone = seq.clone();
                                let mut interval = tokio::time::interval(Duration::from_millis(interval_ms));
                                heartbeat_task = Some(tokio::spawn(async move {
                                    loop {
                                        interval.tick().await;
                                        let s = *seq_clone.lock().await;
                                        let payload = json!({"op": 1, "d": s});
                                        if write_clone
                                            .lock()
                                            .await
                                            .send(Message::Text(payload.to_string()))
                                            .await
                                            .is_err()
                                        {
                                            break;
                                        }
                                    }
                                }));
                            }
                        }
                        Some(0) => {
                            if let Some(s) = v["s"].as_u64() {
                                *seq.lock().await = Some(s);
                            }
                            if let Some(event) = v["t"].as_str() {
                                let data = v["d"].clone();
                                let _ = self
                                    .event_tx
                                    .send(GatewayEvent::Dispatch { event: event.to_string(), data });
                            }
                        }
                        Some(11) => {
                            let _ = self.event_tx.send(GatewayEvent::HeartbeatAck);
                        }
                        _ => {}
                    }
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

        if let Some(task) = heartbeat_task {
            task.abort();
        }

        Ok(should_reconnect)
    }
}
