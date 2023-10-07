mod constants;
mod montecarlo;

use crate::montecarlo::MonteCarloSimulation;
use ::holdem_hand_evaluator::Hand;
use pyo3::prelude::*;

#[pyfunction]
fn calculate_equity(
    my_cards: String,
    common_cards: String,
    other_player_count: i8,
    n_rounds: i32,
) -> f32 {
    let my_cards_string = String::from(my_cards);
    let common_cards_string = String::from(common_cards);
    let simulation = MonteCarloSimulation::new(
        &my_cards_string,
        &common_cards_string,
        other_player_count,
        n_rounds,
    );
    return simulation.run_simulation();
}

#[pyfunction]
fn evaluate_hand(hand_str: String) -> u16 {
    let hand = hand_str.parse::<Hand>().unwrap();
    return hand.evaluate();
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn holdem_hand_evaluator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_equity, m)?)?;
    m.add_function(wrap_pyfunction!(evaluate_hand, m)?)?;
    Ok(())
}
