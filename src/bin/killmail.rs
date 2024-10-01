use chrono::NaiveDate;
use env_logger;
use evetech::esi::api::Uid;
use evetech::esi::EveApi;
use evetech::killmails::Killmail;
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
            info!("Need to get {} killmails from zkillboard.com", map.len());

            for (id, hash) in map {
                let uid = Uid::Killmail(id, hash.clone());
                if let Ok(killmail) = api.load::<Killmail>(&uid).await {
                    post(&client, &killmail, &zkbinfo_save_api).await?;
                } else {
                    for i in 1..7 {
                        info!("Retry {i} for {{ {id} {hash} }}");
                        thread::sleep(Duration::from_secs(3));
                        if let Ok(killmail) = api.load::<Killmail>(&uid).await {
                            post(&client, &killmail, &zkbinfo_save_api).await?;
                            break;
                        }
                    }
                }
            }
        }
    } else {
        usage(&args[0])
    }
    Ok(())
}

async fn post(client: &reqwest::Client, killmail: &Killmail, api: &String) -> anyhow::Result<()> {
    client.post(api).json(&killmail).send().await?;
    info!("{} -> {}", killmail.killmail_id, killmail.killmail_time);
    Ok(())
}

fn usage(app: &String) -> () {
    println!("Usage:\n\t{app} <YYYY-MM-DD>");
}
