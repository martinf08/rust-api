use crate::db::*;

use actix_web::web::Query;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

pub fn configure_api(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("products").route(web::get().to(index)));
}

async fn index(pool: web::Data<Pool>, params: Query<Pagination>) -> HttpResponse {
    let conn = pool.get().unwrap();
    let products = get_products(conn, params);

    HttpResponse::Ok().json(products)
}
