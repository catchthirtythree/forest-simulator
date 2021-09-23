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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let (seed, width, height, total_months) = parse_arguments(&args)?;

    let mut forest = Forest::new(seed, width, height);

    while forest.months_elapsed != total_months {
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

fn parse_arguments(args: &Vec<String>) -> Result<(u64, usize, usize, u32), Box<dyn std::error::Error>> {
    const DEFAULT_WIDTH: usize = 12;
    const DEFAULT_HEIGHT: usize = 8;
    const DEFAULT_MONTHS: u32 = 4800;

    match args.len() {
        1 => Ok((rand::thread_rng().next_u64(), DEFAULT_WIDTH, DEFAULT_HEIGHT, DEFAULT_MONTHS)),
        2 => Ok((args[1].parse()?, DEFAULT_WIDTH, DEFAULT_HEIGHT, DEFAULT_MONTHS)),
        3 => Ok((args[1].parse()?, args[2].parse()?, args[2].parse::<usize>()? / 16 * 9, DEFAULT_MONTHS)),
        4 => Ok((args[1].parse()?, args[2].parse()?, args[3].parse()?, DEFAULT_MONTHS)),
        _ => Ok((args[1].parse()?, args[2].parse()?, args[3].parse()?, args[4].parse()?)),
    }
}
