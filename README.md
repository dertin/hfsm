## 👋 Overview <a name="overview"></a>

`hfsm` is a Rust library that provides tools for managing Hierarchical Finite State Machines (HFSM). This library allows you to build and run `SuperStates` and `States` using custom contexts that implement the `HFSMContext` trait.

## Key Features

- Creation and management of hierarchical states (`SuperStates`)
- Definition of individual states (`States`)
- Customizable contexts for handling data between states
- Flexible transitions between states
- Intuitive API for building and running HFSMs

## Quick Start

To start using the `hfsm` library, follow these steps:

1. Create an `HFSM` instance with an initial `SuperState`.
2. Add more `SuperStates` and `States` to the `HFSM` as needed.
3. Run the HFSM with a custom context.

## Basic Example

```rust
use hfsm::{HFSMContext, HashMapContext, SuperStateBuilder, Transition, HFSM};

fn main() {
    // Create a custom context
    let mut context = HashMapContext::new();

    // Build a SuperState
    let flow_a = SuperStateBuilder::new()
        .id("FlowA")
        .initial_state("Task1")
        .add_state("Task1",
            Box::new(|ctx: &mut HashMapContext| {
                println!("Executing Task1 in FlowA");
                ctx.set("data", "200".to_string());
                Transition::ToState("Task2".to_string())
            }),
        )
        .add_state("Task2",
            Box::new(|ctx: &mut HashMapContext| {
                println!("Executing Task2 in FlowA");
                Transition::Complete
            }),
        )
        .build()
        .expect("Failed to build FlowA");

    // Create and run the HFSM
    let mut hfsm = HFSM::new(flow_a);
    hfsm.run(&mut context);
}
```
For more complex use cases, consider:

- Nesting `SuperStates` to create deeper hierarchies
- Implementing custom contexts to handle application-specific data
- Using conditional transitions based on context state

## Key Concepts

### SuperState

A `SuperState` is a container for multiple related states.

### State

A `State` represents an individual unit of behavior in the HFSM. Each state can perform actions and determine the next transition.

### Transition

`Transition`s define how the HFSM moves between states. They can be:
- `ToState`: Move to a specific `State` within the current `SuperState`
- `ToSuperState`: Move to a specific `State` within a different `SuperState`
- `Complete`: Finish execution of the current `SuperState`

### Context

The `Context` allows sharing and manipulating data between states. It implements the `HFSMContext` trait to provide custom functionality.
