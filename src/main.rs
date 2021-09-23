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

fn main() {
    const TOTAL_MONTHS: u32 = 4800;

    let mut forest = Forest::new(10);

    while forest.months_elapsed != TOTAL_MONTHS {
        // print!("\x1B[2J\x1B[1;1H");

        forest.update();

        // println!("{}", forest);

        // std::thread::sleep(std::time::Duration::from_millis(500));
    }

    println!("{}", forest);
}
