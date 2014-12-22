#![feature(slicing_syntax)]

use std::collections::hash_map::HashMap;
use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
//use std::string::String;

use std::ascii::OwnedAsciiExt;
use std::str::from_utf8;
use std::fmt::{Show, Error, Formatter};
use std::string::String;

use parser::{MemcachedOp,parse_command};
use cache::CacheManager;

mod parser;
mod response;
mod cache;


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
                    let response = cm.get(&key);
                    match response {
                        Some(s) =>
                            msg.response_channel.send(MemcachedResponse::Found(kkey, s.clone())),
                        None =>
                            msg.response_channel.send(MemcachedResponse::NotFound)
                    }
                },
                MemcachedOp::Increment(key, value) => {
                    let new_val = cm.increment(&key, value);
                    msg.response_channel.send(MemcachedResponse::NotFound);
                },
                MemcachedOp::FlushAll => {
                    msg.response_channel.send(MemcachedResponse::OK);
                }
                _ =>
                    println!("unknown")

            }
        }
    });
    tx
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

enum MemcachedResponse {
    ShuttingDown,
    Stored,
    Found(String, String), // key, value
    NotFound,
    OK,
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
                        }
                        MemcachedResponse::Stored => {
                            stream.write_str("STORED\r\n");
                        }
                        MemcachedResponse::NotFound => {
                            stream.write_str("NOT_FOUND\r\n");
                        }
                        MemcachedResponse::OK => {
                            stream.write_str("OK\r\n");
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
