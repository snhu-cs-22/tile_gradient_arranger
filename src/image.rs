use std::fs::read_dir;
use std::path::Path;

use image::imageops::{resize, FilterType};
use image::io::Reader as ImageReader;
use image::{GenericImageView, ImageBuffer, Rgba};

use super::arrangement::OptionalGrid;
use super::colors::{get_primary_color, Image, ImageColor};

pub fn read_images(dir: &Path, k_means: u32, tile_size: (u32, u32)) -> Vec<ImageColor> {
    read_dir(dir)
        .expect("Path must be directory")
        .filter_map(|x| x.ok())
        .map(|x| x.path())
        .filter(|x| x.is_file())
        .filter_map(|path| ImageReader::open(path).ok())
        .filter_map(|image| image.decode().ok())
        .map(|image| resize(&image, tile_size.0, tile_size.1, FilterType::Nearest).into())
        .map(|image| get_primary_color(image, k_means))
        .collect()
}

pub fn write_image<T: AsRef<Path>>(grid: &OptionalGrid<&Image>, path: T, tile_size: (u32, u32)) {
    let grid_width = <usize as TryInto<u32>>::try_into(grid.cols()).unwrap();
    let grid_height = <usize as TryInto<u32>>::try_into(grid.rows()).unwrap();
    let mut image = ImageBuffer::new(grid_width * tile_size.0, grid_height * tile_size.1);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let grid_x = (x / tile_size.0) as usize;
        let grid_y = (y / tile_size.1) as usize;
        let tile_x = x % tile_size.0;
        let tile_y = y % tile_size.1;

        *pixel = if let Some(image) = &grid[(grid_y, grid_x)] {
            image.get_pixel(tile_x, tile_y)
        } else {
            Rgba::from([0, 0, 0, 0])
        };
    }

    image.save(path).unwrap();
}
