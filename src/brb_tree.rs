use crdt_tree::{Clock, OpMove, State, TreeId, TreeMeta};

use brb::{Actor, BRBDataType};

use serde::Serialize;
use std::{fmt::Debug, hash::Hash};

/// BRBTRee is a BRBDataType wrapper around a Tree CRDT. (crdt_tree)
///
/// This enables Tree operations to be transmitted in a BFT manner
/// using Byzantine Reliable Broadcast.

#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
pub struct BRBTree<ID: TreeId, M: TreeMeta + Clone + Eq + Debug + Hash + Serialize> {
    actor: Actor,
    treestate: State<ID, M, Actor>,
}

impl<ID: TreeId + Debug, M: TreeMeta + Clone + Eq + Debug + Hash + Serialize> BRBTree<ID, M> {
    /// generates a move operation.  (crdt_tree::OpMove)
    pub fn opmove(
        &self,
        clock: Clock<Actor>,
        parent: ID,
        meta: M,
        child: ID,
    ) -> OpMove<ID, M, Actor> {
        OpMove::new(clock, parent, meta, child)
    }

    /// returns the actor
    pub fn actor(&self) -> &Actor {
        &self.actor
    }

    /// returns underlying crdt_tree::State object
    pub fn treestate(&self) -> &State<ID, M, Actor> {
        &self.treestate
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ValidationError {
    SourceNotSameAsOperator { source: Actor, op_actor: Actor },
}

impl<ID: TreeId + Debug + Serialize, M: TreeMeta + Clone + Eq + Debug + Hash + Serialize>
    BRBDataType for BRBTree<ID, M>
{
    type Op = OpMove<ID, M, Actor>;
    type ValidationError = ValidationError;

    /// Create a new BRBTree
    fn new(actor: Actor) -> Self {
        BRBTree {
            actor,
            treestate: State::new(),
        }
    }

    /// Validate an operation.
    fn validate(&self, source: &Actor, op: &Self::Op) -> Result<(), Self::ValidationError> {
        if op.timestamp().actor_id() != source {
            Err(ValidationError::SourceNotSameAsOperator {
                source: *source,
                op_actor: *op.timestamp().actor_id(),
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
