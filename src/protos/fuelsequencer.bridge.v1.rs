// @generated
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<super::Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetSupplyDeltaInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetSupplyDeltaInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub supply_delta_info: ::core::option::Option<super::SupplyDeltaInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetLastEthereumNonceRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetLastEthereumNonceResponse {
    #[prost(string, tag = "1")]
    pub nonce: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetLastEthereumBlockSyncedRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetLastEthereumBlockSyncedResponse {
    #[prost(uint64, tag = "1")]
    pub block: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetEthereumEventIndexOffsetRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetEthereumEventIndexOffsetResponse {
    #[prost(uint64, tag = "1")]
    pub offset: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySequencerAddressFromEthereumAddressRequest {
    #[prost(string, tag = "1")]
    pub ethereum_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySequencerAddressFromEthereumAddressResponse {
    #[prost(string, tag = "1")]
    pub sequencer_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEthereumAddressFromSequencerAddressRequest {
    #[prost(string, tag = "1")]
    pub sequencer_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEthereumAddressFromSequencerAddressResponse {
    #[prost(string, tag = "1")]
    pub ethereum_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetLastEthBlockUpdateTimeRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetLastEthBlockUpdateTimeResponse {
    #[prost(message, optional, tag = "1")]
    pub last_eth_block_update_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetLastConsensusTxsSequenceRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetLastConsensusTxsSequenceResponse {
    #[prost(uint64, tag = "1")]
    pub sequence: u64,
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless
    /// overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    // params defines the module parameters to update.
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<super::Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSupplyDelta {
    /// authority ensures that users cannot execute this message.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSupplyDeltaResponse {
    /// nonce uniquely identifies any message that we send to Ethereum.
    #[prost(string, tag = "1")]
    pub nonce: ::prost::alloc::string::String,
    /// supply_delta reports the change in the bridge token's supply due to mints
    /// and burns.
    #[prost(string, tag = "2")]
    pub supply_delta: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawToEthereum {
    /// from is the user address that is withdrawing the tokens from the Sequencer.
    /// It can be in Hex or Bech32 format.
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    /// to is the user address on Ethereum that will be receiving the tokens.
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
    /// amount is the tokens being sent, which must be in the expected bridge
    /// token.
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawToEthereumResponse {
    /// nonce uniquely identifies any message that we send to Ethereum.
    #[prost(string, tag = "1")]
    pub nonce: ::prost::alloc::string::String,
    /// from is the user address that is withdrawing the tokens from the Sequencer.
    /// It can be in Hex or Bech32 format, as supplied in MsgWithdrawToEthereum.
    /// This address is lowercase, regardless of the one in MsgWithdrawToEthereum.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// to is the user address on Ethereum that will be receiving the tokens.
    /// This address is lowercase, regardless of the one in MsgWithdrawToEthereum.
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    /// amount is the tokens being sent, which must be the expected bridge token.
    #[prost(message, optional, tag = "4")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositFromEthereum {
    /// authority ensures that users cannot execute this message.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// the sending Ethereum address in hex format
    #[prost(string, tag = "2")]
    pub depositor: ::prost::alloc::string::String,
    /// recipient address in hex or bech32 format. If the recipient is the null
    /// address, the Sequencer uses the depositor address as the recipient.
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    /// the amount sent encoded as string to prevent loss of precision. Sign is
    /// also preserved
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
    /// vesting duration encoded in string to prevent loss of precision. Sign is
    /// also preserved. This can be zero if no duration is specified.
    #[prost(string, tag = "5")]
    pub lockup: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositFromEthereumResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIndex {
    /// authority ensures that users cannot execute this message.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// num_injected_event_txs is the number of Ethereum events injected as
    /// txs in the current block, excluding MsgSupplyDelta and MsgIndex.
    #[prost(uint64, tag = "2")]
    pub num_injected_event_txs: u64,
    /// new_ethereum_block is a boolean which indicates whether a new Ethereum
    /// block has been queried from the Sidecar and that the events from it were
    /// fully consumed by the Sequencer. This is needed to determine when
    /// LastEthereumBlockSynced should be incremented by MsgIndex. If
    /// false but the events list is not empty, the block was partially consumed.
    #[prost(bool, tag = "3")]
    pub new_ethereum_block: bool,
    /// block_number is the block that these events belong to. This is expected to
    /// be LastEthereumBlockSynced+1, since the events are from the next block.
    #[prost(uint64, tag = "4")]
    pub block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIndexResponse {}
include!("fuelsequencer.bridge.v1.tonic.rs");
// @@protoc_insertion_point(module)
