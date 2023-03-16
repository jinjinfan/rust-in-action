use std::error::Error;
use std::ops::Add;
use reqwest;

fn reqwest_test() -> Result<(),Box<dyn Error>>{
    let url = "http://www.rustinaction.com/";
    let mut response = reqwest::get(url)?;
    let content = response.text()?;
    println!("{}", content);
    Ok(())
}

use rand;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
struct Dwarf{}

#[derive(Debug)]
struct Elf{}

#[derive(Debug)]
struct Human{}

#[derive(Debug)]
enum Thing {
    Sword,
    Trinket,
}

trait Enchanter : std::fmt::Debug {
    fn competency(&self) -> f64;
    fn enchant(&self, thing : &mut Thing) {
        let probability_of_success = self.competency();
        let spell_is_successful = rand::thread_rng().gen_bool(probability_of_success);
        print!("{:?} mutters incoherently. ", self);
        if spell_is_successful {
            println!("The {:?} glows brightly. ", thing);
        } else {
            println!("The {:?} fizzes, then turns into a worthless trinket. ", thing);
            *thing = Thing::Trinket{};
        }
    }
}
impl Enchanter for Dwarf {
    fn competency(&self) -> f64 {
        0.5
    }
}

impl Enchanter for Elf {
    fn competency(&self) -> f64 {
        0.95
    }
}

impl Enchanter for Human {
    fn competency(&self) -> f64 {
        0.8
    }
}

fn rpg_test() {
    let mut it = Thing::Sword;
    let d = Dwarf{};
    let e = Elf{};
    let h = Human{};
    let party : Vec<&dyn Enchanter> = vec![&d, &h, &e];
    let spellcaster = party.choose(&mut rand::thread_rng()).unwrap();
    spellcaster.enchant(&mut it);
}

use std::io::prelude::*;
use std::net::TcpStream;

fn tcp_test() -> std::io::Result<()> {
    let host = "www.rustinaction.com:80";
    let mut conn = TcpStream::connect(host)?;
    conn.write_all(b"GET / HTTP/1.O")?;
    conn.write_all(b"\r\n")?;
    conn.write_all(b"Host: www.rustinaction.com")?;
    conn.write_all(b"\r\n\r\n")?;
    std::io::copy(&mut conn, &mut std::io::stdout())?;
    Ok(())
}

use std::fs::File;
use std::net::Ipv6Addr;
use std::net::AddrParseError;

#[derive(Debug)]
enum UpstreamError {
    IO(std::io::Error),
    Parsing(AddrParseError),
}

impl std::fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for UpstreamError {}

impl From<std::io::Error> for UpstreamError {
    fn from(value: std::io::Error) -> Self {
        UpstreamError::IO(value)
    }
}

impl From<std::net::AddrParseError> for UpstreamError {
    fn from(value: std::net::AddrParseError) -> Self {
        UpstreamError::Parsing(value)
    }
}
fn error_test() -> Result<(), UpstreamError>{
    // version 1 to use map_err
    /*let _f = File::open("invisible.txt")
        .map_err(UpstreamError::IO)?;
    let _localhost = "::1".parse::<Ipv6Addr>()
        .map_err(UpstreamError::Parsing)?;*/
    let _f = File::open("invisible.txt")?;
    let _localhost = "::1".parse::<Ipv6Addr>()?;
    Ok(())
}

use rand::RngCore;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
struct MacAddress([u8;6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octet = &self.0;
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            octet[0],octet[1], octet[2],
            octet[3], octet[4], octet[5]
        )
    }
}

impl MacAddress {
    fn new() -> MacAddress {
        let mut octets : [u8;6] = [0;6];
        rand::thread_rng().fill_bytes(&mut octets);
        octets[0] |= 0b_0000_0011;
        MacAddress {0:octets}
    }
    fn is_local(&self) -> bool {
        (self.0[0] & 0b_0000_0010) == 0b_0000_0010
    }
    fn is_unicast(&self) -> bool {
        (self.0[0] & 0b_0000_0001) == 0b_0000_0001
    }
}

fn mac_test() {
    let mac = MacAddress::new();
    assert!(mac.is_local());
    assert!(mac.is_unicast());
    println!("mac: {}", mac);
}
fn main() {
    //reqwest_test();
    rpg_test();
    tcp_test();
    //error_test();
    mac_test();
}
