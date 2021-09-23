#![allow(dead_code)]
#![allow(unused_variables)]

mod bear;
mod entity;
mod forest;
mod grid;
mod lumberjack;
mod random;
mod tree;

use crate::forest::Forest;

use rand::RngCore;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (seed, size, total_months): (u64, usize, u32) = match args.len() {
        1 => (rand::thread_rng().next_u64(), 10, 4800),
        2 => (args[1].parse().unwrap(), 10, 4800),
        3 => (args[1].parse().unwrap(), args[2].parse().unwrap(), 4800),
        _ => (args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap()),
    };

    let mut forest = Forest::new(seed, size);

    while forest.months_elapsed != total_months {
        print!("\x1B[2J\x1B[1;1H");

        forest.update();

        println!("{}", forest);

        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
