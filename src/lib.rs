use pyo3::prelude::*;
use ::holdem_hand_evaluator::{Hand};

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
    m.add_function(wrap_pyfunction!(evaluate_hand, m)?)?;
    Ok(())
}