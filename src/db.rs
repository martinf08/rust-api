use crate::api::Pagination;
use crate::models::Product;

use actix_web::web::Query;
use r2d2_sqlite3::SqliteConnectionManager;
use sqlite3::{Statement, Value};
use std::collections::HashMap;

pub const DEFAULT_PAGE: i32 = 1;
pub const DEFAULT_LIMIT: i32 = 10;

pub type Pool = r2d2::Pool<SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<SqliteConnectionManager>;

pub fn create_pool() -> Pool {
    let manager = SqliteConnectionManager::file("data/webscraper.db");
    let pool = Pool::new(manager).unwrap();

    return pool;
}

pub fn get_products(
    conn: Connection,
    Query(Pagination { page, limit }): Query<Pagination>,
) -> Vec<Product> {
    let (p, l) = match (page, limit) {
        (Some(p), Some(l)) => (p, l),
        (Some(p), None) => (p, self::DEFAULT_LIMIT),
        (None, Some(l)) => (self::DEFAULT_PAGE, l),
        (None, None) => (self::DEFAULT_PAGE, self::DEFAULT_LIMIT),
    };

    let mut count = 0;
    conn.iterate("SELECT COUNT(*) FROM products", |pair| {
        let &(_, val) = pair.into_iter().next().unwrap();
        count = val.unwrap().parse::<i32>().unwrap();
        true
    })
    .unwrap();

    if count == 0 {
        return Vec::new();
    }

    let l = if l > 50 || l <= 0 {
        self::DEFAULT_LIMIT
    } else {
        l
    };

    let last_page = (count as f64 / l as f64).floor() as i32;
    let p = if p <= 0 {
        self::DEFAULT_PAGE
    } else if p > last_page {
        last_page
    } else {
        p
    };

    let (limit, offset) = ((p - 1) * l, p * l);

    let query = format!("SELECT * FROM products LIMIT {} OFFSET {}", offset, limit);
    let statement = conn.prepare(query).unwrap();

    let mut raw_vec_map = statement_to_vec_map(statement);

    let products = raw_vec_map
        .drain(..)
        .map(|x| Product::from(x))
        .collect::<Vec<Product>>();

    products
}

fn statement_to_vec_map(statement: Statement) -> Vec<HashMap<String, Value>> {
    let column_names = statement.column_names().unwrap();
    let mut cursor = statement.cursor();

    let mut result = Vec::new();
    while let Some(row) = cursor.next().unwrap() {
        let mut map = HashMap::new();

        (0..column_names.len()).for_each(|i| {
            let value = row.get(i).unwrap().clone();
            map.insert(column_names[i].to_string(), value);
        });

        result.push(map);
    }

    return result;
}

pub fn value_into_string(v: Value) -> String {
    use Value::{Float as FloatValue, Integer as IntegerValue, String as StringValue};

    match v {
        FloatValue(v) => v.to_string(),
        IntegerValue(v) => v.to_string(),
        StringValue(v) => v.to_string(),
        _ => String::new(),
    }
}
