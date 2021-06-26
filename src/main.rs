use actix_web::{App, HttpServer};
use todo_server::todo_api::db::helpers;
use todo_server::todo_api_web::routes::apps_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    helpers::create_table();

    HttpServer::new(|| App::new().configure(apps_routes))
        .workers(num_cpus::get() + 2)
        .bind("127.0.0.1:4000")?
        .run()
        .await
}
