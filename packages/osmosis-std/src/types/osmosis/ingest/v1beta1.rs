use osmosis_std_derive::CosmwasmExt;
/// PoolData represents a structure encapsulating an Osmosis liquidity pool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.ingest.v1beta1.PoolData")]
pub struct PoolData {
    /// ChainModel is the chain representation model of the pool.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub chain_model: ::prost::alloc::vec::Vec<u8>,
    /// SqsModel is additional pool data used by the sidecar query server.
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub sqs_model: ::prost::alloc::vec::Vec<u8>,
    /// TickModel is the tick data of a concentrated liquidity pool.
    /// This field is only valid and set for concentrated pools. It is nil
    /// otherwise.
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub tick_model: ::prost::alloc::vec::Vec<u8>,
}
/// The block process request.
/// Sends taker fees, block height and pools.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.ingest.v1beta1.ProcessBlockRequest")]
pub struct ProcessBlockRequest {
    /// block height is the height of the block being processed.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_height: u64,
    /// taker_fees_map is the map of taker fees for the block.
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub taker_fees_map: ::prost::alloc::vec::Vec<u8>,
    /// pools in the block.
    #[prost(message, repeated, tag = "3")]
    pub pools: ::prost::alloc::vec::Vec<PoolData>,
}
/// The response after completing the block processing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.ingest.v1beta1.ProcessBlockReply")]
pub struct ProcessBlockReply {}
