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
    handlebars.register_template_file("corporation", "./public/templates/corporation.html")?;
    handlebars.register_template_file("alliance", "./public/templates/alliance.html")?;
    handlebars.register_template_file("lost_ship", "./public/templates/lost_ship.html")?;

    let context = web::Data::new(handlebars);

    info!("Try http://{host}:{port}/who/");
    HttpServer::new(move || {
        App::new()
            .app_data(context.clone())
            .service(Files::new("/css", "./public/css").show_files_listing())
            .service(Files::new("/js", "./public/js").show_files_listing())
            .service(favicon)
            .service(who)
            .service(report)
            .service(character)
            .service(corporation)
            .service(alliance)
            .service(character_lost_ships)
            .service(corporation_lost_ships)
            .service(alliance_lost_ships)
            .service(character_lost_in_system)
            .service(corporation_lost_in_system)
            .service(alliance_lost_in_system)
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

#[get("/corporation/{id}/")]
async fn corporation(ctx: Context<'_>, args: web::Path<i32>) -> impl Responder {
    let id = args.into_inner();

    info!("/corporation/{id}/");
    Result::from(ctx.render("corporation", &Id::from(id)))
}

#[get("/alliance/{id}/")]
async fn alliance(ctx: Context<'_>, args: web::Path<i32>) -> impl Responder {
    let id = args.into_inner();

    info!("/alliance/{id}/");
    Result::from(ctx.render("alliance", &Id::from(id)))
}

#[get("/lost/ship/{sid}/character/{id}/")]
async fn character_lost_ships(ctx: Context<'_>, args: web::Path<(i32, i32)>) -> impl Responder {
    let (sid, id) = args.into_inner();

    info!("/lost/ship/{sid}/character/{id}/");
    Result::from(ctx.render("lost_ship", &Lost::from("ship", sid, "character", id)))
}

#[get("/lost/ship/{sid}/corporation/{id}/")]
async fn corporation_lost_ships(ctx: Context<'_>, args: web::Path<(i32, i32)>) -> impl Responder {
    let (sid, id) = args.into_inner();

    info!("/lost/ship/{sid}/corporation/{id}/");
    Result::from(ctx.render("lost_ship", &Lost::from("ship", sid, "corporation", id)))
}

#[get("/lost/ship/{sid}/alliance/{id}/")]
async fn alliance_lost_ships(ctx: Context<'_>, args: web::Path<(i32, i32)>) -> impl Responder {
    let (sid, id) = args.into_inner();

    info!("/lost/ship/{sid}/alliance/{id}/");
    Result::from(ctx.render("lost_ship", &Lost::from("ship", sid, "alliance", id)))
}

#[get("/lost/system/{sid}/character/{id}/")]
async fn character_lost_in_system(ctx: Context<'_>, args: web::Path<(i32, i32)>) -> impl Responder {
    let (sid, id) = args.into_inner();

    info!("/lost/ship/{sid}/character/{id}/");
    Result::from(ctx.render("lost_ship", &Lost::from("system", sid, "character", id)))
}

#[get("/lost/system/{sid}/corporation/{id}/")]
async fn corporation_lost_in_system(ctx: Context<'_>, args: web::Path<(i32, i32)>) -> impl Responder {
    let (sid, id) = args.into_inner();

    info!("/lost/ship/{sid}/corporation/{id}/");
    Result::from(ctx.render("lost_ship", &Lost::from("system", sid, "corporation", id)))
}

#[get("/lost/system/{sid}/alliance/{id}/")]
async fn alliance_lost_in_system(ctx: Context<'_>, args: web::Path<(i32, i32)>) -> impl Responder {
    let (sid, id) = args.into_inner();

    info!("/lost/ship/{sid}/alliance/{id}/");
    Result::from(ctx.render("lost_ship", &Lost::from("system", sid, "alliance", id)))
}

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

#[derive(Deserialize, Serialize, Debug)]
pub struct Lost {
    area: String,
    aid: i32,
    subject: String,
    sid: i32,
}
impl Lost {
    pub fn from<S: Into<String>>(area: S, aid: i32, subject: S, sid: i32) -> Self {
        Self {
            area: area.into(),
            aid,
            subject: subject.into(),
            sid,
        }
    }
}
