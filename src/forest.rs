use rand::Rng;
use std::fmt;

use crate::grid::Grid;

pub struct Forest {
    width: usize,
    height: usize,
    grid: Grid<Option<u32>>,
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

    fn create_grid(size: usize) -> Grid<Option<u32>> {
        // Calculate the grid size
        let grid_size = size * size;

        // Create the empty grid that represents the forest
        let mut grid = Grid::new(size, size);

        // Calculate the initial number of trees and place them on the grid
        let num_trees = (grid_size as f32 * Forest::STARTING_TREES) as usize;
        for _ in 0..num_trees {
            Forest::place_entity(&mut grid, Forest::TREE);
        }

        // Calculate the initial number of lumberjacks and place them on the grid
        let num_lumberjacks = (grid_size as f32 * Forest::STARTING_LUMBERJACKS) as usize;
        for _ in 0..num_lumberjacks {
            Forest::place_entity(&mut grid, Forest::LUMBERJACK);
        }

        // Calculate the initial number of bears and place them on the grid
        let num_bears = (grid_size as f32 * Forest::STARTING_BEARS) as usize;
        for _ in 0..num_bears {
            Forest::place_entity(&mut grid, Forest::BEAR);
        }

        grid
    }

    fn place_entity(grid: &mut Grid<Option<u32>>, entity: u32) {
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

    pub fn update(&mut self) {
        self.months_elapsed += 1;
    }
}

impl fmt::Display for Forest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, cell) in self.grid.data.iter().enumerate() {
            let symbol = match *cell {
                None                     => ".",
                Some(Forest::BEAR)       => "B",
                Some(Forest::LUMBERJACK) => "L",
                Some(Forest::SAPLING)    => "s",
                Some(Forest::TREE)       => "T",
                Some(Forest::ELDER_TREE) => "@",
                Some(_)                  => panic!("The entity could not be found."),
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
