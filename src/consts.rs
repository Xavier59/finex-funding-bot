use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref LIMIT_PER_CURRENCY: HashMap<String, f64> = {
        let mut m = HashMap::new();
        m.insert("USD".to_string(), 150.0);
        m
    };
}