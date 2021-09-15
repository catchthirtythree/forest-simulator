#![allow(dead_code)]
#![allow(unused_variables)]

/// The one biggest mistake I've made during all of this is thinking that
/// structures could be more than data holders. In this instance, they need
/// to be just that so I don't constantly run into silly immutable borrows
/// with the Forest structure.
///
/// Similar to an ECS, the systems, or in this case, the Forest, needs to
/// do all of the heavy lifting and leave the Bears, Lumberjacks and Trees
/// as data holders and nothing more. They can have a few simple functions
/// on them, like the Tree could grow when it needs to grow, and they
/// can know what kind of symbol they would be graphically, but they
/// need to do nothing more otherwise we run into dumb errors that...
/// make total sense.

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
/// Lumberjacks harvest Trees that they wander onto and gain lumber.
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
/// Bears maw Lumberjacks that they wander onto adding to the number of
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

mod bear;
mod entity;
mod forest;
mod grid;
mod lumberjack;
mod tree;

use crate::forest::Forest;

fn main() {
    const TOTAL_MONTHS: u32 = 4800;

    let mut forest = Forest::new(10);

    while forest.months_elapsed != TOTAL_MONTHS {
        print!("\x1B[2J\x1B[1;1H");

        forest.update();

        println!("{}", forest);

        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
