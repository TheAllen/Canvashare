use actix::*;
use actix_cors::Cors;
use actix_web::{http::header, App, web, HttpServer};
use std::sync::{atomic::AtomicUsize, Arc};

mod handlers;
mod models;
mod state;

use handlers::canvas::{canvas_route, get_count};
use models::server_models::CanvasServer;
use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    
    let app_shared_state = AppState {
        visitor_count: Arc::new(AtomicUsize::new(0))
    };
  
    let canvas_server = CanvasServer::new(app_shared_state.visitor_count.clone()).start();
    let local_env = ("127.0.0.1", 8080);
    
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "PATCH", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600)
            )
            .app_data(web::Data::from(app_shared_state.visitor_count.clone()))
            .app_data(web::Data::new(canvas_server.clone()))
            // .service(web::resource("/").to(home_page)) // TODO
            .route("/count", web::get().to(get_count))
            .route("/ws/", web::get().to(canvas_route))
    })
    .workers(2)
    .bind(local_env)?
    .run()
    .await

}
