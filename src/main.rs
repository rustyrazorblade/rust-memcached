use std::collections::hash_map::HashMap;
use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};


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
fn direct_cache_test() {
    let mut cm = CacheManager::new();
    cm._put("test".to_string(), "value".to_string());
    let result = cm._get("test".to_string());
}


fn main() {
    println!("Hello, world!");
    
    println!("creating cache manager");

    let mut cm = CacheManager::new();

    println!("starting up socket server");

    let listener = TcpListener::bind("127.0.0.1:7777");
    
    let mut acceptor = listener.listen();

    fn handle_client(mut stream: TcpStream) {
        // ...
    }
    // accept connections and process them, spawning a new tasks for each one
    for stream in acceptor.incoming() {
        match stream {
            Err(e) => { /* connection failed */ }
            Ok(stream) => spawn(proc() {
                // connection succeeded
                handle_client(stream)
            })
        }
    }

    // close the socket server
    drop(acceptor);

    println!("done");
}
