use crate::entities::entity::{Entity};
use crate::entities::lumberjack::Lumberjack;
use crate::entities::tree::{Tree, TreeKind};
use crate::grid::GridUtils;

use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::fmt;

pub struct Forest {
    pub width: usize,
    pub height: usize,
    // pub bears: Vec<Bear>,
    pub lumberjacks: Vec<Lumberjack>,
    pub trees: Vec<Tree>,
    pub months_elapsed: u32,
    pub yearly_lumber: u32,
}

impl Forest {
    pub const STARTING_TREES: f32 = 0.50;
    pub const STARTING_LUMBERJACKS: f32 = 0.10;
    pub const STARTING_BEARS: f32 = 0.02;

    pub fn new(size: usize) -> Self {
        let grid_size = size * size;

        let lumberjacks = Forest::create_lumberjack_entities(grid_size);
        let trees = Forest::create_tree_entities(grid_size);

        Forest {
            width: size,
            height: size,
            lumberjacks,
            trees,
            months_elapsed: 0,
            yearly_lumber: 0,
        }
    }

    // pub fn get_adjacent_entities(&self, idx: usize) -> Vec<Box<dyn Entity>> {
    //     let adjacent_positions = GridUtils::get_adjacent_positions(idx, self.width, self.height);
    //     let entity_map = Forest::create_entity_map(&self.entities);

    //     let mut adjacent_entities: Vec<Box<dyn Entity>> = vec![];

    //     for position in adjacent_positions {
    //         let idx = GridUtils::to_index(position.x, position.y, self.width);

    //         match entity_map.get(&idx) {
    //             Some(entity) => adjacent_entities.push(entity.clone()),
    //             None         => continue
    //         }
    //     }

    //     adjacent_entities
    // }

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

    // fn place_bear_entities(grid: &mut Grid<Option<Box<dyn Entity>>>, grid_size: usize) {
    //     let num_bears = (grid_size as f32 * Forest::STARTING_BEARS) as usize;

    //     for _ in 0..num_bears {
    //         Forest::place_entity(grid, Forest::BEAR);
    //     }
    // }

    // fn place_lumberjack_entities(grid: &mut Grid<Option<Box<dyn Entity>>>, grid_size: usize) {
    //     let num_lumberjacks = (grid_size as f32 * Forest::STARTING_LUMBERJACKS) as usize;

    //     for _ in 0..num_lumberjacks {
    //         Forest::place_entity(grid, Forest::LUMBERJACK);
    //     }
    // }

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
            t.position == lumberjack.position
                && t.get_tree_kind() != TreeKind::Sapling
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

            // Reset counts

            self.yearly_lumber = 0;
        }
    }
}

impl fmt::Display for Forest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut entity_map: HashMap<usize, &dyn Entity> = HashMap::new();

        // println!("{:?}", self.trees);

        for tree in &self.trees {
            entity_map.insert(tree.position, tree);
        }

        for lumberjack in &self.lumberjacks {
            entity_map.insert(lumberjack.position, lumberjack);
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
