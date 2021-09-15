use crate::entity::{Entity};
use crate::bear::Bear;
use crate::lumberjack::Lumberjack;
use crate::tree::{Tree, TreeKind};
use crate::grid::GridUtils;

use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::fmt;

pub struct Forest {
    pub width: usize,
    pub height: usize,
    pub bears: Vec<Bear>,
    pub lumberjacks: Vec<Lumberjack>,
    pub trees: Vec<Tree>,
    pub months_elapsed: u32,
    pub yearly_lumber: u32,
    pub yearly_maulings: u32,
}

impl Forest {
    pub const STARTING_TREES: f32 = 0.50;
    pub const STARTING_LUMBERJACKS: f32 = 0.10;
    pub const STARTING_BEARS: f32 = 0.02;

    pub fn new(size: usize) -> Self {
        let grid_size = size * size;

        let bears = Forest::create_bear_entities(grid_size);
        let lumberjacks = Forest::create_lumberjack_entities(grid_size);
        let trees = Forest::create_tree_entities(grid_size);

        Forest {
            width: size,
            height: size,
            bears,
            lumberjacks,
            trees,
            months_elapsed: 0,
            yearly_lumber: 0,
            yearly_maulings: 0,
        }
    }

    fn get_open_space<T: Clone + Entity>(grid_size: usize, entities: Vec<T>) -> Option<usize> {
        if entities.len() == grid_size {
            None
        } else {
            let mut rng = rand::thread_rng();

            loop {
                let idx = rng.gen_range(0..grid_size);
                let entity = entities.iter().find(|e| e.get_position() == idx);

                if let None = entity {
                    return Some(idx);
                }
            }
        }
    }

    fn create_bear_entities(grid_size: usize) -> Vec<Bear> {
        let mut bears: Vec<Bear> = vec![];
        let num_bears = (grid_size as f32 * Forest::STARTING_BEARS) as usize;

        for _ in 0..num_bears {
            match Forest::get_open_space(grid_size, bears.clone()) {
                Some(idx) => bears.push(Bear::new(idx)),
                None      => continue
            }
        }

        bears
    }

    fn create_lumberjack_entities(grid_size: usize) -> Vec<Lumberjack> {
        let mut lumberjacks: Vec<Lumberjack> = vec![];
        let num_lumberjacks = (grid_size as f32 * Forest::STARTING_LUMBERJACKS) as usize;

        for _ in 0..num_lumberjacks {
            match Forest::get_open_space(grid_size, lumberjacks.clone()) {
                Some(idx) => lumberjacks.push(Lumberjack::new(idx)),
                None      => continue
            }
        }

        lumberjacks
    }

    fn create_tree_entities(grid_size: usize) -> Vec<Tree> {
        let mut trees: Vec<Tree> = vec![];
        let num_trees = (grid_size as f32 * Forest::STARTING_TREES) as usize;

        for _ in 0..num_trees {
            match Forest::get_open_space(grid_size, trees.clone()) {
                Some(idx) => trees.push(Tree::new(idx, 12)),
                None      => continue
            }
        }

        trees
    }

    fn harvest_tree(&self, lumberjack: &Lumberjack, trees: &Vec<Tree>) -> Option<usize> {
        trees.iter().position(|t| {
            t.position == lumberjack.position && t.get_tree_kind() != TreeKind::Sapling
        })
    }

    fn maul_lumberjack(&self, bear: &Bear, lumberjacks: &Vec<Lumberjack>) -> Option<usize> {
        lumberjacks.iter().position(|l| {
            l.position == bear.position
        })
    }

    fn spawn_sapling(&self, tree: &Tree) -> Option<Tree> {
        let mut rng = rand::thread_rng();
        let chance = tree.get_spawn_chance();
        let choice = rng.gen_range(0..100);

        if choice <= chance {
            let mut adjacent_positions = GridUtils::get_adjacent_positions(
                tree.position, self.width, self.height);

            adjacent_positions.shuffle(&mut rng);

            for position in adjacent_positions {
                let idx = GridUtils::to_index(position.x, position.y, self.width);
                let entity = self.trees.iter().find(|t| t.position == idx);

                if let None = entity {
                    return Some(Tree::new(idx, 0));
                }
            }
        }

        None
    }

    pub fn update(&mut self) {
        self.months_elapsed += 1;

        // Handle bear update logic

        for idx in 0..self.bears.len() {
            let mut occupied_positions = self.bears.iter()
                .map(|b| (b.position, false)).collect::<Vec<(usize, bool)>>();
            occupied_positions.append(&mut self.lumberjacks.iter()
                .map(|l| (l.position, true)).collect::<Vec<(usize, bool)>>());

            let bear = self.bears.get_mut(idx).unwrap();
            bear.wander(self.width, self.height, &occupied_positions);
        }

        for bear in self.bears.iter() {
            let mauled_lumberjack = self.maul_lumberjack(bear, &self.lumberjacks);

            if let Some(idx) = mauled_lumberjack {
                self.yearly_maulings += 1;
                self.lumberjacks.remove(idx);
            }
        }

        // Handle lumberjack update logic

        for idx in 0..self.lumberjacks.len() {
            let mut occupied_positions = self.lumberjacks.iter()
                .map(|l| (l.position, false)).collect::<Vec<(usize, bool)>>();
            occupied_positions.append(&mut self.trees.iter()
                .map(|t| (t.position, true)).collect::<Vec<(usize, bool)>>());

            let lumberjack = self.lumberjacks.get_mut(idx).unwrap();
            lumberjack.wander(self.width, self.height, &occupied_positions);
        }

        for lumberjack in self.lumberjacks.iter() {
            let harvested_tree = self.harvest_tree(lumberjack, &self.trees);

            if let Some(idx) = harvested_tree {
                let tree = self.trees.get(idx).unwrap();
                self.yearly_lumber += tree.get_harvest_amount();
                self.trees.remove(idx);
            }
        }

        // Handle tree update logic

        for tree in self.trees.iter_mut() {
            tree.grow();
        }

        let mut saplings = vec![];
        for tree in self.trees.iter() {
            saplings.push(self.spawn_sapling(tree));
        }

        for sapling in saplings {
            if let Some(sap) = sapling {
                self.trees.push(sap);
            }
        }

        // Handle yearly events

        if self.months_elapsed % 12 == 0 {
            // Handle lumberjack event

            if self.yearly_lumber as usize > self.lumberjacks.len() {
                println!("Conjuring a lumberjack.");

                let excess_lumber = self.yearly_lumber as usize - self.lumberjacks.len();
                let new_lumberjacks = excess_lumber / 10;
                let grid_size = self.width * self.height;

                for _ in 0..new_lumberjacks {
                    match Forest::get_open_space(grid_size, self.lumberjacks.clone()) {
                        Some(idx) => self.lumberjacks.push(Lumberjack::new(idx)),
                        None      => continue
                    }
                }
            } else {
                if self.lumberjacks.len() > 1 {
                    println!("Murdering a lumberjack.");

                    let lumberjack = self.lumberjacks.choose(&mut rand::thread_rng());

                    if let Some(lj) = lumberjack {
                        let idx = self.lumberjacks.iter()
                            .position(|l| l.position == lj.position).unwrap();

                        self.lumberjacks.remove(idx);
                    }
                }
            }

            // Handle bear event

            if self.yearly_maulings as usize == 0 {
                println!("Conjuring a bear.");

                let grid_size = self.width * self.height;

                if let Some(idx) = Forest::get_open_space(grid_size, self.bears.clone()) {
                    self.bears.push(Bear::new(idx));
                }
            } else {
                println!("Murdering a bear.");

                let bear = self.bears.choose(&mut rand::thread_rng());

                if let Some(br) = bear {
                    let idx = self.bears.iter()
                        .position(|b| b.position == br.position).unwrap();

                    self.bears.remove(idx);
                }
            }

            // Reset counts

            self.yearly_lumber = 0;
            self.yearly_maulings = 0;
        }
    }
}

impl fmt::Display for Forest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut entity_map: HashMap<usize, &dyn Entity> = HashMap::new();

        for tree in &self.trees {
            entity_map.insert(tree.position, tree);
        }

        for lumberjack in &self.lumberjacks {
            entity_map.insert(lumberjack.position, lumberjack);
        }

        for bear in &self.bears {
            entity_map.insert(bear.position, bear);
        }

        let grid_size = self.width * self.height;

        for idx in 0..(grid_size) {
            let symbol = match entity_map.get(&idx) {
                None    => ".",
                Some(e) => e.get_symbol()
            };

            if let Err(e) = write!(f, "{} ", symbol) {
                return Err(e);
            }

            let is_last_cell = (idx + 1) != grid_size;
            let is_end_of_line = (idx + 1) % self.width == 0;
            if is_last_cell && is_end_of_line {
                if let Err(e) = writeln!(f, "{}", "") {
                    return Err(e);
                }
            }
        }

        Ok(())
    }
}
