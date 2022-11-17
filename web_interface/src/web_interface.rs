use actix_files::NamedFile;
use actix_web::{HttpRequest, Result, Responder, App, HttpServer, dev::Server, web::{self, JsonBody}, body::BoxBody, HttpResponse, http::header::ContentType};
use serde::{Serialize, Deserialize, Serializer, ser::SerializeMap};
use std::{path::PathBuf, iter::once, io::Bytes, sync::Mutex};

//FIXME: this shouldn't be two separate functions but I don't know how to do it
async fn html(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "frontend/hello.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn js(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "frontend/hello.js".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[derive(Deserialize)]
struct Download {
    name: String,
}

async fn get_data(data: web::Data<ApiData>) -> impl Responder {
    let data = data.world.lock().unwrap();
 
    let response = serde_json::to_string(&(*(data.name))).unwrap();
 
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
 }

#[derive(Serialize, Deserialize)]
pub struct ApiData {
    pub players: Mutex<Vec<Player>>,
    pub world: Mutex<World>,
}

impl Responder for ApiData {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> actix_web::HttpResponse {
        let res_body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
           .content_type(ContentType::json())
           .body(res_body)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub uuid: String,
    pub ping: u16,
}

#[derive(Serialize, Deserialize)]
pub struct World {
    pub name: String,
    pub time: u64,
    pub day: u64,
    pub weather: String,
    pub difficulty: String,
    pub gamemode: String,
    pub chunks: Vec<(i32, i32, Vec<u8>)>,
}

pub fn run_web_interface(api_data: ApiData<'static>) -> Server {
    HttpServer::new(|| {
        App::new()
            .data(&api_data)
            .service(web::resource("/").route(web::get().to(html)))
            .service(web::resource("/hello.js").route(web::get().to(js)))
            .service(web::resource("/api/tickets").route(web::get().to(get_data)))
    })
    .bind(("127.0.0.1", 8080)).unwrap()
    .run()
}
