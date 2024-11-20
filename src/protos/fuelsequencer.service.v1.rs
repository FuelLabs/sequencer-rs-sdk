// @generated
/// DepositEvent represents a deposit event raised by the proxy contract.
/// This message represents the event structure on the Sequencer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositEvent {
    /// the sending Ethereum address in hex format
    #[prost(string, tag = "1")]
    pub depositor: ::prost::alloc::string::String,
    /// recipient address in hex or bech32 format. If the recipient is the null
    /// address, the Sequencer uses the depositor address as the recipient.
    #[prost(string, tag = "2")]
    pub recipient: ::prost::alloc::string::String,
    /// the amount sent encoded as string to prevent loss of precision. Sign is
    /// also preserved
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
    /// vesting duration encoded in string to prevent loss of precision. Sign is
    /// also preserved. This can be zero if no duration is specified.
    #[prost(string, tag = "4")]
    pub lockup: ::prost::alloc::string::String,
}
/// AuthorizeEvent represents an authorize event raised by the proxy contract.
/// This message represents the event structure on the Sequencer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeEvent {
    /// the Ethereum address granting authorization in hex format
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// message to be executed
    #[prost(bytes = "bytes", tag = "2")]
    pub data: ::prost::bytes::Bytes,
}
/// QueryBlockEventsRequest defines the request type for the the GetBlockEvents
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockEventsRequest {
    #[prost(string, tag = "1")]
    pub block_number: ::prost::alloc::string::String,
}
/// QueryBlockEventsResponse defines the response type for the GetBlockEvents
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockEventsResponse {
    /// events defines the list of events.
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
/// Event stores the event data, type and contract address queried from Ethereum.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(string, tag = "1")]
    pub event_type: ::prost::alloc::string::String,
    #[prost(bytes = "bytes", tag = "2")]
    pub data: ::prost::bytes::Bytes,
    #[prost(string, tag = "3")]
    pub contract_address: ::prost::alloc::string::String,
}
include!("fuelsequencer.service.v1.tonic.rs");
// @@protoc_insertion_point(module)
