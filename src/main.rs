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
    use std::sync::atomic::{AtomicBool, Ordering};
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INIT: AtomicBool = AtomicBool::new(true);
    }

    async fn run_test<F, Fut>(f: F) -> ()
    where
        F: FnOnce(db::Pool) -> Fut,
        Fut: Future<Output = ()>,
    {
        let pool = setup();

        f(pool).await;
        teardown();
    }

    fn setup() -> db::Pool {
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::builder().max_size(1).build(manager).unwrap();

        let conn = pool.get().unwrap();
        if INIT.load(Ordering::Relaxed){
            INIT.swap(false, Ordering::Relaxed);

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

insert into main.products (id, uid, category, sub_category, name, origin, cost, description, color, size, review_count, review_stars)
values  (1, '518-128', 'Computers', 'Laptops', 'ThinkPad T540p', 'https://www.webscraper.io/test-sites/e-commerce/static/product/518', 1178.989990234375, '15.6", Core i5-4200M, 4GB, 500GB, Win7 Pro 64bit', '', '128', 2, 1),
        (2, '518-256', 'Computers', 'Laptops', 'ThinkPad T540p', 'https://www.webscraper.io/test-sites/e-commerce/static/product/518', 1178.989990234375, '15.6", Core i5-4200M, 4GB, 500GB, Win7 Pro 64bit', '', '256', 2, 1),
        (3, '516-512', 'Computers', 'Laptops', 'Packard 255 G2', 'https://www.webscraper.io/test-sites/e-commerce/static/product/516', 416.989990234375, '15.6", AMD E2-3800 1.3GHz, 4GB, 500GB, Windows 8.1', '', '512', 2, 2),
        (4, '516-128', 'Computers', 'Laptops', 'Packard 255 G2', 'https://www.webscraper.io/test-sites/e-commerce/static/product/516', 416.989990234375, '15.6", AMD E2-3800 1.3GHz, 4GB, 500GB, Windows 8.1', '', '128', 2, 2),
        (5, '519-256', 'Computers', 'Laptops', 'ProBook', 'https://www.webscraper.io/test-sites/e-commerce/static/product/519', 739.989990234375, '14", Core i5 2.6GHz, 4GB, 500GB, Win7 Pro 64bit', '', '256', 8, 4),
        (6, '519-512', 'Computers', 'Laptops', 'ProBook', 'https://www.webscraper.io/test-sites/e-commerce/static/product/519', 739.989990234375, '14", Core i5 2.6GHz, 4GB, 500GB, Win7 Pro 64bit', '', '512', 8, 4),
        (7, '517-128', 'Computers', 'Laptops', 'Aspire E1-510', 'https://www.webscraper.io/test-sites/e-commerce/static/product/517', 306.989990234375, '15.6", Pentium N3520 2.16GHz, 4GB, 500GB, Linux', '', '128', 2, 3),
        (8, '516-256', 'Computers', 'Laptops', 'Packard 255 G2', 'https://www.webscraper.io/test-sites/e-commerce/static/product/516', 416.989990234375, '15.6", AMD E2-3800 1.3GHz, 4GB, 500GB, Windows 8.1', '', '256', 2, 2),
        (9, '517-256', 'Computers', 'Laptops', 'Aspire E1-510', 'https://www.webscraper.io/test-sites/e-commerce/static/product/517', 306.989990234375, '15.6", Pentium N3520 2.16GHz, 4GB, 500GB, Linux', '', '256', 2, 3),
        (10, '519-128', 'Computers', 'Laptops', 'ProBook', 'https://www.webscraper.io/test-sites/e-commerce/static/product/519', 739.989990234375, '14", Core i5 2.6GHz, 4GB, 500GB, Win7 Pro 64bit', '', '128', 8, 4),
        (11, '518-512', 'Computers', 'Laptops', 'ThinkPad T540p', 'https://www.webscraper.io/test-sites/e-commerce/static/product/518', 1178.989990234375, '15.6", Core i5-4200M, 4GB, 500GB, Win7 Pro 64bit', '', '512', 2, 1),
        (12, '517-512', 'Computers', 'Laptops', 'Aspire E1-510', 'https://www.webscraper.io/test-sites/e-commerce/static/product/517', 306.989990234375, '15.6", Pentium N3520 2.16GHz, 4GB, 500GB, Linux', '', '512', 2, 3),
        (13, '520-128', 'Computers', 'Laptops', 'ThinkPad X240', 'https://www.webscraper.io/test-sites/e-commerce/static/product/520', 1311.989990234375, '12.5", Core i5-4300U, 8GB, 240GB SSD, Win7 Pro 64bit', '', '128', 12, 3),
        (14, '520-256', 'Computers', 'Laptops', 'ThinkPad X240', 'https://www.webscraper.io/test-sites/e-commerce/static/product/520', 1311.989990234375, '12.5", Core i5-4300U, 8GB, 240GB SSD, Win7 Pro 64bit', '', '256', 12, 3),
        (15, '520-512', 'Computers', 'Laptops', 'ThinkPad X240', 'https://www.webscraper.io/test-sites/e-commerce/static/product/520', 1311.989990234375, '12.5", Core i5-4300U, 8GB, 240GB SSD, Win7 Pro 64bit', '', '512', 12, 3),
        (16, '523-128', 'Computers', 'Laptops', 'Pavilion', 'https://www.webscraper.io/test-sites/e-commerce/static/product/523', 609.989990234375, '15.6", Core i5-4200U, 6GB, 750GB, Windows 8.1', '', '128', 4, 1),
        (17, '521-128', 'Computers', 'Laptops', 'Aspire E1-572G', 'https://www.webscraper.io/test-sites/e-commerce/static/product/521', 581.989990234375, '15.6", Core i5-4200U, 8GB, 1TB, Radeon R7 M265, Windows 8.1', '', '128', 2, 1),
        (18, '523-256', 'Computers', 'Laptops', 'Pavilion', 'https://www.webscraper.io/test-sites/e-commerce/static/product/523', 609.989990234375, '15.6", Core i5-4200U, 6GB, 750GB, Windows 8.1', '', '256', 4, 1),
        (19, '521-256', 'Computers', 'Laptops', 'Aspire E1-572G', 'https://www.webscraper.io/test-sites/e-commerce/static/product/521', 581.989990234375, '15.6", Core i5-4200U, 8GB, 1TB, Radeon R7 M265, Windows 8.1', '', '256', 2, 1),
        (20, '521-512', 'Computers', 'Laptops', 'Aspire E1-572G', 'https://www.webscraper.io/test-sites/e-commerce/static/product/521', 581.989990234375, '15.6", Core i5-4200U, 8GB, 1TB, Radeon R7 M265, Windows 8.1', '', '512', 2, 1),
        (21, '522-128', 'Computers', 'Laptops', 'ThinkPad Yoga', 'https://www.webscraper.io/test-sites/e-commerce/static/product/522', 1033.989990234375, '12.5" Touch, Core i3-4010U, 4GB, 500GB + 16GB SSD Cache,', '', '128', 13, 2),
        (22, '522-256', 'Computers', 'Laptops', 'ThinkPad Yoga', 'https://www.webscraper.io/test-sites/e-commerce/static/product/522', 1033.989990234375, '12.5" Touch, Core i3-4010U, 4GB, 500GB + 16GB SSD Cache,', '', '256', 13, 2),
        (23, '523-512', 'Computers', 'Laptops', 'Pavilion', 'https://www.webscraper.io/test-sites/e-commerce/static/product/523', 609.989990234375, '15.6", Core i5-4200U, 6GB, 750GB, Windows 8.1', '', '512', 4, 1),
        (24, '522-512', 'Computers', 'Laptops', 'ThinkPad Yoga', 'https://www.webscraper.io/test-sites/e-commerce/static/product/522', 1033.989990234375, '12.5" Touch, Core i3-4010U, 4GB, 500GB + 16GB SSD Cache,', '', '512', 13, 2),
        (25, '524-512', 'Computers', 'Laptops', 'Inspiron 15', 'https://www.webscraper.io/test-sites/e-commerce/static/product/524', 745.989990234375, 'Moon Silver, 15.6", Core i7-4510U, 8GB, 1TB, Radeon HD R7 M265 2GB,', '', '512', 12, 3),
        (26, '524-128', 'Computers', 'Laptops', 'Inspiron 15', 'https://www.webscraper.io/test-sites/e-commerce/static/product/524', 745.989990234375, 'Moon Silver, 15.6", Core i7-4510U, 8GB, 1TB, Radeon HD R7 M265 2GB,', '', '128', 12, 3),
        (27, '524-256', 'Computers', 'Laptops', 'Inspiron 15', 'https://www.webscraper.io/test-sites/e-commerce/static/product/524', 745.989990234375, 'Moon Silver, 15.6", Core i7-4510U, 8GB, 1TB, Radeon HD R7 M265 2GB,', '', '256', 12, 3),
        (28, '525-128', 'Computers', 'Laptops', 'Dell XPS 13', 'https://www.webscraper.io/test-sites/e-commerce/static/product/525', 1281.989990234375, '13.3" Touch, Core i5-4210U, 8GB, 128GB SSD, Windows 8.1', '', '128', 4, 3),
        (29, '525-256', 'Computers', 'Laptops', 'Dell XPS 13', 'https://www.webscraper.io/test-sites/e-commerce/static/product/525', 1281.989990234375, '13.3" Touch, Core i5-4210U, 8GB, 128GB SSD, Windows 8.1', '', '256', 4, 3),
        (30, '525-512', 'Computers', 'Laptops', 'Dell XPS 13', 'https://www.webscraper.io/test-sites/e-commerce/static/product/525', 1281.989990234375, '13.3" Touch, Core i5-4210U, 8GB, 128GB SSD, Windows 8.1', '', '512', 4, 3),
        (31, '526-512', 'Computers', 'Laptops', 'ThinkPad X230', 'https://www.webscraper.io/test-sites/e-commerce/static/product/526', 1244.989990234375, '12.5", Core i5 2.6GHz, 8GB, 180GB SSD, Win7 Pro 64bit', '', '512', 10, 3),
        (32, '527-128', 'Computers', 'Laptops', 'HP 250 G3', 'https://www.webscraper.io/test-sites/e-commerce/static/product/527', 520.989990234375, '15.6", Core i5-4210U, 4GB, 500GB, Windows 8.1', '', '128', 13, 2),
        (33, '527-512', 'Computers', 'Laptops', 'HP 250 G3', 'https://www.webscraper.io/test-sites/e-commerce/static/product/527', 520.989990234375, '15.6", Core i5-4210U, 4GB, 500GB, Windows 8.1', '', '512', 13, 2),
        (34, '527-256', 'Computers', 'Laptops', 'HP 250 G3', 'https://www.webscraper.io/test-sites/e-commerce/static/product/527', 520.989990234375, '15.6", Core i5-4210U, 4GB, 500GB, Windows 8.1', '', '256', 13, 2),
        (35, '526-256', 'Computers', 'Laptops', 'ThinkPad X230', 'https://www.webscraper.io/test-sites/e-commerce/static/product/526', 1244.989990234375, '12.5", Core i5 2.6GHz, 8GB, 180GB SSD, Win7 Pro 64bit', '', '256', 10, 3),
        (36, '526-128', 'Computers', 'Laptops', 'ThinkPad X230', 'https://www.webscraper.io/test-sites/e-commerce/static/product/526', 1244.989990234375, '12.5", Core i5 2.6GHz, 8GB, 180GB SSD, Win7 Pro 64bit', '', '128', 10, 3),
        (37, '528-128', 'Computers', 'Laptops', 'ThinkPad Yoga', 'https://www.webscraper.io/test-sites/e-commerce/static/product/528', 1223.989990234375, '12.5" Touch, Core i5 4200U, 8GB, 500GB + 16GB SSD Cache, Windows', '', '128', 2, 3),
        (38, '528-256', 'Computers', 'Laptops', 'ThinkPad Yoga', 'https://www.webscraper.io/test-sites/e-commerce/static/product/528', 1223.989990234375, '12.5" Touch, Core i5 4200U, 8GB, 500GB + 16GB SSD Cache, Windows', '', '256', 2, 3),
        (39, '528-512', 'Computers', 'Laptops', 'ThinkPad Yoga', 'https://www.webscraper.io/test-sites/e-commerce/static/product/528', 1223.989990234375, '12.5" Touch, Core i5 4200U, 8GB, 500GB + 16GB SSD Cache, Windows', '', '512', 2, 3),
        (40, '529-256', 'Computers', 'Laptops', 'HP 350 G1', 'https://www.webscraper.io/test-sites/e-commerce/static/product/529', 577.989990234375, '15.6", Core i5-4200U, 4GB, 750GB, Radeon HD8670M 2GB, Windows', '', '256', 10, 2),
        (41, '529-128', 'Computers', 'Laptops', 'HP 350 G1', 'https://www.webscraper.io/test-sites/e-commerce/static/product/529', 577.989990234375, '15.6", Core i5-4200U, 4GB, 750GB, Radeon HD8670M 2GB, Windows', '', '128', 10, 2),
        (42, '531-512', 'Computers', 'Laptops', 'Dell Vostro 15', 'https://www.webscraper.io/test-sites/e-commerce/static/product/531', 488.7799987792969, 'Dell Vostro 15 (3568) Black, 15.6" FHD, Core i5-7200U, 4GB, 128GB SSD, Radeon R5 M420 2GB, Linux', '', '512', 14, 4),
        (43, '531-128', 'Computers', 'Laptops', 'Dell Vostro 15', 'https://www.webscraper.io/test-sites/e-commerce/static/product/531', 488.7799987792969, 'Dell Vostro 15 (3568) Black, 15.6" FHD, Core i5-7200U, 4GB, 128GB SSD, Radeon R5 M420 2GB, Linux', '', '128', 14, 4),
        (44, '531-256', 'Computers', 'Laptops', 'Dell Vostro 15', 'https://www.webscraper.io/test-sites/e-commerce/static/product/531', 488.7799987792969, 'Dell Vostro 15 (3568) Black, 15.6" FHD, Core i5-7200U, 4GB, 128GB SSD, Radeon R5 M420 2GB, Linux', '', '256', 14, 4),
        (45, '529-512', 'Computers', 'Laptops', 'HP 350 G1', 'https://www.webscraper.io/test-sites/e-commerce/static/product/529', 577.989990234375, '15.6", Core i5-4200U, 4GB, 750GB, Radeon HD8670M 2GB, Windows', '', '512', 10, 2),
        (46, '530-128', 'Computers', 'Laptops', 'Asus VivoBook Max', 'https://www.webscraper.io/test-sites/e-commerce/static/product/530', 399, 'Asus VivoBook Max X541NA-GQ041 Black Chocolate, 15.6" HD, Pentium N4200 1.1GHz, 4GB, 500GB, Windows 10 Home', '', '128', 4, 1),
        (47, '530-256', 'Computers', 'Laptops', 'Asus VivoBook Max', 'https://www.webscraper.io/test-sites/e-commerce/static/product/530', 399, 'Asus VivoBook Max X541NA-GQ041 Black Chocolate, 15.6" HD, Pentium N4200 1.1GHz, 4GB, 500GB, Windows 10 Home', '', '256', 4, 1),
        (48, '530-512', 'Computers', 'Laptops', 'Asus VivoBook Max', 'https://www.webscraper.io/test-sites/e-commerce/static/product/530', 399, 'Asus VivoBook Max X541NA-GQ041 Black Chocolate, 15.6" HD, Pentium N4200 1.1GHz, 4GB, 500GB, Windows 10 Home', '', '512', 4, 1),
        (49, '532-128', 'Computers', 'Laptops', 'Acer Spin 5', 'https://www.webscraper.io/test-sites/e-commerce/static/product/532', 564.97998046875, 'Acer Spin 5 SP513-51 Black, 13.3" FHD Touch, Core i3-7100U, 4GB, 128GB SSD, Windows 10 Home', '', '128', 0, 2),
        (50, '532-256', 'Computers', 'Laptops', 'Acer Spin 5', 'https://www.webscraper.io/test-sites/e-commerce/static/product/532', 564.97998046875, 'Acer Spin 5 SP513-51 Black, 13.3" FHD Touch, Core i3-7100U, 4GB, 128GB SSD, Windows 10 Home', '', '256', 0, 2),
        (51, '532-512', 'Computers', 'Laptops', 'Acer Spin 5', 'https://www.webscraper.io/test-sites/e-commerce/static/product/532', 564.97998046875, 'Acer Spin 5 SP513-51 Black, 13.3" FHD Touch, Core i3-7100U, 4GB, 128GB SSD, Windows 10 Home', '', '512', 0, 2),
        (52, '533-128', 'Computers', 'Laptops', 'Acer Aspire A515-51-5654', 'https://www.webscraper.io/test-sites/e-commerce/static/product/533', 679, 'Acer Aspire A515-51-5654, Black, 15.6", FHD, Core i5-8250U, 8GB DDR4, 256GB SSD, Windows 10 Home, ENG', '', '128', 9, 2),
        (53, '533-256', 'Computers', 'Laptops', 'Acer Aspire A515-51-5654', 'https://www.webscraper.io/test-sites/e-commerce/static/product/533', 679, 'Acer Aspire A515-51-5654, Black, 15.6", FHD, Core i5-8250U, 8GB DDR4, 256GB SSD, Windows 10 Home, ENG', '', '256', 9, 2),
        (54, '533-512', 'Computers', 'Laptops', 'Acer Aspire A515-51-5654', 'https://www.webscraper.io/test-sites/e-commerce/static/product/533', 679, 'Acer Aspire A515-51-5654, Black, 15.6", FHD, Core i5-8250U, 8GB DDR4, 256GB SSD, Windows 10 Home, ENG', '', '512', 9, 2),
        (55, '534-128', 'Computers', 'Laptops', 'Dell Inspiron 15', 'https://www.webscraper.io/test-sites/e-commerce/static/product/534', 679, 'Dell Inspiron 15 (5567) Fog Gray, 15.6" FHD, Core i5-7200U, 8GB, 1TB, Radeon R7 M445 4GB, Linux', '', '128', 7, 2),
        (56, '534-256', 'Computers', 'Laptops', 'Dell Inspiron 15', 'https://www.webscraper.io/test-sites/e-commerce/static/product/534', 679, 'Dell Inspiron 15 (5567) Fog Gray, 15.6" FHD, Core i5-7200U, 8GB, 1TB, Radeon R7 M445 4GB, Linux', '', '256', 7, 2),
        (57, '534-512', 'Computers', 'Laptops', 'Dell Inspiron 15', 'https://www.webscraper.io/test-sites/e-commerce/static/product/534', 679, 'Dell Inspiron 15 (5567) Fog Gray, 15.6" FHD, Core i5-7200U, 8GB, 1TB, Radeon R7 M445 4GB, Linux', '', '512', 7, 2),
        (58, '535-128', 'Computers', 'Laptops', 'Asus VivoBook S14', 'https://www.webscraper.io/test-sites/e-commerce/static/product/535', 729, 'Asus VivoBook S14 (S406UA-BV041T) Starry Grey, 14", Core i5-8250U, 8GB, 256GB SSD, Windows 10 Home, Eng kbd', '', '128', 2, 1),
        (59, '535-256', 'Computers', 'Laptops', 'Asus VivoBook S14', 'https://www.webscraper.io/test-sites/e-commerce/static/product/535', 729, 'Asus VivoBook S14 (S406UA-BV041T) Starry Grey, 14", Core i5-8250U, 8GB, 256GB SSD, Windows 10 Home, Eng kbd', '', '256', 2, 1),
        (60, '535-512', 'Computers', 'Laptops', 'Asus VivoBook S14', 'https://www.webscraper.io/test-sites/e-commerce/static/product/535', 729, 'Asus VivoBook S14 (S406UA-BV041T) Starry Grey, 14", Core i5-8250U, 8GB, 256GB SSD, Windows 10 Home, Eng kbd', '', '512', 2, 1),
        (61, '536-128', 'Computers', 'Laptops', 'Asus ROG STRIX GL553VD-DM256', 'https://www.webscraper.io/test-sites/e-commerce/static/product/536', 799, 'Asus ROG STRIX GL553VD-DM256, 15.6" FHD, Core i5-7300HQ, 8GB, 1TB, GeForce GTX 1050 2GB, No OS', '', '128', 7, 2),
        (62, '536-256', 'Computers', 'Laptops', 'Asus ROG STRIX GL553VD-DM256', 'https://www.webscraper.io/test-sites/e-commerce/static/product/536', 799, 'Asus ROG STRIX GL553VD-DM256, 15.6" FHD, Core i5-7300HQ, 8GB, 1TB, GeForce GTX 1050 2GB, No OS', '', '256', 7, 2),
        (63, '536-512', 'Computers', 'Laptops', 'Asus ROG STRIX GL553VD-DM256', 'https://www.webscraper.io/test-sites/e-commerce/static/product/536', 799, 'Asus ROG STRIX GL553VD-DM256, 15.6" FHD, Core i5-7300HQ, 8GB, 1TB, GeForce GTX 1050 2GB, No OS', '', '512', 7, 2),
        (64, '537-128', 'Computers', 'Laptops', 'Acer Nitro 5 AN515-51', 'https://www.webscraper.io/test-sites/e-commerce/static/product/537', 809, 'Acer Nitro 5 AN515-51, 15.6" FHD IPS, Core i5-7300HQ, 8GB, 1TB, GeForce GTX 1050 2GB, Windows 10 Home', '', '128', 0, 1),
        (65, '537-256', 'Computers', 'Laptops', 'Acer Nitro 5 AN515-51', 'https://www.webscraper.io/test-sites/e-commerce/static/product/537', 809, 'Acer Nitro 5 AN515-51, 15.6" FHD IPS, Core i5-7300HQ, 8GB, 1TB, GeForce GTX 1050 2GB, Windows 10 Home', '', '256', 0, 1),
        (66, '537-512', 'Computers', 'Laptops', 'Acer Nitro 5 AN515-51', 'https://www.webscraper.io/test-sites/e-commerce/static/product/537', 809, 'Acer Nitro 5 AN515-51, 15.6" FHD IPS, Core i5-7300HQ, 8GB, 1TB, GeForce GTX 1050 2GB, Windows 10 Home', '', '512', 0, 1),
        (67, '538-512', 'Computers', 'Laptops', 'Asus ROG STRIX GL553VD-DM256', 'https://www.webscraper.io/test-sites/e-commerce/static/product/538', 899, 'Asus ROG STRIX GL553VD-DM256, 15.6" FHD, Core i5-7300HQ, 8GB, 1TB, GeForce GTX 1050 2GB, No OS + Windows 10 Home', '', '512', 7, 1),
        (68, '538-128', 'Computers', 'Laptops', 'Asus ROG STRIX GL553VD-DM256', 'https://www.webscraper.io/test-sites/e-commerce/static/product/538', 899, 'Asus ROG STRIX GL553VD-DM256, 15.6" FHD, Core i5-7300HQ, 8GB, 1TB, GeForce GTX 1050 2GB, No OS + Windows 10 Home', '', '128', 7, 1),
        (69, '538-256', 'Computers', 'Laptops', 'Asus ROG STRIX GL553VD-DM256', 'https://www.webscraper.io/test-sites/e-commerce/static/product/538', 899, 'Asus ROG STRIX GL553VD-DM256, 15.6" FHD, Core i5-7300HQ, 8GB, 1TB, GeForce GTX 1050 2GB, No OS + Windows 10 Home', '', '256', 7, 1),
        (70, '539-128', 'Computers', 'Laptops', 'Lenovo ThinkPad L570', 'https://www.webscraper.io/test-sites/e-commerce/static/product/539', 999, 'Lenovo ThinkPad L570, 15.6" FHD, Core i7-7500U, 8GB, 256GB SSD, Windows 10 Pro', '', '128', 11, 3),
        (71, '539-256', 'Computers', 'Laptops', 'Lenovo ThinkPad L570', 'https://www.webscraper.io/test-sites/e-commerce/static/product/539', 999, 'Lenovo ThinkPad L570, 15.6" FHD, Core i7-7500U, 8GB, 256GB SSD, Windows 10 Pro', '', '256', 11, 3),
        (72, '539-512', 'Computers', 'Laptops', 'Lenovo ThinkPad L570', 'https://www.webscraper.io/test-sites/e-commerce/static/product/539', 999, 'Lenovo ThinkPad L570, 15.6" FHD, Core i7-7500U, 8GB, 256GB SSD, Windows 10 Pro', '', '512', 11, 3),
        (73, '540-128', 'Computers', 'Laptops', 'Lenovo Legion Y520-15IKBM', 'https://www.webscraper.io/test-sites/e-commerce/static/product/540', 1149, 'Lenovo Legion Y520-15IKBM, 15.6" FHD IPS, Core i7-7700HQ, 8GB, 128GB SSD + 1TB, GeForce GTX 1060 Max-Q 6GB, DOS', '', '128', 11, 3),
        (74, '540-256', 'Computers', 'Laptops', 'Lenovo Legion Y520-15IKBM', 'https://www.webscraper.io/test-sites/e-commerce/static/product/540', 1149, 'Lenovo Legion Y520-15IKBM, 15.6" FHD IPS, Core i7-7700HQ, 8GB, 128GB SSD + 1TB, GeForce GTX 1060 Max-Q 6GB, DOS', '', '256', 11, 3),
        (75, '540-512', 'Computers', 'Laptops', 'Lenovo Legion Y520-15IKBM', 'https://www.webscraper.io/test-sites/e-commerce/static/product/540', 1149, 'Lenovo Legion Y520-15IKBM, 15.6" FHD IPS, Core i7-7700HQ, 8GB, 128GB SSD + 1TB, GeForce GTX 1060 Max-Q 6GB, DOS', '', '512', 11, 3),
        (76, '541-128', 'Computers', 'Laptops', 'Lenovo Legion Y720', 'https://www.webscraper.io/test-sites/e-commerce/static/product/541', 1399, 'Lenovo Legion Y720, 15.6" FHD IPS, Core i7-7700HQ, 8GB, 128GB SSD + 2TB HDD, GeForce GTX 1060 6GB, DOS, RGB backlit keyboard', '', '128', 8, 3),
        (77, '541-256', 'Computers', 'Laptops', 'Lenovo Legion Y720', 'https://www.webscraper.io/test-sites/e-commerce/static/product/541', 1399, 'Lenovo Legion Y720, 15.6" FHD IPS, Core i7-7700HQ, 8GB, 128GB SSD + 2TB HDD, GeForce GTX 1060 6GB, DOS, RGB backlit keyboard', '', '256', 8, 3),
        (78, '541-512', 'Computers', 'Laptops', 'Lenovo Legion Y720', 'https://www.webscraper.io/test-sites/e-commerce/static/product/541', 1399, 'Lenovo Legion Y720, 15.6" FHD IPS, Core i7-7700HQ, 8GB, 128GB SSD + 2TB HDD, GeForce GTX 1060 6GB, DOS, RGB backlit keyboard', '', '512', 8, 3),
        (79, '542-128', 'Computers', 'Laptops', 'Asus ROG Strix GL702ZC-GC154T', 'https://www.webscraper.io/test-sites/e-commerce/static/product/542', 1769, 'Asus ROG Strix GL702ZC-GC154T, 17.3" FHD, Ryzen 7 1700, 16GB, 256GB + 1TB HDD, Radeon RX 580 4GB, Windows 10 Home', '', '128', 7, 4),
        (80, '542-256', 'Computers', 'Laptops', 'Asus ROG Strix GL702ZC-GC154T', 'https://www.webscraper.io/test-sites/e-commerce/static/product/542', 1769, 'Asus ROG Strix GL702ZC-GC154T, 17.3" FHD, Ryzen 7 1700, 16GB, 256GB + 1TB HDD, Radeon RX 580 4GB, Windows 10 Home', '', '256', 7, 4),
        (81, '543-128', 'Computers', 'Laptops', 'Asus ROG Strix GL702ZC-GC209T', 'https://www.webscraper.io/test-sites/e-commerce/static/product/543', 1769, 'Asus ROG Strix GL702ZC-GC209T, 17.3" FHD IPS, Ryzen 7 1700, 16GB, 256GB SSD + 1TB HDD, Radeon RX 580 4GB, Windows 10 Home', '', '128', 8, 1),
        (82, '542-512', 'Computers', 'Laptops', 'Asus ROG Strix GL702ZC-GC154T', 'https://www.webscraper.io/test-sites/e-commerce/static/product/542', 1769, 'Asus ROG Strix GL702ZC-GC154T, 17.3" FHD, Ryzen 7 1700, 16GB, 256GB + 1TB HDD, Radeon RX 580 4GB, Windows 10 Home', '', '512', 7, 4),
        (83, '543-256', 'Computers', 'Laptops', 'Asus ROG Strix GL702ZC-GC209T', 'https://www.webscraper.io/test-sites/e-commerce/static/product/543', 1769, 'Asus ROG Strix GL702ZC-GC209T, 17.3" FHD IPS, Ryzen 7 1700, 16GB, 256GB SSD + 1TB HDD, Radeon RX 580 4GB, Windows 10 Home', '', '256', 8, 1),
        (84, '543-512', 'Computers', 'Laptops', 'Asus ROG Strix GL702ZC-GC209T', 'https://www.webscraper.io/test-sites/e-commerce/static/product/543', 1769, 'Asus ROG Strix GL702ZC-GC209T, 17.3" FHD IPS, Ryzen 7 1700, 16GB, 256GB SSD + 1TB HDD, Radeon RX 580 4GB, Windows 10 Home', '', '512', 8, 1),
        (85, '544-128', 'Computers', 'Laptops', 'Asus ROG Strix SCAR Edition GL503VM-ED115T', 'https://www.webscraper.io/test-sites/e-commerce/static/product/544', 1799, 'Asus ROG Strix SCAR Edition GL503VM-ED115T, 15.6" FHD 120Hz, Core i7-7700HQ, 16GB, 256GB SSD + 1TB SSHD, GeForce GTX 1060 6GB, Windows 10 Home', '', '128', 8, 3),
        (86, '544-256', 'Computers', 'Laptops', 'Asus ROG Strix SCAR Edition GL503VM-ED115T', 'https://www.webscraper.io/test-sites/e-commerce/static/product/544', 1799, 'Asus ROG Strix SCAR Edition GL503VM-ED115T, 15.6" FHD 120Hz, Core i7-7700HQ, 16GB, 256GB SSD + 1TB SSHD, GeForce GTX 1060 6GB, Windows 10 Home', '', '256', 8, 3),
        (87, '544-512', 'Computers', 'Laptops', 'Asus ROG Strix SCAR Edition GL503VM-ED115T', 'https://www.webscraper.io/test-sites/e-commerce/static/product/544', 1799, 'Asus ROG Strix SCAR Edition GL503VM-ED115T, 15.6" FHD 120Hz, Core i7-7700HQ, 16GB, 256GB SSD + 1TB SSHD, GeForce GTX 1060 6GB, Windows 10 Home', '', '512', 8, 3),
        (88, '545-128', 'Computers', 'Laptops', 'Asus VivoBook X441NA-GA190', 'https://www.webscraper.io/test-sites/e-commerce/static/product/545', 295.989990234375, 'Asus VivoBook X441NA-GA190 Chocolate Black, 14", Celeron N3450, 4GB, 128GB SSD, Endless OS, ENG kbd', '', '128', 14, 3),
        (89, '545-512', 'Computers', 'Laptops', 'Asus VivoBook X441NA-GA190', 'https://www.webscraper.io/test-sites/e-commerce/static/product/545', 295.989990234375, 'Asus VivoBook X441NA-GA190 Chocolate Black, 14", Celeron N3450, 4GB, 128GB SSD, Endless OS, ENG kbd', '', '512', 14, 3),
        (90, '545-256', 'Computers', 'Laptops', 'Asus VivoBook X441NA-GA190', 'https://www.webscraper.io/test-sites/e-commerce/static/product/545', 295.989990234375, 'Asus VivoBook X441NA-GA190 Chocolate Black, 14", Celeron N3450, 4GB, 128GB SSD, Endless OS, ENG kbd', '', '256', 14, 3),
        (91, '546-128', 'Computers', 'Laptops', 'Prestigio SmartBook 133S Dark Grey', 'https://www.webscraper.io/test-sites/e-commerce/static/product/546', 299, 'Prestigio SmartBook 133S Dark Grey, 13.3" FHD IPS, Celeron N3350 1.1GHz, 4GB, 32GB, Windows 10 Pro + Office 365 1 gadam', '', '128', 8, 2),
        (92, '546-256', 'Computers', 'Laptops', 'Prestigio SmartBook 133S Dark Grey', 'https://www.webscraper.io/test-sites/e-commerce/static/product/546', 299, 'Prestigio SmartBook 133S Dark Grey, 13.3" FHD IPS, Celeron N3350 1.1GHz, 4GB, 32GB, Windows 10 Pro + Office 365 1 gadam', '', '256', 8, 2),
        (93, '546-512', 'Computers', 'Laptops', 'Prestigio SmartBook 133S Dark Grey', 'https://www.webscraper.io/test-sites/e-commerce/static/product/546', 299, 'Prestigio SmartBook 133S Dark Grey, 13.3" FHD IPS, Celeron N3350 1.1GHz, 4GB, 32GB, Windows 10 Pro + Office 365 1 gadam', '', '512', 8, 2),
        (94, '547-256', 'Computers', 'Laptops', 'Prestigio SmartBook 133S Gold', 'https://www.webscraper.io/test-sites/e-commerce/static/product/547', 299, 'Prestigio SmartBook 133S Gold, 13.3" FHD IPS, Celeron N3350 1.1GHz, 4GB, 32GB, Windows 10 Pro + Office 365 1 gadam', '', '256', 12, 4),
        (95, '547-128', 'Computers', 'Laptops', 'Prestigio SmartBook 133S Gold', 'https://www.webscraper.io/test-sites/e-commerce/static/product/547', 299, 'Prestigio SmartBook 133S Gold, 13.3" FHD IPS, Celeron N3350 1.1GHz, 4GB, 32GB, Windows 10 Pro + Office 365 1 gadam', '', '128', 12, 4),
        (96, '547-512', 'Computers', 'Laptops', 'Prestigio SmartBook 133S Gold', 'https://www.webscraper.io/test-sites/e-commerce/static/product/547', 299, 'Prestigio SmartBook 133S Gold, 13.3" FHD IPS, Celeron N3350 1.1GHz, 4GB, 32GB, Windows 10 Pro + Office 365 1 gadam', '', '512', 12, 4),
        (97, '548-128', 'Computers', 'Laptops', 'Lenovo V110-15IAP', 'https://www.webscraper.io/test-sites/e-commerce/static/product/548', 321.94000244140625, 'Lenovo V110-15IAP, 15.6" HD, Celeron N3350 1.1GHz, 4GB, 128GB SSD, Windows 10 Home', '', '128', 5, 3),
        (98, '548-512', 'Computers', 'Laptops', 'Lenovo V110-15IAP', 'https://www.webscraper.io/test-sites/e-commerce/static/product/548', 321.94000244140625, 'Lenovo V110-15IAP, 15.6" HD, Celeron N3350 1.1GHz, 4GB, 128GB SSD, Windows 10 Home', '', '512', 5, 3),
        (99, '548-256', 'Computers', 'Laptops', 'Lenovo V110-15IAP', 'https://www.webscraper.io/test-sites/e-commerce/static/product/548', 321.94000244140625, 'Lenovo V110-15IAP, 15.6" HD, Celeron N3350 1.1GHz, 4GB, 128GB SSD, Windows 10 Home', '', '256', 5, 3),
        (100, '549-128', 'Computers', 'Laptops', 'Lenovo V110-15IAP', 'https://www.webscraper.io/test-sites/e-commerce/static/product/549', 356.489990234375, 'Asus VivoBook 15 X540NA-GQ008T Chocolate Black, 15.6" HD, Pentium N4200, 4GB, 500GB, Windows 10 Home, En kbd', '', '128', 6, 2);

        "#,
            ).unwrap();

            // conn
            //     .iterate("SELECT * FROM products", |pairs| {
            //         for &(column, value) in pairs.iter() {
            //             println!("{} = {}", column, value.unwrap());
            //         }
            //         true
            //     })
            //     .unwrap();

        }

        pool
    }

    fn teardown() {}

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
        }).await;


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
        }).await;
    }
}
