# Rand Crate Knowledge Base (v0.10.0)

## Overview
`rand` is the standard crate for random number generation in Rust. It provides utilities to generate random numbers, convert them to useful types and distributions, and implements various randomness-related algorithms.

## Quick Start
The `prelude` import is essential for common functionality like `Rng::random`, `Rng::sample`, `SliceRandom::shuffle`, and `IndexedRandom::choose`.

```rust
use rand::prelude::*;

// Accessing a thread-local generator:
let mut rng = rand::rng();

// Generate a random value (e.g., a char):
let c = rng.random::<char>();

// Generate a random value in a range:
let n = rng.random_range(0..10);

// Shuffle a slice/vector:
let mut nums = vec![1, 2, 3, 4, 5];
nums.shuffle(&mut rng);
```

## Core Components

### Key Functions
- **`rand::rng()`**: (New in 0.10) Accesses a fast, pre-initialized, thread-local random number generator.
- **`rand::random()`**: Generates a random value using the thread-local generator.
- **`rand::random_range(range)`**: Generates a random value within the specified range.
- **`rand::random_bool(p)`**: Returns a `bool` with probability `p` of being `true`.

### Key Traits
- **`Rng`**: The primary trait for random number generation. Methods include `random`, `gen`, `sample`, `random_range`, etc.
- **`RngCore`**: Low-level trait for generating raw bits.
- **`SeedableRng`**: Allows creating an RNG from a specific seed (useful for deterministic testing).

### Common Modules
- **`rand::distr`**: Contains probability distributions (e.g., `Alphanumeric`, `Uniform`) used with the `sample` method.
- **`rand::prelude`**: Re-exports the most commonly used items (`Rng`, `RngCore`, etc.).

## Implementation Tips for Project
- **Deterministic Testing**: When writing unit tests that require specific random outcomes, use `SeedableRng` with a fixed seed instead of `rand::rng()`.
- **Efficient Generation**: Use `rand::rng()` for most tasks to leverage the efficient thread-local generator.
