// Copyright 2021 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

//! BRBTree is a BRBDataType wrapper around a Tree CRDT. (crdt_tree)
//!
//! This wrapper enables Tree operations to be transmitted in a BFT manner
//! using Byzantine Reliable Broadcast.

#![deny(missing_docs)]

mod brb_tree;
pub use brb_tree::BRBTree;
