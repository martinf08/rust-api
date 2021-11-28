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
    use crate::models::Product;
    use actix_web::{http, test, web, App};
    use r2d2::Pool;
    use r2d2_sqlite3::SqliteConnectionManager;

    fn get_single_connection_pool() -> db::Pool {
        let manager = SqliteConnectionManager::file("data/webscraper.db");
        Pool::builder().max_size(1).build(manager).unwrap()
    }

    #[actix_rt::test]
    async fn test_pagination_params() {
        let pages_params = vec!["page=1", "page=2", "page=3", "page=4", "page=5"];
        let limit_params = vec!["limit=1", "limit=2", "limit=3", "limit=4", "limit=5"];

        let pool = get_single_connection_pool();

        let mut app = test::init_service(
            App::new()
                .data(pool)
                .service(web::scope("/api").configure(api::configure_api)),
        )
            .await;

        for (index, (page, limit)) in pages_params.iter().zip(limit_params.iter()).enumerate() {
            let req = test::TestRequest::get()
                .uri(&*format!("/api/products?{}&{}", page, limit))
                .to_request();
            let res = test::call_service(&mut app, req).await;

            assert_eq!(&res.status(), &http::StatusCode::OK);

            let products: Vec<Product> = test::read_body_json(res).await;

            assert_eq!(products.len(), index + 1);
        }
    }

    #[actix_rt::test]
    async fn test_products() {

        let pool = get_single_connection_pool();

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

    #[actix_rt::test]
    async fn test_pagination_pages() {
        let pages_params = vec!["page=-1000", "page=-22", "page=-1"];

        let pool = get_single_connection_pool();

        let mut app = test::init_service(
            App::new()
                .data(pool)
                .service(web::scope("/api").configure(api::configure_api)),
        )
            .await;

        for page in pages_params.iter() {
            let req = test::TestRequest::get()
                .uri(&*format!("/api/products?{}", page))
                .to_request();
            let res = test::call_service(&mut app, req).await;

            assert_eq!(&res.status(), &http::StatusCode::OK);

            let products: Vec<Product> = test::read_body_json(res).await;

            assert_eq!(products.get(0).unwrap().id, 1);
        }
    }
}
