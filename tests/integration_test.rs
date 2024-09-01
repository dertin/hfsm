#[cfg(test)]
mod tests {
    use hfsm::{HFSMContext, HashMapContext, SuperStateBuilder, Transition, HFSM};

    #[test]
    fn test_hfsm() {
        let mut context = HashMapContext::new();
        println!("Initiaizing context: {:?}", context);

        let flow_a = SuperStateBuilder::new()
            .id("Flow_A")
            .initial_state("Task_A1")
            .add_state(
                "Task_A1",
                Box::new(|ctx: &mut HashMapContext| {
                    println!("Executing Task A1 in Flow A");
                    ctx.set("data", "200".to_string());
                    println!("  Set data: {:?}", ctx.get("data"));
                    Transition::ToState("Task_A2".to_string())
                }),
            )
            .add_state(
                "Task_A2",
                Box::new(|ctx: &mut HashMapContext| {
                    println!("Executing Task A2 in Flow A");
                    if ctx.get("data").unwrap() == "200" {
                        println!("  Transitioning to Flow B");
                        Transition::ToSuperState{
                            call_super_state: "Flow_B".to_string(),
                            call_state: None,
                            next_state: Some("Task_A3".to_string()), // TODO: Contiune with X state after Flow B is completed
                        }
                    } else {
                        Transition::Complete
                    }
                }),
            ).add_state(
                "Task_A3",
                Box::new(|_ctx: &mut HashMapContext| {
                    unreachable!();
                    // println!("Executing Task A3 in Flow A");
                    // Transition::Complete

                }),
            )
            .build()
            .expect("Failed to build Flow A");

        let flow_b = SuperStateBuilder::new()
            .id("Flow_B")
            .initial_state("Task_B1")
            .add_state(
                "Task_B1",
                Box::new(|ctx: &mut HashMapContext| {
                    println!("Executing Task B1 in Flow B");
                    ctx.set("data", "300".to_string());
                    println!("  Set data: {:?}", ctx.get("data"));
                    Transition::ToState("Task_B2".to_string())
                }),
            ).add_state(
                "Task_B2",
                Box::new(|_ctx: &mut HashMapContext| {
                    println!("Executing Task B2 in Flow B");
                    Transition::Complete
                }),
            )
            .build()
            .expect("Failed to build Flow B");

        let mut hfsm = HFSM::new(flow_a);
        hfsm.add_super_state(flow_b);

        hfsm.run(&mut context);

        println!("Final data: {:?}", context.get("data"));
    }
}
