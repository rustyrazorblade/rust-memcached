use std::collections::hash_map::HashMap;

struct CacheManager {
    data: HashMap<String, String>   
    
}

impl CacheManager {
    fn new() -> CacheManager {
        let mut map: HashMap<String, String> = HashMap::new();
        CacheManager{data:map}
    }

    fn put(&mut self, key:String, val:String) {
        self.data.insert(key, val);
    }

    fn get(&self, key:String) -> Option<&String> {
        self.data.get(&key)
    }
}

#[test]
fn simple_cache_test() {
    let mut cm = CacheManager::new();
    cm.put("test".to_string(), "value".to_string());
    let result = cm.get("test".to_string());
}


fn main() {
    println!("Hello, world!");
    
    println!("creating cache manager");

    let mut cm = CacheManager::new();

    println!("starting up cache manager");

    println!("starting up socket server");

    println!("done");
}
