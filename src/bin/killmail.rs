use chrono::NaiveDate;
use env_logger;
use evetech::esi::EveApi;
use log::info;

use std::collections::HashMap;
use std::env;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = env::var("ZKBINFO_HOST").unwrap_or(String::from("localhost"));
    let port = env::var("ZKBINFO_PORT")
        .unwrap_or_default()
        .parse::<u16>()
        .unwrap_or(8080);

    let args = env::args().collect::<Vec<String>>();
    if args.len() != 1 {
        let api = EveApi::new();
        let client = reqwest::Client::new();
        let zkbinfo_save_api = format!("http://{host}:{port}/killmail/save");
        info!("zkbinfo API SAVE url: {zkbinfo_save_api}");

        if let Ok(date) = NaiveDate::parse_from_str(args[1].as_str(), "%Y-%m-%d") {
            let zkb_api = format!(
                "https://zkillboard.com/api/history/{}.json",
                date.format("%Y%m%d").to_string()
            );
            info!("zkillboard.com API: {zkb_api}");
            let zkbinfo_ids_api = format!(
                "http://{host}:{port}/killmail/{}",
                date.format("%Y-%m-%d").to_string()
            );
            info!("zkbinfo API IDS url: {zkbinfo_ids_api}");

            let mut map = reqwest::get(&zkb_api)
                .await?
                .json::<HashMap<i32, String>>()
                .await?;
            info!("Received {} killmails from zkillboard.com", map.len());

            let ids = reqwest::get(&zkbinfo_ids_api)
                .await?
                .json::<Vec<i32>>()
                .await?;
            info!("Received {} killmails from zkbinfo", ids.len());

            for key in ids {
                map.remove(&key);
            }

            for (id, hash) in map {
                info!("{id} -> {hash}");
                if let Ok(killmail) = api.load_killmail(id as u32, hash.clone()).await {
                    client
                        .post(&zkbinfo_save_api)
                        .json(&killmail)
                        .send()
                        .await?;
                } else {
                    println!();
                    for _ in 0..10 {
                        print!(".");
                        thread::sleep(Duration::from_secs(1));
                    }
                    if let Ok(killmail) = api.load_killmail(id as u32, hash).await {
                        client
                            .post(&zkbinfo_save_api)
                            .json(&killmail)
                            .send()
                            .await?;
                    }
                }
            }
        }
    } else {
        usage(&args[0])
    }
    Ok(())
}

fn usage(app: &String) -> () {
    println!("Usage:\n\t{app} <YYYY-MM-DD>");
}
