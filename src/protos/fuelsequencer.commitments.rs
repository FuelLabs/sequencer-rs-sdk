// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BridgeCommitmentLeaf {
    /// height is the block from which the last results hash was derived.
    #[prost(uint64, tag="1")]
    pub height: u64,
    /// The ResultsHash of blocks is derived at (Height + 1) in the LastResultsHash
    /// variable in the Tendermint block header, ref:
    /// <https://github.com/cometbft/cometbft/blob/v0.38.5/proto/tendermint/types/types.proto#L66.>
    /// Thus, to reconstruct this root at Height X, you would need the transactions
    /// results from Height X - 1.
    #[prost(bytes="bytes", tag="2")]
    pub last_results_hash: ::prost::bytes::Bytes,
}
/// BinaryMerkleProof is a merkle proof with hex byte fields, since we want this
/// to be encoded in hex in results.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryMerkleProof {
    /// total is the total number of items.
    #[prost(int64, tag="1")]
    pub total: i64,
    /// index is the index of the item to prove.
    #[prost(int64, tag="2")]
    pub index: i64,
    /// leaf_hash is the hash of the item value.
    #[prost(bytes="bytes", tag="3")]
    pub leaf_hash: ::prost::bytes::Bytes,
    /// aunts are the hashes from the leaf's sibling to a root's child.
    #[prost(bytes="bytes", repeated, tag="4")]
    pub aunts: ::prost::alloc::vec::Vec<::prost::bytes::Bytes>,
}
// @@protoc_insertion_point(module)
