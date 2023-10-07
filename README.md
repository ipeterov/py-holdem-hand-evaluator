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

### How to release a new version

1. Change the version in [cargo.toml](cargo.toml) and push the changes
2. Manually "Create a new release" with GitHub - this will trigger the workflow