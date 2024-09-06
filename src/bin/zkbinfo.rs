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

    let port = env::var("ZKBINFO_PORT")
        .unwrap_or_default()
        .parse::<u16>()
        .unwrap_or(8080);
    info!("The ZKBINFO port: {port}");

    let conn = SqliteConnection::establish(&uri)?;
    let api = Api::new(conn);
    let context = web::Data::new(AppState::new(api));
    let ctx = context.clone();
    actix_rt::spawn(async move {
        let mut interval = actix_rt::time::interval(Duration::from_secs(60 * 10));
        loop {
            interval.tick().await;
            if let Ok(mut api) = ctx.api.try_lock() {
                match api.cleanup(30) {
                    Ok(count) => info!("Clean up performed. Deleted {count} killmails"),
                    Err(err) => error!("Clean up failed: {err}"),
                }
            }
        }
    });

    info!("Launching server at {host}:{port}");
    let allowed = "character|corporation|alliance|faction";
    let friends_route = format!("/friendly/{{object:{allowed}}}/for/{{subject:{allowed}}}/{{id}}");
    let enemies_route = format!("/enemy/{{object:{allowed}}}/for/{{subject:{allowed}}}/{{id}}");

    HttpServer::new(move || {
        App::new()
            .app_data(context.clone())
            .service(
                web::scope("/api")
                    .route(&friends_route, web::get().to(friends))
                    .route(&enemies_route, web::get().to(enemies)),
            )
            /*
                            .route("/statistic", web::get().to(api::statistic))
                                .route("/killmail/ids/{date}/", web::get().to(api::saved_ids))
                                .route(
                                    "/character/{id}/lost/{ship}/",
                                    web::get().to(api::character::lost_ship),
                                )
                                .route(
                                    "/corporation/{id}/lost/{ship}/",
                                    web::get().to(api::corporation::lost_ship),
                                )
                                .route(
                                    "/alliance/{id}/lost/{ship}/",
                                    web::get().to(api::alliance::lost_ship),
                                )
                                .route(
                                    "/character/activity/{id}/",
                                    web::get().to(api::character::activity),
                                )
                                .route(
                                    "/corporation/activity/{id}/",
                                    web::get().to(api::corporation::activity),
                                )
                                .route(
                                    "/alliance/activity/{id}/",
                                    web::get().to(api::alliance::activity),
                                )
                                .route(
                                    "/character/activity/hourly/{id}/",
                                    web::get().to(api::character::activity_hourly),
                                )
                                .route(
                                    "/corporation/activity/hourly/{id}/",
                                    web::get().to(api::corporation::activity_hourly),
                                )
                                .route(
                                    "/alliance/activity/hourly/{id}/",
                                    web::get().to(api::alliance::activity_hourly),
                                )
                                .route(
                                    "/character/friends/char/{id}/",
                                    web::get().to(api::character::friends_char),
                                )
                                .route(
                                    "/character/enemies/char/{id}/",
                                    web::get().to(api::character::enemies_char),
                                )
                                .route(
                                    "/character/friends/corp/{id}/",
                                    web::get().to(api::character::friends_corp),
                                )
                                .route(
                                    "/character/enemies/corp/{id}/",
                                    web::get().to(api::character::enemies_corp),
                                )
                                .route(
                                    "/character/friends/alli/{id}/",
                                    web::get().to(api::character::friends_alli),
                                )
                                .route(
                                    "/character/enemies/alli/{id}/",
                                    web::get().to(api::character::enemies_alli),
                                )
                                .route(
                                    "/corporation/friends/char/{id}/",
                                    web::get().to(api::corporation::friends_char),
                                )
                                .route(
                                    "/corporation/enemies/char/{id}/",
                                    web::get().to(api::corporation::enemies_char),
                                )
                                .route(
                                    "/corporation/friends/corp/{id}/",
                                    web::get().to(api::corporation::friends_corp),
                                )
                                .route(
                                    "/corporation/enemies/corp/{id}/",
                                    web::get().to(api::corporation::enemies_corp),
                                )
                                .route(
                                    "/corporation/friends/alli/{id}/",
                                    web::get().to(api::corporation::friends_alli),
                                )
                                .route(
                                    "/corporation/enemies/alli/{id}/",
                                    web::get().to(api::corporation::enemies_alli),
                                )
                                .route(
                                    "/alliance/friends/char/{id}/",
                                    web::get().to(api::alliance::friends_char),
                                )
                                .route(
                                    "/alliance/enemies/char/{id}/",
                                    web::get().to(api::alliance::enemies_char),
                                )
                                .route(
                                    "/alliance/friends/corp/{id}/",
                                    web::get().to(api::alliance::friends_corp),
                                )
                                .route(
                                    "/alliance/enemies/corp/{id}/",
                                    web::get().to(api::alliance::enemies_corp),
                                )
                                .route(
                                    "/alliance/friends/alli/{id}/",
                                    web::get().to(api::alliance::friends_alli),
                                )
                                .route(
                                    "/alliance/enemies/alli/{id}/",
                                    web::get().to(api::alliance::enemies_alli),
                                ),
                        )
            */
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

fn subject<S: Into<String>>(arg: S, id: u32) -> SubjectType {
    match arg.into().as_str() {
        "character" => SubjectType::Character(id),
        "corporation" => SubjectType::Corporation(id),
        "alliance" => SubjectType::Alliance(id),
        "faction" => SubjectType::Faction(id),
        _ => unreachable!(),
    }
}

async fn friends(ctx: Context, args: web::Path<(String, String, u32)>) -> impl Responder {
    let (obj, subj, id) = args.into_inner();

    info!("object: {obj}");
    info!("subject: {subj}");
    info!("id: {id}");

    let result = ctx
        .api
        .try_lock()
        .map_err(|e| anyhow!("{e}"))
        .and_then(|mut api| api.friends(subject(subj, id), object(obj)))
        .map_err(|e| anyhow!("{e}"));

    Result::from(result)
}

async fn enemies(ctx: Context, args: web::Path<(String, String, u32)>) -> impl Responder {
    let (obj, subj, id) = args.into_inner();

    info!("object: {obj}");
    info!("subject: {subj}");
    info!("id: {id}");

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
