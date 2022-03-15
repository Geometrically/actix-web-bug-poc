use actix_multipart::Multipart;
use actix_web::{post, middleware, web, HttpResponse, HttpServer, App};
use futures_util::stream::StreamExt as _;

#[post("/")]
async fn index(mut payload: Multipart) -> HttpResponse {
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();

        while let Some(chunk) = field.next().await {
            return HttpResponse::Ok().body("this does not show up :(");
        }
    }

    HttpResponse::Ok().body("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default().log_target("http_log"))
            .service(index)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}