/// Forest Simulation
///
/// The simulation revolves around a forest.
/// The forest is a grid that is NxN in size.
///
/// There are three types of entities in the forest.
/// 1) Trees
/// 2) Lumberjacks
/// 3) Bears
///
/// When the simulation starts, the entities will make up a certain percentage
/// of the space in the forest where the rest will be empty space.
/// 1) Trees will take up 50%
/// 2) Lumberjacks will take up 10%
/// 3) Bears will take up 2%
///
/// We will be simulating months in a year for 4800 months.
/// Each month, events will take placef or each entity in the forest.
/// The events are as follows:
///
/// Trees
/// -----
/// Trees have a 10% chance of spawning a Sapling in a random open space
/// adjacent to them. If there are no open spots available, nothing happens.
///
/// Saplings will become Trees 12 months after existence.
///
/// Trees will become Elder Trees 120 months after existence.
///
/// Elder Trees have a 20% chance of spawning a Sapling instead of 10%.
///
/// Lumberjacks
/// -----------
/// Lumberjacks wander randomly three times at the end of a month.
///
/// Lumberjacks harvest Trees that they wander into and gain lumber.
/// Once they've harvested a Tree, the cannot wander anymore.
///
/// Lumberjacks do not cut down Saplings, only Trees and Elder Trees.
///
/// Lumberjacks gain 1 piece of lumber per Tree, and 2 pieces per Elder Tree.
///
/// Every 12 months, the higher Lumberjacks decide if the current Lumberjacks
/// have done good enough job to warrant hiring new ones.
/// 1) If the number of lumber is equal to or exceeds the number of Lumberjacks,
/// the Lumberjacks keep their jobs. For every extra 10 lumber than there are
/// Lumberjacks, a new Lumberjack is hired and placed randomly in the forest.
/// 2) If the number of lumber is below the current number of Lumberjacks,
/// one random Lumberjack is fired. The higher Lumberjacks will never fire
/// every single Lumberjack.
///
/// Bears
/// -----
/// Bears wander randomly five times at the end of a month.
///
/// Bears maw Lumberjacks that they wander into adding to the number of
/// accidents in the forest. Once a Bear maws a Lumberjack, they cannot
/// wander anymore.
///
/// Every 12 months, the park rangers will decide if Bears should be added
/// or removed from the forest depending on the number of accidents.
/// 1) If there are zero accidents, the park rangers will add a Bear randomly
/// to the forest.
/// 2) If there are any accidents, the park rangers will trap and remove a
/// Bear from the forest. The park rangers absolutely will remove the Bear
/// population from the forest if need be, however, they will continue to add
/// them back as they Maw accidents reach zero.
///
/// Trees, Bears and Lumberjacks
/// ----------------------------
/// Bear and Lumberjacks cannot wander onto spots that the same entity has
/// already wandered onto. If they attempt to wander onto spots with the
/// same entity twice in a row, they will stop wandering.
///
/// Lumberjacks can wander onto Bears in which they will get mawed by.
///
/// Bears can wander onto Tree spots and nothing will happen.
///
/// Thoughts
/// --------
/// The order of precedence seems to be:
/// Bears -> Lumberjacks -> Trees
///
/// If we were to only make the maw / harvest / spawn events happen at the
/// end of the month rather than immediately on each entity's turn, then
/// bears would maw first, lumberjacks would cut second and trees would
/// spawn last.
///
/// I guess we will see how I go about designing things before I think too
/// hard about how everything will work.
///

use rand::Rng;
use std::fmt;

struct Forest {
    size: usize,
    grid: Vec<u32>,
    months_elapsed: u32,
}

impl Forest {
    const TOTAL_MONTHS: u32 = 4800;

    const EMPTY: u32 = 0;
    const BEAR: u32 = 1;
    const LUMBERJACK: u32 = 2;
    const SAPLING: u32 = 3;
    const TREE: u32 = 4;
    const ELDER_TREE: u32 = 5;

    fn new(size: usize) -> Self {
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

    fn is_finished(&self) -> bool {
        self.months_elapsed == Forest::TOTAL_MONTHS
    }

    fn update(&mut self) {
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

fn main() {
    let mut forest = Forest::new(10);

    while !forest.is_finished() {
        forest.update();
    }

    println!("{}", forest);
}
