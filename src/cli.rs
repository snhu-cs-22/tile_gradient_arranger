use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    /// Path to the directory of input image files.
    #[arg(short, long)]
    pub input: PathBuf,

    /// Path of the output image file.
    #[arg(short, long, default_value_os_t = PathBuf::from("./mosaic.png"))]
    pub output: PathBuf,

    /// Number of k-means clusters (1 = simple average).
    #[arg(short, long, default_value_t = 3)]
    #[arg(value_parser = clap::value_parser!(u32).range(1..))]
    #[arg(alias = "kmeans")]
    pub k_means: u32,

    /// Width to scale each input image to in pixels.
    #[arg(long, default_value_t = 100)]
    #[arg(value_parser = clap::value_parser!(u32).range(1..))]
    pub tile_width: u32,

    /// Height to scale each input image to in pixels.
    #[arg(long, default_value_t = 100)]
    #[arg(value_parser = clap::value_parser!(u32).range(1..))]
    pub tile_height: u32,

    /// Strategy for arranging images.
    #[arg(long, value_enum, default_value_t = ArrangementStrategy::Raster)]
    #[arg(alias = "strategy")]
    #[arg(alias = "strat")]
    pub arrangement_strategy: ArrangementStrategy,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum ArrangementStrategy {
    Raster,
    Spiral,
}
