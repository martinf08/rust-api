mod api;
mod db;
mod models;

use crate::db::create_pool;
use actix_web::middleware::Logger;
use actix_web::{web, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let pool = create_pool();

    HttpServer::new(move || {
        actix_web::App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .service(web::scope("/api").configure(api::configure_api))
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_pool;
    use actix_web::{http, test, web, App};

    #[actix_rt::test]
    async fn test_index() {
        let pool = create_pool();

        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .service(web::scope("/api").configure(api::configure_api)),
        )
        .await;

        let req = test::TestRequest::get().uri("/api/products").to_request();

        let res = test::call_service(&mut app, req).await;

        assert_eq!(res.status(), http::StatusCode::OK);
    }
}
