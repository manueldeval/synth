use std::thread;
use std::thread::JoinHandle;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use crate::synth::dsp::registry::AudioNodeRegistry;
use crate::synth::commands::systemcommand::*;

fn index() -> impl Responder {
    HttpResponse::Ok()
      .body("<html><head><meta http-equiv=\"refresh\" content=\"0;URL=./static/index.html\"></head></html>")
}

fn dspnodes() -> impl Responder {
    HttpResponse::Ok()
      .content_type("application/json")
      .body(serde_json::to_string(&AudioNodeRegistry::node_infos()).unwrap())
}

fn command(command: web::Json<SystemCommand>) -> impl Responder {
    println!("Command received: {}",command);
    HttpResponse::Ok()
}

pub fn start_web_server() -> JoinHandle<()> {
  thread::spawn(move || {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(fs::Files::new("/static", "./web/").show_files_listing())
            .route("/dspnodes", web::get().to(dspnodes))
            .route("/commands", web::post().to(command))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
  })
}