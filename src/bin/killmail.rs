use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use dotenvy::dotenv;
use evetech::esi::EveApi;
use evetech::models::Api;
// use evetech::killmails::Killmail;
use chrono::NaiveDate;
use env_logger;
use log::info;

use std::collections::HashMap;
use std::env;
use std::thread;
use std::time::Duration;

pub fn establish_connection() -> anyhow::Result<SqliteConnection> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = SqliteConnection::establish(&url)?;
    Ok(conn)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let conn = establish_connection()?;
    let mut db = Api::new(conn);

    let args = env::args().collect::<Vec<String>>();
    if args.len() != 1 {
        let api = EveApi::new();

        if let Ok(date) = NaiveDate::parse_from_str(args[1].as_str(), "%Y-%m-%d") {
            let zkb_api = format!(
                "https://zkillboard.com/api/history/{}.json",
                date.format("%Y%m%d").to_string()
            );
            info!("zkillboard.com API: {zkb_api}");

            let map = reqwest::get(&zkb_api)
                .await?
                .json::<HashMap<i32, String>>()
                .await?;
            info!("Received {} killmails from zkillboard.com", map.len());

            for (id, hash) in map {
                info!("{id} -> {hash}");
                if let Ok(killmail) = api.load_killmail(id as u32, hash.clone()).await {
                    db.save(&killmail)?;
                } else {
                    println!();
                    for _ in 0..10 {
                        print!(".");
                        thread::sleep(Duration::from_secs(1));
                    }
                    if let Ok(killmail) = api.load_killmail(id as u32, hash).await {
                        db.save(&killmail)?;
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
