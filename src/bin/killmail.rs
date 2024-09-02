use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use dotenvy::dotenv;
use evetech::esi::EveApi;
// use evetech::killmails::Killmail;
use std::env;

pub fn establish_connection() -> anyhow::Result<SqliteConnection> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = SqliteConnection::establish(&url)?;
    Ok(conn)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let api = EveApi::new();

    let killmail = api
        .load_killmail(120480909, "9c01e82d5a65818c816a72e6bcc24dd045dde2f8")
        .await?;

    println!("{:?}", killmail);

    // let conn = establish_connection()?;

    Ok(())
}
