## Rust algorithms
My goal is to learn more about algorithms and the Rust programming language and these are some algorithms I wrote in Rust to solve Leetcode problems.

For each solution I included tests to validate the answer. I try to approach these TDD, meaning I first write the tests and validate the correct output before submitting my answer.

I purposefully did not use any AI tooling to create these algorithms, as that would be contrary to my goal.

### Installing Rust
To install Rust, please have a look at the official docs: https://www.rust-lang.org/tools/install

### The application itself
This codebase is setup as a Cargo crate, where for each solution a new module is created and the functions are exposed in there. Accordingly, this means this application does not run as such, but rather exposes functionality to be used by others when they import this crate.

You can verify the functionality of the exposed modules in this crate by running the tests.

### Running the tests
You can either run the tests locally or use a Docker container to run them. 
* To run all tests, run the command `cargo run`
* To run a single test, run the command `cargo run --test example_code_tests`