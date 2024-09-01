use crate::HFSMContext;

pub enum Transition {
    ToState(String),
    ToSuperState { call_super_state: String, call_state: Option<String>, next_state: Option<String> },
    Complete,
}

pub struct State<C: HFSMContext> {
    pub id: String,
    action: Box<dyn Fn(&mut C) -> Transition>,
}

impl<C: HFSMContext> State<C> {
    pub fn new(id: &str, action: Box<dyn Fn(&mut C) -> Transition>) -> Self {
        State { id: id.to_string(), action }
    }

    pub fn run(&self, context: &mut C) -> Transition {
        (self.action)(context)
    }
}
