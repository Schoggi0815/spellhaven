use crate::world_generation::chunk_loading::chunk_node_children::ChunkNodeChildren;

#[derive(Clone)]
pub enum NodeState {
    Branch {
        children: ChunkNodeChildren,
    },
    Leaf {
        spawned_task: bool,
    },
    LeafToBranch {
        children: ChunkNodeChildren,
        top_left_done: bool,
        top_right_done: bool,
        bottom_left_done: bool,
        bottom_right_done: bool,
    },
    BranchToLeaf {
        spawned_task: bool,
        children: ChunkNodeChildren,
    },
}

impl NodeState {
    pub fn is_leaf(&self) -> bool {
        match self {
            Self::Leaf { .. } => true,
            _ => false,
        }
    }

    pub fn is_branch(&self) -> bool {
        match self {
            Self::Branch { .. } => true,
            _ => false,
        }
    }
}
