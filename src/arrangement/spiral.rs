use super::{Image, OptionalGrid};

pub fn arrange<'a>(
    grid: &mut OptionalGrid<&'a Image>,
    build_order: impl Iterator<Item = &'a Image>,
) {
    let coords = SpiralGridCoords::new((grid.cols() / 2, grid.rows() / 2));
    for (coords, image) in coords.zip(build_order) {
        grid[coords] = Some(image);
    }
}

struct SpiralGridCoords {
    current_coords: (usize, usize),
    current_direction: Direction,
    until_side_length: usize,
    current_side_length: usize,
}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Down,
    Right,
    Up,
    Left,
}

impl SpiralGridCoords {
    pub fn new(start_coords: (usize, usize)) -> Self {
        Self {
            current_coords: start_coords,
            current_direction: Direction::Down,
            until_side_length: 0,
            current_side_length: 0,
        }
    }

    fn get_next(&mut self) -> (usize, usize) {
        if self.current_side_length == 0 {
            self.current_side_length += 1;
            return self.current_coords;
        }

        self.current_coords = match self.current_direction {
            Direction::Down => (self.current_coords.0, self.current_coords.1 + 1),
            Direction::Right => (self.current_coords.0 + 1, self.current_coords.1),
            Direction::Up => (self.current_coords.0, self.current_coords.1 - 1),
            Direction::Left => (self.current_coords.0 - 1, self.current_coords.1),
        };

        self.until_side_length += 1;
        if self.until_side_length == self.current_side_length {
            self.until_side_length = 0;
            if self.current_direction == Direction::Left
                || self.current_direction == Direction::Right
            {
                self.current_side_length += 1;
            }
            self.current_direction = match self.current_direction {
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
            };
        }

        self.current_coords
    }
}

impl Iterator for SpiralGridCoords {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get_next())
    }
}
