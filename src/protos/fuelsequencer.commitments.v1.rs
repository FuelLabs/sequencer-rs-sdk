// @generated
/// QueryBridgeCommitmentRequest is request type for the Query/BridgeCommitment
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBridgeCommitmentRequest {
    /// start is the start of the block range that the bridge commitment covers.
    #[prost(uint64, tag = "1")]
    pub start: u64,
    /// end is the exclusive end of the block range that the bridge commitment
    /// covers.
    #[prost(uint64, tag = "2")]
    pub end: u64,
}
/// QueryBridgeCommitmentResponse contains the merkle root of successive
/// BridgeCommitmentLeaf.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBridgeCommitmentResponse {
    #[prost(bytes = "bytes", tag = "1")]
    pub bridge_commitment: ::prost::bytes::Bytes,
}
/// QueryBridgeCommitmentInclusionProofRequest is request type for the
/// Query/BridgeCommitmentInclusionProof RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBridgeCommitmentInclusionProofRequest {
    /// height is the block from which the last results hash will be obtained.
    #[prost(int64, tag = "1")]
    pub height: i64,
    /// tx_index is the index of the transaction at its original block height.
    #[prost(int64, tag = "2")]
    pub tx_index: i64,
    /// start is the start of the block range including the height.
    #[prost(uint64, tag = "3")]
    pub start: u64,
    /// end is the exclusive end of the block range including the height.
    #[prost(uint64, tag = "4")]
    pub end: u64,
}
/// QueryBridgeCommitmentInclusionProofResponse contains merkle proofs to show
/// that a transaction response was used to construct the BridgeCommitment merkle
/// root. It also includes the marshalled deterministic transaction result.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBridgeCommitmentInclusionProofResponse {
    /// bridge_commitment_leaf is the bridge commitment leaf involved in the
    /// BridgeCommitmentMerkleProof and also the one containing the LastResultsHash
    /// that is the root of the LastResultsMerkleProof.
    #[prost(message, optional, tag = "1")]
    pub bridge_commitment_leaf: ::core::option::Option<super::BridgeCommitmentLeaf>,
    /// bridge_commitment_proof is a merkle proof proving a BridgeCommitmentLeaf
    /// was used to construct the BridgeCommitment merkle root.
    #[prost(message, optional, tag = "2")]
    pub bridge_commitment_proof: ::core::option::Option<super::BinaryMerkleProof>,
    /// tx_result_marshalled is the marshalled deterministic form of the queried
    /// transaction's ExecTxResult.
    #[prost(bytes = "bytes", tag = "3")]
    pub tx_result_marshalled: ::prost::bytes::Bytes,
    /// last_results_proof is a merkle proof proving a transaction response was
    /// used to form the LastResultsHash merkle root.
    #[prost(message, optional, tag = "4")]
    pub last_results_proof: ::core::option::Option<super::BinaryMerkleProof>,
}
include!("fuelsequencer.commitments.v1.tonic.rs");
// @@protoc_insertion_point(module)
