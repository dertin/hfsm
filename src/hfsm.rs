use crate::context::HFSMContext;
use crate::state::Transition;
use crate::super_state::SuperState;
use std::collections::{HashMap, VecDeque};

pub struct HFSM<C: HFSMContext> {
    super_states: HashMap<String, SuperState<C>>,
    current_super_state: String,
    current_state: String,
    callbacks: VecDeque<(String, String)>,
}

impl<C: HFSMContext> HFSM<C> {
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

    pub fn run(&mut self, context: &mut C) {
        loop {
            let super_state = self.super_states.get(&self.current_super_state).unwrap();
            let state = super_state.get_state(&self.current_state).unwrap();

            let transition = state.run(context);

            match transition {
                Transition::ToState(next_state) => {
                    self.current_state = next_state;
                }
                Transition::Complete => {
                    if let Some((super_state_id, state_id)) = self.callbacks.pop_back() {
                        self.current_super_state = super_state_id;
                        self.current_state = state_id;
                    } else {
                        break;
                    }
                }
                Transition::ToSuperState { call_super_state, call_state, next_state: _ } => {
                    self.callbacks.push_back((self.current_super_state.clone(), self.current_state.clone()));
                    self.current_super_state = call_super_state;

                    if let Some(state_id) = call_state {
                        self.current_state = state_id;
                    } else {
                        self.current_state = self.super_states.get(&self.current_super_state).unwrap().initial_state().clone();
                    }
                },
            }
        }
    }
}
