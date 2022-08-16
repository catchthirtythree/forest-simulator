use forest_lib::{
    config::ForestConfig,
    consts::{BEAR_MASK, JACK_MASK, TREE_MASK},
    forest::Forest,
};
use rand::RngCore;
use std::{env, time::Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    let args: Vec<String> = env::args().collect();
    let config = parse_arguments(&args)?;

    let mut forest = Forest::new(config);

    forest.draw_map();

    while forest.months_elapsed < forest.config.months {
        forest.update();
    }

    println!();

    forest.draw_map();
    draw_info(&forest.map, forest.months_elapsed);

    let end_time = Instant::now() - start_time;
    println!("Time to run: {:?}", end_time);

    Ok(())
}

fn parse_arguments(args: &Vec<String>) -> Result<ForestConfig, Box<dyn std::error::Error>> {
    const DEFAULT_WIDTH: usize = 12;
    const DEFAULT_HEIGHT: usize = 8;
    const DEFAULT_MONTHS: u32 = 4800;

    let mut iter = args.iter();
    let _ = iter.next();

    let seed: u64 = match iter.next() {
        Some(seed) => seed.parse()?,
        None => rand::thread_rng().next_u64(),
    };

    let (width, height): (usize, usize) = match (iter.next(), iter.next()) {
        (None, _) => (DEFAULT_WIDTH, DEFAULT_HEIGHT),
        (Some(width), None) => (width.parse()?, DEFAULT_HEIGHT),
        (Some(width), Some(height)) => (width.parse()?, height.parse()?),
    };

    let months: u32 = match iter.next() {
        Some(months) => months.parse()?,
        None => DEFAULT_MONTHS,
    };

    Ok(ForestConfig::new(seed, width, height, months))
}

fn draw_info(map: &[u32], months_elapsed: u32) {
    println!(
        "{} | {}",
        get_formatted_time(months_elapsed),
        get_formatted_entities(map)
    )
}

fn get_formatted_time(months_elapsed: u32) -> String {
    let years: u32 = months_elapsed / 12;
    let months: u32 = months_elapsed % 12;
    format!("year {}, month {}", years, months)
}

fn get_formatted_entities(map: &[u32]) -> String {
    let mut num_bears = 0;
    let mut num_jacks = 0;
    let mut num_trees = 0;

    for i in 0..map.len() {
        let cell = map[i];

        if (cell & BEAR_MASK) > 0 {
            num_bears += 1;
        }

        if (cell & JACK_MASK) > 0 {
            num_jacks += 1;
        }

        if (cell & TREE_MASK) > 0 {
            num_trees += 1;
        }
    }

    format!(
        "bears {}, jacks {}, trees {}",
        num_bears, num_jacks, num_trees
    )
}
