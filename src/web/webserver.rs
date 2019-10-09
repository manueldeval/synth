use std::thread;
use std::thread::JoinHandle;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;

use crossbeam::crossbeam_channel::Sender;

use crate::synth::dsp::registry::AudioNodeRegistry;
use crate::synth::commands::systemcommand::*;
use crate::patch::manager::*;
use crate::patch::patch::*;

struct AppState {
  sender: Sender<SystemCommand>,
  patch_manager: PatchManager
}

pub struct Webserver {
  ip: String,
  port: u16,
  state: web::Data<AppState>
}

impl Webserver {
  pub fn new(ip: &str, port: u16, sender: Sender<SystemCommand>, patch_manager: PatchManager) -> Webserver {
    Webserver { 
      ip: String::from(ip), 
      port, 
      state: web::Data::new(AppState { sender, patch_manager })}
  }

  pub fn start(&self) -> JoinHandle<()> {
    let bind_ip_port = format!("{}:{}", self.ip, self.port); 
    let data = self.state.clone();
    thread::spawn(move || {
      HttpServer::new(move || {  
          App::new().register_data(data.clone())
              .route("/", web::get().to(index))
              .service(fs::Files::new("/static", "./web/").show_files_listing())
              .route("/dspnodes", web::get().to(dspnodes))
              .route("/commands", web::post().to(command))
              .route("/patches", web::get().to(get_patches))
              .route("/patches/{patch_name}", web::get().to(get_patch))
              .route("/patches/{patch_name}", web::post().to(post_patch))
      })
      .disable_signals()
      .bind(bind_ip_port)
      .unwrap()
      .run()
      .unwrap();
    })
  }
}

fn post_patch(state: web::Data<AppState>,patch_name: web::Path<String>,patch: web::Json<Patch>) -> impl Responder {
  match state.patch_manager.save_patch(&patch,&patch_name) {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError()
  }
}

fn get_patch(state: web::Data<AppState>,patch_name: web::Path<String>) -> impl Responder {
  match state.patch_manager.load_patch(&patch_name) {
    Ok(p) => HttpResponse::Ok()
              .content_type("application/json")
              .body(serde_json::to_string(&p).unwrap()),
    Err(s) => HttpResponse::InternalServerError()
              .content_type("text/plain")
              .body(s)
  }
}


fn get_patches(state: web::Data<AppState>) -> impl Responder {
  match state.patch_manager.patches() {
    Ok(p) => HttpResponse::Ok()
              .content_type("application/json")
              .body(serde_json::to_string(&p).unwrap()),
    Err(s) => HttpResponse::InternalServerError()
              .content_type("text/plain")
              .body(s)
  }
}

fn index() -> impl Responder {
    HttpResponse::Ok()
      .body(r#"
    <html>
      <head>
        <meta http-equiv="refresh" content="0;URL=./static/index.html">
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
      Ok(_)   => HttpResponse::Ok(),
      Err(_)  => HttpResponse::InternalServerError()
    }
}
