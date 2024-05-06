use grid::Grid;
use itertools::Itertools;
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::graph::UnGraph;
use petgraph::visit::{Dfs, Walker};

use super::colors::{color_similarity, Image, ImageColor};

/// NOTE: Grid is assumed to be in row-major order
pub type OptionalGrid<T> = Grid<Option<T>>;

type ImageGraph = UnGraph<ImageColor, f32>;

pub fn arrange_images(graph: &ImageGraph, image_count: usize) -> OptionalGrid<&Image> {
    // Build out the graph from the node with the most neighbors
    let most_popular = graph
        .node_indices()
        .max_by_key(|i| graph.neighbors(*i).count())
        .unwrap();
    let build_order = Dfs::new(&graph, most_popular)
        .iter(&graph)
        .map(|i| &graph[i].image);

    // TODO: Arrange the images on the image grid
    let square_size = (image_count as f32).sqrt().ceil() as usize;
    let mut grid = Grid::init(square_size, square_size, None);
    let coords = SpiralGridCoords::new((square_size / 2, square_size / 2));
    for (image, coords) in build_order.zip(coords) {
        grid[coords] = Some(image);
    }
    grid
}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Down,
    Right,
    Up,
    Left,
}

struct SpiralGridCoords {
    current_coords: (usize, usize),
    current_direction: Direction,
    until_side_length: usize,
    current_side_length: usize,
}

impl SpiralGridCoords {
    pub fn new(start_coords: (usize, usize)) -> Self {
        Self {
            current_coords: start_coords,
            current_direction: Direction::Down,
            until_side_length: 0,
            current_side_length: 0,
        }
    }

    fn get_next(&mut self) -> (usize, usize) {
        if self.current_side_length == 0 {
            self.current_side_length += 1;
            return self.current_coords;
        }

        self.current_coords = match self.current_direction {
            Direction::Down => (self.current_coords.0, self.current_coords.1 + 1),
            Direction::Right => (self.current_coords.0 + 1, self.current_coords.1),
            Direction::Up => (self.current_coords.0, self.current_coords.1 - 1),
            Direction::Left => (self.current_coords.0 - 1, self.current_coords.1),
        };

        self.until_side_length += 1;
        if self.until_side_length == self.current_side_length {
            self.until_side_length = 0;
            if self.current_direction == Direction::Left
                || self.current_direction == Direction::Right {
                self.current_side_length += 1;
            }
            self.current_direction = match self.current_direction {
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
            };
        }

        self.current_coords
    }
}

impl Iterator for SpiralGridCoords {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get_next())
    }
}

pub fn build_graph(image_colors: Vec<ImageColor>) -> ImageGraph {
    let node_count = image_colors.len();
    let edge_count = (node_count * (node_count - 1)) / 2;
    let mut graph = ImageGraph::with_capacity(node_count, edge_count);

    // Add nodes
    for image_color in image_colors {
        graph.add_node(image_color);
    }

    // Add edges
    let node_pairs = graph.node_indices().permutations(2);
    for pair in node_pairs {
        let a = pair[0];
        let b = pair[1];
        graph.add_edge(a, b, color_similarity(graph[a].color, graph[b].color));
    }

    ImageGraph::from_elements(min_spanning_tree(&graph))
}
