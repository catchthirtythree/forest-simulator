use crate::entities::entity::{Entity, EntityType};
use crate::grid::GridUtils;

use rand::seq::SliceRandom;

#[derive(Clone, Debug)]
pub struct Lumberjack {
    pub position: usize,
}

impl Lumberjack {
    pub fn new(position: usize) -> Self {
        Self {
            position,
        }
    }

    pub fn wander(&mut self, width: usize, height: usize,
                    occupied_positions: &Vec<(usize, bool)>) {
        let mut rng = rand::thread_rng();
        let grid_size = width * height;

        let mut wanders = 0;
        let mut attempts = 0;

        while wanders < 3 {
            attempts += 1;

            let adjacent_positions = GridUtils::get_adjacent_positions(self.position, width, height);
            let picked_position = adjacent_positions.choose(&mut rng).unwrap();
            let picked_idx = GridUtils::to_index(picked_position.x, picked_position.y, width);
            let position = occupied_positions.iter().find(|op| op.0 == picked_idx);

            match position {
                Some(pos) => {
                    if pos.1 {
                        self.position = picked_idx;
                        break;
                    }
                },

                None => {
                    self.position = picked_idx;
                    wanders += 1;
                    attempts = 0;
                }
            }
            if attempts == 2 {
                break;
            }
        }
    }
}

impl Entity for Lumberjack {
    fn get_entity_type(&self) -> EntityType {
        EntityType::Lumberjack
    }

    fn get_position(&self) -> usize {
        self.position
    }

    fn get_symbol(&self) -> &str {
        "@"
    }
}
