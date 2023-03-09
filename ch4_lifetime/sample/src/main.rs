
#[derive(Debug)]
struct CubeSat {
    id : u64,
}

#[derive(Debug)]
enum StatusMessage {
    OK,
}

#[derive(Debug)]
struct Mailbox {
    messages : Vec<Message>,
}
#[derive(Debug)]
struct Message {
    to : u64,
    content : String,
}
struct GroundStation;

impl GroundStation {
    fn send(&self, mailbox : &mut Mailbox, msg : Message) {
        mailbox.post(msg);
    }
    fn connect(&self, sat_id : u64) -> CubeSat {
        CubeSat { id: sat_id }
    }
}

impl CubeSat {
    fn recv(&self, mailbox : &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}
impl Mailbox {
    fn post(&mut self, msg : Message) {
        self.messages.push(msg);
    }
    fn deliver(&mut self, recipient : &CubeSat) ->Option<Message> {
        for i in 0 .. self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}
fn check_status(sat_id : u64) -> StatusMessage {
    StatusMessage::OK
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1,2,3]
}

use std::rc::Rc;
use std::cell::{RefCell};

#[derive(Debug)]
struct GroundStation2 {
    radio_freq : f64
}
fn main() {
    let base = GroundStation {};
    
    /*
    let mut sat_a = CubeSat {
        id : 0,
        mailbox : Mailbox { messages: vec![] },
    };
    println!("t0: {:?}", sat_a);
    base.send(&mut sat_a, Message::from("hello there!"));
    println!("t1: {:?}", sat_a);
    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);
    println!("msg: {:?}", msg);
    */
    //
    let mut mail = Mailbox{messages : vec![]};
    let sat_ids = fetch_sat_ids();
    for sat_id in &sat_ids {
        let mut sat = base.connect(*sat_id);
        let msg = Message {to : *sat_id, content: String::from("hello")};
        base.send(&mut mail, msg);
    }
    for sat_id in &sat_ids {
        let sat = base.connect(*sat_id);
        let msg = sat.recv(&mut mail);
        println!("{:?}: {:?}", sat, msg);
    }

    let base : Rc<RefCell<GroundStation2>> = Rc::new(RefCell::new(
        GroundStation2 {
            radio_freq : 87.65
        }
    ));
    println!("base: {:?}", base);
    {
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -= 12.34;
        println!("base_2 : {:?}", base_2);
    }
    println!("base: {:?}", base);
    let mut base_3 = base.borrow_mut();
    base_3.radio_freq += 43.21;
    println!("base: {:?}", base);
    println!("base_3: {:?}", base_3);

}
