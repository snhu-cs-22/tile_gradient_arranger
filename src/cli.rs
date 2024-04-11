use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    /// Paths of the image files to use.
    #[arg()]
    pub input: Vec<PathBuf>,

    /// Path of the output image file.
    #[arg(short, long, default_value_os_t = PathBuf::from("./mosaic.png"))]
    pub output: PathBuf,

    /// Number of k-means clusters (1 = simple average).
    #[arg(short, long, default_value_t = 3)]
    #[arg(alias = "kmeans")]
    pub k_means: u32,

    /// Width to scale each input image to in pixels.
    #[arg(long, default_value_t = 100)]
    pub tile_width: u32,

    /// Height to scale each input image to in pixels.
    #[arg(long, default_value_t = 100)]
    pub tile_height: u32,
}
