use std::path::Path;

use image::imageops::{resize, FilterType};
use image::io::Reader as ImageReader;
use image::{GenericImageView, ImageBuffer, Rgba};

use super::arrangement::OptionalGrid;
use super::colors::{get_primary_color, Image, ImageColor, PrimaryColorMethod};

// TODO: resize based on first image or user preference
const IMAGE_WIDTH: u32 = 100;
const IMAGE_HEIGHT: u32 = 100;

pub fn read_images<T>(paths: T) -> Vec<ImageColor>
where
    T: Iterator,
    <T as Iterator>::Item: AsRef<Path>,
{
    paths
        .map(|path| ImageReader::open(path).unwrap().decode().unwrap())
        .map(|image| resize(&image, IMAGE_WIDTH, IMAGE_HEIGHT, FilterType::Nearest).into())
        .map(|image| get_primary_color(image, PrimaryColorMethod::Average))
        .collect()
}

pub fn write_image(grid: &OptionalGrid<&Image>, path: &str) {
    let grid_width = <usize as TryInto<u32>>::try_into(grid.cols()).unwrap();
    let grid_height = <usize as TryInto<u32>>::try_into(grid.rows()).unwrap();
    let mut image = ImageBuffer::new(grid_width * IMAGE_WIDTH, grid_height * IMAGE_HEIGHT);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let grid_x = (x / IMAGE_WIDTH) as usize;
        let grid_y = (y / IMAGE_HEIGHT) as usize;
        let tile_x = x % IMAGE_WIDTH;
        let tile_y = y % IMAGE_HEIGHT;

        *pixel = if let Some(image) = &grid[(grid_y, grid_x)] {
            image.get_pixel(tile_x, tile_y)
        } else {
            Rgba::from([0, 0, 0, 0])
        };
    }

    image.save(path).unwrap();
}
