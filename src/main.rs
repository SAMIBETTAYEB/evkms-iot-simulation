#![allow(unused_imports)]
use std::env;

use dotenv::dotenv;
// use NodesVec from lib.rs
use evkms_metrics_simulation::{
    initialize_network,
    methods::{evkms, matrix},
    NodesVec,
};

fn main() {
    dotenv().ok();
    simulate();
}

fn simulate() {
    let number_of_nodes = env::var("NUMBER_OF_NODES")
        .expect("NUMBER_OF_NODES must be set")
        .parse::<i32>()
        .expect("NUMBER_OF_NODES must be a number");
    let number_of_gateways = env::var("NUMBER_OF_GATEWAYS")
        .expect("NUMBER_OF_GATEWAYS must be set")
        .parse::<i32>()
        .expect("NUMBER_OF_GATEWAYS must be a number");
    let number_of_min_possible_neighbors = env::var("NUMBER_OF_MIN_POSSIBLE_NEIGHBORS")
        .expect("NUMBER_OF_MIN_POSSIBLE_NEIGHBORS must be set")
        .parse::<i32>()
        .expect("NUMBER_OF_MIN_POSSIBLE_NEIGHBORS must be a number");
    let number_of_max_possible_neighbors = env::var("NUMBER_OF_MAX_POSSIBLE_NEIGHBORS")
        .expect("NUMBER_OF_MAX_POSSIBLE_NEIGHBORS must be set")
        .parse::<i32>()
        .expect("NUMBER_OF_MAX_POSSIBLE_NEIGHBORS must be a number");
    let number_of_gateway_members = env::var("NUMBER_OF_GATEWAY_MEMBERS")
        .expect("NUMBER_OF_GATEWAY_MEMBERS must be set")
        .parse::<i32>()
        .expect("NUMBER_OF_GATEWAY_MEMBERS must be a number");

    let mut evkms_multiplications: Vec<(u32, u32)> = Vec::new();
    let mut matrix_multiplications: Vec<(u32, u32)> = Vec::new();
    let mut evkms_groupwise_computation_energy: Vec<(u32, f32)> = Vec::new();
    let mut matrix_groupwise_computation_energy: Vec<(u32, f32)> = Vec::new();
    let mut evkms_groupwise_total_energy: Vec<(u32, f32)> = Vec::new();
    let mut matrix_groupwise_total_energy: Vec<(u32, f32)> = Vec::new();

    for i in (10..=number_of_nodes).step_by(10) {
        let mut evkms_sum: u32 = 0;
        let mut matrix_sum: u32 = 0;
        let mut evkms_groupwise_computation_energy_sum: f32 = 0.0;
        let mut matrix_groupwise_computation_energy_sum: f32 = 0.0;
        let mut evkms_groupwise_total_energy_sum: f32 = 0.0;
        let mut matrix_groupwise_total_energy_sum: f32 = 0.0;
        // Simulate 1000 times
        for iteration in 0..1000 {
            println!("Simulation: Number of nodes: {}, iteration: {}", i, iteration);
            let nodes: NodesVec = initialize_network(
                i,
                i/10,
                number_of_min_possible_neighbors,
                number_of_max_possible_neighbors,
            );
            evkms_sum += evkms::number_of_multiplications(nodes.clone());
            matrix_sum += matrix::number_of_multiplications(nodes.clone());

            evkms_groupwise_computation_energy_sum +=
                evkms::groupwise_computation_energy(nodes.clone());
            matrix_groupwise_computation_energy_sum += matrix::groupwise_computation_energy(
                nodes.clone());

            evkms_groupwise_total_energy_sum += evkms::groupwise_total_energy(nodes.clone());
            matrix_groupwise_total_energy_sum += matrix::groupwise_total_energy(nodes.clone());
        }
        evkms_multiplications.push((i as u32, evkms_sum / 1000));
        matrix_multiplications.push((i as u32, matrix_sum / 1000));
        evkms_groupwise_computation_energy.push((i as u32, evkms_groupwise_computation_energy_sum / 1000.0));
        matrix_groupwise_computation_energy.push((i as u32, matrix_groupwise_computation_energy_sum / 1000.0));
        evkms_groupwise_total_energy.push((i as u32, evkms_groupwise_total_energy_sum / 1000.0));
        matrix_groupwise_total_energy.push((i as u32, matrix_groupwise_total_energy_sum / 1000.0));
    }

    println!("evkms_multiplications: {:?}", evkms_multiplications);
    println!("matrix_multiplications: {:?}", matrix_multiplications);
    println!("evkms_groupwise_computation_energy: {:?}", evkms_groupwise_computation_energy);
    println!("matrix_groupwise_computation_energy: {:?}", matrix_groupwise_computation_energy);
    println!("evkms_groupwise_total_energy: {:?}", evkms_groupwise_total_energy);
    println!("matrix_groupwise_total_energy: {:?}", matrix_groupwise_total_energy);
}
