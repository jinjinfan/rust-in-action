use bincode::serialize as to_bincode;
use serde_cbor::to_vec as to_cbor;
use serde_json::to_string as to_json;
use serde_derive::{Serialize};

#[derive(Serialize)]
struct City {
    name : String,
    population : usize,
    latitude : f64,
    longitude : f64,
}

fn data_storage() {
    let calabar = City {
        name : String::from("Calabar"),
        population : 470_000,
        latitude : 4.95,
        longitude : 8.33
    };
    let as_json = to_json(&calabar).unwrap();
    let as_cbor = to_cbor(&calabar).unwrap();
    let as_bincode = to_bincode(&calabar).unwrap();
    println!("json:\n{}\n", &as_json);
    println!("cbor:\n{:?}\n", &as_cbor);
    println!("bincode:\n{:?}\n", &as_bincode);
    println!("json (as UTF-8):\n{}\n", String::from_utf8_lossy(as_json.as_bytes()));
    println!("cbor (as UTF-8):\n{:?}\n", String::from_utf8_lossy(&as_cbor));
    println!("bincode (as UTF-8):\n{:?}\n", String::from_utf8_lossy(&as_bincode));

}

use std::io::prelude::*;

const BYTES_PER_LINE :usize = 16;
const INPUT : &'static [u8] = br#"
fn main() {
    println!("Hello, world!")
}
"#;

fn hexdump_clone() -> std::io::Result<()> {
    let mut buffer : Vec<u8> = vec!();
    INPUT.read_to_end(&mut buffer)?;
    let mut position_in_input = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        println!("[0x{:08x}] ", position_in_input);
        for byte in line {
            print!("{:02x} ", byte);
        }
        println!();
        position_in_input += BYTES_PER_LINE;
    }
    Ok(())
}
use std::fs::File;
use std::env;

fn fview_cli() {
    let arg1 = env::args().nth(1);
    let fname = arg1.expect("usage : sample FILENAME");
    let mut f = File::open(&fname).expect("Unable to open file");
    let mut pos = 0;
    let mut buffer = [0;BYTES_PER_LINE];
    while let Ok(_) = f.read_exact(&mut buffer){
        println!("[0x{:08x}] ", pos);
        for byte in &buffer {
            match *byte {
                0x00 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{:2x} ", byte),
            }
        }
        println!("");
        pos += BYTES_PER_LINE;
    }
}

fn file_test() {
    let mut hello = std::path::PathBuf::from("/tmp/hello.txt");
    hello.pop();
    println!("{:?}", hello.display());

}

use std::io::Cursor;
use byteorder::{LittleEndian};
use byteorder::{ReadBytesExt, WriteBytesExt};

fn write_numbers_to_file() -> (u32, i8, f64) {
    let mut w = vec![];
    let one : u32 = 1;
    let two : i8 = 2;
    let three : f64 = 3.0;
    w.write_u32::<LittleEndian>(one).unwrap();
    println!("{:?}", &w);
    w.write_i8(two).unwrap();
    println!("{:?}", &w);
    w.write_f64::<LittleEndian>(three).unwrap();
    println!("{:?}", &w);
    (one, two, three)
}

fn read_numbers_from_file() -> (u32, i8, f64) {
    let mut r = Cursor::new(vec![1,0,0,0,2,0,0,0,0,0,0,8,64]);
    let one_ = r.read_u32::<LittleEndian>().unwrap();
    let two_ = r.read_i8().unwrap();
    let three_ = r.read_f64::<LittleEndian>().unwrap();
    (one_, two_, three_)
}

fn byte_test() {
    let (one,two,three) = write_numbers_to_file();
    let (one_, two_,three_) = read_numbers_from_file();
    assert_eq!(one, one_);
    assert_eq!(two, two_);
    assert_eq!(three, three_);
}
fn parity_bit(bytes :&[u8]) -> u8 {
    let mut n_ones : u32 = 0;
    for byte in bytes {
        let ones = byte.count_ones();
        n_ones += ones;
        println!("{} (0b{:08b}) has {} one bits", byte, byte, ones);
    }
    (n_ones % 2 == 0) as u8
}
fn parity_test() {
    let abc = b"abc";
    println!("input : {:?}", abc);
    println!("output : {:08x}", parity_bit(abc));
    println!();
    let abcd = b"abcd";
    println!("input : {:?}", abcd);
    println!("output : {:08x}", parity_bit(abcd));

}

use std::collections::BTreeMap;

fn map_test() {
    let mut voc = BTreeMap::new();
    voc.insert(3_697_195, "Amesterdam");
    voc.insert(1_300_405, "Middelburg");
    voc.insert(469_400, "Delflt");
    for (guilders, kamer) in &voc {
        println!("{} invested {}", kamer, guilders);
    }
    println!("smaller chambers" );
    for (_guilders, kamer) in voc.range(0..500_000) {
        print!("{} ", kamer);
    }
    println!();
}
fn main() {
    data_storage();
    println!();
    hexdump_clone();
    //fview_cli();
    println!();
    file_test();
    println!();
    byte_test();
    println!();
    parity_test();
    println!();
    map_test();
}