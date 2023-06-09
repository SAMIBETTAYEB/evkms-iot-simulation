use std::env;

use lazy_static::lazy_static;

use crate::{NodeType, NodesVec};

lazy_static! {
    static ref EPSB: f32 = env::var("EPSB")
        .unwrap_or(0.0001.to_string())
        .parse::<f32>()
        .unwrap();
    static ref EPRB: f32 = env::var("EPRB")
        .unwrap_or(0.0001.to_string())
        .parse::<f32>()
        .unwrap();
    static ref HASH_ENERGY: f32 = env::var("HASH_ENERGY")
        .unwrap_or(0.0001.to_string())
        .parse::<f32>()
        .unwrap();
    static ref ENCRYPTION_ENERGY: f32 = env::var("ENCRYPTION_ENERGY")
        .unwrap_or(0.0001.to_string())
        .parse::<f32>()
        .unwrap();
    static ref SENT_MESSAGE_SIZE: u32 = env::var("SENT_MESSAGE_SIZE")
        .unwrap_or(16.to_string())
        .parse::<u32>()
        .unwrap();
    static ref RECEIVED_MESSAGE_SIZE: u32 = env::var("RECEIVED_MESSAGE_SIZE")
        .unwrap_or(16.to_string())
        .parse::<u32>()
        .unwrap();
    static ref NODE_ID_SIZE: f32 = env::var("NODE_ID_SIZE")
        .unwrap_or(4.to_string())
        .parse::<f32>()
        .unwrap();
    static ref MESSAGE_TYPE_SIZE: f32 = env::var("MESSAGE_TYPE_SIZE")
        .unwrap_or(1.to_string())
        .parse::<f32>()
        .unwrap();
    static ref NONCE_SIZE: f32 = env::var("NONCE_SIZE")
        .unwrap_or(4.to_string())
        .parse::<f32>()
        .unwrap();
}

pub fn number_of_multiplications(nodes: NodesVec) -> u32 {
    let mut number_of_multiplications = 0;
    for node in nodes.iter() {
        number_of_multiplications += node.neighbors.len() as u32;
    }
    number_of_multiplications
}

pub fn number_pairwise_hashes(nodes: NodesVec) -> u32 {
    let mut number_of_hashes = 0;
    for node in nodes.iter() {
        number_of_hashes += 2 * node.neighbors.len() as u32;
    }
    number_of_hashes
}

pub fn number_of_pairwise_encryptions(_nodes: NodesVec) -> u32 {
    0
}

pub fn pairwise_communication_energy(nodes: NodesVec, mac_size: u32) -> f32 {
    let mut energy = 0.0;
    for node in nodes.iter() {
        if node.kind == NodeType::Gateway {
            continue;
        }
        energy += (*MESSAGE_TYPE_SIZE + *NODE_ID_SIZE + *NONCE_SIZE + mac_size as f32)
            * *EPSB;
        energy += node.neighbors.len() as f32
            * (*MESSAGE_TYPE_SIZE + *NODE_ID_SIZE + *NONCE_SIZE + mac_size as f32)
            * *EPRB;
        // Only 1/2 of nodes will send reply with an encrypted message and the other half will receive it
        energy += node.neighbors.len() as f32 * mac_size as f32 * *EPSB / 2 as f32;
        energy += node.neighbors.len() as f32 * mac_size as f32 * *EPRB / 2 as f32;
    }
    energy
}

pub fn pairwise_communication_sent(nodes: NodesVec, mac_size: u32) -> f32 {
    let mut sent = 0.0;
    for node in nodes.iter() {
        if node.kind == NodeType::Gateway {
            continue;
        }
        sent += *MESSAGE_TYPE_SIZE + *NODE_ID_SIZE + *NONCE_SIZE + mac_size as f32;
        // Only 1/2 of nodes will send reply with an encrypted message and the other half will receive it
        sent += node.neighbors.len() as f32 * mac_size as f32 / 2 as f32;
    }
    sent
}

pub fn pairwise_communication_received(nodes: NodesVec, mac_size: u32) -> f32 {
    let mut received = 0.0;
    for node in nodes.iter() {
        if node.kind == NodeType::Gateway {
            continue;
        }
        received += node.neighbors.len() as f32
            * (*MESSAGE_TYPE_SIZE + *NODE_ID_SIZE + *NONCE_SIZE + mac_size as f32);
        // Only 1/2 of nodes will send reply with an encrypted message and the other half will receive it
        received += node.neighbors.len() as f32 * mac_size as f32 / 2 as f32;
    }
    received
}

pub fn groupwise_communication_energy(_nodes: NodesVec) -> f32 {
    0.0
}

pub fn groupwise_encryptions_energy(nodes: NodesVec) -> f32 {
    let mut energy = 0.0;
    let constrained_nodes: NodesVec = NodesVec(
        nodes
            .iter()
            .filter(|node| node.kind == NodeType::Constrained)
            .map(|node| node.clone())
            .collect(),
    );
    for node in constrained_nodes.iter() {
        energy += 2 as f32 * node.neighbors.len() as f32 * *ENCRYPTION_ENERGY;
    }
    energy
}

pub fn groupwise_hashes_energy(nodes: NodesVec) -> f32 {
    let mut energy = 0.0;
    let constrained_nodes: NodesVec = NodesVec(
        nodes
            .iter()
            .filter(|node| node.kind == NodeType::Constrained)
            .map(|node| node.clone())
            .collect(),
    );
    for node in constrained_nodes.iter() {
        energy += node.neighbors.len() as f32 * *HASH_ENERGY;
    }
    energy
}

pub fn groupwise_computation_energy(nodes: NodesVec) -> f32 {
    let mut energy = 0.0;
    let constrained_nodes: NodesVec = NodesVec(
        nodes
            .iter()
            .filter(|node| node.kind == NodeType::Constrained)
            .map(|node| node.clone())
            .collect(),
    );
    // Calculate the hashes energy
    let total_hash_energy: f32 = groupwise_hashes_energy(constrained_nodes.clone());
    // Calculate the encryptions energy
    let total_encryption_energy: f32 = groupwise_encryptions_energy(constrained_nodes.clone());
    // Sum all the energies
    energy += total_hash_energy + total_encryption_energy;
    energy
}

pub fn groupwise_total_energy(nodes: NodesVec) -> f32 {
    let mut energy = 0.0;
    // Calculate the pairwise communication energy
    let groupewise_communication_energy: f32 = groupwise_communication_energy(nodes.clone());
    // Calculate the groupwise computation energy
    let groupwise_computation_energy: f32 = groupwise_computation_energy(nodes.clone());
    // Sum all the energies
    energy += groupewise_communication_energy + groupwise_computation_energy;
    energy
}
