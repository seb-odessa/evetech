use env_logger;
use log::{error, info};
use tokio::time::{sleep, Duration};
use websockets::{Frame, WebSocket};

use evetech::killmails::killmail::Killmail;

use std::env;

pub enum HandleResult {
    Ok,
    Close,
}

struct Handler {
    api: String,
    client: reqwest::Client,
    payloads: Vec<String>,
}
impl Handler {
    pub fn new(host: &String, port: u16) -> Self {
        let api = format!("http://{host}:{port}/killmail/save");
        Self {
            api,
            client: reqwest::Client::new(),
            payloads: Vec::new(),
        }
    }

    async fn text(
        &mut self,
        payload: &String,
        continuation: bool,
        fin: bool,
    ) -> anyhow::Result<()> {
        self.payloads.push(payload.clone());

        if !continuation || fin {
            let json = self.payloads.join("");
            self.payloads.clear();

            match serde_json::from_str::<Killmail>(&json) {
                Ok(killmail) => {
                    self.save(&killmail).await?;
                }
                Err(what) => {
                    error!(": {what}");
                }
            }
        }

        Ok(())
    }

    async fn save(&self, killmail: &Killmail) -> anyhow::Result<()> {
        info!(
            "killmail_id: {} {} {}",
            killmail.killmail_id, killmail.solar_system_id, killmail.killmail_time
        );

        let res = self.client.post(&self.api).json(&killmail).send().await;
        if res.is_err() {
            sleep(Duration::from_secs(10)).await;
            self.client.post(&self.api).json(&killmail).send().await?;
        }

        Ok(())
    }

    async fn handle(&mut self, frame: Frame) -> anyhow::Result<HandleResult> {
        match frame {
            Frame::Text {
                payload,
                continuation,
                fin,
            } => {
                let len = payload.len();
                info!("Frame::Text {{ {len}, {continuation}, {fin} }}",);
                self.text(&payload, continuation, fin).await?;
                Ok(HandleResult::Ok)
            }
            Frame::Binary {
                payload,
                continuation,
                fin,
            } => {
                let len = payload.len();
                info!("Frame::Binary {{ {len}, {continuation}, {fin} }}",);
                Ok(HandleResult::Ok)
            }
            Frame::Ping { payload } => {
                info!("Frame::Ping {{ {:?} }}", payload);
                Ok(HandleResult::Ok)
            }
            Frame::Pong { payload } => {
                info!("Frame::Ping {{ {:?} }}", payload);
                Ok(HandleResult::Ok)
            }
            Frame::Close { payload } => {
                info!("Frame::Ping {{ {:?} }}", payload);
                Ok(HandleResult::Close)
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = env::var("ZKBINFO_HOST").unwrap_or(String::from("localhost"));
    let port = env::var("ZKBINFO_PORT")
        .unwrap_or_default()
        .parse::<u16>()
        .unwrap_or(8080);
    let api = format!("http://{host}:{port}/killmail/save");
    info!("zkbinfo API url: {api}");

    let mut handler = Handler::new(&host, port);
    info!("Handler created");

    let wss = "wss://zkillboard.com/websocket/";
    let enable = r#"{"action":"sub","channel":"killstream"}"#;
    let mut ws = WebSocket::connect(wss).await?;
    info!("Web Socket {:?} created", ws);

    ws.send_text(enable.to_string()).await?;
    info!("Web Socket request sent");
    loop {
        let frame = ws.receive().await?;
        if let Ok(HandleResult::Close) = handler.handle(frame).await {
            break;
        }
    }
    Ok(())
}
