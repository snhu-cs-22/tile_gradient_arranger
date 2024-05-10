use std::collections::HashSet;

use itertools::Itertools;
use grid::Grid;

use super::{color_similarity, ImageColor, OptionalGrid};

type ImageColorGrid<'a> = OptionalGrid<&'a ImageColor>;

pub fn arrange<'a>(
    grid: &mut ImageColorGrid<'a>,
    mut build_order: impl Iterator<Item = &'a ImageColor>,
) {
    // Place center tile
    let center = (grid.cols() / 2, grid.rows() / 2);
    grid[center] = build_order.next();

    // Keep track of empty boundary cells
    let mut empty_neighbors = neighbors(grid, center).collect::<HashSet<_>>();

    for image_color in build_order {
        // Find best empty slot around perimeter of shape of placed tiles
        let best_neighbor = *empty_neighbors
            .iter()
            .min_by(|a, b| neighbor_fitness(grid, **a, image_color).total_cmp(&neighbor_fitness(grid, **b, image_color)))
            .unwrap();
        grid[best_neighbor] = Some(image_color);

        empty_neighbors.remove(&best_neighbor);
        empty_neighbors.extend(
            neighbors(grid, best_neighbor)
                .filter(|i| grid[*i].is_none())
        );
    }
}

fn neighbor_fitness<'a>(grid: &'a ImageColorGrid<'a>, index: (usize, usize), image_color: &ImageColor) -> f32 {
    neighbors(grid, index)
        .map(|i| {
            match grid[i] {
                Some(tile) => color_similarity(image_color.color, tile.color),
                _ => 0.0,
            }
        })
        .sum()
}

fn neighbors<T>(grid: &Grid<T>, index: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    // Make sure look-aheads and look-behinds don't go out of bounds
    let (max_y, max_x) = (grid.rows() - 1, grid.cols() - 1);
    let (start_y, end_y) = (index.0.saturating_sub(1), max_y.min(index.0 + 1));
    let (start_x, end_x) = (index.1.saturating_sub(1), max_x.min(index.1 + 1));

    (start_y..=end_y)
        .cartesian_product(start_x..=end_x)
        .filter(move |i| *i != index)
}
