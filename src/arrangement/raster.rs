use super::{Image, OptionalGrid};

pub fn arrange<'a>(
    grid: &mut OptionalGrid<&'a Image>,
    build_order: impl Iterator<Item = &'a Image>,
) {
    for (cell, image) in grid.iter_mut().zip(build_order) {
        *cell = Some(image);
    }
}
