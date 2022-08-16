use crate::{forest_config::ForestConfig, random::Random};

pub const STARTING_TREES: f32 = 0.50;
pub const STARTING_JACKS: f32 = 0.10;
pub const STARTING_BEARS: f32 = 0.02;

pub const NONE_MASK: u32 = 0x0000;
pub const BEAR_MASK: u32 = 0xF000;
pub const JACK_MASK: u32 = 0x0F00;
pub const TREE_MASK: u32 = 0x00FF;

pub const BEAR_REMOVE_MASK: u32 = 0x0FFF;
pub const JACK_REMOVE_MASK: u32 = 0xF0FF;
pub const TREE_REMOVE_MASK: u32 = 0xFF00;

pub const BEAR_SHIFT: u32 = 4 * 3;
pub const JACK_SHIFT: u32 = 4 * 2;
pub const TREE_SHIFT: u32 = 4 * 0;

pub const SAPLING_SPAWN_CHANCE: u32 = 0;
pub const MATURE_SPAWN_CHANCE: u32 = 10;
pub const ELDER_SPAWN_CHANCE: u32 = 20;

pub const SAPLING_HARVEST_CHANCE: u32 = 99;
pub const MATURE_HARVEST_CHANCE: u32 = 75;
pub const ELDER_HARVEST_CHANCE: u32 = 66;

pub const JACK_MAX_LEVEL: u32 = 5;

pub const SAPLING_GROW_AGE: u32 = 12;
pub const MATURE_GROW_AGE: u32 = 120;

pub const BEAR_WANDERS_PER_MONTH: u32 = 3;
pub const BEAR_WANDER_ATTEMPTS: u32 = 2;

pub const JACK_WANDERS_PER_MONTH: u32 = 3;
pub const JACK_WANDER_ATTEMPTS: u32 = 2;

enum TreeKind {
    None,
    Sapling,
    Mature,
    Elder,
}

pub struct Forest {
    pub config: ForestConfig,
    rng: Random,
    pub map: Vec<u32>,
    pub months_elapsed: u32,
    yearly_lumber: u32,
    yearly_mauls: u32,
}

impl Forest {
    pub fn new(config: ForestConfig) -> Self {
        let mut rng = Random::new(config.seed);
        let mut map = vec![NONE_MASK; config.width * config.height];

        Self::initialize_map(&mut rng, &mut map);

        Self {
            config,
            rng,
            map,
            months_elapsed: 0,
            yearly_lumber: 0,
            yearly_mauls: 0,
        }
    }

    fn initialize_map(rng: &mut Random, map: &mut [u32]) {
        let num_bears = f32::ceil(map.len() as f32 * STARTING_BEARS) as usize;
        for _ in 0..num_bears {
            Self::randomly_place_entity(rng, map, BEAR_MASK, BEAR_SHIFT);
        }

        let num_jacks = f32::ceil(map.len() as f32 * STARTING_JACKS) as usize;
        for _ in 0..num_jacks {
            Self::randomly_place_entity(rng, map, JACK_MASK, JACK_SHIFT);
        }

        let num_trees = f32::ceil(map.len() as f32 * STARTING_TREES) as usize;
        for _ in 0..num_trees {
            Self::randomly_place_entity(rng, map, TREE_MASK, TREE_SHIFT);
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

            if ((cell & mask) >> shift) == 0 {
                Self::place_entity(map, next, shift);
                break;
            }
        }
    }

    fn place_entity(map: &mut [u32], index: usize, shift: u32) {
        map[index] += 0x1 << shift;
    }

    pub fn draw_map(&self) {
        for i in 0..self.map.len() {
            if i > 0 && i % self.config.width == 0 {
                println!();
            }

            let cell = self.map[i];

            match (((cell & BEAR_MASK) >> BEAR_SHIFT), ((cell & JACK_MASK) >> JACK_SHIFT), Self::get_tree_kind(cell)) {
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
        let positions = Self::get_entity_positions(&map, TREE_MASK, TREE_SHIFT);
        for i in positions {
            let cell = map[i];

            Self::age_tree(map, i, cell);

            let spawn_chance = Self::get_sapling_spawn_chance(cell);
            let result = rng.next() as u32 % 100;

            if result <= spawn_chance {
                let adjacent_positions = Self::get_adjacent_positions(i, &config);
                let mut position_candidates = adjacent_positions.iter()
                    .filter(|&position| {
                        let cell = map[*position];
                        cell & TREE_MASK == 0
                    }).collect();

                if let Some(&choice) = rng.choose(&mut position_candidates) {
                    Self::place_entity(map, choice, TREE_SHIFT)
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

            positions.push(Self::convert_position_to_index(x as usize, y as usize, config.width));
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
        let kind = Self::get_tree_kind(cell);
        match kind {
            TreeKind::Sapling => SAPLING_SPAWN_CHANCE,
            TreeKind::Mature  => MATURE_SPAWN_CHANCE,
            TreeKind::Elder   => ELDER_SPAWN_CHANCE,
            TreeKind::None    => 0,
        }
    }

    fn trigger_jack_event(rng: &mut Random, map: &mut [u32], config: &ForestConfig, lumber: &mut u32) {
        let positions = Self::get_entity_positions(&map, JACK_MASK, JACK_SHIFT);
        for i in positions {
            let mut wanders = 0;
            let mut current_position = i;

            while wanders < JACK_WANDERS_PER_MONTH {
                let mut has_wandered = false;
                let mut wander_attempts = 0;

                let adjacent_positions = Self::get_adjacent_positions(current_position, &config);
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
                            Self::remove_entity(map, current_position, JACK_REMOVE_MASK);
                            Self::place_entity(map, next_position, JACK_SHIFT);

                            let chosen_cell = map[next_position];
                            if (chosen_cell & TREE_MASK) > 0 {
                                let result = rng.next() as u32 % 100;
                                if result < Self::get_tree_harvest_chance(chosen_cell) {
                                    let harvest_amount = Self::get_harvest_amount(chosen_cell);
                                    *lumber += harvest_amount;
                                    Self::remove_entity(map, next_position, TREE_REMOVE_MASK);
                                    Self::level_up_jack(map, next_position, harvest_amount);
                                } else {
                                    // let harvest_amount = Self::get_harvest_amount(chosen_cell) / 2;
                                    // *lumber += harvest_amount;
                                    Self::de_age_tree(map, next_position);
                                    // Self::level_up_jack(map, next_position, harvest_amount);
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
        match Self::get_tree_kind(cell) {
            TreeKind::Sapling => SAPLING_HARVEST_CHANCE,
            TreeKind::Mature  => MATURE_HARVEST_CHANCE,
            TreeKind::Elder   => ELDER_HARVEST_CHANCE,
            TreeKind::None    => 0,
        }
    }

    fn de_age_tree(map: &mut [u32], index: usize) {
        let cell = map[index];

        match Self::get_tree_kind(cell) {
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
        match Self::get_tree_kind(cell) {
            TreeKind::Sapling => 0,
            TreeKind::Mature  => 1,
            TreeKind::Elder   => 2,
            TreeKind::None    => 0,
        }
    }

    fn trigger_bear_event(rng: &mut Random, map: &mut [u32], config: &ForestConfig, mauls: &mut u32) {
        let positions = Self::get_entity_positions(&map, BEAR_MASK, BEAR_SHIFT);
        for i in positions {
            let mut wanders = 0;
            let mut current_position = i;

            while wanders < BEAR_WANDERS_PER_MONTH {
                let mut has_wandered = false;
                let mut wander_attempts = 0;

                let adjacent_positions = Self::get_adjacent_positions(current_position, &config);
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
                            Self::remove_entity(map, current_position, BEAR_REMOVE_MASK);
                            Self::place_entity(map, next_position, BEAR_SHIFT);

                            let chosen_cell = map[next_position];
                            if (chosen_cell & JACK_MASK) > 0 {
                                let result = rng.next() as u32 % 100;
                                if result < Self::get_jack_maul_chance(chosen_cell) {
                                    *mauls += 1;
                                    Self::remove_entity(map, next_position, JACK_REMOVE_MASK);
                                } else {
                                    Self::de_level_jack(map, next_position);
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
            let jacks = Self::get_entity_positions(&map, JACK_MASK, JACK_SHIFT);
            if *lumber as usize > jacks.len() {
                let excess_lumber = *lumber as usize - jacks.len();
                let new_lumberjacks = excess_lumber / 10;

                for _ in 0..new_lumberjacks {
                    if let Some(index) = Self::get_open_space(rng, map) {
                        Self::place_entity(map, index, JACK_SHIFT);
                    }
                }
            } else {
                if jacks.len() > 1 {
                    if let Some(index) = rng.choose(&jacks) {
                        Self::remove_entity(map, index, JACK_REMOVE_MASK);
                    }
                }
            }
        }

        {
            let bears = Self::get_entity_positions(&map, BEAR_MASK, BEAR_SHIFT);
            if *mauls as usize == 0 {
                if let Some(index) = Self::get_open_space(rng, map) {
                    Self::place_entity(map, index, BEAR_SHIFT);
                }
            } else {
                if bears.len() > 1 {
                    if let Some(index) = rng.choose(&bears) {
                        Self::remove_entity(map, index, BEAR_REMOVE_MASK);
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

    pub fn update(&mut self) {
        self.months_elapsed += 1;

        Self::trigger_tree_event(&mut self.rng, &mut self.map, &self.config);
        Self::trigger_jack_event(&mut self.rng, &mut self.map, &self.config, &mut self.yearly_lumber);
        Self::trigger_bear_event(&mut self.rng, &mut self.map, &self.config, &mut self.yearly_mauls);

        if self.months_elapsed % 12 == 0 {
            Self::trigger_yearly_events(&mut self.rng, &mut self.map, &mut self.yearly_lumber, &mut self.yearly_mauls);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
