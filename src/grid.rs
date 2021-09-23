use crate::entity::Entity;
use crate::random::Random;

#[derive(Debug, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct GridUtils;

impl GridUtils {
    pub fn get_adjacent_positions(idx: usize, width: usize, height: usize) -> Vec<Position> {
        let adjacent_movements: Vec<(isize, isize)> = vec![
            ( -1, -1), ( 0, -1), ( 1, -1),
            ( -1,  0),           ( 1,  0),
            ( -1,  1), ( 0,  1), ( 1,  1),
        ];

        let cell = GridUtils::to_coords(idx, width);
        let mut adjacent_positions = vec![];

        for movement in adjacent_movements {
            let x = cell.x as isize + movement.0;
            let y = cell.y as isize + movement.1;

            if x < 0 || y < 0 {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            if x >= width || y >= height {
                continue;
            }

            adjacent_positions.push(Position { x, y });
        }

        adjacent_positions
    }

    pub fn get_open_space<T: Entity>(
        random: &mut Random,
        grid_size: usize,
        entities: &Vec<T>
    ) -> Option<usize> {
        if entities.len() == grid_size {
            None
        } else {
            loop {
                let idx = random.next() as usize % grid_size;
                let entity = entities.iter().find(|e| e.get_position() == idx);

                if let None = entity {
                    return Some(idx);
                }
            }
        }
    }

    pub fn to_coords(idx: usize, width: usize) -> Position {
        let x = (idx % width) as usize;
        let y = (idx / width) as usize;

        Position { x, y }
    }

    pub fn to_index(x: usize, y: usize, width: usize) -> usize {
        y * width + x
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::Position;
    use crate::grid::GridUtils;

    #[test]
    fn test_grid_get_adjacent_cells() {
        let width = 5;
        let height = 3;

        let cells = GridUtils::get_adjacent_positions(0, width, height);
        assert!(cells == vec![
            Position { x: 1, y: 0 },
            Position { x: 0, y: 1 },
            Position { x: 1, y: 1 },
        ]);

        let cells = GridUtils::get_adjacent_positions(7, width, height);
        assert!(cells == vec![
            Position { x: 1, y: 0 },
            Position { x: 2, y: 0 },
            Position { x: 3, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 3, y: 1 },
            Position { x: 1, y: 2 },
            Position { x: 2, y: 2 },
            Position { x: 3, y: 2 },
        ]);

        let cells = GridUtils::get_adjacent_positions(14, width, height);
        assert!(cells == vec![
            Position { x: 3, y: 1 },
            Position { x: 4, y: 1 },
            Position { x: 3, y: 2 },
        ]);
    }

    #[test]
    fn test_grid_to_coords() {
        let width = 5;
        let height = 3;

        let cell = GridUtils::to_coords(0, width);
        assert!(cell.x == 0);
        assert!(cell.y == 0);

        let cell = GridUtils::to_coords(14, width);
        assert!(cell.x == 4);
        assert!(cell.y == 2);

        let cell = GridUtils::to_coords(4, width);
        assert!(cell.x == 4);
        assert!(cell.y == 0);

        let cell = GridUtils::to_coords(11, width);
        assert!(cell.x == 1);
        assert!(cell.y == 2);
    }

    #[test]
    fn test_grid_to_index() {
        let width = 5;
        let height = 3;

        let cell = GridUtils::to_index(0, 0, width);
        assert!(cell == 0);

        let cell = GridUtils::to_index(4, 2, width);
        assert!(cell == 14);

        let cell = GridUtils::to_index(4, 0, width);
        assert!(cell == 4);

        let cell = GridUtils::to_index(1, 2, width);
        assert!(cell == 11);
    }
}
