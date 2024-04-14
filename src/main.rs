use clap::Parser;

use tile_gradient_arranger::arrangement::{arrange_images, build_graph};
use tile_gradient_arranger::cli::Cli;
use tile_gradient_arranger::image::{read_images, write_image};

fn main() {
    let args = Cli::parse();
    eprintln!("Reading images...");
    let image_colors = read_images(
        &args.input,
        args.k_means,
        (args.tile_width, args.tile_height),
    );
    let image_count = image_colors.len();
    eprintln!("Building graph...");
    let graph = build_graph(image_colors);
    eprintln!("Arranging images...");
    let grid = arrange_images(&graph, image_count);
    eprintln!("Writing image...");
    write_image(&grid, &args.output, (args.tile_width, args.tile_height));
}
