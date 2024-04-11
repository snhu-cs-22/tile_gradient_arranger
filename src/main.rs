use clap::Parser;

use tile_gradient_arranger::arrangement::{arrange_images, build_graph};
use tile_gradient_arranger::cli::Cli;
use tile_gradient_arranger::image::{read_images, write_image};

fn main() {
    let args = Cli::parse();
    let image_colors = read_images(
        args.input,
        args.k_means,
        (args.tile_width, args.tile_height),
    );
    let graph = build_graph(image_colors);
    let grid = arrange_images(&graph);
    write_image(&grid, &args.output, (args.tile_width, args.tile_height));
}
