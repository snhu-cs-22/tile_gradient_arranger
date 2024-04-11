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
}
