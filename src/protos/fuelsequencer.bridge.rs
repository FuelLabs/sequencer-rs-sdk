// @generated
/// An EthOwnedBaseAccount wraps a BaseAccount that is known to be owned and
/// controlled by an Ethereum address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthOwnedBaseAccount {
    #[prost(message, optional, tag = "1")]
    pub base_account:
        ::core::option::Option<super::super::cosmos::auth::v1beta1::BaseAccount>,
    /// account_owner is the Ethereum address that owns and controls this account.
    #[prost(string, tag = "2")]
    pub account_owner: ::prost::alloc::string::String,
}
/// An EthOwnedContinuousVestingAccount wraps a ContinuousVestingAccount
/// that is known to be owned and controlled by an Ethereum address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthOwnedContinuousVestingAccount {
    #[prost(message, optional, tag = "1")]
    pub vesting_account: ::core::option::Option<
        super::super::cosmos::vesting::v1beta1::ContinuousVestingAccount,
    >,
    /// account_owner is the Ethereum address that owns and controls this account.
    #[prost(string, tag = "2")]
    pub account_owner: ::prost::alloc::string::String,
}
/// AuthorizeTx contains a list of sdk.Msg's. It should be used when sending
/// transactions from Ethereum to the Sequencer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeTx {
    #[prost(message, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSupplyDeltaReported {
    /// supply_delta is the latest supply delta reported to Ethereum.
    #[prost(string, tag = "1")]
    pub supply_delta: ::prost::alloc::string::String,
    /// nonce is the latest nonce reported to Ethereum.
    #[prost(string, tag = "2")]
    pub nonce: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventWithdrawToEthereumReported {
    /// nonce uniquely identifies any message that we send to Ethereum.
    #[prost(string, tag = "1")]
    pub nonce: ::prost::alloc::string::String,
    /// from is the user address that is withdrawing the tokens from the Sequencer.
    /// can be in Hex or Bech32 format.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// to is the user address on Ethereum that will be receiving the tokens.
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    /// amount is the tokens being sent, which must be the expected bridge token.
    #[prost(message, optional, tag = "4")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDepositEventProcessed {
    /// the sending Ethereum address in hex format
    #[prost(string, tag = "1")]
    pub depositor: ::prost::alloc::string::String,
    /// recipient Ethereum address in hex or bech32 format
    #[prost(string, tag = "2")]
    pub recipient: ::prost::alloc::string::String,
    /// amount is the tokens being sent, which must be denominated in the expected
    /// bridge token.
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// vesting duration encoded in string to prevent loss of precision. Sign is
    /// also preserved. This can be zero if no duration is specified.
    #[prost(string, tag = "4")]
    pub lockup: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDepositEventFailed {
    /// event_details is the marshalled event that failed to be processed.
    #[prost(bytes = "bytes", tag = "1")]
    pub event_details: ::prost::bytes::Bytes,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventEthereumBlockSynced {
    /// block_number is the Ethereum block that was synced partially or fully.
    #[prost(uint64, tag = "1")]
    pub block_number: u64,
    /// full_sync is true if the Ethereum block has been fully synced.
    #[prost(bool, tag = "2")]
    pub full_sync: bool,
}
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// bridge_denom is the assumed denom for the bridged token, used when minting
    /// upon deposits, burning when withdrawing, and tracking changes in its
    /// supply that will be reported to Ethereum, amongst other scenarios.
    #[prost(string, tag = "1")]
    pub bridge_denom: ::prost::alloc::string::String,
    /// bridge_denom_total_supply is the assumed total supply of the bridged token.
    /// It is intended to be used instead of the staking token supply reported by
    /// the staking module, when calculating inflation. The staking module does
    /// not know about tokens that have not been bridged to the Sequencer, so we
    /// would mint the wrong amount if we use the staking module's supply value.
    ///
    /// Unless this parameter is changed, the mint module will simply mint the
    /// same amount of tokens at every block based on the value of this param.
    #[prost(string, tag = "2")]
    pub bridge_denom_total_supply: ::prost::alloc::string::String,
    /// ethereum_proxy_contract_address is the contract address we expect to
    /// receive deposit and authorize messages from.
    #[prost(string, tag = "3")]
    pub ethereum_proxy_contract_address: ::prost::alloc::string::String,
    /// authorize_messages_allowed is a whitelist for authorize messages that we
    /// can receive and process.
    #[prost(string, repeated, tag = "4")]
    pub authorize_messages_allowed:
        ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// supply_delta_period is the frequency in block at which we report supply
    /// delta info to Ethereum.
    #[prost(uint64, tag = "5")]
    pub supply_delta_period: u64,
    /// vesting_start_time is the common vesting starting time for vesting accounts
    /// that will be created through deposits from Ethereum.
    #[prost(message, optional, tag = "6")]
    pub vesting_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// additional_blocked_addresses is a list of Cosmos SDK-based bech32 addresses
    /// that are explicitly disallowed from being controlled by authorize messages
    /// within the Sequencer system. This can include addresses of module accounts,
    /// validator operators, or any other addresses deemed necessary to protect
    /// from unauthorized control actions.
    #[prost(string, repeated, tag = "7")]
    pub additional_blocked_addresses:
        ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// max_eth_block_update_delay is the maximum amount of time that the Sequencer
    /// allows validators to not sync up with Ethereum. Once
    /// max_eth_block_update_delay is exceeded, the Sequencer's block production
    /// will halt until validators sync up with next Ethereum block.
    #[prost(message, optional, tag = "8")]
    pub max_eth_block_update_delay: ::core::option::Option<::prost_types::Duration>,
    /// injected_event_tx_max_bytes is the maximum amount of block space that an
    /// injected event tx can take in terms of bytes. An Authorize event gets
    /// skipped and never included in a block if it can't be converted into a tx
    /// that can respect this limit. On the other hand, if a Deposit event cannot
    /// be converted into a Tx that can respect this limit, the chain halts.
    #[prost(uint64, tag = "9")]
    pub injected_event_tx_max_bytes: u64,
    /// sequencer_txs_allocation is a percentage that controls the maximum amount
    /// of block space that is allocated to Sequencer-native transactions during
    /// heavy bridge usage. This ensures Sequencer blocks are not solely filled
    /// with event transactions if the Ethereum blocks being synced are large.
    /// NOTE: sequencer_txs_allocation is ignored by the consensus algorithm if it
    /// can fit more Sequencer-native or event transactions.
    #[prost(string, tag = "10")]
    pub sequencer_txs_allocation: ::prost::alloc::string::String,
    /// max_authorize_messages is the maximum amount of Cosmos SDK messages that an
    /// Authorize transaction can have
    #[prost(uint64, tag = "11")]
    pub max_authorize_messages: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SupplyDeltaInfo {
    /// last_supply is the supply of the bridge token that was recorded
    /// when the most recent report to Ethereum was constructed.
    #[prost(string, tag = "1")]
    pub last_supply: ::prost::alloc::string::String,
    /// offset is used to account for changes in supply that we do not want to
    /// report to Ethereum, including deposits and withdrawals, because Ethereum
    /// will know about these anyways. Note that offset can be positive (to account
    /// for unreported burn) or negative (to account for unreported mint).
    #[prost(string, tag = "2")]
    pub offset: ::prost::alloc::string::String,
    /// to_report is the amount of bridge tokens that should be minted (positive)
    /// or burned (negative) on Ethereum. Reset after being relayed to Ethereum.
    #[prost(string, tag = "3")]
    pub to_report: ::prost::alloc::string::String,
}
/// GenesisState defines the bridge module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// supply_delta_info is the starting point for changes in the bridge token
    /// supply that we should report to Ethereum.
    #[prost(message, optional, tag = "2")]
    pub supply_delta_info: ::core::option::Option<SupplyDeltaInfo>,
    /// last_ethereum_nonce is the last nonce used in messages towards Ethereum.
    /// In other words, the next nonce to be used is this value +1.
    #[prost(bytes = "bytes", tag = "3")]
    pub last_ethereum_nonce: ::prost::bytes::Bytes,
    /// last_ethereum_block_synced is the last Ethereum block synced.
    /// In other words, the next block to be synced is this value +1.
    #[prost(uint64, tag = "4")]
    pub last_ethereum_block_synced: u64,
    /// ethereum_event_index_offset is the number of events to skip
    /// from the next Ethereum block to query from the Sidecar. This
    /// is used if queried events are larger than the maximum block size.
    #[prost(uint64, tag = "5")]
    pub ethereum_event_index_offset: u64,
    /// last_eth_block_update_time is the time of the last Sequencer
    /// block at which consensus was reach by the validators to sync
    /// up with an Ethereum block.
    #[prost(message, optional, tag = "6")]
    pub last_eth_block_update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// last_consensus_txs_sequence is the last sequence used in consensus txs.
    /// In other words, the next sequence to be used is this value +1.
    /// It is unique for each consensus tx, ensuring a unique tx hash.
    #[prost(uint64, tag = "7")]
    pub last_consensus_txs_sequence: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Index {
    /// num_injected_txs_total is the number of Ethereum events injected as
    /// txs in the current block, with any SupplyDelta, but excluding MsgIndex.
    #[prost(uint64, tag = "1")]
    pub num_injected_txs_total: u64,
    /// num_injected_txs_ante is the number of injected transactions that
    /// have been seen by the AnteHandler.
    #[prost(uint64, tag = "2")]
    pub num_injected_txs_ante: u64,
    /// num_failed_special_txs is the number of failed special transactions, i.e.
    /// MsgIndex, MsgDepositFromEthereum, and MsgSupplyDelta. We should stop block
    /// production if any of these fails because something is really wrong.
    #[prost(uint64, tag = "3")]
    pub num_failed_special_txs: u64,
}
// @@protoc_insertion_point(module)
