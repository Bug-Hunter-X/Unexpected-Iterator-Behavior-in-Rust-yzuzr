# Unexpected Iterator Behavior in Rust

This repository demonstrates a common, yet subtle, error when working with iterators in Rust. The code iterates over a vector, but attempts to access elements beyond the vector's bounds, leading to unexpected `None` values.

## Bug Description
The provided code demonstrates how using an iterator and then attempting to access more elements than are available can lead to unexpected behavior.  The loop attempts to access 5 elements, but the vector only contains 3.  This results in `None` values being returned for elements 4 and 5. 

## Solution
The solution modifies the loop to stop once the iterator returns `None`, avoiding the out-of-bounds access.

## How to Reproduce
1. Clone the repository.
2. Run `cargo run`.
3. Observe the output, which shows the unexpected `None` values.
4. Run `cargo run --example solution` to see the corrected output.