use std::fs::read_dir;
use std::path::Path;

use image::imageops::{resize, FilterType};
use image::io::Reader as ImageReader;
use image::{GenericImage, ImageBuffer};

use super::arrangement::OptionalGrid;
use super::colors::{get_primary_color, ImageColor};

pub fn read_images(dir: &Path, k_means: u32, tile_size: (u32, u32)) -> Vec<ImageColor> {
    read_dir(dir)
        .expect("Path must be directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .filter_map(|path| {
            let path_str = &path.display();
            ImageReader::open(&path)
                .inspect_err(|e| eprintln!("\"{path_str}\": {e}"))
                .ok()?
                .decode()
                .inspect_err(|e| eprintln!("\"{path_str}\": {e}"))
                .ok()
        })
        .map(|image| resize(&image, tile_size.0, tile_size.1, FilterType::CatmullRom).into())
        .enumerate()
        .map(|(i, image)| {
            eprintln!("Getting primary color of image #{i}");
            get_primary_color(image, k_means)
        })
        .collect()
}

pub fn write_image<T: AsRef<Path>>(
    grid: &OptionalGrid<&ImageColor>,
    path: T,
    tile_size: (u32, u32),
) {
    let grid_width = <usize as TryInto<u32>>::try_into(grid.cols()).unwrap();
    let grid_height = <usize as TryInto<u32>>::try_into(grid.rows()).unwrap();
    let mut output = ImageBuffer::new(grid_width * tile_size.0, grid_height * tile_size.1);

    for ((y, x), tile) in grid.indexed_iter() {
        if let Some(image_color) = tile {
            output
                .copy_from(
                    &image_color.image,
                    <usize as TryInto<u32>>::try_into(x).unwrap() * tile_size.0,
                    <usize as TryInto<u32>>::try_into(y).unwrap() * tile_size.1,
                )
                .unwrap();
        }
    }

    output.save(path).unwrap();
}
