use std::ops::{Deref, DerefMut};

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

pub mod methods;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NodeType {
    Gateway,
    Constrained,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    pub kind: NodeType,
    pub neighbors: Vec<usize>,
    pub max_possible_neighbors: usize,
}

#[derive(Debug, Clone)]
pub struct NodesVec(Vec<Node>);

impl NodesVec {
    fn new() -> Self {
        Self(Vec::new())
    }
}

impl Deref for NodesVec {
    type Target = Vec<Node>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NodesVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Node {
    fn new(
        id: usize,
        kind: NodeType,
        neighbors: Vec<usize>,
        max_possible_neighbors: usize,
    ) -> Self {
        Self {
            id,
            kind,
            neighbors,
            max_possible_neighbors,
        }
    }
}

pub fn initialize_network(
    number_of_nodes: i32,
    number_of_gateways: i32,
    number_of_min_possible_neighbors: i32,
    number_of_max_possible_neighbors: i32,
) -> NodesVec {
    let mut nodes: NodesVec = NodesVec::new();

    let mut rng: ThreadRng = thread_rng();

    // Push gateway nodes
    for _ in 0..number_of_gateways {
        let number_of_neighbors: i32 =
            rng.gen_range(number_of_min_possible_neighbors..=number_of_max_possible_neighbors);
        let node = Node::new(
            nodes.len(),
            NodeType::Gateway,
            vec![],
            number_of_neighbors as usize,
        );
        nodes.push(node);
    }

    // Push constrained nodes
    for _ in 0..(number_of_nodes - number_of_gateways) {
        let number_of_neighbors: i32 =
            rng.gen_range(number_of_min_possible_neighbors..=number_of_max_possible_neighbors);
        let node = Node::new(
            nodes.len(),
            NodeType::Constrained,
            vec![],
            number_of_neighbors as usize,
        );
        nodes.push(node);
    }

    // Sort nodes randomly
    nodes.shuffle(&mut rng);

    // Update id of each node to match its index in the nodes array
    for i in 0..nodes.len() {
        nodes[i].id = i;
    }

    // Start adding neighbors to each node
    for i in 0..nodes.len() {
        let current_node_id: usize = nodes[i].id;
        let number_of_current_neighbors: usize = nodes[i].neighbors.len();
        let number_of_current_max_possible_neighbors: usize = nodes[i].max_possible_neighbors;
        let number_of_current_remaining_possible_neighbors: usize =
            number_of_current_max_possible_neighbors - number_of_current_neighbors;
        // For each remaining possible neighbor, add it to the current node if it is not already a neighbor
        for _j in 0..number_of_current_remaining_possible_neighbors {
            // Get the list of nodes that are not already neighbors of the current node and that are not the current node itself and that do not have the maximum number of neighbors
            let possible_neighbors: Vec<usize> = (0..nodes.len())
                .filter(|&k| k != i)
                .filter(|&k| !nodes[i].neighbors.contains(&k))
                .filter(|&k| nodes[k].neighbors.len() < nodes[k].max_possible_neighbors)
                .collect();

            // If there are no possible neighbors, break the loop
            if possible_neighbors.len() == 0 {
                break;
            }

            // Pick a random node from the list of possible neighbors
            let neighbor_index: usize = rng.gen_range(0..possible_neighbors.len());
            let neighbor: usize = possible_neighbors[neighbor_index];
            let neighbor_id: usize = nodes[neighbor].id;
            // Add the neighbor id to the current node's list of neighbors
            nodes[i].neighbors.push(neighbor_id);
            // Add the current node id to the neighbor's list of neighbors
            nodes[neighbor].neighbors.push(current_node_id);
        }
    }

    nodes
}
