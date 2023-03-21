use std::{thread, time};
fn thread_test() {
    for n in 1..1001 {
        let mut handlers : Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);
        let start = time::Instant::now();
        for _m in 0..n {
            let handler = thread::spawn(|| {
                let pause = time::Duration::from_millis(20);
                thread::sleep(pause);
            });
            handlers.push(handler);
        }
        while let Some(handle) = handlers.pop() {
            handle.join();
        }
        let finish = time::Instant::now();
        println!("{}\t{:02?}", n, finish.duration_since(start));
    }
}

fn thread_spin_loop() {
    for n in 1..1001 {
        let mut handlers : Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);
        let start = time::Instant::now();
        for _m in 0..n {
            let handler = thread::spawn(|| {
                let start = time::Instant::now();
                let pause = time::Duration::from_millis(20);
                while start.elapsed() < pause {
                    thread::yield_now();
                }
            });
            handlers.push(handler);
        }
        while let Some(handle) = handlers.pop() {
            handle.join();
        }
        let finish = time::Instant::now();
        println!("{}\t{:02?}", n, finish.duration_since(start));
    }
}

#[macro_use]
extern crate crossbeam;
use crossbeam::channel::unbounded;
use crate::ConnectivityCheck::*;

fn channel_test() {
    let (tx, rx) = unbounded();
    thread::spawn(move || {
        tx.send(42).unwrap();
    });
    select! {
        recv(rx) -> msg => println!("message: {:?}", msg),
    }
}

#[derive(Debug)]
enum ConnectivityCheck {
    Ping,
    Pong,
    Pang,
}

fn channel_test2() {
    let n_messages = 3;
    let (request_tx, request_rx) = unbounded();
    let (response_tx, response_rx) = unbounded();
    thread::spawn(move || loop {
        match request_rx.recv().unwrap() {
            Pong => eprintln!("unexpected pong response"),
            Ping => response_tx.send(Pong).unwrap(),
            Pang => return,
        }
    });
    for _ in 0..n_messages {
        request_tx.send(Ping).unwrap();
    }
    request_tx.send(Pang).unwrap();
    for _ in 0 ..n_messages {
        select! {
            recv(response_rx) -> msg => println!("{:?}", msg),
        }
    }
}
fn main() {
    //thread_test();
    //thread_spin_loop();
    channel_test();
    channel_test2();
}
