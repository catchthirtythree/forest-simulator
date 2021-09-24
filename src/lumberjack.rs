use crate::entity::{Entity, EntityType, WanderResult};
use crate::grid::GridUtils;
use crate::random::Random;

#[derive(Clone, Debug)]
pub struct Lumberjack {
    pub position: usize,
}

impl Lumberjack {
    const WANDERS_PER_MONTH: usize = 3;
    const WANDER_ATTEMPTS: u32 = 2;

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
        lumberjack_positions: Vec<usize>,
        tree_positions: Vec<usize>,
    ) -> WanderResult {
        let grid_size = width * height;
        let mut wanders = 0;
        let mut wander_attempts = 0;

        while wanders < Lumberjack::WANDERS_PER_MONTH {
            let adjacent_positions = GridUtils::get_adjacent_positions(
                self.position, width, height);
            let position = random.choose(&adjacent_positions).unwrap();
            let position = GridUtils::to_index(position.x, position.y, width);

            // Check if the lumberjack landed on another lumberjack
            if lumberjack_positions.iter().any(|&pos| pos == position) {
                wander_attempts += 1;

                if wander_attempts == Lumberjack::WANDER_ATTEMPTS {
                    return WanderResult::Wandered;
                }

                continue;
            }

            // Check if lumberjack harvested a tree
            if tree_positions.iter().any(|&pos| pos == position) {
                self.position = position;
                return WanderResult::Harvested(position);
            }

            self.position = position;
            wanders += 1;
            wander_attempts = 0;
        }

        return WanderResult::Wandered;
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
