use actix_files::NamedFile;
use actix_web::{HttpRequest, Result, Responder, App, HttpServer, dev::Server, web::{self, Data}, body::BoxBody, HttpResponse, http::header::ContentType, get};
use serde::{Serialize, Deserialize};
use std::{path::PathBuf};

#[get("/data")]
async fn get_data(data: web::Data<ApiData>) -> impl Responder {
    let data = &data;
 
    let response = serde_json::to_string(data).unwrap();

    println!("Response: {:#?}", response);
 
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}

#[get("/{id}")]//FIXME: this is dumb and these two functions should only be one
async fn file(id: web::Path<String>) -> Result<NamedFile> {
    let path: PathBuf = format!("frontend/dist/{id}").parse().unwrap();
    println!("Generated Path: {:?}", path);
    Ok(NamedFile::open(path)?)
}

#[get("/assets/{id}")]
async fn assets(id: web::Path<String>) -> Result<NamedFile> {
    let path: PathBuf = format!("frontend/dist/assets/{id}").parse().unwrap();
    println!("Generated Path: {:?}", path);
    Ok(NamedFile::open(path)?)
}

#[derive(Serialize, Deserialize)]
pub struct ApiData {
    pub players: Vec<ApiPlayer>,
    pub world: ApiWorld,
    pub server: ApiServer,
}

impl Responder for ApiData {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> actix_web::HttpResponse {
        let res_body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
           .content_type(ContentType::json())
           .body(res_body)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiPlayer {
    pub name: String,
    pub uuid: String,
    pub ping: u16,
    pub health: f32,
    pub food: f32,
    pub x: f32,
    pub z: f32,
    pub suspicious: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ApiWorld {
    pub name: String,
    pub time: u64,
    pub day: u64,
    pub weather: String,
    pub difficulty: String,
    pub gamemode: String,
    pub chunks: Vec<(i32, i32, Vec<u8>)>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiServer {
    pub version: String,
    pub max_players: u32,
    pub render_distance: u32,
    pub ram: u64,
    pub tps: f32,
    pub cpu: f32,
    pub console: Vec<String>,
}

pub fn run_web_interface(_api_data: ApiData) -> Server {
    HttpServer::new(|| {
        App::new()
        .app_data(Data::new(ApiData {
            players: vec![ApiPlayer {
                name: "Blechdavier".to_string(),
                uuid: "a543b05eac4048d790ba2cd2e70ff169".to_string(),
                ping: 0,
                health: 20.0,
                food: 19.2,
                x: 0.0,
                z: 0.0,
                suspicious: false,
            },
            ApiPlayer {
                name: "Num3ricly".to_string(),
                uuid: "9659ce434ed94ff5848baf74c82d0a19".to_string(),
                ping: 0,
                health: 16.5,
                food: 20.0,
                x: 0.0,
                z: 0.0,
                suspicious: true,
            }],
            world: ApiWorld {
                name: "New World".to_string(),
                time: 0,
                day: 0,
                weather: "clear".to_string(),
                difficulty: "easy".to_string(),
                gamemode: "survival".to_string(),
                chunks: Vec::new(),
            },
            server: ApiServer {
                version: "1.19.2".to_string(),
                max_players: 20,
                render_distance: 10,
                ram: 2839212827,
                tps: 20.0,
                cpu: 712.3,
                console: vec!["[INFO] Starting minecraft server version 1.19.2".to_string(), "[INFO] Loading properties".to_string(), "[INFO] Default game type: SURVIVAL".to_string(), "[INFO] Generating keypair".to_string(), "[INFO] Starting Minecraft server on *:25565".to_string(), "[INFO] Using epoll channel type".to_string()],
            },
        }))
        .service(get_data)
        .service(assets)
        .service(file)
    })
    .bind(("127.0.0.1", 8080)).unwrap()
    .run()
}
