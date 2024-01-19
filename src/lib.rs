pub enum BTNode {
    Sequence(Vec<BTNode>), 
    Selector(Vec<BTNode>),

    Parallel(ParallelMode, Vec<BTNode>),
    Action(fn() -> BTStatus),
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

            // BTNode::Parallel(mode, _) => self.tick_parallel(mode),
            BTNode::Parallel(mode, _) => {
                // let mode_copy = mode.clone();
                // self.tick_parallel(&mode_copy);

                match mode {
                    ParallelMode::AllSuccess => self.tick_parallel(&ParallelMode::AllSuccess),
                    ParallelMode::AnySuccess => self.tick_parallel(&ParallelMode::AnySuccess),
                }
            },
        }
    }


    fn tick_parallel(&mut self, mode: &ParallelMode) -> BTStatus {
        if let BTNode::Parallel(_, nodes) = self {
            let mut all_success = true;
            for node in nodes {
                match node.tick() {
                    // 并行模式:全部成功模式, 有一个失败则失败
                    BTStatus::Failure if *mode == ParallelMode::AllSuccess => return BTStatus::Failure,
                    // 并行模式:任意成功模式, 有一个成功则成功
                    BTStatus::Success if *mode == ParallelMode::AnySuccess => return BTStatus::Success,
                    
                    // 有一个失败，标记全部成功为 false 
                    BTStatus::Failure => all_success = false,
                    _ => {}
                }
            }

            match all_success {
                true => BTStatus::Success,
                false => BTStatus::Failure,
            }
        } else {
            BTStatus::Failure  // 非并行节点类型，异常情况 
        }
    }

}

