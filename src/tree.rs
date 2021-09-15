use crate::entity::{Entity, EntityType};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum TreeKind {
    Sapling,
    Mature,
    Elder,
}

#[derive(Clone)]
pub struct Tree {
    pub position: usize,
    pub age: u32,
}

impl Tree {
    const SAPLING_SPAWN_CHANCE: u32 = 0;
    const MATURE_SPAWN_CHANCE: u32 = 10;
    const ELDER_SPAWN_CHANCE: u32 = 20;

    const SAPLING_GROW_AGE: u32 = 12;
    const MATURE_GROW_AGE: u32 = 120;

    pub fn new(position: usize, age: u32) -> Self {
        Self {
            position,
            age,
        }
    }

    pub fn get_harvest_amount(&self) -> u32 {
        match self.get_tree_kind() {
            TreeKind::Sapling => 0,
            TreeKind::Mature => 1,
            TreeKind::Elder => 2,
        }
    }

    pub fn get_tree_kind(&self) -> TreeKind {
        if self.age < 12 {
            TreeKind::Sapling
        } else if self.age < 120 {
            TreeKind::Mature
        } else {
            TreeKind::Elder
        }
    }

    pub fn get_spawn_chance(&self) -> u32 {
        let kind = self.get_tree_kind();
        if kind == TreeKind::Mature {
            Tree::MATURE_SPAWN_CHANCE
        } else if kind == TreeKind::Elder {
            Tree::ELDER_SPAWN_CHANCE
        } else {
            Tree::SAPLING_SPAWN_CHANCE
        }
    }

    pub fn grow(&mut self) {
        self.age += 1;
    }
}

impl Entity for Tree {
    fn get_entity_type(&self) -> EntityType {
        EntityType::Tree
    }

    fn get_position(&self) -> usize {
        self.position
    }

    fn get_symbol(&self) -> &str {
        let kind = self.get_tree_kind();
        match kind {
            TreeKind::Sapling => "~",
            TreeKind::Mature  => "t",
            TreeKind::Elder   => "!",
        }
    }
}

impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Tree")
            .field("position", &self.position)
            .field("age", &self.age)
            .finish()
    }
}
