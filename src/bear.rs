use crate::entity::{Entity, EntityType};
use crate::grid::GridUtils;
use crate::random::Random;

#[derive(Clone)]
pub struct Bear {
    pub position: usize,
}

impl Bear {
    pub fn new(position: usize) -> Self {
        Self {
            position,
        }
    }

    pub fn wander(
        &mut self,
        random: &mut Random,
        width: usize,
        height: usize,
        occupied_positions: &Vec<(usize, bool)>
    ) {
        let grid_size = width * height;

        let mut wanders = 0;
        let mut attempts = 0;

        while wanders < 5 {
            attempts += 1;

            let adjacent_positions = GridUtils::get_adjacent_positions(self.position, width, height);
            let picked_position = random.choose(&adjacent_positions).unwrap();
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

impl Entity for Bear {
    fn get_entity_type(&self) -> EntityType {
        EntityType::Bear
    }

    fn get_position(&self) -> usize {
        self.position
    }

    fn get_symbol(&self) -> &str {
        "B"
    }
}
