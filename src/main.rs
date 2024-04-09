use std::env::args;

use tile_gradient_arranger::arrangement::{arrange_images, build_graph};
use tile_gradient_arranger::image::{read_images, write_image};

fn main() {
    let image_paths = args().skip(1);
    let image_colors = read_images(image_paths);
    let graph = build_graph(image_colors);
    let grid = arrange_images(&graph);
    write_image(&grid, "./image.png");
}
