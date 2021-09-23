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

struct ForestConfig {
    seed: u64,
    width: usize,
    height: usize,
    months: u32,
}

impl ForestConfig {
    fn new(seed: u64, width: usize, height: usize, months: u32) -> Self {
        Self { seed, width, height, months }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config = parse_arguments(&args)?;

    let mut forest = Forest::new(config.seed, config.width, config.height);

    while forest.months_elapsed != config.months {
        print!("\x1B[2J\x1B[1;1H");

        forest.update();

        println!("{}\n{}", forest, format_time(&forest));

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ok(())
}

fn format_time(forest: &Forest) -> String {
    let years: u32 = forest.months_elapsed / 12;
    let months: u32 = forest.months_elapsed % 12;

    format!("year {}, month {}", years, months)
}

fn parse_arguments(args: &Vec<String>) -> Result<ForestConfig, Box<dyn std::error::Error>> {
    const DEFAULT_WIDTH: usize = 12;
    const DEFAULT_HEIGHT: usize = 8;
    const DEFAULT_MONTHS: u32 = 4800;

    let mut iter = args.iter();
    let _ = iter.next();

    let seed: u64 = match iter.next() {
        Some(seed) => seed.parse()?,
        None       => rand::thread_rng().next_u64()
    };

    let width: usize = match iter.next() {
        Some(width) => width.parse()?,
        None        => DEFAULT_WIDTH
    };

    let height: usize = match iter.next() {
        Some(height) => height.parse()?,
        None         => (width as f32 / 16. * 9.) as usize
    };

    let months: u32 = match iter.next() {
        Some(months) => months.parse()?,
        None         => DEFAULT_MONTHS
    };

    Ok(ForestConfig::new(seed, width, height, months))
}
