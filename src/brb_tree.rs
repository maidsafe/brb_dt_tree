// Copyright 2021 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

use crdt_tree::{Clock, OpMove, State, TreeId, TreeMeta};

use brb::BRBDataType;

use serde::Serialize;
use std::{fmt::Debug, hash::Hash};
use thiserror::Error;

/// A BRBDataType wrapper around crdt_tree::State
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

/// An enumeration of possible Validation Errors
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ValidationError {
    #[error("The source actor does not match the actor associated with the operation")]
    SourceDoesNotMatchOp,
}

impl<
        A: Hash + Ord + Clone + Debug + Serialize + 'static,
        ID: TreeId + Debug + Serialize,
        M: TreeMeta + Eq + Debug + Hash + Serialize,
    > BRBDataType<A> for BRBTree<A, ID, M>
{
    type Op = OpMove<ID, M, A>;
    type ValidationError = ValidationError;

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
            Err(ValidationError::SourceDoesNotMatchOp)
        } else {
            Ok(())
        }
    }

    /// Apply an operation to the underlying Tree datatype
    fn apply(&mut self, op: Self::Op) {
        self.treestate.apply_op(op);
    }
}
