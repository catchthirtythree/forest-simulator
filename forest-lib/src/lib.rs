mod random;

pub mod consts {
    pub const STARTING_TREES: f32 = 0.50;
    pub const STARTING_JACKS: f32 = 0.10;
    pub const STARTING_BEARS: f32 = 0.02;

    pub const NONE_MASK: u16 = 0x0000;
    pub const TREE_MASK: u16 = 0x00FF;
    pub const JACK_MASK: u16 = 0x0F00;
    pub const BEAR_MASK: u16 = 0xF000;

    pub const TREE_REMOVE_MASK: u16 = 0xFF00;
    pub const JACK_REMOVE_MASK: u16 = 0xF0FF;
    pub const BEAR_REMOVE_MASK: u16 = 0x0FFF;

    pub const TREE_SHIFT: u16 = 4 * 0;
    pub const JACK_SHIFT: u16 = 4 * 2;
    pub const BEAR_SHIFT: u16 = 4 * 3;

    pub const SAPLING_SPAWN_CHANCE: u32 = 0;
    pub const MATURE_SPAWN_CHANCE: u32 = 10;
    pub const ELDER_SPAWN_CHANCE: u32 = 20;

    pub const SAPLING_HARVEST_CHANCE: u32 = 99;
    pub const MATURE_HARVEST_CHANCE: u32 = 75;
    pub const ELDER_HARVEST_CHANCE: u32 = 66;

    pub const NONE_HARVEST_AMOUNT: u32 = 0;
    pub const SAPLING_HARVEST_AMOUNT: u32 = 1;
    pub const MATURE_HARVEST_AMOUNT: u32 = 2;
    pub const ELDER_HARVEST_AMOUNT: u32 = 4;

    pub const JACK_MAX_LEVEL: u16 = 5;
    pub const JACK_MIN_MAUL_PROTECTION: u16 = 75;

    pub const SAPLING_GROW_AGE: u16 = 12;
    pub const MATURE_GROW_AGE: u16 = 120;

    pub const BEAR_WANDERS_PER_MONTH: u32 = 3;
    pub const BEAR_WANDER_ATTEMPTS: u32 = 2;

    pub const JACK_WANDERS_PER_MONTH: u32 = 3;
    pub const JACK_WANDER_ATTEMPTS: u32 = 2;
}

pub mod forest {
    use crate::random::Random;

    use crate::consts::{
        BEAR_MASK, BEAR_REMOVE_MASK, BEAR_SHIFT, BEAR_WANDERS_PER_MONTH, BEAR_WANDER_ATTEMPTS,
        ELDER_HARVEST_CHANCE, ELDER_SPAWN_CHANCE, JACK_MASK, JACK_MAX_LEVEL, JACK_REMOVE_MASK,
        JACK_SHIFT, JACK_WANDERS_PER_MONTH, JACK_WANDER_ATTEMPTS, MATURE_GROW_AGE,
        MATURE_HARVEST_CHANCE, MATURE_SPAWN_CHANCE, NONE_MASK, SAPLING_GROW_AGE,
        SAPLING_HARVEST_CHANCE, SAPLING_SPAWN_CHANCE, STARTING_BEARS, STARTING_JACKS,
        STARTING_TREES, TREE_MASK, TREE_REMOVE_MASK, TREE_SHIFT, SAPLING_HARVEST_AMOUNT, MATURE_HARVEST_AMOUNT, ELDER_HARVEST_AMOUNT, NONE_HARVEST_AMOUNT, JACK_MIN_MAUL_PROTECTION,
    };

    enum TreeKind {
        None,
        Sapling,
        Mature,
        Elder,
    }

    pub struct Forest {
        rng: Random,
        pub map: Vec<u16>,
        pub width: usize,
        pub height: usize,
        pub months_elapsed: u32,
        pub yearly_lumber: u32,
        pub yearly_mauls: u32,
    }

    impl Forest {
        pub fn new(seed: u64, width: usize, height: usize) -> Self {
            let mut rng = Random::new(seed);
            let mut map = vec![NONE_MASK; width * height];

            Self::initialize_map(&mut rng, &mut map);

            Self {
                rng,
                map,
                width,
                height,
                months_elapsed: 0,
                yearly_lumber: 0,
                yearly_mauls: 0,
            }
        }

        fn initialize_map(rng: &mut Random, map: &mut [u16]) {
            let num_bears = f32::ceil(map.len() as f32 * STARTING_BEARS) as usize;
            for n in 0..num_bears {
                Self::randomly_place_entity(rng, map, n, BEAR_MASK, BEAR_SHIFT);
            }

            let num_jacks = f32::ceil(map.len() as f32 * STARTING_JACKS) as usize;
            for n in 0..num_jacks {
                Self::randomly_place_entity(rng, map, n, JACK_MASK, JACK_SHIFT);
            }

            let num_trees = f32::ceil(map.len() as f32 * STARTING_TREES) as usize;
            for n in 0..num_trees {
                Self::randomly_place_entity(rng, map, n, TREE_MASK, TREE_SHIFT);
            }
        }

        fn randomly_place_entity(rng: &mut Random, map: &mut [u16], num_ents: usize, mask: u16, shift: u16) {
            if num_ents == map.len() {
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

        fn place_entity(map: &mut [u16], index: usize, shift: u16) {
            map[index] += 0x1 << shift;
        }

        pub fn draw_map(&self) {
            for i in 0..self.map.len() {
                if i > 0 && i % self.width == 0 {
                    println!();
                }

                let cell = self.map[i];

                match (
                    ((cell & BEAR_MASK) >> BEAR_SHIFT),
                    ((cell & JACK_MASK) >> JACK_SHIFT),
                    Self::get_tree_kind(cell),
                ) {
                    (0, 0, TreeKind::None) => print!("."),
                    (1, 0, TreeKind::None) => print!("B"),
                    (0, _, TreeKind::None) => print!("@"),
                    (0, 0, TreeKind::Sapling) => print!("t"),
                    (0, 0, TreeKind::Mature) => print!("T"),
                    (0, 0, TreeKind::Elder) => print!("E"),
                    (1, _, TreeKind::None) => print!("3"),
                    (1, 0, _) => print!("4"),
                    (0, _, _) => print!("5"),
                    (1, _, _) => print!("6"),
                    (_, _, _) => print!("?"),
                }
            }

            println!();
        }

        fn get_tree_kind(cell: u16) -> TreeKind {
            match cell & TREE_MASK {
                0 => TreeKind::None,
                age => {
                    if age < SAPLING_GROW_AGE {
                        TreeKind::Sapling
                    } else if age < MATURE_GROW_AGE {
                        TreeKind::Mature
                    } else {
                        TreeKind::Elder
                    }
                }
            }
        }

        fn trigger_tree_event(&mut self) {
            let positions = Self::get_entity_positions(&self.map, TREE_MASK, TREE_SHIFT);
            for i in positions {
                let cell = self.map[i];

                Self::age_tree(&mut self.map, i, cell);

                let spawn_chance = Self::get_sapling_spawn_chance(cell);
                let result = self.rng.next() as u32 % 100;

                if result <= spawn_chance {
                    let adjacent_positions = self.get_adjacent_positions(i);
                    let mut position_candidates = adjacent_positions
                        .iter()
                        .filter(|&position| {
                            let cell = self.map[*position];
                            cell & TREE_MASK == 0
                        })
                        .collect();

                    if let Some(&choice) = self.rng.choose(&mut position_candidates) {
                        Self::place_entity(&mut self.map, choice, TREE_SHIFT)
                    }
                }
            }
        }

        fn age_tree(map: &mut [u16], index: usize, cell: u16) {
            let tree_age = cell & TREE_MASK;
            if tree_age < 255 {
                map[index] += 0x1;
            }
        }

        fn get_entity_positions(map: &[u16], mask: u16, shift: u16) -> Vec<usize> {
            (0..map.len())
                .into_iter()
                .filter(|&i| ((map[i] & mask) >> shift) > 0)
                .collect::<Vec<usize>>()
        }

        fn get_adjacent_positions(&self, index: usize) -> Vec<usize> {
            let mut positions: Vec<usize> = vec![];

            let adjacent_movements: Vec<(isize, isize)> = vec![
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ];

            let x = (index % self.width) as usize;
            let y = (index / self.width) as usize;

            for movement in adjacent_movements {
                let x = x as isize + movement.0;
                let y = y as isize + movement.1;
                if x < 0 || y < 0 {
                    continue;
                }

                let x = x as usize;
                let y = y as usize;
                if x >= self.width || y >= self.height {
                    continue;
                }

                positions.push(Self::convert_position_to_index(
                    x as usize,
                    y as usize,
                    self.width,
                ));
            }

            positions
        }

        fn convert_index_to_position(index: usize, width: usize) -> (usize, usize) {
            (index % width, index / width)
        }

        fn convert_position_to_index(x: usize, y: usize, width: usize) -> usize {
            y * width + x
        }

        fn get_sapling_spawn_chance(cell: u16) -> u32 {
            let kind = Self::get_tree_kind(cell);
            match kind {
                TreeKind::Sapling => SAPLING_SPAWN_CHANCE,
                TreeKind::Mature => MATURE_SPAWN_CHANCE,
                TreeKind::Elder => ELDER_SPAWN_CHANCE,
                TreeKind::None => 0,
            }
        }

        fn trigger_jack_event(&mut self) {
            let positions = Self::get_entity_positions(&self.map, JACK_MASK, JACK_SHIFT);
            for i in positions {
                let mut wanders = 0;
                let mut current_position = i;

                while wanders < JACK_WANDERS_PER_MONTH {
                    let mut has_wandered = false;
                    let mut wander_attempts = 0;

                    let adjacent_positions = self.get_adjacent_positions(current_position);
                    let mut position_candidates: Vec<&usize> = adjacent_positions
                        .iter()
                        .filter(|&position| {
                            let cell = self.map[*position];
                            (cell & JACK_MASK) == 0
                        })
                        .collect();

                    if position_candidates.len() == 0 {
                        break;
                    }

                    while wander_attempts < JACK_WANDER_ATTEMPTS && !has_wandered {
                        match self.rng.choose(&mut position_candidates) {
                            Some(&next_position) => {
                                Self::remove_entity(&mut self.map, current_position, JACK_REMOVE_MASK);
                                Self::place_entity(&mut self.map, next_position, JACK_SHIFT);

                                let chosen_cell = self.map[next_position];
                                if (chosen_cell & TREE_MASK) > 0 {
                                    let result = self.rng.next() as u32 % 100;
                                    if result < Self::get_tree_harvest_chance(chosen_cell) {
                                        let harvest_amount = Self::get_harvest_amount(chosen_cell);
                                        self.yearly_lumber += harvest_amount;
                                        Self::remove_entity(&mut self.map, next_position, TREE_REMOVE_MASK);
                                        Self::level_up_jack(&mut self.map, next_position, harvest_amount);
                                    } else {
                                        // let harvest_amount = Self::get_harvest_amount(chosen_cell) / 2;
                                        // self.yearly_lumber += harvest_amount;
                                        Self::de_age_tree(&mut self.map, next_position);
                                        // Self::level_up_jack(map, next_position, harvest_amount);
                                    }

                                    wanders = JACK_WANDERS_PER_MONTH;
                                } else {
                                    wanders += 1;
                                }

                                current_position = next_position;
                                has_wandered = true;
                            }
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

        fn get_tree_harvest_chance(cell: u16) -> u32 {
            match Self::get_tree_kind(cell) {
                TreeKind::Sapling => SAPLING_HARVEST_CHANCE,
                TreeKind::Mature => MATURE_HARVEST_CHANCE,
                TreeKind::Elder => ELDER_HARVEST_CHANCE,
                TreeKind::None => 0,
            }
        }

        fn de_age_tree(map: &mut [u16], index: usize) {
            let cell = map[index];

            match Self::get_tree_kind(cell) {
                TreeKind::Sapling => {
                    map[index] -= (cell & TREE_MASK) - 1;
                }
                TreeKind::Mature => {
                    map[index] -= (cell & TREE_MASK) - SAPLING_GROW_AGE;
                }
                TreeKind::Elder => {
                    map[index] -= (cell & TREE_MASK) - MATURE_GROW_AGE;
                }
                TreeKind::None => return,
            }
        }

        fn level_up_jack(map: &mut [u16], index: usize, lumber: u32) {
            let cell = map[index];
            let current_level = (cell & JACK_MASK) >> JACK_SHIFT;

            if current_level <= JACK_MAX_LEVEL {
                let level = u16::min(current_level + lumber as u16, JACK_MAX_LEVEL);
                map[index] &= JACK_REMOVE_MASK;
                map[index] += level << JACK_SHIFT;
            }
        }

        fn remove_entity(map: &mut [u16], index: usize, remove_mask: u16) {
            map[index] &= remove_mask;
        }

        fn get_harvest_amount(cell: u16) -> u32 {
            match Self::get_tree_kind(cell) {
                TreeKind::None => NONE_HARVEST_AMOUNT,
                TreeKind::Sapling => SAPLING_HARVEST_AMOUNT,
                TreeKind::Mature => MATURE_HARVEST_AMOUNT,
                TreeKind::Elder => ELDER_HARVEST_AMOUNT,
            }
        }

        fn trigger_bear_event(&mut self) {
            let positions = Self::get_entity_positions(&self.map, BEAR_MASK, BEAR_SHIFT);
            for i in positions {
                let mut wanders = 0;
                let mut current_position = i;

                while wanders < BEAR_WANDERS_PER_MONTH {
                    let mut has_wandered = false;
                    let mut wander_attempts = 0;

                    let adjacent_positions = self.get_adjacent_positions(current_position);
                    let mut position_candidates: Vec<&usize> = adjacent_positions
                        .iter()
                        .filter(|&position| {
                            let cell = self.map[*position];
                            (cell & BEAR_MASK) == 0
                        })
                        .collect();

                    if position_candidates.len() == 0 {
                        break;
                    }

                    while wander_attempts < BEAR_WANDER_ATTEMPTS && !has_wandered {
                        match self.rng.choose(&mut position_candidates) {
                            Some(&next_position) => {
                                Self::remove_entity(&mut self.map, current_position, BEAR_REMOVE_MASK);
                                Self::place_entity(&mut self.map, next_position, BEAR_SHIFT);

                                let chosen_cell = self.map[next_position];
                                if (chosen_cell & JACK_MASK) > 0 {
                                    let result = self.rng.next() as u32 % 100;
                                    if result < Self::get_jack_maul_chance(chosen_cell) {
                                        self.yearly_mauls += 1;
                                        Self::remove_entity(&mut self.map, next_position, JACK_REMOVE_MASK);
                                    } else {
                                        Self::de_level_jack(&mut self.map, next_position);
                                    }

                                    wanders = BEAR_WANDERS_PER_MONTH;
                                } else {
                                    wanders += 1;
                                }

                                current_position = next_position;
                                has_wandered = true;
                            }
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

        fn get_jack_maul_chance(cell: u16) -> u32 {
            let level = (cell & JACK_MASK) >> JACK_SHIFT;
            let base_maul_protection = level * 10;
            let low_level_protection_bonus = 10 - u16::min(level.pow(2), 10);
            let maul_protection = base_maul_protection + low_level_protection_bonus;
            let maul_chance = 100 - u16::min(maul_protection, JACK_MIN_MAUL_PROTECTION);
            maul_chance as u32
        }

        fn de_level_jack(map: &mut [u16], index: usize) {
            let cell = map[index];
            let level = (cell & JACK_MASK) >> JACK_SHIFT;

            if level > 1 {
                map[index] &= JACK_REMOVE_MASK;
                map[index] += (level - 1) << JACK_SHIFT;
            }
        }

        fn trigger_yearly_events(&mut self) {
            {
                let jacks = Self::get_entity_positions(&self.map, JACK_MASK, JACK_SHIFT);
                if self.yearly_lumber as usize > jacks.len() {
                    let excess_lumber = self.yearly_lumber as usize - jacks.len();
                    let new_jacks = excess_lumber / 10;

                    for _ in 0..new_jacks {
                        if let Some(index) = Self::get_open_space(&mut self.rng, &mut self.map) {
                            Self::place_entity(&mut self.map, index, JACK_SHIFT);
                        }
                    }
                } else {
                    if jacks.len() > 1 {
                        if let Some(index) = self.rng.choose(&jacks) {
                            Self::remove_entity(&mut self.map, index, JACK_REMOVE_MASK);
                        }
                    }
                }
            }

            {
                let bears = Self::get_entity_positions(&self.map, BEAR_MASK, BEAR_SHIFT);
                if self.yearly_mauls as usize == 0 {
                    if let Some(index) = Self::get_open_space(&mut self.rng, &mut self.map) {
                        Self::place_entity(&mut self.map, index, BEAR_SHIFT);
                    }
                } else {
                    if bears.len() > 1 {
                        if let Some(index) = self.rng.choose(&bears) {
                            Self::remove_entity(&mut self.map, index, BEAR_REMOVE_MASK);
                        }
                    }
                }
            }

            self.yearly_lumber = 0;
            self.yearly_mauls = 0;
        }

        fn get_open_space(rng: &mut Random, map: &[u16]) -> Option<usize> {
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

            self.trigger_tree_event();
            self.trigger_jack_event();
            self.trigger_bear_event();

            if self.months_elapsed % 12 == 0 {
                self.trigger_yearly_events();
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    }
}