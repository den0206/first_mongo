mod api;
mod models;
mod repositry;

use actix_web::{web::Data, App, HttpServer};
use api::user_api::create_user;
use api::user_api::delete_user;
use api::user_api::get_user;
use api::user_api::update_user;
use repositry::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _db = MongoRepo::init().await;
    let _db_data = Data::new(_db);
    HttpServer::new(move || {
        App::new()
            .app_data(_db_data.clone())
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
