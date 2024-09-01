fn main() {
    use hfsm::{HFSMContext, HashMapContext, SuperStateBuilder, Transition, HFSM};

    let mut context = HashMapContext::new();

    let flow_a = SuperStateBuilder::new()
        .id("FlowA")
        .initial_state("Task1")
        .add_state(
            "Task1",
            Box::new(|ctx: &mut HashMapContext| {
                println!("Executing Task1 in FlowA");
                ctx.set("data", "200".to_string());
                Transition::ToState("Task2".to_string())
            }),
        )
        .add_state(
            "Task2",
            Box::new(|_ctx: &mut HashMapContext| {
                println!("Executing Task2 in FlowA");
                Transition::Complete

            }),
        )
        .build()
        .expect("Failed to build FlowA");

    let mut hfsm = HFSM::new(flow_a);

    hfsm.run(&mut context);
}
