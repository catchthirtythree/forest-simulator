use crate::entities::entity::Entity;
use crate::forest::Forest;
use crate::grid::Grid;

use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Clone, PartialEq)]
pub enum TreeKind {
    Sapling,
    Mature,
    Elder,
}

#[derive(Clone)]
pub struct Tree {
    age: u32,
    kind: TreeKind,
}

impl Tree {
    const SAPLING_SPAWN_CHANCE: u32 = 0;
    const MATURE_SPAWN_CHANCE: u32 = 10;
    const ELDER_SPAWN_CHANCE: u32 = 20;

    const SAPLING_GROW_AGE: u32 = 12;
    const MATURE_GROW_AGE: u32 = 120;

    pub fn new(kind: TreeKind) -> Self {
        Self {
            age: 0,
            kind,
        }
    }

    fn get_spawn_chance(&self) -> u32 {
        if self.kind == TreeKind::Mature {
            Tree::MATURE_SPAWN_CHANCE
        } else if self.kind == TreeKind::Elder {
            Tree::ELDER_SPAWN_CHANCE
        } else {
            Tree::SAPLING_SPAWN_CHANCE
        }
    }

    fn grow(&mut self) {
        self.age += 1;

        if self.age == Tree::SAPLING_GROW_AGE {
            self.kind = TreeKind::Mature;
        } else if self.age == Tree::MATURE_GROW_AGE{
            self.kind = TreeKind::Elder;
        }
    }

    // @TODO Instead of passing a mutable grid, perhaps we should return an
    // Option for the Forest to place itself.
    fn spawn_sapling(&self, idx: usize, grid: &mut Grid<Option<Box<dyn Entity>>>) {
        let mut rng = rand::thread_rng();
        let mut adjacent_cells = grid.get_adjacent_cells(idx);

        adjacent_cells.shuffle(&mut rng);

        for cell in adjacent_cells {
            let idx = grid.to_index(cell.x, cell.y);
            if let None = grid.data[idx] {
                grid.place(Some(Box::new(Tree::new(TreeKind::Sapling))), cell.x, cell.y);

                break;
            }
        }
    }
}

impl Entity for Tree {
    fn get_symbol(&self) -> &str {
        match self.kind {
            TreeKind::Sapling => "~",
            TreeKind::Mature  => "t",
            TreeKind::Elder   => "T",
        }
    }

    fn update(&mut self, idx: usize, grid: &mut Grid<Option<Box<dyn Entity>>>) {
        let mut rng = rand::thread_rng();
        let chance = self.get_spawn_chance();
        let choice = rng.gen_range(0..100);

        if choice <= chance {
            self.spawn_sapling(idx, grid);
        }

        self.grow();
    }
}
