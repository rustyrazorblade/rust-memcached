#![feature(slicing_syntax)]

use std::collections::hash_map::HashMap;
use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
//use std::string::String;

use std::str::from_utf8;

use std::vec::Vec;


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

enum MemcachedOp {
    SetOp(String, String, int), // key, value, expire in seconds
    GetOp(String), // key
    IncrementOp
}

fn parse_command(s: String) -> MemcachedOp {
    // tokenize - simply split on spaces
    let ss = s.as_slice().trim_left_chars(' ');
    let mut lines = ss.split('\n');

    let mut tokens = lines.next().unwrap().split(' ');
    let command = tokens.next().unwrap();

    if command == "set" {
        return MemcachedOp::SetOp("test".to_string(), "value".to_string(), 0);
    } else if command == "get" {
        return MemcachedOp::GetOp("test".to_string());
    }

    return MemcachedOp::GetOp("test".to_string());
    
}

#[test]
fn test_parse_set_basic() {
    let parsed = parse_command("set jon\nhaddad".to_string());
    match parsed {
        MemcachedOp::SetOp(key, value, expire) =>
            println!("OK"),
        _ =>
            panic!("wrong type")
            
    }
}

#[test]
fn test_parse_get_basic() {
    let parsed = parse_command("GET jon".to_string());
    match parsed {
        MemcachedOp::GetOp(key) =>
            println!("OK"),
        _ =>
            panic!("wrong type")
            
    }
}

#[test]
fn direct_cache_test() {
    let mut cm = CacheManager::new();
    cm.put("test".to_string(), "value".to_string());
    let result = cm.get("test".to_string());
}


fn main() {
    println!("Hello, world!");
    
    println!("creating cache manager");

    let mut cm = CacheManager::new();

    println!("starting up socket server");

    let listener = TcpListener::bind("127.0.0.1:11211");
    
    println!("binding to port 7777");

    let mut acceptor = listener.listen();

    fn handle_client(mut stream: TcpStream) {
        // for now we just do an echo server
        println!("OK");
        let mut buf = [0u8, ..4096];
        //let mut buf = [0u8];

        loop {
            let result = stream.read(&mut buf).unwrap();
            let s = buf.slice(0, result);
            let rep = from_utf8(s);
            println!("{}", rep);
        }
    }

    // accept connections and process them, spawning a new tasks for each one
    for stream in acceptor.incoming() {
        match stream {
            Err(e) => { 
                println!("could not accept connection {}", e);
            }
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
