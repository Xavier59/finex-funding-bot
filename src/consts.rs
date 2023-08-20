use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref LIMIT_PER_CURRENCY: HashMap<String, f64> = {
        let mut m = HashMap::new();
        m.insert("USD".to_string(), 150.0);
        m.insert("SOL".to_string(), 1.0);
        m.insert("BTC".to_string(), 0.00002);
        m.insert("USDT".to_string(), 150.0);
        m
    };
}
