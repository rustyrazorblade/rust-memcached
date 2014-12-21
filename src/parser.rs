
use std::ascii::OwnedAsciiExt;

pub enum MemcachedOp {
    SetOp(String, String, int), // key, value, expire in seconds
    GetOp(String), // key
    Increment(String, i64),
    Shutdown,
    Delete,
    Err,
    FlushAll,
    Touch(String), //
}


pub fn parse_command(s: String) -> MemcachedOp {
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
    } else if command_lowered == "flush_all" {
        return MemcachedOp::FlushAll
    } else if command_lowered == "touch" {
        let key = tokens.next().unwrap();
        return MemcachedOp::Touch(key.to_string());
    }

    return MemcachedOp::Err;

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
fn test_parse_flush_all() {
    match parse_command("flush_all\r\n".to_string()) {
        MemcachedOp::FlushAll => (),
        _ => panic!("Bad flush_all parse")
    }
}
