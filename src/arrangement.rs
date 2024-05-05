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
    let mut build_order = Dfs::new(&graph, most_popular)
        .iter(&graph)
        .map(|i| &graph[i].image);

    // TODO: Arrange the images on the image grid
    let square_size = (image_count as f32).sqrt().ceil() as usize;
    let mut grid = Grid::init(square_size, square_size, None);
    for cell in grid.iter_mut() {
        *cell = build_order.next();
    }
    grid
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
