use crate::test_utils::TestRandom;
use crate::*;

use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use ssz_types::FixedVector;
use test_random_derive::TestRandom;
use tree_hash_derive::TreeHash;

/// Historical block and state roots.
///
/// Spec v0.12.1
#[derive(
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    TreeHash,
    TestRandom,
    arbitrary::Arbitrary,
)]
#[arbitrary(bound = "T: EthSpec")]
pub struct HistoricalBatch<T: EthSpec> {
    pub block_roots: FixedVector<Hash256, T::SlotsPerHistoricalRoot>,
    pub state_roots: FixedVector<Hash256, T::SlotsPerHistoricalRoot>,
}

#[cfg(test)]
mod tests {
    use super::*;

    pub type FoundationHistoricalBatch = HistoricalBatch<MainnetEthSpec>;

    ssz_and_tree_hash_tests!(FoundationHistoricalBatch);
}
