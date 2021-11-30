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
    use std::future::Future;

    async fn run_test<F, Fut>(f: F) -> ()
    where
        F: FnOnce(db::Pool) -> Fut,
        Fut: Future<Output = ()>,
    {
        let pool = setup();

        f(pool.clone()).await;
        teardown(pool);
    }

    fn setup() -> db::Pool {
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::builder().max_size(1).build(manager).unwrap();

        let conn = pool.get().unwrap();

        conn.execute(
            r#"
CREATE TABLE IF NOT EXISTS products (
  id INTEGER PRIMARY KEY NOT NULL,
  uid VARCHAR(255) NOT NULL,
  category VARCHAR(255) NOT NULL,
  sub_category VARCHAR(255) NOT NULL,
  name VARCHAR(255) NOT NULL,
  origin VARCHAR(255) NOT NULL,
  cost INTEGER NOT NULL,
  description TEXT,
  color VARCHAR(255),
  size VARCHAR(255),
  review_count INTEGER,
  review_stars INTEGER
);
        "#,
        )
        .unwrap();

        let values = (0..500).map(|i| {
            format!("({}, '518-128', 'Computers', 'Laptops', 'ThinkPad T540p', 'https://www.webscraper.io/test-sites/e-commerce/static/product/518', 1178.989990234375, '15.6\", Core i5-4200M, 4GB, 500GB, Win7 Pro 64bit', '', '128', 2, 1)", i + 1)
        }).collect::<Vec<String>>();

        conn.execute(format!(r#"
insert into main.products (id, uid, category, sub_category, name, origin, cost, description, color, size, review_count, review_stars)
values {};
         "#, values.join(", "))).unwrap();

        pool
    }

    fn teardown(pool: db::Pool) {
        let conn = pool.get().unwrap();
        conn.execute(r#"DELETE FROM products;"#).unwrap();
    }

    #[actix_rt::test]
    async fn test_count_products() {
        run_test(|pool| async move {
            let conn = pool.get().unwrap();
            conn.iterate("SELECT count(*) as nb_products FROM products", |pairs| {
                for &(column, value) in pairs.iter() {
                    assert_eq!(column, "nb_products");
                    assert_eq!(value, Some("500"));
                }
                true
            })
            .unwrap();
        })
        .await;
    }

    #[actix_rt::test]
    async fn test_pagination_params() {
        run_test(|pool| async move {
            let pages_params = vec!["page=1", "page=2", "page=3", "page=4", "page=5"];
            let limit_params = vec!["limit=1", "limit=2", "limit=3", "limit=4", "limit=5"];

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
        })
        .await;
    }

    #[actix_rt::test]
    async fn test_products() {
        run_test(|pool| async move {
            let mut app = test::init_service(
                App::new()
                    .data(pool.clone())
                    .service(web::scope("/api").configure(api::configure_api)),
            )
            .await;

            let req = test::TestRequest::get().uri("/api/products").to_request();
            let res = test::call_service(&mut app, req).await;

            assert_eq!(res.status(), http::StatusCode::OK);
        })
        .await;
    }

    #[actix_rt::test]
    async fn test_pagination_pages() {
        run_test(|pool| async move {
            let pages_params = vec!["page=-1000", "page=-22", "page=-1"];

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
        })
        .await;
    }
}
