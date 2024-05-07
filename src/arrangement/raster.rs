use super::{ImageColor, OptionalGrid};

pub fn arrange<'a>(
    grid: &mut OptionalGrid<&'a ImageColor>,
    build_order: impl Iterator<Item = &'a ImageColor>,
) {
    for (cell, image_color) in grid.iter_mut().zip(build_order) {
        *cell = Some(&image_color);
    }
}
