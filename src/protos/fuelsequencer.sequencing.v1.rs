// @generated
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<super::Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetTopicRequest {
    #[prost(bytes="bytes", tag="1")]
    pub id: ::prost::bytes::Bytes,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetTopicResponse {
    #[prost(message, optional, tag="1")]
    pub topic: ::core::option::Option<super::Topic>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllTopicRequest {
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllTopicResponse {
    #[prost(message, repeated, tag="1")]
    pub topic: ::prost::alloc::vec::Vec<super::Topic>,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless
    /// overwritten).
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    // params defines the module parameters to update.

    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<super::Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPostBlob {
    /// from is the address on FuelSequencer that is posting the blob.
    #[prost(string, tag="1")]
    pub from: ::prost::alloc::string::String,
    /// topic is the Topic that this blob belongs to, it is a 32-byte hash.
    #[prost(bytes="bytes", tag="2")]
    pub topic: ::prost::bytes::Bytes,
    /// order is the blob sequencer order.
    #[prost(string, tag="3")]
    pub order: ::prost::alloc::string::String,
    /// data is the blob data bytes.
    #[prost(bytes="bytes", tag="4")]
    pub data: ::prost::bytes::Bytes,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPostBlobResponse {
    /// nonce uniquely identifies any message that we send to Ethereum.
    #[prost(string, tag="1")]
    pub nonce: ::prost::alloc::string::String,
    /// from is the address on FuelSequencer that is posting the blob.
    /// This address is lowercase, regardless of the one in MsgPostBlob.
    #[prost(string, tag="2")]
    pub from: ::prost::alloc::string::String,
    /// topic is the Topic that this blob belongs to, it is a 32-byte hash.
    #[prost(bytes="bytes", tag="3")]
    pub topic: ::prost::bytes::Bytes,
    /// order is the blob sequencer order.
    #[prost(string, tag="4")]
    pub order: ::prost::alloc::string::String,
    /// data is the blob data bytes.
    #[prost(bytes="bytes", tag="5")]
    pub data: ::prost::bytes::Bytes,
}
include!("fuelsequencer.sequencing.v1.tonic.rs");
// @@protoc_insertion_point(module)