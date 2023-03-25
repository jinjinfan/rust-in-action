use std::time;
use std::process;
use std::thread::{sleep};

fn process_interrupt() {
    let delay = time::Duration::from_secs(1);
    let pid = process::id();
    println!("{}", pid);

    for i in 1..=60 {
        sleep(delay);
        println!(". {}", i);
    }
}

use rand;
static mut SHUT_DOWN : bool = false;

fn global_test() {
    loop {
        unsafe {
            SHUT_DOWN = rand::random();
        }
        print!(".");
        if unsafe {SHUT_DOWN} {
          break;  
        };
    }
}

#[cfg(not(windows))]
use std::time::{Duration};
use libc::{SIGTERM, SIGUSR1};


fn signal_test() {
    register_signal_handlers();
    let delay = Duration::from_secs(1);
    let pid = process::id();
    println!("{}", pid);

    for i in 1_usize.. {
        println!(". {}", i);
        unsafe {
            if SHUT_DOWN {
                println!("*");
                return;
            }
        }
        sleep(delay);
        let signal = if i > 2 {
            SIGTERM
        } else {
            SIGUSR1
        };
        unsafe {
            libc::raise(signal);
        }
    }
    unreachable!();
}

fn register_signal_handlers() {
    unsafe {
        libc::signal(SIGTERM, handle_sigterm as usize);
        libc::signal(SIGUSR1, handle_sigusr1 as usize);
    }
}

#[allow(dead_code)]
fn handle_sigterm(_signal : i32) {
    register_signal_handlers();
    println!("SIGTERM");
    unsafe {
        SHUT_DOWN = true;
    }
}

#[allow(dead_code)]
fn handle_sigusr1(_signal : i32) {
    register_signal_handlers();
    println!("SIGUSR1");
}

fn noop() {}
 
fn fn_address() {
  let fn_ptr = noop as usize;
  let typed_fn_ptr = noop as *const fn() -> ();
 
  println!("noop as usize:    0x{:x}", fn_ptr);
  println!("noop as *const T: {:p}", typed_fn_ptr);
}

use libc::{signal, raise};
use libc::{SIG_DFL, SIG_IGN};
fn signal_ignore() {
    unsafe {
        signal(SIGTERM, SIG_IGN);
        raise(SIGTERM);
    }
    println!("ok");
    unsafe {
        signal(SIGTERM, SIG_DFL);
        raise(SIGTERM);
    }
    println!("not ok");
}

fn print_depth(depth: usize) {
    for _ in 0..depth {
        print!("#");
    }
    println!("");
}

fn dive(depth: usize, max_depth: usize) {
    print_depth(depth);
    if depth >= max_depth {
        return;
    } else {
        dive(depth+1, max_depth);
    }
    print_depth(depth);
}
fn main() {
    //signal_test();
    //fn_address();
    //signal_ignore();
    dive(0,5);
}