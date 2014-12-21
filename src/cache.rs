
use std::collections::hash_map::HashMap;
use std::cmp::PartialEq;

pub struct CacheManager {
    data: HashMap<String, String>,
}

impl CacheManager {

    pub fn new() -> Box<CacheManager> {
        let map: HashMap<String, String> = HashMap::new();
        box CacheManager{data:map}
    }


    pub fn put(&mut self, key:String, val:String) {
        self.data.insert(key.clone(), val);
    }

    pub fn set(&mut self, key:&String, val:&String) {
        self.data.insert(key.clone(), val.clone());
    }

    pub fn get(&self, key:&String) -> Option<String> {
        let result = self.data.get(key);
        match result {
            Some(x) => Some(x.clone()),
            None => None
        }
    }

    pub fn decrement(&mut self, key:&String, value:i64) -> Option<String> {
        self.increment(key, 0i64-value)
    }

    pub fn increment(&mut self, key:&String, value:i64) -> Option<String> {
        let data = self.get(key);
        //let data : Option<String> = Some("1".to_string());

        match data {
            Some(val) => {
                let ival: i64 = val.to_int().unwrap();
                let new_val: i64 = ival + value;
                let new_val_str = new_val.to_string();
                self.set(key, &new_val_str);
                return Some(new_val_str);
            }
            None => {
                return None
            }
        }
    }
}

#[test]
fn direct_cache_test() {
    let mut cm = CacheManager::new();
    cm.put("test".to_string(), "value".to_string());
    let result = cm.get(&"test".to_string());
}

#[test]
fn test_cache_manager_get() {
    let mut c = CacheManager::new();
}

#[test]
fn increment_derement_test() {
    let mut c = CacheManager::new();
    let s = "test".to_string();

    c.set(&s, &"0".to_string());
    match c.increment(&s, 1i64) {
        // should return the correct value
        Some(x) =>
            (),
        None =>
            panic!("Expected a return value.")
    }

    let result = c.get(&s).unwrap();
    assert_eq!(result, "1".to_string());

    match c.decrement(&s, 4i64) {
        // should return the correct value
        Some(x) =>
            (),
        None =>
            panic!("Expected a return value.")
    }

    let result = c.get(&s).unwrap();
    assert_eq!(result, "-3".to_string());
}


trait ConvertToInt {
    fn to_int(&self) -> Option<i64>;
}

impl ConvertToInt for String {
    fn to_int(&self) -> Option<i64> {
        let buf = self.as_slice();
        let result: Option<i64> = from_str(buf);
        return result
    }

}

#[test]
fn test_convert_to_int() {
    let a = "10".to_string().to_int().unwrap();
    assert_eq!(10i64, a);

    let a = "11".to_string().to_int().unwrap();
    assert_eq!(11i64, a);
}
