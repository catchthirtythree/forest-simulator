#![allow(dead_code)]

use rand::RngCore;
use std::time::Instant;
use std::env;

use random::Random;

mod random;

const STARTING_TREES: f32 = 0.50;
const STARTING_JACKS: f32 = 0.10;
const STARTING_BEARS: f32 = 0.02;

const NONE_MASK: u32 = 0x0000;
const BEAR_MASK: u32 = 0xF000;
const JACK_MASK: u32 = 0x0F00;
const TREE_MASK: u32 = 0x00FF;

const BEAR_REMOVE_MASK: u32 = 0x0FFF;
const JACK_REMOVE_MASK: u32 = 0xF0FF;
const TREE_REMOVE_MASK: u32 = 0xFF00;

const BEAR_SHIFT: u32 = 4 * 3;
const JACK_SHIFT: u32 = 4 * 2;
const TREE_SHIFT: u32 = 4 * 0;

const SAPLING_SPAWN_CHANCE: u32 = 0;
const MATURE_SPAWN_CHANCE: u32 = 10;
const ELDER_SPAWN_CHANCE: u32 = 20;

const SAPLING_HARVEST_CHANCE: u32 = 99;
const MATURE_HARVEST_CHANCE: u32 = 75;
const ELDER_HARVEST_CHANCE: u32 = 66;

const JACK_MAX_LEVEL: u32 = 5;

const SAPLING_GROW_AGE: u32 = 12;
const MATURE_GROW_AGE: u32 = 120;

const BEAR_WANDERS_PER_MONTH: u32 = 3;
const BEAR_WANDER_ATTEMPTS: u32 = 2;

const JACK_WANDERS_PER_MONTH: u32 = 3;
const JACK_WANDER_ATTEMPTS: u32 = 2;

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
    let start_time = Instant::now();

    let args: Vec<String> = env::args().collect();
    let config = parse_arguments(&args)?;

    let mut rng = Random::new(config.seed);
    let mut map = vec![NONE_MASK; config.width * config.height];

    initialize_map(&mut rng, &mut map);

    // println!("Bear on the map - {:b}", 0x1000);
    // println!("Jack on the map - {:b}", 0x0100);
    // println!("Tree on the map - {:b}", 0x0001);

    // println!("Initial bears - {}", f32::ceil(map.len() as f32 * STARTING_BEARS));
    // println!("Initial jacks - {}", f32::ceil(map.len() as f32 * STARTING_JACKS));
    // println!("Initial trees - {}", f32::ceil(map.len() as f32 * STARTING_TREES));

    // let position = map.len() / 5;
    // map[position] = 120 + (1 << 8) + (1 << 12);
    // println!("map[position]                     - {:b}", map[position]);
    // println!("(map[position] & BEAR_MASK) >> 12 - {:b}", (map[position] & BEAR_MASK) >> 12);
    // println!("(map[position] & BEAR_MASK) >> 12 - {:x}", (map[position] & BEAR_MASK) >> 12);
    // println!("(map[position] & JACK_MASK) >> 8  - {:b}", (map[position] & JACK_MASK) >> 8);
    // println!("(map[position] & JACK_MASK) >> 8  - {:x}", (map[position] & JACK_MASK) >> 8);
    // println!("map[position] & TREE_MASK         - {:b}", map[position] & TREE_MASK);
    // println!("map[position] & TREE_MASK         - {:x}", map[position] & TREE_MASK);

    draw_map(&map, config.width);

    let mut months_elapsed = 0;
    let mut yearly_lumber = 0;
    let mut yearly_mauls = 0;

    while months_elapsed < config.months {
        months_elapsed += 1;

        trigger_tree_event(&mut rng, &mut map, &config);
        trigger_jack_event(&mut rng, &mut map, &config, &mut yearly_lumber);
        trigger_bear_event(&mut rng, &mut map, &config, &mut yearly_mauls);

        if months_elapsed % 12 == 0 {
            trigger_yearly_events(&mut rng, &mut map, &mut yearly_lumber, &mut yearly_mauls);
        }
    }

    println!();

    draw_map(&map, config.width);
    draw_info(&map, months_elapsed);

    let end_time = Instant::now() - start_time;
    println!("Time to run: {:?}", end_time);

    Ok(())
}

fn parse_arguments(
    args: &Vec<String>
) -> Result<ForestConfig, Box<dyn std::error::Error>> {
    const DEFAULT_WIDTH: usize = 12;
    const DEFAULT_HEIGHT: usize = 8;
    const DEFAULT_MONTHS: u32 = 4800;

    let mut iter = args.iter();
    let _ = iter.next();

    let seed: u64 = match iter.next() {
        Some(seed) => seed.parse()?,
        None       => rand::thread_rng().next_u64()
    };

    let (width, height): (usize, usize) = match (iter.next(), iter.next()) {
        (None, _)                   => (DEFAULT_WIDTH, DEFAULT_HEIGHT),
        (Some(width), None)         => (width.parse()?, DEFAULT_HEIGHT),
        (Some(width), Some(height)) => (width.parse()?, height.parse()?),
    };

    let months: u32 = match iter.next() {
        Some(months) => months.parse()?,
        None         => DEFAULT_MONTHS
    };

    Ok(ForestConfig::new(seed, width, height, months))
}

fn initialize_map(rng: &mut Random, map: &mut [u32]) {
    let num_bears = f32::ceil(map.len() as f32 * STARTING_BEARS) as usize;
    for _ in 0..num_bears {
        randomly_place_entity(rng, map, BEAR_MASK, BEAR_SHIFT);
    }

    let num_jacks = f32::ceil(map.len() as f32 * STARTING_JACKS) as usize;
    for _ in 0..num_jacks {
        randomly_place_entity(rng, map, JACK_MASK, JACK_SHIFT);
    }

    let num_trees = f32::ceil(map.len() as f32 * STARTING_TREES) as usize;
    for _ in 0..num_trees {
        randomly_place_entity(rng, map, TREE_MASK, TREE_SHIFT);
    }
}

fn randomly_place_entity(rng: &mut Random, map: &mut [u32], mask: u32, shift: u32) {
    let num_bears: u32 = map.iter()
        .map(|cell| (cell & mask) >> shift)
        .sum();

    if num_bears as usize == map.len() {
        return;
    }

    loop {
        let next = rng.next() as usize % map.len();
        let cell = map[next];
        if ((cell & mask) >> shift) > 0 {
            continue;
        }

        map[next] += 0x1 << shift;
        break;
    }
}

fn place_entity(map: &mut [u32], index: usize, shift: u32) {
    map[index] += 0x1 << shift;
}

enum TreeKind {
    None,
    Sapling,
    Mature,
    Elder,
}

fn draw_map(map: &[u32], width: usize) {
    for i in 0..map.len() {
        if i > 0 && i % width == 0 {
            println!();
        }

        let cell = map[i];

        match (((cell & BEAR_MASK) >> BEAR_SHIFT), ((cell & JACK_MASK) >> JACK_SHIFT), get_tree_kind(cell)) {
            (0, 0, TreeKind::None)    => print!("."),
            (1, 0, TreeKind::None)    => print!("B"),
            (0, _, TreeKind::None)    => print!("@"),
            (0, 0, TreeKind::Sapling) => print!("t"),
            (0, 0, TreeKind::Mature)  => print!("T"),
            (0, 0, TreeKind::Elder)   => print!("E"),
            (1, _, TreeKind::None)    => print!("3"),
            (1, 0, _)                 => print!("4"),
            (0, _, _)                 => print!("5"),
            (1, _, _)                 => print!("6"),
            (_, _, _)                 => print!("?"),
        }
    }

    println!();
}

fn get_tree_kind(cell: u32) -> TreeKind {
    match cell & TREE_MASK {
        0   => TreeKind::None,
        age => {
            if age < SAPLING_GROW_AGE {
                TreeKind::Sapling
            } else if age < MATURE_GROW_AGE {
                TreeKind::Mature
            } else {
                TreeKind::Elder
            }
        },
    }
}

fn trigger_tree_event(rng: &mut Random, map: &mut [u32], config: &ForestConfig) {
    let positions = get_entity_positions(&map, TREE_MASK, TREE_SHIFT);
    for i in positions {
        let cell = map[i];

        age_tree(map, i, cell);

        let spawn_chance = get_sapling_spawn_chance(cell);
        let result = rng.next() as u32 % 100;

        if result <= spawn_chance {
            let adjacent_positions = get_adjacent_positions(i, &config);
            let mut position_candidates = adjacent_positions.iter()
                .filter(|&position| {
                    let cell = map[*position];
                    cell & TREE_MASK == 0
                }).collect();

            if let Some(&choice) = rng.choose(&mut position_candidates) {
                place_entity(map, choice, TREE_SHIFT)
            }
        }
    }
}

fn age_tree(map: &mut [u32], index: usize, cell: u32) {
    let tree_age = cell & TREE_MASK;
    if tree_age < 255 {
        map[index] += 0x1;
    }
}

fn get_entity_positions(map: &[u32], mask: u32, shift: u32) -> Vec<usize> {
    (0..map.len()).into_iter()
        .filter(|&i| ((map[i] & mask) >> shift) > 0)
        .collect::<Vec<usize>>()
}

fn get_adjacent_positions(index: usize, config: &ForestConfig) -> Vec<usize> {
    let mut positions: Vec<usize> = vec![];

    let adjacent_movements: Vec<(isize, isize)> = vec![
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];

    let x = (index % config.width) as usize;
    let y = (index / config.width) as usize;

    for movement in adjacent_movements {
        let x = x as isize + movement.0;
        let y = y as isize + movement.1;
        if x < 0 || y < 0 {
            continue;
        }

        let x = x as usize;
        let y = y as usize;
        if x >= config.width || y >= config.height {
            continue;
        }

        positions.push(convert_position_to_index(x as usize, y as usize, config.width));
    }

    positions
}

fn convert_index_to_position(index: usize, width: usize) -> (usize, usize) {
    (index % width, index / width)
}

fn convert_position_to_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn get_sapling_spawn_chance(cell: u32) -> u32 {
    let kind = get_tree_kind(cell);
    match kind {
        TreeKind::Sapling => SAPLING_SPAWN_CHANCE,
        TreeKind::Mature  => MATURE_SPAWN_CHANCE,
        TreeKind::Elder   => ELDER_SPAWN_CHANCE,
        TreeKind::None    => 0,
    }
}

fn trigger_jack_event(rng: &mut Random, map: &mut [u32], config: &ForestConfig, lumber: &mut u32) {
    let positions = get_entity_positions(&map, JACK_MASK, JACK_SHIFT);
    for i in positions {
        let mut wanders = 0;
        let mut current_position = i;

        while wanders < JACK_WANDERS_PER_MONTH {
            let mut has_wandered = false;
            let mut wander_attempts = 0;

            let adjacent_positions = get_adjacent_positions(current_position, &config);
            let mut position_candidates: Vec<&usize> = adjacent_positions.iter()
                .filter(|&position| {
                    let cell = map[*position];
                    (cell & JACK_MASK) == 0
                }).collect();

            if position_candidates.len() == 0 {
                break;
            }

            while wander_attempts < JACK_WANDER_ATTEMPTS && !has_wandered {
                match rng.choose(&mut position_candidates) {
                    Some(&next_position) => {
                        remove_entity(map, current_position, JACK_REMOVE_MASK);
                        place_entity(map, next_position, JACK_SHIFT);

                        let chosen_cell = map[next_position];
                        if (chosen_cell & TREE_MASK) > 0 {
                            let result = rng.next() as u32 % 100;
                            if result < get_tree_harvest_chance(chosen_cell) {
                                let harvest_amount = get_harvest_amount(chosen_cell);
                                *lumber += harvest_amount;
                                remove_entity(map, next_position, TREE_REMOVE_MASK);
                                level_up_jack(map, next_position, harvest_amount);
                            } else {
                                // let harvest_amount = get_harvest_amount(chosen_cell) / 2;
                                // *lumber += harvest_amount;
                                de_age_tree(map, next_position);
                                // level_up_jack(map, next_position, harvest_amount);
                            }

                            wanders = JACK_WANDERS_PER_MONTH;
                        } else {
                            wanders += 1;
                        }

                        current_position = next_position;
                        has_wandered = true;
                    },
                    None => {
                        wander_attempts += 1;
                    }
                }
            }

            if !has_wandered {
                break;
            }
        }
    }
}

fn get_tree_harvest_chance(cell: u32) -> u32 {
    match get_tree_kind(cell) {
        TreeKind::Sapling => SAPLING_HARVEST_CHANCE,
        TreeKind::Mature  => MATURE_HARVEST_CHANCE,
        TreeKind::Elder   => ELDER_HARVEST_CHANCE,
        TreeKind::None    => 0,
    }
}

fn de_age_tree(map: &mut [u32], index: usize) {
    let cell = map[index];

    match get_tree_kind(cell) {
        TreeKind::Sapling => {
            map[index] -= (cell & TREE_MASK) - 1;
        },
        TreeKind::Mature  => {
            map[index] -= (cell & TREE_MASK) - SAPLING_GROW_AGE;
        },
        TreeKind::Elder   => {
            map[index] -= (cell & TREE_MASK) - MATURE_GROW_AGE;
        },
        TreeKind::None    => return,
    }
}

fn level_up_jack(map: &mut [u32], index: usize, lumber: u32) {
    let cell = map[index];
    let level = (cell & JACK_MASK) >> JACK_SHIFT;

    if level <= JACK_MAX_LEVEL {
        let level = u32::min(level + lumber, JACK_MAX_LEVEL);
        map[index] &= JACK_REMOVE_MASK;
        map[index] += level << JACK_SHIFT;
    }
}

fn remove_entity(map: &mut [u32], index: usize, remove_mask: u32) {
    map[index] &= remove_mask;
}

fn get_harvest_amount(cell: u32) -> u32 {
    match get_tree_kind(cell) {
        TreeKind::Sapling => 0,
        TreeKind::Mature  => 1,
        TreeKind::Elder   => 2,
        TreeKind::None    => 0,
    }
}

fn trigger_bear_event(rng: &mut Random, map: &mut [u32], config: &ForestConfig, mauls: &mut u32) {
    let positions = get_entity_positions(&map, BEAR_MASK, BEAR_SHIFT);
    for i in positions {
        let mut wanders = 0;
        let mut current_position = i;

        while wanders < BEAR_WANDERS_PER_MONTH {
            let mut has_wandered = false;
            let mut wander_attempts = 0;

            let adjacent_positions = get_adjacent_positions(current_position, &config);
            let mut position_candidates: Vec<&usize> = adjacent_positions.iter()
                .filter(|&position| {
                    let cell = map[*position];
                    (cell & BEAR_MASK) == 0
                }).collect();

            if position_candidates.len() == 0 {
                break;
            }

            while wander_attempts < BEAR_WANDER_ATTEMPTS && !has_wandered {
                match rng.choose(&mut position_candidates) {
                    Some(&next_position) => {
                        remove_entity(map, current_position, BEAR_REMOVE_MASK);
                        place_entity(map, next_position, BEAR_SHIFT);

                        let chosen_cell = map[next_position];
                        if (chosen_cell & JACK_MASK) > 0 {
                            let result = rng.next() as u32 % 100;
                            if result < get_jack_maul_chance(chosen_cell) {
                                *mauls += 1;
                                remove_entity(map, next_position, JACK_REMOVE_MASK);
                            } else {
                                de_level_jack(map, next_position);
                            }

                            wanders = BEAR_WANDERS_PER_MONTH;
                        } else {
                            wanders += 1;
                        }

                        current_position = next_position;
                        has_wandered = true;
                    },
                    None => {
                        wander_attempts += 1;
                    }
                }
            }

            if !has_wandered {
                break;
            }
        }
    }
}

fn get_jack_maul_chance(cell: u32) -> u32 {
    let level = (cell & JACK_MASK) >> JACK_SHIFT;
    100 - (level * 10)
}

fn de_level_jack(map: &mut [u32], index: usize) {
    let cell = map[index];
    let level = cell & JACK_MASK;

    if level > 1 {
        map[index] &= JACK_REMOVE_MASK;
        map[index] += (level - 1) << JACK_SHIFT;
    }
}

fn trigger_yearly_events(rng: &mut Random, map: &mut [u32], lumber: &mut u32, mauls: &mut u32) {
    {
        let jacks = get_entity_positions(&map, JACK_MASK, JACK_SHIFT);
        if *lumber as usize > jacks.len() {
            let excess_lumber = *lumber as usize - jacks.len();
            let new_lumberjacks = excess_lumber / 10;

            for _ in 0..new_lumberjacks {
                if let Some(index) = get_open_space(rng, map) {
                    place_entity(map, index, JACK_SHIFT);
                }
            }
        } else {
            if jacks.len() > 1 {
                if let Some(index) = rng.choose(&jacks) {
                    remove_entity(map, index, JACK_REMOVE_MASK);
                }
            }
        }
    }

    {
        let bears = get_entity_positions(&map, BEAR_MASK, BEAR_SHIFT);
        if *mauls as usize == 0 {
            if let Some(index) = get_open_space(rng, map) {
                place_entity(map, index, BEAR_SHIFT);
            }
        } else {
            if bears.len() > 1 {
                if let Some(index) = rng.choose(&bears) {
                    remove_entity(map, index, BEAR_REMOVE_MASK);
                }
            }
        }
    }

    *lumber = 0;
    *mauls = 0;
}

fn get_open_space(rng: &mut Random, map: &[u32]) -> Option<usize> {
    let mut spaces: Vec<usize> = vec![];
    for i in 0..map.len() {
        let cell = map[i];
        if cell > 0 {
            continue;
        }
        spaces.push(i);
    }
    rng.choose(&spaces)
}

fn draw_info(map: &[u32], months_elapsed: u32) {
    println!("{} | {}", get_formatted_time(months_elapsed), get_formatted_entities(map))
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

    format!("bears {}, jacks {}, trees {}", num_bears, num_jacks, num_trees)
}
