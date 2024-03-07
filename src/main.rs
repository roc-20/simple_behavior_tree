use simple_behavior_tree::{BTNode, BTStatus};

// Condition function
fn is_player_in_range() -> BTStatus {
    // Here, you would have logic to determine if the player is in range
    println!("Checking if player is in range...");
    // This is a placeholder result
    BTStatus::Success // or BTStatus::Failure based on your condition
}

// Action function
fn attack_player() -> BTStatus {
    println!("Attacking player!");
    BTStatus::Success // or BTStatus::Failure based on the action outcome
}

fn main() {
    // Create a sequence node with a condition and an action
    let mut bt = BTNode::Sequence(vec![
        BTNode::Condition(is_player_in_range),
        BTNode::Action(attack_player),
    ]);

    // Execute the behavior tree
    println!("Executing Behavior Tree:");
    match bt.tick() {
        BTStatus::Success => println!("Behavior Tree succeeded."),
        BTStatus::Failure => println!("Behavior Tree failed."),
        BTStatus::Running => println!("Behavior Tree is still running."),
    }
}