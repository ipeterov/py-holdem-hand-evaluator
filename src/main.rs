mod constants;
mod montecarlo;

use crate::montecarlo::MonteCarloSimulation;
use std::io;

fn main() {
    println!("Enter your cards separated by spaces:");
    let mut my_cards = String::new();
    io::stdin()
        .read_line(&mut my_cards)
        .expect("Failed to read line");

    println!("Enter common cards separated by spaces:");
    let mut common_cards = String::new();
    io::stdin()
        .read_line(&mut common_cards)
        .expect("Failed to read line");

    println!("Enter the number of opponents:");
    let mut other_player_count_str = String::new();
    io::stdin()
        .read_line(&mut other_player_count_str)
        .expect("Failed to read line");
    let other_player_count: usize = other_player_count_str
        .trim()
        .parse()
        .expect("Have to input an integer");

    println!("How many simulations to run:");
    let mut n_rounds_str = String::new();
    io::stdin()
        .read_line(&mut n_rounds_str)
        .expect("Failed to read line");
    let n_rounds: i32 = n_rounds_str
        .trim()
        .parse()
        .expect("Have to input an integer");

    let simulation =
        MonteCarloSimulation::new(&my_cards, &common_cards, other_player_count, n_rounds);

    let result = simulation.run_simulation();
    println!("Your equity is: {}", result);
}
