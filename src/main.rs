#![feature(slicing_syntax)]

use std::collections::hash_map::HashMap;
use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
//use std::string::String;

use std::ascii::OwnedAsciiExt;
use std::str::from_utf8;
use std::fmt::{Show, Error, Formatter};
use std::string::String;

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

    spawn(move || {
        for msg in rx.iter() {

            match msg.msg {
                MemcachedOp::Shutdown => {
                    println!("received shutdown");
                    msg.response_channel.send(MemcachedResponse::ShuttingDown);
                    return
                },
                MemcachedOp::SetOp(key, value, expire) => {
                    //println!("Putting!");
                    cm.put(key, value);
                    msg.response_channel.send(MemcachedResponse::Stored)
                },
                MemcachedOp::GetOp(key) => {
                    let kkey = key.clone();
                    let response = cm.get(key);
                    match response {
                        Some(s) =>
                            msg.response_channel.send(MemcachedResponse::Found(kkey, s.clone())),
                        None =>
                            msg.response_channel.send(MemcachedResponse::NotFound)
                    }
                },
                MemcachedOp::Increment(key, value) => {

                },
                _ =>
                    println!("unknown"),
            }
        }
    });
    tx
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

// send a message & wait for response.
fn send(tx: &Sender<MemcachedMsg>, m: MemcachedOp) -> MemcachedResponse {
    let (response_channel_tx, response_channel_rx) = channel::<MemcachedResponse>();
    let msg = MemcachedMsg{msg:m, response_channel:response_channel_tx};
    tx.send(msg);
    response_channel_rx.recv()
}

#[test]
fn test_event_loop() {
    let cm = CacheManager::new();

    let tx = event_loop(cm);

    for x in range(0i, 100) {
        match send(&tx, MemcachedOp::SetOp("k".to_string(), "v".to_string(), 0)) {
            MemcachedResponse::Stored =>
                println!("OK"),
            _ =>
                panic!("was expecting ok")
        }
    }

    match send(&tx, MemcachedOp::Shutdown) {
        MemcachedResponse::ShuttingDown =>
            println!("OK"),
        _ =>
            panic!("was expecting shutdown")
    }

}
enum MemcachedOp {
    SetOp(String, String, int), // key, value, expire in seconds
    GetOp(String), // key
    Increment(String, i64),
    Shutdown,
    Delete,
    Err
}


enum MemcachedResponse {
    ShuttingDown,
    Stored,
    Found(String, String), // key, value
    NotFound,
}

fn parse_command(s: String) -> MemcachedOp {
    // tokenize - simply split on spaces
    let ss = s.as_slice().trim_left_chars(' ');
    let mut lines = ss.split_str("\r\n");

    let mut tokens = lines.next().unwrap().split(' ');

    let command = tokens.next().unwrap().to_string();
    let lowered = command.into_ascii_lower();
    let command_lowered = lowered.as_slice();

    if command_lowered == "set" {
        let key = tokens.next().unwrap();
        let val = lines.next().unwrap();
        return MemcachedOp::SetOp(key.to_string(), val.to_string(), 0);
    } else if command_lowered == "get" {
        let key = tokens.next().unwrap();
        return MemcachedOp::GetOp(key.to_string());
    } else if command_lowered == "incr" {
        let key = tokens.next().unwrap();
        return MemcachedOp::Increment(key.to_string(), 1);
    }

    return MemcachedOp::Err;

}
#[test]
fn test_incr() {
    // create a key, set to zero

    let parsed = parse_command("INCR test 1".to_string());
    match parsed {
        MemcachedOp::Increment(key, value) => {
            assert_eq!(key, "test".to_string());
            assert_eq!(value, 1);
        }
        _ => panic!("Was expectring an increment call")
    }
}

#[test]
fn test_parse_set_basic() {
    let parsed = parse_command("SET jon\r\nhaddad\r\n".to_string());
    match parsed {
        MemcachedOp::SetOp(key, value, expire) => {
            assert_eq!(6, value.len());
            assert_eq!(value, "haddad".to_string());
            assert_eq!(key, "jon".to_string());
            },
        _ =>
            panic!("wrong type")

    }
}

#[test]
fn test_parse_get_basic() {
    let parsed = parse_command("GET jon\r\n".to_string());
    match parsed {
        MemcachedOp::GetOp(key) => {
            assert_eq!(key, "jon".to_string());
            println!("OK")
            }
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
    let tx = event_loop(cm);

    println!("starting up socket server");

    let listener = TcpListener::bind("127.0.0.1:11211");

    println!("binding to port 11211");

    let mut acceptor = listener.listen();

    fn handle_client(mut stream: TcpStream, tx: Sender<MemcachedMsg>) {
        // for now we just do an echo server
        let mut buf = [0u8, ..4096];
        //let mut buf = [0u8];

        loop {
            match stream.read(&mut buf) {
                Ok(result)  =>  {
                    let s = buf.slice(0, result);
                    let rep = from_utf8(s).unwrap().to_string();
                    let parsed = parse_command(rep);
                    let response = send(&tx, parsed);
                    match response {
                        MemcachedResponse::Found(key, s) => {

                            // VALUE <key> <flags> <bytes> [<cas unique>]\r\n
                            // <data block>\r\n
                            //
                            // Example:
                            // VALUE test 0 3
                            // val
                            // END

                            let len = s.len();
                            //println!("Found [{}]", s);
                            let tcp_response = format!("VALUE {} 0 {}\r\n{}\r\nEND\r\n", key, len, s);

                            stream.write_str(tcp_response.as_slice());
                        },
                        MemcachedResponse::Stored => {
                            stream.write_str("STORED\r\n");
                        }
                        _ =>
                            println!("Umm")
                    }
                },
                Err(e) => {
                    println!("hangup {}", e);
                    break;
                }
            }
        }
    }

    // accept connections and process them, spawning a new tasks for each one
    for stream in acceptor.incoming() {
        let new_tx = tx.clone();
        match stream {
            Err(e) => {
                println!("could not accept connection {}", e);
            }
            Ok(stream) => spawn(move || {
                // connection succeeded
                handle_client(stream, new_tx)
            })
        }
    }

    // close the socket server
    drop(acceptor);

    println!("done");
}
