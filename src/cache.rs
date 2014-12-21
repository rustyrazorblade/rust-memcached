
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

    pub fn set(&mut self, key:&String, val:String) {
        self.data.insert(key.clone(), val);
    }

    pub fn get(&self, key:String) -> Option<&String> {
        self.data.get(&key)
    }
    pub fn increment(&self, key:&String, value:i64) -> Option<i64> {
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

#[test]
fn increment_test() {
    let mut c = CacheManager::new();
    let s = "test".to_string();

    c.set(&s, "0".to_string());
    c.increment(&s, 1i64);

    let result = c.get(s).unwrap();
    //assert_eq!(result, "1".to_string());
}
