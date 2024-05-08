use super::{ImageColor, OptionalGrid};

pub fn arrange<'a>(
    grid: &mut OptionalGrid<&'a ImageColor>,
    build_order: impl Iterator<Item = &'a ImageColor>,
) {
    let coords = SpiralGridCoords::new((grid.cols() / 2, grid.rows() / 2));
    for (coords, image_color) in coords.zip(build_order) {
        grid[coords] = Some(&image_color);
    }
}

struct SpiralGridCoords {
    current_coords: (usize, usize),
    current_direction: Direction,
    until_side_length: usize,
    current_side_length: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Down,
    Right,
    Up,
    Left,
}

impl Direction {
    #[inline]
    pub fn is_horizontal(&self) -> bool {
        *self == Direction::Left || *self == Direction::Right
    }

    #[inline]
    pub fn is_vertical(&self) -> bool {
        !self.is_horizontal()
    }

    #[inline]
    pub fn clockwise(&self) -> Self {
        match self {
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
            Self::Right => Self::Down,
        }
    }

    #[inline]
    pub fn counter_clockwise(&self) -> Self {
        match self {
            Self::Down => Self::Right,
            Self::Right => Self::Up,
            Self::Up => Self::Left,
            Self::Left => Self::Down,
        }
    }
}

fn ahead(index: (usize, usize), direction: Direction) -> (usize, usize) {
    match direction {
        Direction::Down => (index.0, index.1 + 1),
        Direction::Right => (index.0 + 1, index.1),
        Direction::Up => (index.0, index.1 - 1),
        Direction::Left => (index.0 - 1, index.1),
    }
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
}

impl Iterator for SpiralGridCoords {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_side_length == 0 {
            self.current_side_length += 1;
            return Some(self.current_coords);
        }

        self.current_coords = ahead(self.current_coords, self.current_direction);
        self.until_side_length += 1;
        if self.until_side_length == self.current_side_length {
            self.until_side_length = 0;
            if self.current_direction.is_horizontal() {
                self.current_side_length += 1;
            }
            self.current_direction = self.current_direction.counter_clockwise();
        }

        Some(self.current_coords)
    }
}
