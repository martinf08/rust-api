use crate::db::value_into_string;
use serde::{Deserialize, Serialize};
use sqlite3::Value;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Product {
    id: i32,
    uid: String,
    category: String,
    sub_category: String,
    name: String,
    origin: String,
    cost: f32,
    description: String,
    color: String,
    size: String,
    review_count: u32,
    review_stars: u32,
}

impl Default for Product {
    fn default() -> Self {
        Product {
            id: 0,
            uid: String::new(),
            category: String::new(),
            sub_category: String::new(),
            name: String::new(),
            origin: String::new(),
            cost: 0.0,
            description: String::new(),
            color: String::new(),
            size: String::new(),
            review_count: 0,
            review_stars: 0,
        }
    }
}

impl<K> From<HashMap<K, Value>> for Product
where
    K: Hash + Eq + AsRef<str>,
{
    fn from(map: HashMap<K, Value>) -> Self {
        let mut product = Product::default();
        for (key, value) in map {
            match key.as_ref() {
                "id" => product.id = value_into_string(value).parse().unwrap(),
                "uid" => product.uid = value_into_string(value),
                "category" => product.category = value_into_string(value),
                "sub_category" => product.sub_category = value_into_string(value),
                "name" => product.name = value_into_string(value),
                "origin" => product.origin = value_into_string(value),
                "cost" => product.cost = value_into_string(value).parse().unwrap(),
                "description" => product.description = value_into_string(value),
                "color" => product.color = value_into_string(value),
                "size" => product.size = value_into_string(value),
                "review_count" => product.review_count = value_into_string(value).parse().unwrap(),
                "review_stars" => product.review_stars = value_into_string(value).parse().unwrap(),
                _ => (),
            }
        }

        product
    }
}
