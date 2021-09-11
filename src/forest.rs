use rand::Rng;
use std::fmt;

use crate::entities::entity::Entity;
use crate::entities::tree::{TreeKind, Tree};
use crate::grid::Grid;

pub struct Forest {
    width: usize,
    height: usize,
    grid: Grid<Option<Box<dyn Entity>>>,
    pub months_elapsed: u32,
}

impl Forest {
    pub const STARTING_TREES: f32 = 0.50;
    pub const STARTING_LUMBERJACKS: f32 = 0.10;
    pub const STARTING_BEARS: f32 = 0.02;

    pub const BEAR: u32 = 1;
    pub const LUMBERJACK: u32 = 2;
    pub const SAPLING: u32 = 3;
    pub const TREE: u32 = 4;
    pub const ELDER_TREE: u32 = 5;

    pub fn new(size: usize) -> Self {
        let grid = Forest::create_grid(size);

        Forest {
            width: size,
            height: size,
            grid,
            months_elapsed: 0
        }
    }

    fn create_grid(size: usize) -> Grid<Option<Box<dyn Entity>>> {
        // Calculate the grid size
        let grid_size = size * size;

        // Create the empty grid that represents the forest
        let mut grid = Grid::<Option<Box<dyn Entity>>>::new(None, size, size);

        // Place entities on the grid
        // Forest::place_bear_entities(&mut grid, grid_size);
        // Forest::place_lumberjack_entities(&mut grid, grid_size);
        Forest::place_tree_entities(&mut grid, grid_size);

        grid
    }

    fn place_entity(grid: &mut Grid<Option<Box<dyn Entity>>>, entity: Box<dyn Entity>) {
        let mut rng = rand::thread_rng();

        loop {
            let idx = rng.gen_range(0..grid.data.len());
            if let None = grid.data[idx] {
                let cell = grid.to_coords(idx);
                grid.place(Some(entity), cell.x, cell.y);

                break;
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

    fn place_tree_entities(grid: &mut Grid<Option<Box<dyn Entity>>>, grid_size: usize) {
        let num_trees = (grid_size as f32 * Forest::STARTING_TREES) as usize;

        for _ in 0..num_trees {
            Forest::place_entity(grid, Box::new(Tree::new(TreeKind::Mature)));
        }
    }

    pub fn update(&mut self) {
        self.months_elapsed += 1;

        // for cell in &self.grid {
        //     match *cell {
        //         Forest::BEAR => {

        //         },

        //         Forest::LUMBERJACK => {

        //         },

        //         Forest::SAPLING => {

        //         },

        //         Forest::TREE => {

        //         },

        //         Forest::ELDER_TREE => {

        //         },

        //         _ => continue
        //     }
        // }
    }
}

impl fmt::Display for Forest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, cell) in self.grid.data.iter().enumerate() {
            // let symbol = match *cell {
            //     None                     => ".",
            //     Some(Forest::BEAR)       => "B",
            //     Some(Forest::LUMBERJACK) => "L",
            //     Some(Forest::SAPLING)    => "s",
            //     Some(Forest::TREE)       => "T",
            //     Some(Forest::ELDER_TREE) => "@",
            //     Some(_)                  => panic!("The entity could not be found."),
            // };

            let symbol = match cell {
                None    => ".",
                Some(e) => e.get_symbol()
            };

            if let Err(e) = write!(f, "{} ", symbol) {
                return Err(e);
            }

            let is_last_cell = (idx + 1) != self.grid.data.len();
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
