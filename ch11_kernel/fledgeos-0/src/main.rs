#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(lang_items)]

use core::intrinsics;
use core::panic::PanicInfo;
use core::fmt;
use core::fmt::Write;
use x86_64::instructions::hlt;

#[allow(unused)]
#[derive(Clone,Copy)]
#[repr(u8)]
enum Color {
    Black = 0x0,    White = 0xF,
    Blue = 0x1,     BrightBlue = 0x9,
    Green = 0x2,    BrightGreen = 0xA,
    Cyan = 0x3,     BrightCyan = 0xB,
    Red = 0x4,      BrightRed = 0xC,
    Magenta = 0x5,  BrightMagenta = 0xD,
    Brown = 0x6,    Yellow = 0xE,
    Gray = 0x7,     DarkGray = 0x8
}

struct Cursor {
    position : isize,
    foreground : Color,
    background : Color,
}

impl Cursor {
    fn color(&self) -> u8 {
        let fg = self.foreground as u8;
        let bg = (self.background as u8) << 4;
        fg | bg
    }
    fn print(&mut self, text : &[u8]) {
        let color = self.color();
        let framebuffer = 0xb8000 as *mut u8;
        for &character in text {
            unsafe {
                framebuffer.offset(self.position).write_volatile(character);
                framebuffer.offset(self.position+1).write_volatile(color);
            }
            self.position += 2;
        }
    }
}

impl core::fmt::Write for Cursor {
    fn write_str(&mut self, s : &str) -> core::fmt:: Result {
        self.print(s.as_bytes());
        Ok(())
    }
}
#[panic_handler]
#[no_mangle]
pub fn panic(_info : &PanicInfo) -> ! {
    let mut cursor = Cursor {
        position : 0,
        foreground : Color::White,
        background: Color::Red,
    };
    for _ in 0 .. (80*25) {
        cursor.print(b" ");
    }
    cursor.position = 0;
    write!(cursor, "{}", _info).unwrap();
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    /*let framebuffer = 0xb8000 as *mut u8;
    unsafe {
        framebuffer
            .offset(1)
            .write_volatile(0x30);
    }*/
    let text = b"Rust in Action";
    let mut cursor = Cursor {
        position : 0,
        foreground : Color::BrightCyan,
        background: Color::Black,
    };
    cursor.print(text);

    loop {
        hlt();
    }
}

