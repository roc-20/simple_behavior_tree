# Simple Behavior Tree for Rust

This Rust crate provides a straightforward implementation of behavior trees, offering a compact yet powerful tool for developing AI behaviors and decision-making systems in games and robotics. Behavior trees are a popular and highly effective technique for modeling complex behaviors through a combination of simple tasks.

## Features

- **Sequence Nodes**: Execute children nodes sequentially, stopping at the first failure.
- **Selector Nodes**: Execute children nodes until one succeeds.
- **Parallel Nodes**: Execute children nodes in parallel, supporting both "all must succeed" and "any can succeed" modes.
- **Action Nodes**: Execute custom logic through Rust closures or functions.

## Getting Started

### Prerequisites

Ensure you have Rust and Cargo installed on your machine. Visit [The Rust Programming Language](https://www.rust-lang.org/tools/install) website for installation instructions if needed.

### Installation

Add `simple_behavior_tree` to your project's `Cargo.toml`:

```toml
[dependencies]
simple_behavior_tree = "0.1.1"
```

### Basic Usage

Here's a quick example to get you started with `simple_behavior_tree`:

```rust
use simple_behavior_tree::{BTNode, BTStatus, ParallelMode};

fn main() {
    let action_success = || {
        println!("Action success executed");
        BTStatus::Success
    };

    let action_failure = || {
        println!("Action failure executed");
        BTStatus::Failure
    };

    let mut bt = BTNode::Parallel(
        ParallelMode::AnySuccess,
        vec![
            BTNode::Action(action_success),
            BTNode::Action(action_failure),
        ],
    );

    match bt.tick() {
        BTStatus::Success => println!("Behavior Tree succeeded"),
        BTStatus::Failure => println!("Behavior Tree failed"),
        BTStatus::Running => println!("Behavior Tree is still running"),
    }
}
```

This example creates a parallel node that succeeds if any of its children succeed.

## Contributing

Contributions to `simple_behavior_tree` are welcome! Whether it's bug reports, feature requests, or pull requests, all contributions help make this project better. To contribute:

1. Fork the repository.
2. Create a new branch for each feature or improvement.
3. Send a pull request from each feature branch to the main branch for review.

## License

`simple_behavior_tree` is distributed under the MIT License, see LICENSE for more information.

## Acknowledgments

- Thanks to the Rust community for providing excellent documentation and resources.
- Inspired by various behavior tree implementations and discussions in the game development community.
