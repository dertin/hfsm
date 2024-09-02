// #![warn(missing_docs)]

//! Hierarchical Finite State Machine (HFSM) library.
//! This library provides tools for managing Hierarchical Finite State Machines (HFSM).
//!
//! It allows you to build and run `SuperStates` and `States` using custom contexts that implement the `StateMachineContext` trait.
//!
//! To get started, you can create a new `HFSM` instance with an initial `SuperState`.
//! Then, you can add more `SuperStates` and `States` to the `HFSM` as needed.
//!
//! WARNING: This library is still in development and is not ready for production use.
//!

mod context;
mod hfsm;
mod state;
mod super_state;

pub use context::{InMemoryStateContext, StateContext, StateData};
pub use hfsm::HFSM;
pub use state::{State, Transition};
pub use super_state::{SuperState, SuperStateBuilder};
