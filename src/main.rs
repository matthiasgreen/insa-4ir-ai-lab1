#![allow(unused)] // suppress warnings for unused code (there is plenty when you start)

// declare other modules that are in other files and must be compiled
mod board;
mod heuristics;
mod min_heap;
mod search;

// import the content of the modules
use board::*;
use heuristics::*;
use search::*;

fn main() {
    let board = Board::new([
        [1, 2, 3],
        [4, 8, 5],
        [0, 7, 6]
    ]);
    for (n, instance) in INSTANCES {
        // let blind_res = search(instance, &&Heuristic::Blind, 1);
        // let hamming_res = search(instance, &&Heuristic::Hamming, 1);
        let manhattan_res = search(instance, &&Heuristic::Manhattan, 1);
        let manhattan_weight_2_res = search(instance, &&Heuristic::Manhattan, 2);
        let manhattan_weight_3_res = search(instance, &&Heuristic::Manhattan, 3);

        println!("Diff between optimal and Manhattan weight 2 length: {}", manhattan_weight_2_res.0.unwrap().len() - manhattan_res.0.clone().unwrap().len());
        println!("Diff between optimal and Manhattan weight 3 length: {}", manhattan_weight_3_res.0.unwrap().len() - manhattan_res.0.unwrap().len());
        println!("Manhattan: {:?}\nManhattan weight 2: {:?}\nManhattan weight 3: {:?}\n", manhattan_res.1, manhattan_weight_2_res.1, manhattan_weight_3_res.1);
    }
}
