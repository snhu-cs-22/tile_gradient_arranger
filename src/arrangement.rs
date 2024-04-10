use grid::Grid;
use itertools::Itertools;
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::graph::Graph;
use petgraph::visit::{Dfs, Walker};
use petgraph::Undirected;

use super::colors::{color_similarity, ColorSimilarityMethod, Image, ImageColor};

/// NOTE: Grid is assumed to be in row-major order
pub type OptionalGrid<T> = Grid<Option<T>>;

type ImageGraph = Graph<ImageColor, u32, Undirected>;

pub fn arrange_images(graph: &ImageGraph) -> OptionalGrid<&Image> {
    // Build out the graph from the node with the most neighbors
    let most_popular = graph
        .node_indices()
        .max_by_key(|i| graph.neighbors(*i).count())
        .unwrap();
    let mut build_order = Dfs::new(&graph, most_popular)
        .iter(&graph)
        .map(|i| &graph[i].image);

    // TODO: Arrange the images on the image grid
    let mut grid = Grid::init(1, 1, build_order.next());
    for image in build_order {
        grid.push_col(vec![Some(image)]);
    }
    grid
}

pub fn build_graph(image_colors: Vec<ImageColor>) -> ImageGraph {
    // Add nodes
    let mut graph = ImageGraph::with_capacity(image_colors.len(), image_colors.len().pow(2));
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
            graph.add_edge(
                a,
                b,
                color_similarity(
                    graph[a].color,
                    graph[b].color,
                    ColorSimilarityMethod::DotProduct,
                ),
            );
        }
    }

    ImageGraph::from_elements(min_spanning_tree(&graph))
}
