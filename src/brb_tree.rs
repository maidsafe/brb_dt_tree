use crdt_tree::{Clock, OpMove, State, TreeId, TreeMeta};

use brb::BRBDataType;

use serde::Serialize;
use std::{fmt::Debug, hash::Hash};

/// BRBTRee is a BRBDataType wrapper around a Tree CRDT. (crdt_tree)
///
/// This enables Tree operations to be transmitted in a BFT manner
/// using Byzantine Reliable Broadcast.

#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
pub struct BRBTree<A: Clone + Hash + Ord, ID: TreeId, M: TreeMeta> {
    actor: A,
    treestate: State<ID, M, A>,
}

impl<A: Clone + Hash + Ord, ID: TreeId, M: TreeMeta> BRBTree<A, ID, M> {
    /// generates a move operation.  (crdt_tree::OpMove)
    pub fn opmove(&self, clock: Clock<A>, parent: ID, meta: M, child: ID) -> OpMove<ID, M, A> {
        OpMove::new(clock, parent, meta, child)
    }

    /// returns the actor
    pub fn actor(&self) -> &A {
        &self.actor
    }

    /// returns underlying crdt_tree::State object
    pub fn treestate(&self) -> &State<ID, M, A> {
        &self.treestate
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ValidationError<A> {
    SourceNotSameAsOperator { source: A, op_actor: A },
}

impl<
        A: Hash + Ord + Clone + Debug + Serialize + 'static,
        ID: TreeId + Debug + Serialize,
        M: TreeMeta + Eq + Debug + Hash + Serialize,
    > BRBDataType<A> for BRBTree<A, ID, M>
{
    type Op = OpMove<ID, M, A>;
    type ValidationError = ValidationError<A>;

    /// Create a new BRBTree
    fn new(actor: A) -> Self {
        BRBTree {
            actor,
            treestate: State::new(),
        }
    }

    /// Validate an operation.
    fn validate(&self, source: &A, op: &Self::Op) -> Result<(), Self::ValidationError> {
        if op.timestamp().actor_id() != source {
            Err(ValidationError::SourceNotSameAsOperator {
                source: source.clone(),
                op_actor: op.timestamp().actor_id().clone(),
            })
        } else {
            Ok(())
        }
    }

    /// Apply an operation to the underlying Tree datatype
    fn apply(&mut self, op: Self::Op) {
        self.treestate.apply_op(op);
    }
}
