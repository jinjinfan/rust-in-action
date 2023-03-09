use std::mem::size_of;
use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_char;

static B:[u8;10] = [99,97,114,114,121,116,111,119,101,108];
static C:[u8;11] = [106,104,97, 110, 107, 115, 102, 105, 115, 114, 0];

fn memory_test() {
    let a :usize = 42;
    let b : &[u8;10] = &B;
    let c : Box<[u8]> = Box::new(C);
    println!("a (an unsigned integer)");
    println!(" location : {:p}", &a);
    println!(" size: {:?} bytes", size_of::<usize>());
    println!(" value: {:?}", a);
    println!();
    println!("b (a reference to B)");
    println!(" location : {:p}", &b);
    println!(" size: {:?} bytes", size_of::<&[u8;10]>());
    println!(" points to: {:p}", b);
    println!();
    println!("c (a box of C):");
    println!(" location : {:p}", &c);
    println!(" size: {:?} bytes", size_of::<Box<[u8]>>());
    println!(" points to: {:p}", c);
    println!();

    /*let d : String;
    let e: Cow<str>;
    unsafe {
        let b_ptr = &B as *const u8 as *mut u8;
        d = String::from_raw_parts(b_ptr, 10, 10);
        let c_ptr = &C as *const u8 as *const c_char;
        e = CStr::from_ptr(c_ptr).to_string_lossy();
    }
    println!("d : {} e :{}", d,e);*/
}

use graphics::math::{Vec2d, add,mul_scalar};
use piston_window::*;
use rand::prelude::*;
use std::alloc::{GlobalAlloc, System, Layout};
use std::time::Instant;

#[global_allocator]
static ALLOCATOR : ReportingAllocator = ReportingAllocator;
struct ReportingAllocator;

unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc(&self, layout :Layout) -> *mut u8 {
        let start = Instant::now();
        let ptr = System.alloc(layout);
        let end = Instant::now();
        let time_taken = end - start;
        let bytes_requested = layout.size();
        eprintln!("{}\t{}", bytes_requested, time_taken.as_nanos());
        ptr
    }
    unsafe fn dealloc(&self, ptr : *mut u8, layout :Layout) {
        System.dealloc(ptr, layout);
    }
}

struct Particle {
    height : f64,
    width : f64,
    position : Vec2d<f64>,
    velocity : Vec2d<f64>,
    acceleration : Vec2d<f64>,
    color : [f32;4],
}
impl Particle {
    fn new(world : &World) -> Particle {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..=world.width);
        let y = world.height;
        let x_velocity = 0.0;
        let y_velocity = rng.gen_range(-2.0..0.0);
        let x_acceleration = 0.0;
        let y_acceleration = rng.gen_range(0.0..0.15);
        Particle { height: 4.0, width: 4.0, 
                position: [x,y].into(), 
                velocity: [x_velocity,y_velocity].into(), 
                acceleration: [x_acceleration, y_acceleration].into(), 
                color: [1.0, 1.0, 1.0, 0.99] }
    }
    fn update(&mut self) {
        self.velocity = add(self.velocity, self.acceleration);
        self.position = add(self.position, self.velocity);
        self.acceleration = mul_scalar(self.acceleration, 0.7);
        self.color[3] *= 0.995;
    }
}
struct World {
    current_turn : u64,
    particles : Vec<Box<Particle>>,
    height : f64,
    width : f64,
    rng  : ThreadRng,
}

impl World {
    fn new(width:f64, height:f64) -> World {
        World { current_turn: 0, 
            particles: Vec::<Box<Particle>>::new(), 
            height: height, 
            width: width, 
            rng: thread_rng(), }
    }
    fn add_shapes(&mut self, n:i32) {
        for _ in 0..n.abs() {
            let particle = Particle::new(&self);
            let boxed_particle = Box::new(particle);
            self.particles.push(boxed_particle);
        }
    }
    fn remove_shapes(&mut self, n : i32) {
        for _ in 0..n.abs() {
            let mut to_delete = None;
            let particle_iter = self.particles
                .iter().enumerate();
            for (i, particle) in particle_iter {
                if particle.color[3] < 0.02 {
                    to_delete = Some(i);
                }
                break;
            }
            if let Some(i) = to_delete {
                self.particles.remove(i);
            } else{
                self.particles.remove(0);
            }
        }
    }
    fn update(&mut self) {
        let n = self.rng.gen_range(-3..=3);
        if n > 0  {
            self.add_shapes(n);
        } else{
            self.remove_shapes(n);
        }
        self.particles.shrink_to_fit();
        for shape in &mut self.particles {
            shape.update();
        }
        self.current_turn += 1;
    }
}
fn analyze_alloc_dealloc() {
    let (width, height) = (1280.0, 960.0);
    let mut window : PistonWindow = WindowSettings::new(
        "particles", [width, height]
    )
    .exit_on_esc(true)
    .build()
    .expect("Could not create a window");

    let mut world = World::new(width, height);
    world.add_shapes(1000);
    while let Some(event)  = window.next() {
        world.update();
        window.draw_2d(&event, |ctx, renderer, _device|{
            clear([0.15, 0.17, 0.17, 0.9], renderer);
            for s in &mut world.particles {
                let size = [s.position[0], s.position[1],s.width, s.height];
                rectangle(s.color, size, ctx.transform, renderer);
            }
        });
    }
}
fn main() {
    memory_test();
    analyze_alloc_dealloc();
}