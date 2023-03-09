#![allow(unused_variables)]

struct Hostname(String);

fn connect(host : Hostname) {
    println!("connected to {}", host.0);
}

fn connect_test() {
    let ordinary_string = String::from("localhost");
    let host = Hostname(ordinary_string.clone());
    connect(host);
}

use rand::prelude::*;

fn one_in(denominator : u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}


trait Read {
    fn read(self : &Self, save_to : &mut Vec<u8>) -> Result<usize, String>;
}

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}
use std::fmt::{Display, write};
impl Display for FileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FileState::Closed => write!(f, "CLOSED"),
            FileState::Open => write!(f, "OPEN"),
        }
    }
}

#[derive(Debug)]
pub struct File {
    pub name : String,
    data : Vec<u8>,
    pub state : FileState,
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    pub fn new(name : & str) -> File {
        File {
            name : String::from(name),
            data : Vec::new(),
            state : FileState::Closed,
        }
    }
    fn new_with_data(name : & str, data : &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }
    fn read(self : &File, save_to : &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}
fn open(mut f : File) -> Result<File, String> {
    f.state = FileState::Open;
    /*if one_in(10_000) {
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }*/
    Ok(f)
}

fn close(mut f : File) -> Result<File, String> {
    f.state = FileState::Closed;
    /*if one_in(10_000) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    }*/
    Ok(f)
}



fn file_test() {
    let f3_data : Vec<u8> = vec![114,117,115, 116, 33];
    let mut f2 = File::new_with_data("2.txt", &f3_data);
    
    let mut buffer : Vec<u8> = vec![];
    if f2.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }
    f2 = open(f2).unwrap();
    let f2_length = f2.read(&mut buffer).unwrap();
    f2 = close(f2).unwrap();
    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_length);
    println!("{}", text);
    println!("{}", f2);
}

#[derive(Debug)]
enum Event {
    Update,
    Delete,
    Unknown,
}

type Message = String;

fn parse_log(line : &str) -> (Event, Message) {
    let parts : Vec<_> = line.splitn(2, ' ').collect();
    if parts.len() == 1 {
        return (Event::Unknown, String::from(line))
    }

    let event = parts[0];
    let rest = String::from(parts[1]);
    match event {
        "UPDATE" | "update" => (Event::Update, rest),
        "DELETE" | "delete" => (Event::Delete, rest),
        _ => (Event::Unknown, String::from(line)),
    }
}

fn enum_test() {
    let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";
    for line in log.lines() {
        let parse_result = parse_log(line);
        println!("{:?}", parse_result);
    }
}
fn main() {
    println!("Connect host test");
    connect_test();
    println!("");
    println!("File test");
    file_test();
    println!("");
    println!("enum test");
    enum_test();
    println!("");
}
    