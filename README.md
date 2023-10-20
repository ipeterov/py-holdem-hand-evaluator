# py-holdem-hand-evaluator
Python bindings for [holdem-hand-evaluator](https://github.com/ipeterov/py-holdem-hand-evaluator/blob/main/pyproject.toml)


## Installation

```shell
pip install holdem-hand-evaluator
```

### Usage

Currently only the `evaluate_hand` function is implemented.

```
>>> import holdem_hand_evaluator
>>> holdem_hand_evaluator.evaluate_hand("AcKcQcJcTc9c8c")
32777
```

Also, the library includes a monte-carlo simulation that I wrote. It calculates equity - the chance that you will win based on the cards that you can see right now.

It uses `holdem_hand_evaluator` to determine the outcomes of games. Here's how you can use it:

```
>>> import holdem_hand_evaluator
>>> holdem_hand_evaluator.calculate_equity(
    my_cards="Ah Ac",
    common_cards="2d 3d 4d",
    other_player_count=1,
    n_rounds=1000000,
)
0.7054749727249146
```

On my M1 Mac it's able to run a million simulations in 50ms. This makes sure that 95% of the results are within a 0.2% band.

### How to release a new version

1. Change the version in [cargo.toml](cargo.toml) and push the changes
2. Manually "Create a new release" with GitHub - this will trigger the workflow