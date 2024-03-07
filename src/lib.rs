pub enum BTNode {
    Sequence(Vec<BTNode>), 
    Selector(Vec<BTNode>),

    Parallel(ParallelMode, Vec<BTNode>),
    Action(fn() -> BTStatus),
    Condition(fn() -> BTStatus),
}


#[derive(Clone, PartialEq)]
pub enum ParallelMode {
    AllSuccess,
    AnySuccess,
}


pub enum BTStatus {
    Success, 
    Failure, 
    Running, 
}

impl BTNode {
    /// Sequence 节点：
    /// tick 所有子节点 
    /// Failure => Failure
    /// Running => Running
    /// Success => continue run the next node
    /// 
    /// selector 节点
    /// tick 所有节点
    /// Success => Success
    /// Running => Running
    /// Failure => continue run the next node
    pub fn tick(&mut self) -> BTStatus {
        // 判断节点类型
        match self { 
            BTNode::Sequence(nodes) => {
                // 遍历子节点
                for node in nodes {
                    
                    // tick 所有的子节点
                    match node.tick() {
                        // 有一个节点失败则失败
                        BTStatus::Failure => return BTStatus::Failure,
                        BTStatus::Running => return BTStatus::Running,
                        BTStatus::Success => continue,
                    }
                }
                BTStatus::Success
            }

            BTNode::Selector(nodes) => {
                for node in nodes {
                    match node.tick() {
                        BTStatus::Success => return BTStatus::Success,
                        BTStatus::Running => return BTStatus::Running,
                        BTStatus::Failure => continue
                    }
                }
                BTStatus::Failure
            }

            BTNode::Action(action) => action(),

            BTNode::Condition(condition) => condition(),

            // BTNode::Parallel(mode, _) => self.tick_parallel(mode),
            BTNode::Parallel(_, _) => self.tick_parallel(),
            
        }
    }


    fn tick_parallel(&mut self) -> BTStatus {
        if let BTNode::Parallel(mode, nodes) = self {
            let mut success_count = 0;
            let mut failure_count = 0;

            for node in nodes.iter_mut() {
                match node.tick() {
                    BTStatus::Success => success_count += 1,
                    BTStatus::Failure => failure_count += 1,
                    BTStatus::Running => return BTStatus::Running,
                }
            }

            match mode {
                ParallelMode::AllSuccess if success_count == nodes.len() => BTStatus::Success,
                ParallelMode::AnySuccess if success_count > 0 => BTStatus::Success,
                _ if failure_count > 0 => BTStatus::Failure,
                _ => BTStatus::Running,
            }
        } else {
            // 如果不是Parallel节点类型，这是个编码逻辑错误
            panic!("tick_parallel called on a non-parallel node!");
        }
    }
    
}

