use actix_cors::Cors;
use actix_web::{http, App, HttpServer};
use actix_web::web::Data;
use std::sync::Mutex;
use std::io;

mod api;
use api::{vote, results, AppState};

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Create and share the app state
    let app_state = Data::new(AppState {
        votes: Mutex::new(Vec::new()),
    });

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone()) // Share the state with the app
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE])
                    .max_age(3600),
            )
            .service(vote) // Vote endpoint
            .service(results) // Results endpoint
    })
    .bind("0.0.0.0:8080")? // Bind the server to localhost on port 8080
    .run()
    .await
}
