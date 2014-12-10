#![feature(slicing_syntax)]

use std::collections::hash_map::HashMap;
use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
//use std::string::String;

use std::ascii::OwnedAsciiExt;
use std::str::from_utf8;


struct CacheManager {
    data: HashMap<String, String>,
}

impl CacheManager {

    fn new() -> Box<CacheManager> {
        let map: HashMap<String, String> = HashMap::new();
        box CacheManager{data:map}
    }
    

    fn put(&mut self, key:String, val:String) {
        self.data.insert(key, val);
    }

    fn get(&self, key:String) -> Option<&String> {
        self.data.get(&key)
    }
}

struct MemcachedMsg {
    msg: MemcachedOp,
    response_channel: Sender<MemcachedResponse>,
}

fn event_loop(mut cm: Box<CacheManager>) -> Sender<MemcachedMsg> {
    // cm is moved, freed after this function
    // messages that come in must include a response channel (tx)
    
    // event_loop listening channel
    let (tx, rx) = channel::<MemcachedMsg>();

    spawn(proc() {
        loop {
            let msg = rx.recv();
            
            match msg.msg {
                MemcachedOp::Shutdown => {
                    println!("received shutdown");
                    return
                },
                MemcachedOp::SetOp(key, value, expire) => {
                    println!("setting");
                    cm.put(key, value);
                }
                _ => 
                    println!("unknown"),
            }
        }
    });
    tx
}

fn send(tx: Sender<MemcachedMsg>, m: MemcachedOp) -> MemcachedResponse {
    let (response_channel_tx, response_channel_rx) = channel::<MemcachedResponse>();
    let msg = MemcachedMsg{msg:m, response_channel:response_channel_tx};
    tx.send(msg);
    response_channel_rx.recv()
}

#[test]
fn test_event_loop() {
    let cm = CacheManager::new();

    let tx = event_loop(cm);
    
    let response = send(tx, MemcachedOp::Shutdown);
}
enum MemcachedOp {
    SetOp(String, String, int), // key, value, expire in seconds
    GetOp(String), // key
    IncrementOp,
    Shutdown
}

enum MemcachedResponse {

}

fn parse_command(s: String) -> MemcachedOp {
    // tokenize - simply split on spaces
    let ss = s.as_slice().trim_left_chars(' ');
    let mut lines = ss.split('\n');

    let mut tokens = lines.next().unwrap().split(' ');

    let command = tokens.next().unwrap().to_string();
    let lowered = command.into_ascii_lower();
    let command_lowered = lowered.as_slice();

    if command_lowered == "set" {
        let val = lines.next().unwrap();
        return MemcachedOp::SetOp("test".to_string(), val.to_string(), 0);
    } else if command_lowered == "get" {
        return MemcachedOp::GetOp("test".to_string());
    }

    return MemcachedOp::GetOp("test".to_string());
    
}

#[test]
fn test_parse_set_basic() {
    let parsed = parse_command("SET jon\nhaddad".to_string());
    match parsed {
        MemcachedOp::SetOp(key, value, expire) =>
            if value != "haddad".to_string() {
                panic!("looking for haddad found {}", value)
            },
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

#[test]
fn test_cache_manager_get() {
    let mut c = CacheManager::new();
}

fn main() {
    println!("Hello, world!");
    
    println!("creating cache manager");

    let cm = CacheManager::new();

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
