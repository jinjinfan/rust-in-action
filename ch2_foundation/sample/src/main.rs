use std::vec;

use num::complex::Complex;

fn calculate_mandelbrot (
    max_iters : usize,
    x_min : f64,
    x_max : f64,
    y_min : f64,
    y_max : f64,
    width : usize,
    height : usize
)-> Vec<Vec<usize>> {
    let mut rows : Vec<_> = Vec::with_capacity(width);
    for img_y in 0 .. height {
        let mut row : Vec<usize> = Vec::with_capacity(height);
        for img_x in 0 .. width {
            let x_percentage = (img_x as f64 / width as f64);
            let y_percentage = (img_y as f64 / height as f64);
            let cx = x_min + (x_max -x_min) * x_percentage;
            let cy = y_min + (y_max -y_min) * y_percentage;
            let escape_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escape_at);
        }
        rows.push(row);
    }
    rows
}

fn mandelbrot_at_point (
    cx : f64,
    cy : f64,
    max_iters : usize
) -> usize {
    let mut z = Complex {re : 0.0, im : 0.0};
    let c = Complex::new(cx,cy);
    for i in 0 ..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iters
}

fn render_mandelbrot(escape_vals : Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '@',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ =>'%',
            };
            line.push(val);
        }
        println!("{}", line);
    }
}

fn grep_lite() {
    let search_term = "picture";
    let quote = "\
    Every face, every shop, bedroom window, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books.
    What do we seek through millions of pages?";
    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            println!("{} : {}", i+1, line);
        }
    }
}

fn grep_number() {
    let ctx_line = 2;
    let needle = "oo";
    let haystack = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";
    let mut tags :Vec<usize> = vec![];
    let mut ctx : Vec<Vec<(usize, String)>> = vec![];
    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);
            let v = Vec::with_capacity(2*ctx_line + 1);
            ctx.push(v);
        }
    }
    if tags.is_empty() {
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_line);
            let upper_bound = tag + ctx_line;
            if (i >= lower_bound) && (i <=upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }
    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            println!("{}: {}", i + 1, line);
        }
    }

}

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App, Arg};

fn main() {
    //
    //let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 100, 24);
    //render_mandelbrot(mandelbrot);
    //
    //grep_lite();
    //
    //grep_number();
    //
    let args = App::new("grep-lite")
        .version("1.0")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .help("File to search")
            .takes_value(true)
            .required(true))
        .get_matches();
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap_or("-");

    if input == "-" {
        let stdin = std::io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}

fn process_lines<T: BufRead + Sized>(reader : T, re : Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
