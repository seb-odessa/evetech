use actix_files::Files;
use actix_files::NamedFile;
use actix_web::HttpRequest;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::anyhow;

use handlebars::Handlebars;
use log::{error, info};
use serde::{Deserialize, Serialize};

// use std::collections::HashMap;
use std::env;

pub type Context<'a> = web::Data<Handlebars<'a>>;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let host = env::var("ZKBGUI_HOST").unwrap_or(String::from("localhost"));
    let port = env::var("ZKBGUI_PORT")
        .unwrap_or_default()
        .parse::<u16>()
        .unwrap_or(8088);

    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("who", "./public/templates/who.html")?;
    handlebars.register_template_file("report", "./public/templates/report.html")?;
    handlebars.register_template_file("character", "./public/templates/character.html")?;

    let context = web::Data::new(handlebars);

    info!("Try http://{host}:{port}/");
    HttpServer::new(move || {
        App::new()
            .app_data(context.clone())
            .service(Files::new("/css", "./public/css").show_files_listing())
            .service(Files::new("/js", "./public/js").show_files_listing())
            .service(favicon)
            .service(who)
            .service(report)
            .service(character)
        // .service(report_by_id)
        // .service(lost_ships)
    })
    .workers(2)
    .bind((host.as_str(), port))?
    .run()
    .await
    .map_err(|e| anyhow!(e))
}

#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    NamedFile::open_async("./public/favicon.ico").await
}

#[get("/who/")]
async fn who(ctx: Context<'_>) -> impl Responder {
    info!("/who/");
    Result::from(ctx.render("who", &{}))
}

#[post("/who/report/")]
async fn report(ctx: Context<'_>, query: web::Form<Who>) -> impl Responder {
    let args: Characters = query.into_inner().into();

    info!("/who/report/ <- {:?}", args);
    Result::from(ctx.render("report", &args))
}

#[get("/character/{id}/")]
async fn character(ctx: Context<'_>, args: web::Path<i32>) -> impl Responder {
    let id = args.into_inner();

    info!("/character/{id}/");
    Result::from(ctx.render("character", &Id::from(id)))
}

/*
#[get("/gui/{target}/{name}/")]
async fn report(ctx: Context<'_>, path: web::Path<(String, String)>) -> HttpResponse {
    let (target, name) = path.into_inner();
    let body = match target.as_str() {
        "character" => match CharacterProps::named(name).await {
            Ok(prop) => wrapper(ctx, "character", &prop),
            Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
        },
        "corporation" => match CorporationProps::named(name).await {
            Ok(prop) => wrapper(ctx, "corporation", &prop),
            Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
        },
        "alliance" => match AllianceProps::named(name).await {
            Ok(prop) => wrapper(ctx, "alliance", &prop),
            Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
        },
        _ => wrapper(ctx, "error", &Error::from(format!("Unknown Target"))),
    };
    HttpResponse::Ok().body(body)
}

#[get("/gui/{target}/id/{id}/")]
async fn report_by_id(ctx: Context<'_>, path: web::Path<(String, i32)>) -> HttpResponse {
    let (target, id) = path.into_inner();
    let body = match target.as_str() {
        "character" => match CharacterProps::from(id).await {
            Ok(prop) => wrapper(ctx, "character", &prop),
            Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
        },
        "corporation" => match CorporationProps::from(id).await {
            Ok(prop) => wrapper(ctx, "corporation", &prop),
            Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
        },
        "alliance" => match AllianceProps::from(id).await {
            Ok(prop) => wrapper(ctx, "alliance", &prop),
            Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
        },
        _ => wrapper(ctx, "error", &Error::from(format!("Unknown Target"))),
    };
    HttpResponse::Ok().body(body)
}

#[get("/gui/{target}/{name}/lost/{ship}/")]
async fn lost_ships(ctx: Context<'_>, path: web::Path<(String, i32, i32)>) -> HttpResponse {
    let (target, id, ship_id) = path.into_inner();
    let body = match target.as_str() {
        "character" => match LostProps::from(id, ship_id, SearchCategory::Character).await {
            Ok(prop) => wrapper(ctx, "losts", &prop),
            Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
        },
        "corporation" => match LostProps::from(id, ship_id, SearchCategory::Corporation).await {
            Ok(prop) => wrapper(ctx, "losts", &prop),
            Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
        },
        "alliance" => match LostProps::from(id, ship_id, SearchCategory::Alliance).await {
            Ok(prop) => wrapper(ctx, "losts", &prop),
            Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
        },
        _ => wrapper(ctx, "error", &Error::from(format!("Unknown Target"))),
    };

    HttpResponse::Ok().body(body)
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
struct Error {
    error: String,
}
impl Error {
    pub fn from(error: String) -> Self {
        Error { error }
    }
}
*/

// fn wrapper<T: Serialize>(ctx: Context<'_>, template: &str, obj: &T) -> String {
//     match ctx.render(template, &obj) {
//         Ok(ok_body) => ok_body,
//         Err(what) => {
//             error!("{what}");
//             format!("{what}")
//         }
//     }
// }

pub struct Result {
    content: String,
}
impl From<String> for Result {
    fn from(content: String) -> Self {
        Self { content }
    }
}
impl From<handlebars::RenderError> for Result {
    fn from(err: handlebars::RenderError) -> Self {
        let content = format!("error: {err}");
        error!("{content}");
        Self { content }
    }
}
impl From<std::result::Result<String, handlebars::RenderError>> for Result {
    fn from(result: std::result::Result<String, handlebars::RenderError>) -> Self {
        match result {
            Ok(content) => Self::from(content),
            Err(err) => Self::from(err),
        }
    }
}
impl Responder for Result {
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok()
            .content_type(actix_web::http::header::ContentType::html())
            .body(self.content)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Who {
    names: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Characters {
    names: Vec<String>,
}
impl From<Who> for Characters {
    fn from(other: Who) -> Self {
        Self {
            names: other
                .names
                .split("\r\n")
                .map(|name| String::from(name.trim()))
                .filter(|name| !name.is_empty())
                .collect(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Id {
    id: i32,
}
impl Id {
    pub fn from(id: i32) -> Self {
        Self { id }
    }
}
