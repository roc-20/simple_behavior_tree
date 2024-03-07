use simple_behavior_tree::{BTStatus, ParallelMode, BTNode};

fn main() {
    let action_success = || {
        println!("Action success executed");
        BTStatus::Success
    };

    let action_failure = || {
        println!("Action failure executed");
        BTStatus::Failure
    };

    // 测试 AnySuccess 模式
    let mut bt_any_success = BTNode::Parallel(
        ParallelMode::AnySuccess,
        vec![
            BTNode::Action(action_success),
            BTNode::Action(action_failure),
        ]
    );

    println!("Testing AnySuccess Mode:");
    match bt_any_success.tick() {
        BTStatus::Success => println!("BT AnySuccess succeeded"),
        BTStatus::Failure => println!("BT AnySuccess failed"),
        BTStatus::Running => println!("BT AnySuccess is still running"),
    }

    // 测试 AllSuccess 模式
    let mut bt_all_success = BTNode::Parallel(
        ParallelMode::AllSuccess,
        vec![
            BTNode::Action(action_success),
            BTNode::Action(action_failure),
        ]
    );

    println!("\nTesting AllSuccess Mode:");
    match bt_all_success.tick() {
        BTStatus::Success => println!("BT AllSuccess succeeded"),
        BTStatus::Failure => println!("BT AllSuccess failed"),
        BTStatus::Running => println!("BT AllSuccess is still running"),
    }
}
