use crate::entity::{Entity, WanderResult};
use crate::bear::Bear;
use crate::lumberjack::Lumberjack;
use crate::tree::{Tree, TreeKind};
use crate::grid::GridUtils;
use crate::random::Random;

use std::collections::HashMap;
use std::fmt;

pub struct Forest {
    pub random: Random,
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

    pub fn new(seed: u64, width: usize, height: usize) -> Self {
        let random = Random(seed);
        let mut forest = Forest {
            random,
            width,
            height,
            bears: vec![],
            lumberjacks: vec![],
            trees: vec![],
            months_elapsed: 0,
            yearly_lumber: 0,
            yearly_maulings: 0,
        };

        forest.setup();
        forest
    }

    fn create_bear_entities(&mut self, grid_size: usize) {
        let grid_size = self.width * self.height;
        let num_bears = (grid_size as f32 * Forest::STARTING_BEARS) as usize;

        for _ in 0..num_bears {
            match GridUtils::get_open_space(&mut self.random, grid_size, &self.bears) {
                Some(idx) => self.bears.push(Bear::new(idx)),
                None      => continue
            }
        }
    }

    fn create_lumberjack_entities(&mut self, grid_size: usize) {
        let grid_size = self.width * self.height;
        let num_lumberjacks = (grid_size as f32 * Forest::STARTING_LUMBERJACKS) as usize;

        for _ in 0..num_lumberjacks {
            match GridUtils::get_open_space(&mut self.random, grid_size, &self.lumberjacks) {
                Some(idx) => self.lumberjacks.push(Lumberjack::new(idx)),
                None      => continue
            }
        }
    }

    fn create_tree_entities(&mut self, grid_size: usize) {
        let grid_size = self.width * self.height;
        let num_trees = (grid_size as f32 * Forest::STARTING_TREES) as usize;

        for _ in 0..num_trees {
            match GridUtils::get_open_space(&mut self.random, grid_size, &self.trees) {
                Some(idx) => self.trees.push(Tree::new(idx, 12)),
                None      => continue
            }
        }
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

    fn spawn_sapling(&mut self, idx: usize) -> Option<Tree> {
        let tree = self.trees.get(idx).unwrap();
        let chance = tree.get_spawn_chance();
        let choice = self.random.next() as u32 % 100;

        if choice <= chance {
            let mut adjacent_positions = GridUtils::get_adjacent_positions(
                tree.position, self.width, self.height);

            self.random.shuffle(&mut adjacent_positions);

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

    pub fn setup(&mut self) {
        self.create_bear_entities(self.width * self.height);
        self.create_lumberjack_entities(self.width * self.height);
        self.create_tree_entities(self.width * self.height);
    }

    pub fn update(&mut self) {
        self.months_elapsed += 1;

        // Handle bear wander and maulings

        for idx in 0..self.bears.len() {
            let bear_positions = self.bears
                .iter().map(|b| b.position).collect::<Vec<usize>>();
            let lumberjack_positions = self.lumberjacks
                .iter().map(|l| l.position).collect::<Vec<usize>>();
            let bear = self.bears.get_mut(idx).unwrap();

            let result = bear.wander(&mut self.random,
                self.width, self.height, bear_positions, lumberjack_positions);

            if let WanderResult::Mauled(idx) = result {
                let position = self.lumberjacks
                    .iter().position(|l| l.position == idx).unwrap();
                self.lumberjacks.remove(position);
                self.yearly_maulings += 1;
            }
        }

        // Handle lumberjack wander and harvests

        for idx in 0..self.lumberjacks.len() {
            let lumberjack_positions = self.lumberjacks
                .iter().map(|l| l.position).collect::<Vec<usize>>();
            let tree_positions = self.trees
                .iter().map(|t| t.position).collect::<Vec<usize>>();
            let lumberjack = self.lumberjacks.get_mut(idx).unwrap();

            let result = lumberjack.wander(&mut self.random,
                self.width, self.height, lumberjack_positions, tree_positions);

            if let WanderResult::Harvested(idx) = result {
                let position = self.trees
                    .iter().position(|t| t.position == idx).unwrap();
                let tree = self.trees.get(position).unwrap();
                self.yearly_lumber += tree.get_harvest_amount();
                self.trees.remove(position);
            }
        }

        // Handle tree update logic

        for tree in self.trees.iter_mut() {
            tree.grow();
        }

        for idx in 0..self.trees.len() {
            match self.spawn_sapling(idx) {
                Some(t) => self.trees.push(t),
                None => continue
            }
        }

        // Handle yearly events

        if self.months_elapsed % 12 == 0 {
            // Handle lumberjack event

            if self.yearly_lumber as usize > self.lumberjacks.len() {
                let excess_lumber = self.yearly_lumber as usize - self.lumberjacks.len();
                let new_lumberjacks = excess_lumber / 10;
                let grid_size = self.width * self.height;

                for _ in 0..new_lumberjacks {
                    match GridUtils::get_open_space(&mut self.random, grid_size, &self.lumberjacks) {
                        Some(idx) => self.lumberjacks.push(Lumberjack::new(idx)),
                        None      => continue
                    }
                }
            } else {
                if self.lumberjacks.len() > 1 {
                    let lumberjack = self.random.choose(&self.lumberjacks);
                    if let Some(lj) = lumberjack {
                        let idx = self.lumberjacks.iter()
                            .position(|l| l.position == lj.position).unwrap();

                        self.lumberjacks.remove(idx);
                    }
                }
            }

            // Handle bear event

            if self.yearly_maulings as usize == 0 {
                let grid_size = self.width * self.height;

                if let Some(idx) = GridUtils::get_open_space(&mut self.random, grid_size, &self.bears) {
                    self.bears.push(Bear::new(idx));
                }
            } else {
                let bear = self.random.choose(&self.bears);
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
