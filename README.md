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
simple_behavior_tree = "0.1.2"
```

### Basic Usage

This crate allows you to construct behavior trees using sequence nodes, selector nodes, parallel nodes, and action nodes. Below are quick examples demonstrating how to create a simple behavior tree with these node types:



#### Sequence Node Example

Sequence nodes execute their children sequentially, stopping at the first failure. Here's how to create and execute a sequence node:

```rust
use simple_behavior_tree::{BTNode, BTStatus};

fn main() {
    let action_success = || {
        println!("Action success executed");
        BTStatus::Success
    };

    let action_failure = || {
        println!("Action failure executed");
        BTStatus::Failure
    };

    // Create a sequence node
    let sequence_node = BTNode::Sequence(vec![
        BTNode::Action(action_success),
        BTNode::Action(action_failure),
    ]);

    // Execute the sequence node
    println!("Executing Sequence Node:");
    match sequence_node.tick() {
        BTStatus::Success => println!("Sequence Node succeeded"),
        BTStatus::Failure => println!("Sequence Node failed"),
        BTStatus::Running => println!("Sequence Node is still running"),
    }
}
```

#### Selector Node Example

Selector nodes execute their children until one succeeds. Here's how to create and execute a selector node:

```rust
use simple_behavior_tree::{BTNode, BTStatus};

fn main() {
    let action_success = || {
        println!("Action success executed");
        BTStatus::Success
    };

    let action_failure = || {
        println!("Action failure executed");
        BTStatus::Failure
    };

    // Create a selector node
    let selector_node = BTNode::Selector(vec![
        BTNode::Action(action_failure),
        BTNode::Action(action_success),
    ]);

    // Execute the selector node
    println!("\nExecuting Selector Node:");
    match selector_node.tick() {
        BTStatus::Success => println!("Selector Node succeeded"),
        BTStatus::Failure => println!("Selector Node failed"),
        BTStatus::Running => println!("Selector Node is still running"),
    }
}
```


#### Parallel Node Example

Parallel nodes execute all children in parallel and succeed based on the configured mode (e.g., AnySuccess). Here's how to create and execute a parallel node with AnySuccess mode:

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

    // Create a parallel node with AnySuccess mode
    let parallel_node = BTNode::Parallel(
        ParallelMode::AnySuccess,
        vec![
            BTNode::Action(action_success),
            BTNode::Action(action_failure),
        ],
    );

    // Execute the parallel node
    println!("\nExecuting Parallel Node:");
    match parallel_node.tick() {
        BTStatus::Success => println!("Parallel Node succeeded"),
        BTStatus::Failure => println!("Parallel Node failed"),
        BTStatus::Running => println!("Parallel Node is still running"),
    }
}
```

This example demonstrates the basic setup for sequence, selector, and parallel nodes within a behavior tree. Each type of node has its own unique behavior:

- **Sequence Nodes**: Execute their children sequentially, stopping at the first failure.
- **Selector Nodes**: Execute their children until one succeeds.
- **Parallel Nodes**: Execute all children in parallel and succeed based on the configured mode (e.g., AnySuccess).

By combining these nodes, you can build complex behavior trees for AI systems in games, robotics, and more.


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
