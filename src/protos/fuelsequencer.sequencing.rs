// @generated
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// max_blob_size_bytes is the maximum size of blob that can be submitted in
    /// bytes.
    #[prost(uint64, tag = "1")]
    pub max_blob_size_bytes: u64,
    /// sequencer_tx_max_bytes specifies the maximum size, in bytes, that
    /// Sequencer-native transactions can be. Sequencer-native transactions
    /// with size bigger than this limit will get automatically rejected.
    /// This is done to avoid having large transactions sitting in the
    /// mempool forever due to never having enough block space. This happens
    /// when size(tx) > RequestPrepareProposal.MaxBytes - size(MsgIndexTx).
    #[prost(uint64, tag = "2")]
    pub sequencer_tx_max_bytes: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Topic {
    /// id uniquely identifies the topic with a 32-byte hash.
    #[prost(bytes = "bytes", tag = "1")]
    pub id: ::prost::bytes::Bytes,
    /// owner is the sequencer address, who is the owner of this topic, assigned at
    /// creation.
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// order is the sequential order of the topic blobs.
    #[prost(string, tag = "3")]
    pub order: ::prost::alloc::string::String,
}
/// GenesisState defines the sequencing module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// topicList hold all the list of topics.
    #[prost(message, repeated, tag = "2")]
    pub topic_list: ::prost::alloc::vec::Vec<Topic>,
}
// @@protoc_insertion_point(module)
