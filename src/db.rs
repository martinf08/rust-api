use crate::models::Product;
use r2d2_sqlite3::SqliteConnectionManager;
use sqlite3::{Statement, Value};
use std::collections::HashMap;

pub type Pool = r2d2::Pool<SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<SqliteConnectionManager>;

pub fn create_pool() -> Pool {
    let manager = SqliteConnectionManager::file("data/webscraper.db");
    let pool = Pool::new(manager).unwrap();

    return pool;
}

pub fn get_products(conn: Connection) -> Vec<Product> {
    let statement = conn.prepare("SELECT * FROM 'products' LIMIT 0,30").unwrap();

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
