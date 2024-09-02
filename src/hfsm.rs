use crate::context::StateContext;
use crate::state::Transition;
use crate::super_state::SuperState;
use std::collections::{HashMap, VecDeque};

struct ReturnCallback {
    status: bool,
    return_to_super_state: String,
    return_to_state: String,
}

pub struct HFSM<C: StateContext> {
    super_states: HashMap<String, SuperState<C>>,
    current_super_state: String,
    current_state: String,
    callbacks: VecDeque<ReturnCallback>,
}

impl<C: StateContext> HFSM<C> {
    pub fn new(initial_super_state: SuperState<C>) -> Self {
        let initial_state = initial_super_state.initial_state().clone();
        let initial_super_state_id = initial_super_state.id.clone();

        let mut hfsm = HFSM {
            super_states: HashMap::new(),
            current_super_state: initial_super_state_id.clone(),
            current_state: initial_state.clone(),
            callbacks: VecDeque::new(),
        };
        hfsm.add_super_state(initial_super_state);
        hfsm
    }

    pub fn add_super_state(&mut self, super_state: SuperState<C>) {
        self.super_states.insert(super_state.id.clone(), super_state);
    }

    fn register_callback(&mut self, return_to_super_state: String, return_to_state: String) {
        self.callbacks.push_back(ReturnCallback { status: false, return_to_super_state, return_to_state });
    }

    fn execute_callback(&mut self) -> Option<(String, String)> {
        if let Some(mut last_callback) = self.callbacks.pop_back() {
            last_callback.status = true;
            return Some((last_callback.return_to_super_state.clone(), last_callback.return_to_state.clone()));
        }
        None
    }

    pub fn run(&mut self, context: &mut C) {
        loop {
            let super_state = self.super_states.get(&self.current_super_state).unwrap();
            let state = super_state.get_state(&self.current_state).unwrap();

            let transition = state.run(context);

            match transition {
                Transition::ToState(next_state) => {
                    self.current_state = next_state;
                }
                Transition::ToSuperState { call_super_state, call_state, next_state } => {
                    if let Some(next) = next_state {
                        self.register_callback(self.current_super_state.clone(), next);
                    }

                    self.current_super_state = call_super_state;
                    self.current_state = call_state.unwrap_or_else(|| {
                        self.super_states.get(&self.current_super_state).unwrap().initial_state().clone()
                    });
                }
                Transition::Complete => {
                    if let Some((super_state_id, state_id)) = self.execute_callback() {
                        self.current_super_state = super_state_id;
                        self.current_state = state_id;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}
