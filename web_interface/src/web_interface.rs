use std::sync::Arc;

use actix::{Actor, StreamHandler};
use actix_web::{HttpRequest, Result, Responder, App, HttpServer, dev::Server, web::{self, Data}, body::BoxBody, HttpResponse, Error, http::header::{ContentType, ContentDisposition, DispositionType}, get};
use serde::{Serialize, Deserialize};
use actix_web_actors::ws;
use actix_files as afs;

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

struct MyWs {
    data: Data<ApiData>
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&self.data).unwrap());
    }
}

async fn ws(req: HttpRequest, stream: web::Payload, data: web::Data<ApiData>) -> Result<HttpResponse, Error> {
    println!("received ws request {}", req.path());
    let resp = ws::start(MyWs {data}, &req, stream);
    println!("{:?}", resp);
    resp
}

pub fn web_interface() -> actix_web::dev::Server {
    let data = ApiData {
        players: Vec::new(),
        world: ApiWorld {
            name: "undefined".to_string(),
            time: 0,
            day: 0,
            weather: "undefined".to_string(),
            difficulty: "undefined".to_string(),
            gamemode: "undefined".to_string(),
            chunks: Vec::new(),
        },
        server: ApiServer {
            version: "undefined".to_string(),
            max_players: 0,
            render_distance: 0,
            ram: 0,
            tps: 0.0,
            cpu: 0.0,
            console: vec!["undefined".to_string()],
        },
    };
    let data_arc = Arc::new(data);

    let thread_data = Data::from(data_arc);

    HttpServer::new(move || App::new()
    .app_data(Data::clone(&thread_data))
    .route("/ws", web::get().to(ws))
    .service( afs::Files::new("/", "./frontend/dist/")
    .show_files_listing()
    .use_last_modified(true)))
        .bind(("127.0.0.1", 8080)).unwrap()
        .run()
}