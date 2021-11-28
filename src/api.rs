use crate::db::*;

use actix_web::{web, HttpResponse};

pub fn configure_api(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("products").route(web::get().to(index)));
}

async fn index(pool: web::Data<Pool>) -> HttpResponse {
    let conn = pool.get().unwrap();
    let products = get_products(conn);

    HttpResponse::Ok().json(products)
}
