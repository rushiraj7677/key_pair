// #[warn(unused_imports)]
// use std::{
//     ops::Deref,
//     sync::{atomic::AtomicU64, Arc, RwLock},
// };
pub struct HashMapDb {
    hash_map: std::collections::HashMap<String, u32>,
}

impl HashMapDb {
    fn new() -> Self {
        Self {
            hash_map: std::collections::HashMap::new(),
        }
    }
}
pub struct SledDb {
    sled_db: sled::Db,
}

impl SledDb {
    fn new() -> Self {
        Self {
            sled_db: sled::open("sled_db").expect("sled_db creation failed"),
        }
    }
}

impl Database for SledDb {
    type Key = String;
    type Value = u8;
    fn set_value(&mut self, key: Self::Key, value: Self::Value) {
        self.sled_db.insert(key, sled::IVec::from(vec![value]));
    }
    fn get_value(&self, key: Self::Key) -> Option<Self::Value> {
        let res = self.sled_db.get(&key).ok()??;
        Some((*res)[0])
    }
}

impl Database for HashMapDb {
    type Key = String;
    type Value = u32;
    fn set_value(&mut self, key: Self::Key, value: Self::Value) {
        self.hash_map.insert(key, value);
    }
    fn get_value(&self, key: Self::Key) -> Option<Self::Value> {
        Some(self.hash_map.get(&key)?.clone())
    }
}
pub trait Database {
    type Key;
    type Value;
    fn set_value(&mut self, key: Self::Key, value: Self::Value);
    fn get_value(&self, key: Self::Key) -> Option<Self::Value>;
}
fn main() {
    let mut hdb = HashMapDb::new();
    hdb.set_value("name".to_string(), 7677);
    let l = hdb.get_value("name".to_string());
    println!("{:#?}", l);
    let mut sled_db = SledDb::new();
    sled_db.set_value("name".to_string(), 77_u8);
    let g = sled_db.get_value("name".to_string());
    println!("{:#?}", g);
}
