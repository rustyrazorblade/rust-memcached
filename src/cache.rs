
use std::collections::hash_map::HashMap;

pub struct CacheManager {
    data: HashMap<String, String>,
}

impl CacheManager {

    pub fn new() -> Box<CacheManager> {
        let map: HashMap<String, String> = HashMap::new();
        box CacheManager{data:map}
    }


    pub fn put(&mut self, key:String, val:String) {
        self.data.insert(key, val);
    }

    pub fn get(&self, key:String) -> Option<&String> {
        self.data.get(&key)
    }
    pub fn increment(&self, key:String) -> Option<i64> {
        return Some(0i64);
    }
}

#[test]
fn direct_cache_test() {
    let mut cm = CacheManager::new();
    cm.put("test".to_string(), "value".to_string());
    let result = cm.get("test".to_string());
}

#[test]
fn test_cache_manager_get() {
    let mut c = CacheManager::new();
}
