use simple_behavior_tree::{BTStatus, ParallelMode, BTNode};

fn main() {
    let action1 = || {
        println!("Action 1 executed");
        BTStatus::Success
    };

    let action2 = || {
        println!("Action 2 executed");
        BTStatus::Failure
    };


    let mut bt = BTNode::Parallel(
        ParallelMode::AnySuccess,
        vec![
            BTNode::Action(action1),
            BTNode::Action(action2),
        ] 
    );

    match bt.tick() {
        BTStatus::Success => println!("BT successed"),
        BTStatus::Failure => println!("BT failed"),
        BTStatus::Running => println!("BT is still runing"),
    }
}
