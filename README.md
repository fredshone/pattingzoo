# Pattingzoo - Pettingzoo Environment using Rust

Experiment using [maturin](https://github.com/PyO3/maturin) and [pyo3](https://pyo3.rs) to provide a Rust implementation of a [Pettingzoo](https://github.com/Farama-Foundation/PettingZoo) environment.

PettingZoo is a Python library for conducting research in multi-agent reinforcement learning, akin to a multi-agent version of [Gymnasium](https://github.com/Farama-Foundation/Gymnasium).

These is just a toy project - I've implemented the very simple "escape" game from this [tutorial](https://pettingzoo.farama.org/tutorials/custom_environment/1-project-structure/). Once you've cloned and set up your (python) environment (you will also need rust installed), you can test progress as follows:

```
maturin develop
pat
```

This should build the project (python and rust), then run an API test from the Pettingzoo project.

Many thanks to 	[Maxwell Flitton](https://github.com/maxwellflitton) and [Dr Caroline Morton](https://github.com/CarolineMorton) and [Surrealdb](https://surrealdb.com/) for their workshop!