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
    static ref KEY_RING_SIZE: u32 = env::var("KEY_RING_SIZE")
        .unwrap_or(58.to_string())
        .parse::<u32>()
        .unwrap();
    static ref KEY_POOL_SIZE: u32 = env::var("POOL_SIZE")
        .unwrap_or(1000.to_string())
        .parse::<u32>()
        .unwrap();
    static ref NODE_ID_SIZE: f32 = env::var("NODE_ID_SIZE")
        .unwrap_or(4.to_string())
        .parse::<f32>()
        .unwrap();
    static ref AES_BLOCK_SIZE: u32 = env::var("AES_BLOCK_SIZE")
        .unwrap_or(16.to_string())
        .parse::<u32>()
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

pub fn pairwise_communication_energy(nodes: NodesVec, aes_block_size: u32) -> f32 {
    let mut energy = 0.0;
        let neighbors_ids_size = *NODE_ID_SIZE as f32 ** KEY_RING_SIZE as f32;
        let message_size_before_encryption: f32 = *MESSAGE_TYPE_SIZE + *NODE_ID_SIZE + *NONCE_SIZE + neighbors_ids_size;
        let message = if message_size_before_encryption % aes_block_size as f32 == 0.0 {
            message_size_before_encryption
        } else {
            (message_size_before_encryption as u32 / aes_block_size + 1) as f32 * aes_block_size as f32
        };
        let sent_energy = message as f32 * *EPSB;
    for node in nodes.iter() {
        if node.kind == NodeType::Gateway {
            continue;
        }
        // For each neighbor of this node, we receive NODE_ID_SIZE * KEY_RING_SIZE bytes
        let received_energy = node.neighbors.len() as f32 * message * *EPRB;
        energy += sent_energy + received_energy;
    }
    energy
}
