// #![warn(missing_docs)]

//! Hierarchical Finite State Machine (HFSM) library.
//! This library provides tools for managing Hierarchical Finite State Machines (HFSM).
//!
//! It allows you to build and run `SuperStates` and `States` using custom contexts that implement the `HFSMContext` trait.
//!
//! To get started, you can create a new `HFSM` instance with an initial `SuperState`.
//! Then, you can add more `SuperStates` and `States` to the `HFSM` as needed.
//!
//! Here's a basic example of how to use this library:
//!
//! ```rust
//! use hfsm::{HFSMContext, HashMapContext, SuperStateBuilder, Transition, HFSM};
//!
//! let mut context = HashMapContext::new();
//!
//! let flow_a = SuperStateBuilder::new()
//!     .id("FlowA")
//!     .initial_state("Task1")
//!     .add_state("Task1",
//!       Box::new(|ctx: &mut HashMapContext| {
//!         println!("Executing Task1 in FlowA");
//!         ctx.set("data", "200".to_string());
//!         Transition::ToState("Task2".to_string())
//!       }),
//!     )
//!     .add_state("Task2",
//!       Box::new(|ctx: &mut HashMapContext| {
//!         println!("Executing Task2 in FlowA");
//!         Transition::Complete
//!       }),
//!     )
//!     .build()
//!     .expect("Failed to build FlowA");
//!
//! let mut hfsm = HFSM::new(flow_a);
//!
//! hfsm.run(&mut context);
//! ```
//!

mod context;
mod hfsm;
mod state;
mod super_state;

pub use context::{HFSMContext, HashMapContext};
pub use hfsm::HFSM;
pub use state::{State, Transition};
pub use super_state::{SuperState, SuperStateBuilder};
