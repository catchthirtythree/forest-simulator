use crate::entities::entity::Entity;
use crate::grid::Grid;

#[derive(Clone)]
struct Lumberjack {
    age: u32,
    // lumber: u32,
}

impl Lumberjack {
    fn new() -> Self {
        Self {
            age: 0,
            // lumber: 0,
        }
    }

    fn harvest(&self) {
        // Lumberjacks harvest Trees that they wander into and gain lumber.
        // Once they've harvested a Tree, the cannot wander anymore.

        // Lumberjacks do not cut down Saplings, only Trees and Elder Trees.

        // Lumberjacks gain 1 piece of lumber per Tree, and 2 pieces per Elder Tree.
    }

    fn wander(&self) -> Option<usize> {
        // Lumberjacks wander randomly three times at the end of a month.

        None
    }
}

impl Entity for Lumberjack {
    fn get_symbol(&self) -> &str {
        "l"
    }

    fn update(&mut self, idx: usize, grid: &mut Grid<Option<Box<dyn Entity>>>) {
        // @TODO Wander to a spot. Perhaps return Option.
        // @TODO Harvest from the spot wandered to.
    }
}
