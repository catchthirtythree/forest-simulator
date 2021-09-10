#[derive(Debug, PartialEq)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
}

pub struct Grid<T> {
    width: usize,
    height: usize,
    pub data: Vec<T>,
}

impl<T: Clone> Grid<T> where T: Default {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![T::default(); width * height]
        }
    }

    pub fn get_adjacent_cells(&self, idx: usize) -> Vec<Cell> {
        let adjacent_movements: Vec<(isize, isize)> = vec![
            ( -1, -1), ( 0, -1), ( 1, -1),
            ( -1,  0),           ( 1,  0),
            ( -1,  1), ( 0,  1), ( 1,  1),
        ];

        let cell = self.to_coords(idx);
        let mut adjacent_cells = vec![];

        for movement in adjacent_movements {
            let x = cell.x as isize + movement.0;
            let y = cell.y as isize + movement.1;

            if x < 0 || y < 0 {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            if x > self.width || y > self.height {
                continue;
            }

            adjacent_cells.push(Cell { x, y });
        }

        adjacent_cells
    }

    pub fn to_coords(&self, idx: usize) -> Cell {
        let x = (idx % self.width) as usize;
        let y = (idx / self.width) as usize;

        Cell { x, y }
    }

    pub fn to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn place(&mut self, data: T, x: usize, y: usize) {
        let index = self.to_index(x, y);

        self.data[index] = data;
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::Cell;
    use crate::grid::Grid;

    #[test]
    fn test_grid_new() {
        let width = 5;
        let height = 3;
        let grid = Grid::<u32>::new(width, height);

        assert!(grid.width == width);
        assert!(grid.height == height);
        assert!(grid.data == [
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0
        ]);
    }

    #[test]
    fn test_grid_get_adjacent_cells() {
        let width = 5;
        let height = 3;
        let grid = Grid::<u32>::new(width, height);

        let cells = grid.get_adjacent_cells(0);
        assert!(cells == vec![
            Cell { x: 1, y: 0 },
            Cell { x: 0, y: 1 },
            Cell { x: 1, y: 1 },
        ]);

        let cells = grid.get_adjacent_cells(7);
        assert!(cells == vec![
            Cell { x: 1, y: 0 },
            Cell { x: 2, y: 0 },
            Cell { x: 3, y: 0 },
            Cell { x: 1, y: 1 },
            Cell { x: 3, y: 1 },
            Cell { x: 1, y: 2 },
            Cell { x: 2, y: 2 },
            Cell { x: 3, y: 2 },
        ]);
    }

    #[test]
    fn test_grid_to_coords() {
        let width = 5;
        let height = 3;
        let grid = Grid::<u32>::new(width, height);

        let cell = grid.to_coords(0);
        assert!(cell.x == 0);
        assert!(cell.y == 0);

        let cell = grid.to_coords(14);
        assert!(cell.x == 4);
        assert!(cell.y == 2);

        let cell = grid.to_coords(4);
        assert!(cell.x == 4);
        assert!(cell.y == 0);

        let cell = grid.to_coords(11);
        assert!(cell.x == 1);
        assert!(cell.y == 2);
    }

    #[test]
    fn test_grid_to_index() {
        let width = 5;
        let height = 3;
        let grid = Grid::<u32>::new(width, height);

        let cell = grid.to_index(0, 0);
        assert!(cell == 0);

        let cell = grid.to_index(4, 2);
        assert!(cell == 14);

        let cell = grid.to_index(4, 0);
        assert!(cell == 4);

        let cell = grid.to_index(1, 2);
        assert!(cell == 11);
    }

    #[test]
    fn test_grid_place() {
        let width = 5;
        let height = 3;
        let mut grid = Grid::<u32>::new(width, height);

        for idx in 0..grid.data.len() {
            let cell = grid.to_coords(idx);

            grid.place(idx as u32, cell.x, cell.y);
        }

        assert!(grid.data[0] == 0);
        assert!(grid.data[1] == 1);
        assert!(grid.data[2] == 2);
        assert!(grid.data[3] == 3);
        assert!(grid.data[4] == 4);
        assert!(grid.data[5] == 5);
        assert!(grid.data[6] == 6);
        assert!(grid.data[7] == 7);
        assert!(grid.data[8] == 8);
        assert!(grid.data[9] == 9);
        assert!(grid.data[10] == 10);
        assert!(grid.data[11] == 11);
        assert!(grid.data[12] == 12);
        assert!(grid.data[13] == 13);
        assert!(grid.data[14] == 14);
    }
}
