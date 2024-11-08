use evetech::killmails::killmail::Killmail;

use env_logger;
use log::{debug, error, info, warn};
use std::env;

use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

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
    let saver = Saver::new(&api);
    info!("Saver created: {:#?}", saver);

    let url = "wss://zkillboard.com/websocket/";
    info!("WS URL: {url}");

    let request = url.into_client_request()?;
    let (wss, response) = connect_async(request).await?;
    info!("response: {:#?}", response);

    let (mut ws, mut rs) = wss.split();

    let command = r#"{"action":"sub","channel":"killstream"}"#;
    ws.send(Message::Text(command.into())).await?;
    info!("Sent: {:#?}", command);

    while let Some(Ok(message)) = rs.next().await {
        // info!("Received: {:#?}", message);
        match message {
            Message::Binary(payload) => {
                info!("Message::Binary payload length: {}", payload.len());
                debug!("Message::Binary {:#?}", payload);
            }
            Message::Text(payload) => {
                info!("Message::Text payload length: {}", payload.len());
                debug!("Message::Text {:#?}", payload);
                saver.save(&payload).await?;
            }
            Message::Ping(payload) => {
                info!("Message::Ping {:#?}", payload)
            }
            Message::Pong(payload) => {
                info!("Message::Pong {:#?}", payload)
            }
            Message::Close(frame) => {
                info!("Message::Close {:#?}", frame);
                todo!()
            }
            Message::Frame(frame) => {
                info!("Message::Frame {:#?}", frame);
                todo!()
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
struct Saver {
    api: String,
    client: reqwest::Client,
}
impl Saver {
    pub fn new(api: &String) -> Self {
        Self {
            api: api.clone(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn save(&self, payload: &String) -> anyhow::Result<()> {
        match serde_json::from_str::<Killmail>(&payload) {
            Ok(killmail) => {
                self.post(&killmail).await?;
            }
            Err(what) => {
                error!("{what} {payload}");
            }
        }
        Ok(())
    }

    async fn post(&self, killmail: &Killmail) -> anyhow::Result<()> {
        let tm = &killmail.killmail_time;
        let id = killmail.killmail_id;
        let sid = killmail.solar_system_id;
        info!("ET: {tm} https://zkillboard.com/kill/{id}/ https://zkillboard.com/system/{sid}/");

        let res = self.client.post(&self.api).json(&killmail).send().await;
        if res.is_err() {
            warn!("{:#?}", res);
            sleep(Duration::from_secs(1)).await;
            self.client.post(&self.api).json(&killmail).send().await?;
        }

        Ok(())
    }
}
