
mod user;

use actix_web::{get, post, error, web, App, HttpResponse, HttpServer, Responder,Error};
use user::Info;


#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[get("/users")]
async fn get_user_2(info: web::Query<Info>) -> String {
    let default_friend = "unknown".to_string();
    let friend_message = info.friend.as_ref().unwrap_or(&default_friend);
    format!("Welcome {}!", friend_message)
}

#[get("/ping")]
async fn pong() -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });
        
        App::new()
        .app_data(json_config)
        .service(pong)
        .service(index)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}