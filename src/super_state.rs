use std::collections::HashMap;

use crate::{HFSMContext, State, Transition};

pub struct SuperState<C: HFSMContext> {
    pub id: String,
    pub states: HashMap<String, State<C>>,
    pub initial_state: String,
}

impl<C: HFSMContext> SuperState<C> {
    pub fn new(id: &str, initial_state: &str) -> Self {
        SuperState {
            id: id.to_string(),
            states: HashMap::new(),
            initial_state: initial_state.to_string(),
        }
    }

    pub fn add_state(&mut self, state: State<C>) {
        self.states.insert(state.id.clone(), state);
    }

    pub fn get_state(&self, state_id: &str) -> Option<&State<C>> {
        self.states.get(state_id)
    }

    pub fn initial_state(&self) -> &String {
        &self.initial_state
    }
}

pub struct SuperStateBuilder<C: HFSMContext> {
    id: Option<String>,
    initial_state: Option<String>,
    states: HashMap<String, State<C>>,
    _marker: std::marker::PhantomData<C>,
}

impl<C: HFSMContext> Default for SuperStateBuilder<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: HFSMContext> SuperStateBuilder<C> {

    pub fn new() -> Self {
        SuperStateBuilder {
            id: None,
            initial_state: None,
            states: HashMap::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn initial_state(mut self, initial_state: &str) -> Self {
        self.initial_state = Some(initial_state.to_string());
        self
    }

    pub fn add_state<F>(mut self, state_id: &str, action: F) -> Self
    where
        F: Fn(&mut C) -> Transition + 'static,
    {
        let state = State::new(state_id, Box::new(action));
        self.states.insert(state_id.to_string(), state);
        self
    }

    pub fn build(self) -> Result<SuperState<C>, &'static str> {
        let id = self.id.ok_or("SuperState ID must be set.")?;
        let initial_state = self
            .initial_state
            .ok_or("SuperState initial state must be set.")?;
        Ok(SuperState {
            id,
            states: self.states,
            initial_state,
        })
    }
}
