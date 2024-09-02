## 👋 Overview <a name="overview"></a>

`hfsm` is a Rust library that provides tools for managing Hierarchical Finite State Machines (HFSM). This library allows you to build and run `SuperStates` and `States` using custom contexts that implement the `StateContext` trait.

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
use hfsm::{InMemoryStateContext, StateContext as _, StateData, SuperStateBuilder, Transition, HFSM};

fn main() {
    // Create a custom context
    let mut context = InMemoryStateContext::new();

    // Build a SuperState
    let flow_a = SuperStateBuilder::new()
        .id("FlowA")
        .initial_state("Task1")
        .add_state("Task1",
            Box::new(|ctx: &mut InMemoryStateContext| {
                println!("Executing Task1 in FlowA");
                ctx.set("data", StateData::Text("200".to_string()));
                Transition::ToState("Task2".to_string())
            }),
        )
        .add_state("Task2",
            Box::new(|ctx: &mut InMemoryStateContext| {
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

`Transition`s define how the HFSM (Hierarchical Finite State Machine) navigates between states and superstates. There are three main types of transitions:

- **`ToState(next_state: String)`**:
  - This transition moves the HFSM to a specific `State` within the current `SuperState`. The state identified by `next_state` will be executed next, and the HFSM will remain within the current `SuperState`.

- **`ToSuperState { call_super_state: String, call_state: Option<String>, next_state: Option<String> }`**:
  - This transition moves the HFSM from the current `SuperState` to another `SuperState` specified by `call_super_state`.
  - If `call_state` is provided, the transition will begin with that specific `State` within the new `SuperState`. Otherwise, it will start from the initial state of the `SuperState`.
  - If `next_state` is provided, the HFSM will register a callback. Once the `call_super_state` has completed all its states, the HFSM will return to the original `SuperState` and continue execution from `next_state`.


- **`Complete`**:
  - This transition signifies the completion of the current `SuperState`.
  - If a callback has been registered (from a previous `ToSuperState` transition), the HFSM will return to the `SuperState` and `State` specified by the callback and continue execution from there.
  - If no callback is registered, the HFSM will terminate, as there are no more states to execute.


### Context

The `Context` is a fundamental component in the HFSM (Hierarchical Finite State Machine) framework. It acts as a shared repository for data that needs to be accessed and manipulated by different states during the execution of state transitions. 

#### Implementations:

The `StateContext` trait defines the interface that any context implementation must provide. It includes methods for setting, getting, and removing key-value pairs within the context.

Included implementations:

1. **`InMemoryStateContext`:** 
   - This is an in-memory implementation of the `StateContext`, optimized for scenarios where the context's lifecycle is tied to the runtime of the state machine. 
   - It uses a `HashMap` under the hood to store key-value pairs and is capable of handling various types of data through the use of `StateData`. 
   - `InMemoryStateContext` is suitable for most applications where the context does not need to persist beyond the state machine's execution.
