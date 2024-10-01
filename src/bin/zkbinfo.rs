use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use anyhow::anyhow;
use diesel::{Connection, SqliteConnection};
use env_logger;
use log::{debug, error, info};

use std::env;
use std::sync::Mutex;
use std::time::Duration;

use evetech::models::Api;
use evetech::models::{ObjectType, SubjectType};

type Context = web::Data<AppState>;

pub struct AppState {
    pub api: Mutex<Api>,
}
impl AppState {
    pub fn new(api: Api) -> Self {
        Self {
            api: Mutex::new(api),
        }
    }
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let uri = env::var("ZKBINFO_DB").unwrap_or(String::from("killmails.db"));
    info!("The ZKBINFO Database URI: {uri}");

    let host = env::var("ZKBINFO_HOST").unwrap_or(String::from("localhost"));
    info!("The ZKBINFO host: {host}");

    let port = env::<u16>("ZKBINFO_PORT", 8080);
    info!("The ZKBINFO port: {port}");

    let keep_days: u16 = env::<u16>("ZKBINFO_DAYS", 90);
    info!("The ZKBINFO keep days: {keep_days}");

    let cleanup_period: u64 = env::<u64>("ZKBINFO_PERIOD", 4);
    info!("The ZKBINFO clean up period: {cleanup_period} hours");

    let conn = SqliteConnection::establish(&uri)?;
    let mut api = Api::new(conn);
    cleanup(&mut api, keep_days);

    let context = web::Data::new(AppState::new(api));
    let ctx = context.clone();
    actix_rt::spawn(async move {
        let mut interval = actix_rt::time::interval(Duration::from_secs(60 * 60 * cleanup_period));
        loop {
            interval.tick().await;
            if let Ok(mut api) = ctx.api.try_lock() {
                cleanup(&mut api, keep_days);
            }
        }
    });

    info!("Launching server at {host}:{port}");
    let result = "wins|losses";
    let allowed = "character|corporation|alliance|faction";
    let friends_route = format!("/friendly/{{object:{allowed}}}/for/{{subject:{allowed}}}/{{id}}");
    let enemies_route = format!("/enemy/{{object:{allowed}}}/for/{{subject:{allowed}}}/{{id}}");
    let report_total_route = format!("/{{rtype:{result}}}/{{subject:{allowed}}}/{{id}}");
    let report_ships_route = format!("/{{rtype:{result}}}/{{subject:{allowed}}}/{{id}}/ships");
    let report_systems_route = format!("/{{rtype:{result}}}/{{subject:{allowed}}}/{{id}}/systems");
    let report_lost_ships_route = format!("/lost/ship/{{sid}}/{{subject:{allowed}}}/{{id}}");

    HttpServer::new(move || {
        App::new()
            .app_data(context.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(
                web::scope("/api")
                    .route(&friends_route, web::get().to(friends))
                    .route(&enemies_route, web::get().to(enemies))
                    .route(&report_total_route, web::get().to(report_total))
                    .route(&report_ships_route, web::get().to(report_ships))
                    .route(&report_systems_route, web::get().to(report_systems))
                    .route(&report_lost_ships_route, web::get().to(lost_ships))
            )
            .service(
                web::scope("/killmail")
                    .route("/{date}", web::get().to(ids_by_date))
                    .route("/save", web::post().to(save)),
            )
            .wrap(Logger::default())
    })
    .workers(6)
    .bind((host.as_str(), port))?
    .run()
    .await
    .map_err(|e| anyhow!(e))
}

fn object<S: Into<String>>(arg: S) -> ObjectType {
    match arg.into().as_str() {
        "character" => ObjectType::Character,
        "corporation" => ObjectType::Corporation,
        "alliance" => ObjectType::Alliance,
        "faction" => ObjectType::Faction,
        _ => unreachable!(),
    }
}

fn subject<S: Into<String>>(arg: S, id: i32) -> SubjectType {
    match arg.into().as_str() {
        "character" => SubjectType::Character(id),
        "corporation" => SubjectType::Corporation(id),
        "alliance" => SubjectType::Alliance(id),
        "faction" => SubjectType::Faction(id),
        _ => unreachable!(),
    }
}

async fn report_total(ctx: Context, args: web::Path<(String, String, i32)>) -> impl Responder {
    let (rtype, subj, id) = args.into_inner();
    let result = ctx
        .api
        .try_lock()
        .map_err(|e| anyhow!("{e}"))
        .and_then(|mut api| match rtype.as_str() {
            "wins" => api.wins(subject(subj, id)),
            "losses" => api.losses(subject(subj, id)),
            _ => unreachable!(),
        })
        .map_err(|e| anyhow!("{e}"));

    Result::from(result)
}

async fn report_systems(ctx: Context, args: web::Path<(String, String, i32)>) -> impl Responder {
    let (rtype, subj, id) = args.into_inner();
    let result = ctx
        .api
        .try_lock()
        .map_err(|e| anyhow!("{e}"))
        .and_then(|mut api| match rtype.as_str() {
            "wins" => api.wins_systems(subject(subj, id)),
            "losses" => api.losses_systems(subject(subj, id)),
            _ => unreachable!(),
        })
        .map_err(|e| anyhow!("{e}"));

    Result::from(result)
}

async fn report_ships(ctx: Context, args: web::Path<(String, String, i32)>) -> impl Responder {
    let (rtype, subj, id) = args.into_inner();
    let result = ctx
        .api
        .try_lock()
        .map_err(|e| anyhow!("{e}"))
        .and_then(|mut api| match rtype.as_str() {
            "wins" => api.wins_ships(subject(subj, id)),
            "losses" => api.losses_ships(subject(subj, id)),
            _ => unreachable!(),
        })
        .map_err(|e| anyhow!("{e}"));

    Result::from(result)
}


async fn friends(ctx: Context, args: web::Path<(String, String, i32)>) -> impl Responder {
    let (obj, subj, id) = args.into_inner();
    let result = ctx
        .api
        .try_lock()
        .map_err(|e| anyhow!("{e}"))
        .and_then(|mut api| api.friends(subject(subj, id), object(obj)))
        .map_err(|e| anyhow!("{e}"));

    Result::from(result)
}

async fn enemies(ctx: Context, args: web::Path<(String, String, i32)>) -> impl Responder {
    let (obj, subj, id) = args.into_inner();
    let result = ctx
        .api
        .try_lock()
        .map_err(|e| anyhow!("{e}"))
        .and_then(|mut api| api.enemies(subject(subj, id), object(obj)))
        .map_err(|e| anyhow!("{e}"));

    Result::from(result)
}

async fn ids_by_date(ctx: Context, args: web::Path<String>) -> impl Responder {
    let date = args.into_inner();

    let result = ctx
        .api
        .try_lock()
        .map_err(|e| anyhow!("{e}"))
        .and_then(|mut api| api.ids_by_date(date))
        .map_err(|e| anyhow!("{e}"));

    Result::from(result)
}

async fn lost_ships(ctx: Context, args: web::Path<(i32, String, i32)>) -> impl Responder {
    let (sid, subj, id) = args.into_inner();
    let result = ctx
        .api
        .try_lock()
        .map_err(|e| anyhow!("{e}"))
        .and_then(|mut api| api.lost_ships(subject(subj, id), sid))
        .map_err(|e| anyhow!("{e}"));

    Result::from(result)
}

async fn save(ctx: Context, json: String) -> impl Responder {
    let result = serde_json::from_str::<evetech::killmails::Killmail>(&json)
        .map_err(|e| anyhow!("{e}"))
        .and_then(|killmail| {
            ctx.api
                .try_lock()
                .map_err(|e| anyhow!("{e}"))
                .and_then(|mut api| api.save(&killmail))
        });
    Result::from(result)
}

pub struct Result {
    json: String,
}
impl From<String> for Result {
    fn from(json: String) -> Self {
        debug!("{json}");
        Self { json }
    }
}
impl From<anyhow::Error> for Result {
    fn from(err: anyhow::Error) -> Self {
        let json = format!(r#"{{ "error": "{err}" }}"#);
        error!("{json}");
        Self { json }
    }
}
impl From<anyhow::Result<Vec<i32>>> for Result {
    fn from(result: anyhow::Result<Vec<i32>>) -> Self {
        match result {
            Ok(ids) => match serde_json::to_string(&ids) {
                Ok(json) => Self::from(json),
                Err(err) => Self::from(anyhow!("{err}")),
            },
            Err(err) => Self::from(err),
        }
    }
}
impl From<anyhow::Result<Vec<(i32, i64)>>> for Result {
    fn from(result: anyhow::Result<Vec<(i32, i64)>>) -> Self {
        match result {
            Ok(ids) => match serde_json::to_string(&ids) {
                Ok(json) => Self::from(json),
                Err(err) => Self::from(anyhow!("{err}")),
            },
            Err(err) => Self::from(err),
        }
    }
}
impl From<anyhow::Result<(i64, Option<i64>)>> for Result {
    fn from(result: anyhow::Result<(i64, Option<i64>)>) -> Self {
        match result {
            Ok(ids) => match serde_json::to_string(&ids) {
                Ok(json) => Self::from(json),
                Err(err) => Self::from(anyhow!("{err}")),
            },
            Err(err) => Self::from(err),
        }
    }
}
impl From<anyhow::Result<Vec<(i32, i32, i32, i32, i32, i32, i32, String)>>> for Result {
    fn from(result: anyhow::Result<Vec<(i32, i32, i32, i32, i32, i32, i32, String)>>) -> Self {
        match result {
            Ok(ids) => match serde_json::to_string(&ids) {
                Ok(json) => Self::from(json),
                Err(err) => Self::from(anyhow!("{err}")),
            },
            Err(err) => Self::from(err),
        }
    }
}
impl From<anyhow::Result<i32>> for Result {
    fn from(result: anyhow::Result<i32>) -> Self {
        match result {
            Ok(id) => Self::from(format!(r#"{{ "id": "{id}" }}"#)),
            Err(err) => Self::from(err),
        }
    }
}
impl Responder for Result {
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok()
            .content_type(actix_web::http::header::ContentType::json())
            .body(self.json)
    }
}

fn env<T: std::str::FromStr>(key: &str, default: T) -> T {
    env::var(key)
        .unwrap_or_default()
        .parse::<T>()
        .unwrap_or(default)
}

fn cleanup(api: &mut Api, keep_days: u16) {
    match api.cleanup(keep_days) {
        Ok(count) => info!("Clean up performed. Deleted {count} killmails"),
        Err(err) => error!("Clean up failed: {err}"),
    }
    match api.remove_dangling_attackers() {
        Ok(count) => info!("Clean up performed. Deleted {count} attackers"),
        Err(err) => error!("Clean up failed: {err}"),
    }
    match api.remove_dangling_victims() {
        Ok(count) => info!("Clean up performed. Deleted {count} victims"),
        Err(err) => error!("Clean up failed: {err}"),
    }
}
