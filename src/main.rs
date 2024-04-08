use std::env::args;

// NOTE: Grid is assumed to be in row-major order
use grid::Grid;
use image::imageops::{resize, FilterType};
use image::io::Reader as ImageReader;
use image::{ImageBuffer, DynamicImage, GenericImageView, Rgb, Rgba};
use itertools::Itertools;
use petgraph::Undirected;
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::{Bfs, Dfs, Walker};

fn main() {
    let image_paths = args().skip(1);

    // TODO: resize based on first image or user preference
    let image_width = 100;
    let image_height = 100;

    let image_colors = image_paths
        .map(|path| ImageReader::open(path).unwrap().decode().unwrap())
        .map(|image| resize(&image, image_width, image_height, FilterType::Nearest).into())
        .map(|image| calculate_dominant_color(image, 4))
        .collect::<Vec<_>>();
    let graph = build_graph(image_colors);
    let grid = arrange_images(&graph);
    write_image(&grid, "./image.png", image_width, image_height);
}

type Color = Rgb<u8>;
type ImageGraph = Graph<ImageColor, u32, Undirected>;
type Image = DynamicImage;

#[derive(Clone)]
struct ImageColor {
    pub image: Image,
    pub color: Color,
}

fn calculate_dominant_color(image: Image, clusters: u32) -> ImageColor {
    // TODO: swap average for k-means
    // TODO: speed this up
    let dimensions = image.dimensions();
    let total_pixels = dimensions.0 as u64 * dimensions.1 as u64;
    let sum = image
        .pixels()
        .fold([0u64; 3], |mut acc, e| {
            acc[0] = acc[0].checked_add(e.2[0] as u64).expect("Sum got way too big");
            acc[1] = acc[1].checked_add(e.2[1] as u64).expect("Sum got way too big");
            acc[2] = acc[2].checked_add(e.2[2] as u64).expect("Sum got way too big");
            acc
        });
    let color = Color::from([
        (sum[0] / total_pixels) as u8,
        (sum[1] / total_pixels) as u8,
        (sum[2] / total_pixels) as u8,
    ]);

    ImageColor {
        image,
        color,
    }
}

fn build_graph(image_colors: Vec<ImageColor>) -> ImageGraph {
    // Add nodes
    let mut graph = ImageGraph::new_undirected();
    let mut nodes = Vec::with_capacity(image_colors.len());
    for image_color in image_colors {
        let node = graph.add_node(image_color);
        nodes.push(node);
    }

    // Add edges
    if nodes.len() > 1 {
        let node_pairs = nodes.iter().permutations(2);
        for pair in node_pairs {
            let a = *pair[0];
            let b = *pair[1];
            graph.add_edge(a, b, color_similarity(graph[a].color, graph[b].color));
        }
    }

    ImageGraph::from_elements(min_spanning_tree(&graph))
}

fn color_similarity(a: Color, b: Color) -> u32 {
    // Calculate dot product of two RGB vectors
    a[0] as u32 * b[0] as u32
    + a[1] as u32 * b[1] as u32
    + a[2] as u32 * b[2] as u32
}

fn arrange_images(graph: &ImageGraph) -> Grid<Option<&Image>> {
    // Build out the graph from the node with the most neighbors
    let mut most_popular = NodeIndex::new(0);
    for i in graph.node_indices() {
        if graph.neighbors(i).count() > graph.neighbors(most_popular).count() {
            most_popular = i;
        }
    }
    let mut build_order = Dfs::new(&graph, most_popular).iter(&graph).map(|i| &graph[i].image);

    // TODO: Arrange the images on the image grid
    let mut grid = Grid::init(1, 1, build_order.next());
    for image in build_order {
        grid.push_col(vec![Some(image)]);
    }
    grid
}

fn write_image(grid: &Grid<Option<&Image>>, path: &str, tile_width: u32, tile_height: u32) {
    let grid_width = <usize as TryInto<u32>>::try_into(grid.cols()).unwrap();
    let grid_height = <usize as TryInto<u32>>::try_into(grid.rows()).unwrap();
    let mut image = ImageBuffer::new(grid_width * tile_width,  grid_height * tile_height);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let grid_x = (x / tile_width) as usize;
        let grid_y = (y / tile_height) as usize;
        let tile_x = x % tile_width;
        let tile_y = y % tile_height;

        *pixel = if let Some(image) = &grid[(grid_y, grid_x)] {
            image.get_pixel(tile_x, tile_y)
        } else {
            Rgba::from([0, 0, 0, 0])
        };
    }

    image.save(path).unwrap();
}
