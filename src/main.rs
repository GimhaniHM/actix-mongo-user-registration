mod api;
mod models;
mod repository;

use actix_web::{web::Data, App, HttpServer};
//use api::user_api::{create_user, delete_user, get_all_users, get_user, update_user};
use api::user_api::{create_user, get_all_users, get_user};
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    println!("🚀 Server started successfully");


    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_user)
            .service(get_user)
            .service(get_all_users)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}