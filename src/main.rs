use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

struct AppState {
    ticket_list_entries: Mutex<Vec<TicketListEntry>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct TicketListEntry {
    id: u32,
    ticket_code_main: String,
    ticket_code_sub: String,
    ticket_subject: String,
    publishing_date: i64,
    publishing_text: String,
    publishing_link: String,
    publishing_subtext: String,
    on_progress: bool,
    completed: bool,
    completed_date: i64,
    completed_text: String,
    completed_link: String,
    completed_subtext: String,
}

#[get("/")]
async fn index() -> String {
    "This is the message to check if the server is running.".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        ticket_list_entries: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}