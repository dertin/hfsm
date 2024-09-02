#[cfg(test)]
mod tests {
    use hfsm::{InMemoryStateContext, StateContext as _, StateData, SuperStateBuilder, Transition, HFSM};

    #[test]
    fn test_hfsm_with_counter() {
        let max_counter: i32 = 1_000_000;

        let mut context: InMemoryStateContext = InMemoryStateContext::new();

        context.set("counter", StateData::Integer(0));

        let flow_a = SuperStateBuilder::new()
            .id("FlowA")
            .initial_state("TaskA1")
            .add_state("TaskA1", |ctx: &mut InMemoryStateContext| {
                let counter = ctx.get("counter").and_then(|v| v.as_i32()).unwrap();
                println!("Executing TaskA1 in FlowA, counter: {}", counter);
                Transition::ToSuperState {
                    call_super_state: "FlowB".to_string(),
                    call_state: None,
                    next_state: Some("TaskA2".to_string()),
                }
            })
            .add_state("TaskA2", |ctx: &mut InMemoryStateContext| {
                let counter = ctx.get("counter").and_then(|v| v.as_i32()).unwrap();
                ctx.set("counter", StateData::Integer(counter));
                println!("Executing TaskA2 in FlowA, incrementing counter to: {}", counter);
                Transition::Complete
            })
            .build()
            .expect("Failed to build FlowA");

        let flow_b = SuperStateBuilder::new()
            .id("FlowB")
            .initial_state("TaskB1")
            .add_state("TaskB1", |ctx: &mut InMemoryStateContext| {
                let counter = ctx.get("counter").and_then(|v| v.as_i32()).unwrap();
                println!("Executing TaskB1 in FlowB, counter: {}", counter);
                Transition::ToState("TaskB2".to_string())
            })
            .add_state("TaskB2", move |ctx: &mut InMemoryStateContext| {
                let mut counter = ctx.get("counter").and_then(|v| v.as_i32()).unwrap();
                counter += 1;
                ctx.set("counter", StateData::Integer(counter));
                println!("Executing TaskB2 in FlowB, incrementing counter to: {}", counter);

                if counter < max_counter {
                    Transition::ToSuperState {
                        call_super_state: "FlowA".to_string(),
                        call_state: None,
                        next_state: None,
                    }
                } else {
                    Transition::Complete
                }
            })
            .build()
            .expect("Failed to build FlowB");

        let mut hfsm = HFSM::new(flow_a);
        hfsm.add_super_state(flow_b);

        // Ejecutar la HFSM
        hfsm.run(&mut context);

        // Comprobación final
        let final_counter = context.get("counter").and_then(|v| v.as_i32()).unwrap();
        println!("Final counter value: {}", final_counter);
        assert_eq!(final_counter, max_counter);
    }
}
