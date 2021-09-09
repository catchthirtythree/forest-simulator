use rand::Rng;
use std::fmt;

pub struct Forest {
    size: usize,
    grid: Vec<u32>,
    pub months_elapsed: u32,
}

impl Forest {
    const EMPTY: u32 = 0;
    const BEAR: u32 = 1;
    const LUMBERJACK: u32 = 2;
    const SAPLING: u32 = 3;
    const TREE: u32 = 4;
    const ELDER_TREE: u32 = 5;

    pub fn new(size: usize) -> Self {
        // Calculate the grid size
        let grid_size = size * size;

        // Create the empty grid that represents the forest
        let mut grid = vec![Forest::EMPTY; grid_size];

        // Create a closure for randomly placing entities
        fn place_entity(grid: &mut Vec<u32>, entity: u32) {
            let mut rng = rand::thread_rng();

            loop {
                let idx = rng.gen_range(0..grid.len());
                if grid[idx] == Forest::EMPTY {
                    grid[idx] = entity;
                    break;
                }
            }
        }

        // Calculate the initial number of trees and place them on the grid
        let num_trees = (grid_size as f32 * 0.50) as usize;
        for _ in 0..num_trees {
            place_entity(&mut grid, Forest::TREE);
        }

        // Calculate the initial number of lumberjacks and place them on the grid
        let num_lumberjacks = (grid_size as f32 * 0.10) as usize;
        for _ in 0..num_lumberjacks {
            place_entity(&mut grid, Forest::LUMBERJACK);
        }

        // Calculate the initial number of bears and place them on the grid
        let num_bears = (grid_size as f32 * 0.02) as usize;
        for _ in 0..num_bears {
            place_entity(&mut grid, Forest::BEAR);
        }

        Forest { size, grid, months_elapsed: 0 }
    }

    pub fn update(&mut self) {
        self.months_elapsed += 1;
    }
}

impl fmt::Display for Forest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, cell) in self.grid.iter().enumerate() {
            let symbol = match *cell {
                Forest::EMPTY      => ".",
                Forest::BEAR       => "B",
                Forest::LUMBERJACK => "L",
                Forest::SAPLING    => "s",
                Forest::TREE       => "T",
                Forest::ELDER_TREE => "@",
                _                  => panic!("The entity could not be found.")
            };

            if let Err(e) = write!(f, "{} ", symbol) {
                return Err(e);
            }

            let is_last_cell = (idx + 1) != self.grid.len();
            let is_end_of_line = (idx + 1) % self.size == 0;
            if is_last_cell && is_end_of_line {
                if let Err(e) = writeln!(f, "{}", "") {
                    return Err(e);
                }
            }
        }

        Ok(())
    }
}
