# Initial Conditions (a physics library in Rust)
[![Build Status](https://travis-ci.org/someguynamedmatt/initial_conditions.svg?branch=master)](https://travis-ci.org/someguynamedmatt/initial_conditions)
[![](https://img.shields.io/crates/v/initial_conditions.svg)](https://crates.io/crates/initial_conditions)

[Read the docs](https://someguynamedmatt.github.io/initial_conditions/initial_conditions/index.html)

## How to use:

Add `initial_conditions` to your `Cargo.toml` file:

```
initial_conditions = "0.3.0"
```
> Check the crates.io badge above to see which version is the latest

And now use it in your project...

```rust
extern crate initial_conditions;

use initial_conditions::kinematics::*;

let w = work::work(/* args */);
```

Be sure to [read the documentation](https://someguynamedmatt.github.io/initial_conditions/initial_conditions/index.html) to see what the lib has to offer.

## Units

All arguments and return values are implicitly returned in SI units (i.e. radians. Specific units are noted in the [documentation](https://someguynamedmatt.github.io/initial_conditions/initial_conditions/index.html))


## Capabilities right now:

- Useful constants
- Kinematics functions

## Planned capabilities:

- Electromagnetism
- Quantum Mechanics
- Astro
____

This is still, very much, a work in progress. The goal of this project is to hold simple (Physics 101) equations, but also more detailed work (n-dimensional physics, integration approximations, etc.). Expect most work to come incrementally, starting from the more simple topics to the more advanced.

Please reach out if something is off or there is any confusion

