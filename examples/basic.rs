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
