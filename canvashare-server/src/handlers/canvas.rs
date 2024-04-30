use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use std::{sync::atomic::{AtomicUsize, Ordering}, time::Instant};

use crate::models::{
    server_models::CanvasServer,
    websocket_session::WsCanvasSession
};


static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub async fn get_count(count: web::Data<AtomicUsize>) -> impl Responder {
    let curr_count = count.load(Ordering::SeqCst);
    format!("Visitors: {}", curr_count)
}

/// Entry point for canvas websocket route
pub async fn canvas_route(req: HttpRequest, stream: web::Payload, src: web::Data<Addr<CanvasServer>>) -> Result<HttpResponse, Error> {
    ws::start(
        WsCanvasSession {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            hb: Instant::now(),
            room: "main".to_owned(), // TODO: using a single room for now
            name: None,
            addr: src.get_ref().clone()
        }, 
        &req, 
        stream
    )
}