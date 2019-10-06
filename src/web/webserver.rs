use std::thread;
use std::thread::JoinHandle;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;

use crossbeam::crossbeam_channel::Sender;

use crate::synth::dsp::registry::AudioNodeRegistry;
use crate::synth::commands::systemcommand::*;

struct AppState {
  sender: Sender<SystemCommand>
}

pub struct Webserver {
  ip: String,
  port: u16,
  state: web::Data<AppState>
}

impl Webserver {
  pub fn new(ip: &str, port: u16, sender: Sender<SystemCommand>) -> Webserver {
    Webserver { 
      ip: String::from(ip), 
      port, 
      state: web::Data::new(AppState { sender })}
  }

  pub fn start(&self) -> JoinHandle<()> {
    let bind_ip_port = format!("{}:{}",self.ip,self.port); 
    let data = self.state.clone();
    thread::spawn(move || {
      HttpServer::new(move || {  
          App::new().register_data(data.clone())
              .route("/", web::get().to(index))
              .service(fs::Files::new("/static", "./web/").show_files_listing())
              .route("/dspnodes", web::get().to(dspnodes))
              .route("/commands", web::post().to(command))
      })
      .bind(bind_ip_port)
      .unwrap()
      .run()
      .unwrap();
    })
  }
}

fn index() -> impl Responder {
    HttpResponse::Ok()
      .body(r#"
    <html>
      <head>
        <meta http-equiv="refresh" content="0;URL=./static/index.html\">
      </head>
    </html>
  "#)
}

fn dspnodes() -> impl Responder {
    HttpResponse::Ok()
      .content_type("application/json")
      .body(serde_json::to_string(&AudioNodeRegistry::node_infos()).unwrap())
}

fn command(state: web::Data<AppState>,command: web::Json<SystemCommand>) -> impl Responder {
    println!("Command received: {}",command);
    match state.sender.send(command.into_inner()){
      Ok(_) => HttpResponse::Ok(),
      Err(_) =>  HttpResponse::InternalServerError()
    }
}
