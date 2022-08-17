use osmosis_std_derive::CosmwasmExt;
/// Parameters for changing the weights in a balancer pool smoothly from
/// a start weight and end weight over a period of time.
/// Currently, the only smooth change supported is linear changing between
/// the two weights, but more types may be added in the future.
/// When these parameters are set, the weight w(t) for pool time `t` is the
/// following:
///   t <= start_time: w(t) = initial_pool_weights
///   start_time < t <= start_time + duration:
///     w(t) = initial_pool_weights + (t - start_time) *
///       (target_pool_weights - initial_pool_weights) / (duration)
///   t > start_time + duration: w(t) = target_pool_weights
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.SmoothWeightChangeParams")]
pub struct SmoothWeightChangeParams {
    /// The start time for beginning the weight change.
    /// If a parameter change / pool instantiation leaves this blank,
    /// it should be generated by the state_machine as the current time.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<crate::shim::Timestamp>,
    /// Duration for the weights to change over
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<crate::shim::Duration>,
    /// The initial pool weights. These are copied from the pool's settings
    /// at the time of weight change instantiation.
    /// The amount PoolAsset.token.amount field is ignored if present,
    /// future type refactorings should just have a type with the denom & weight
    /// here.
    #[prost(message, repeated, tag = "3")]
    pub initial_pool_weights: ::prost::alloc::vec::Vec<PoolAsset>,
    /// The target pool weights. The pool weights will change linearly with respect
    /// to time between start_time, and start_time + duration. The amount
    /// PoolAsset.token.amount field is ignored if present, future type
    /// refactorings should just have a type with the denom & weight here.
    ///
    /// Intermediate variable for the 'slope' of pool weights. This is equal to
    /// (target_pool_weights - initial_pool_weights) / (duration)
    /// TODO: Work out precision, and decide if this is good to add
    /// repeated PoolAsset poolWeightSlope = 5 [
    ///  (gogoproto.moretags) = "yaml:\"pool_weight_slope\"",
    ///  (gogoproto.nullable) = false
    /// ];
    #[prost(message, repeated, tag = "4")]
    pub target_pool_weights: ::prost::alloc::vec::Vec<PoolAsset>,
}
/// PoolParams defined the parameters that will be managed by the pool
/// governance in the future. This params are not managed by the chain
/// governance. Instead they will be managed by the token holders of the pool.
/// The pool's token holders are specified in future_pool_governor.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.PoolParams")]
pub struct PoolParams {
    #[prost(string, tag = "1")]
    pub swap_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub exit_fee: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub smooth_weight_change_params: ::core::option::Option<SmoothWeightChangeParams>,
}
/// Pool asset is an internal struct that combines the amount of the
/// token in the pool, and its balancer weight.
/// This is an awkward packaging of data,
/// and should be revisited in a future state migration.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.PoolAsset")]
pub struct PoolAsset {
    /// Coins we are talking about,
    /// the denomination must be unique amongst all PoolAssets for this pool.
    #[prost(message, optional, tag = "1")]
    pub token: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// Weight that is not normalized. This weight must be less than 2^50
    #[prost(string, tag = "2")]
    pub weight: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.Pool")]
pub struct Pool {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub id: u64,
    #[prost(message, optional, tag = "3")]
    pub pool_params: ::core::option::Option<PoolParams>,
    /// This string specifies who will govern the pool in the future.
    /// Valid forms of this are:
    /// {token name},{duration}
    /// {duration}
    /// where {token name} if specified is the token which determines the
    /// governor, and if not specified is the LP token for this pool.duration is
    /// a time specified as 0w,1w,2w, etc. which specifies how long the token
    /// would need to be locked up to count in governance. 0w means no lockup.
    /// TODO: Further improve these docs
    #[prost(string, tag = "4")]
    pub future_pool_governor: ::prost::alloc::string::String,
    /// sum of all LP tokens sent out
    #[prost(message, optional, tag = "5")]
    pub total_shares: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// These are assumed to be sorted by denomiation.
    /// They contain the pool asset and the information about the weight
    #[prost(message, repeated, tag = "6")]
    pub pool_assets: ::prost::alloc::vec::Vec<PoolAsset>,
    /// sum of all non-normalized pool weights
    #[prost(string, tag = "7")]
    pub total_weight: ::prost::alloc::string::String,
}
/// ===================== MsgJoinPool
/// This is really MsgJoinPoolNoSwap
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgJoinPool")]
pub struct MsgJoinPool {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub share_out_amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub token_in_maxs: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgJoinPoolResponse")]
pub struct MsgJoinPoolResponse {}
/// ===================== MsgExitPool
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgExitPool")]
pub struct MsgExitPool {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub share_in_amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub token_out_mins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgExitPoolResponse")]
pub struct MsgExitPoolResponse {}
/// ===================== MsgSwapExactAmountIn
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.SwapAmountInRoute")]
pub struct SwapAmountInRoute {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_out_denom: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgSwapExactAmountIn")]
pub struct MsgSwapExactAmountIn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
    #[prost(message, optional, tag = "3")]
    pub token_in: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "4")]
    pub token_out_min_amount: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgSwapExactAmountInResponse")]
pub struct MsgSwapExactAmountInResponse {
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgSwapExactAmountOut
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.SwapAmountOutRoute")]
pub struct SwapAmountOutRoute {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_in_denom: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgSwapExactAmountOut")]
pub struct MsgSwapExactAmountOut {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
    #[prost(string, tag = "3")]
    pub token_in_max_amount: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub token_out: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgSwapExactAmountOutResponse")]
pub struct MsgSwapExactAmountOutResponse {
    #[prost(string, tag = "1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// ===================== MsgJoinSwapExternAmountIn
/// TODO: Rename to MsgJoinSwapExactAmountIn
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgJoinSwapExternAmountIn")]
pub struct MsgJoinSwapExternAmountIn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(message, optional, tag = "3")]
    pub token_in: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// repeated cosmos.base.v1beta1.Coin tokensIn = 5 [
    ///   (gogoproto.moretags) = "yaml:\"tokens_in\"",
    ///   (gogoproto.nullable) = false
    /// ];
    #[prost(string, tag = "4")]
    pub share_out_min_amount: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgJoinSwapExternAmountInResponse")]
pub struct MsgJoinSwapExternAmountInResponse {
    #[prost(string, tag = "1")]
    pub share_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgJoinSwapShareAmountOut
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgJoinSwapShareAmountOut")]
pub struct MsgJoinSwapShareAmountOut {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub token_in_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub share_out_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub token_in_max_amount: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgJoinSwapShareAmountOutResponse")]
pub struct MsgJoinSwapShareAmountOutResponse {
    #[prost(string, tag = "1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// ===================== MsgExitSwapShareAmountIn
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgExitSwapShareAmountIn")]
pub struct MsgExitSwapShareAmountIn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub token_out_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub share_in_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub token_out_min_amount: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgExitSwapShareAmountInResponse")]
pub struct MsgExitSwapShareAmountInResponse {
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgExitSwapExternAmountOut
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgExitSwapExternAmountOut")]
pub struct MsgExitSwapExternAmountOut {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(message, optional, tag = "3")]
    pub token_out: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "4")]
    pub share_in_max_amount: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.MsgExitSwapExternAmountOutResponse")]
pub struct MsgExitSwapExternAmountOutResponse {
    #[prost(string, tag = "1")]
    pub share_in_amount: ::prost::alloc::string::String,
}
///=============================== Pool
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryPoolRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/Pool",
    response_type = QueryPoolResponse
)]
pub struct QueryPoolRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryPoolResponse")]
pub struct QueryPoolResponse {
    #[prost(message, optional, tag = "1")]
    pub pool: ::core::option::Option<crate::shim::Any>,
}
///=============================== Pools
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryPoolsRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/Pools",
    response_type = QueryPoolsResponse
)]
pub struct QueryPoolsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryPoolsResponse")]
pub struct QueryPoolsResponse {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<crate::shim::Any>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
///=============================== NumPools
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryNumPoolsRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/NumPools",
    response_type = QueryNumPoolsResponse
)]
pub struct QueryNumPoolsRequest {}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryNumPoolsResponse")]
pub struct QueryNumPoolsResponse {
    #[prost(uint64, tag = "1")]
    pub num_pools: u64,
}
///=============================== PoolParams
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryPoolParamsRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/PoolParams",
    response_type = QueryPoolParamsResponse
)]
pub struct QueryPoolParamsRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryPoolParamsResponse")]
pub struct QueryPoolParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<crate::shim::Any>,
}
///=============================== PoolLiquidity
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryTotalPoolLiquidityRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/TotalPoolLiquidity",
    response_type = QueryTotalPoolLiquidityResponse
)]
pub struct QueryTotalPoolLiquidityRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryTotalPoolLiquidityResponse")]
pub struct QueryTotalPoolLiquidityResponse {
    #[prost(message, repeated, tag = "1")]
    pub liquidity: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
///=============================== TotalShares
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryTotalSharesRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/TotalShares",
    response_type = QueryTotalSharesResponse
)]
pub struct QueryTotalSharesRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryTotalSharesResponse")]
pub struct QueryTotalSharesResponse {
    #[prost(message, optional, tag = "1")]
    pub total_shares: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QuerySpotPriceRequest defines the gRPC request structure for a SpotPrice
/// query.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QuerySpotPriceRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/SpotPrice",
    response_type = QuerySpotPriceResponse
)]
pub struct QuerySpotPriceRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub base_asset_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_asset_denom: ::prost::alloc::string::String,
}
/// QuerySpotPriceResponse defines the gRPC response structure for a SpotPrice
/// query.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QuerySpotPriceResponse")]
pub struct QuerySpotPriceResponse {
    /// String of the Dec. Ex) 10.203uatom
    #[prost(string, tag = "1")]
    pub spot_price: ::prost::alloc::string::String,
}
///=============================== EstimateSwapExactAmountIn
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QuerySwapExactAmountInRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/EstimateSwapExactAmountIn",
    response_type = QuerySwapExactAmountInResponse
)]
pub struct QuerySwapExactAmountInRequest {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QuerySwapExactAmountInResponse")]
pub struct QuerySwapExactAmountInResponse {
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
///=============================== EstimateSwapExactAmountOut
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QuerySwapExactAmountOutRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/EstimateSwapExactAmountOut",
    response_type = QuerySwapExactAmountOutResponse
)]
pub struct QuerySwapExactAmountOutRequest {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(message, repeated, tag = "3")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
    #[prost(string, tag = "4")]
    pub token_out: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QuerySwapExactAmountOutResponse")]
pub struct QuerySwapExactAmountOutResponse {
    #[prost(string, tag = "1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryTotalLiquidityRequest")]
#[proto_query(
    path = "/osmosis.gamm.v1beta1.Query/TotalLiquidity",
    response_type = QueryTotalLiquidityResponse
)]
pub struct QueryTotalLiquidityRequest {}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.QueryTotalLiquidityResponse")]
pub struct QueryTotalLiquidityResponse {
    #[prost(message, repeated, tag = "1")]
    pub liquidity: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// Params holds parameters for the incentives module
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.Params")]
pub struct Params {
    #[prost(message, repeated, tag = "1")]
    pub pool_creation_fee:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the gamm module's genesis state.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v1beta1.GenesisState")]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<crate::shim::Any>,
    #[prost(uint64, tag = "2")]
    pub next_pool_number: u64,
    #[prost(message, optional, tag = "3")]
    pub params: ::core::option::Option<Params>,
}
pub struct GammQuerier<'a> {
    querier: cosmwasm_std::QuerierWrapper<'a, cosmwasm_std::Empty>,
}
impl<'a> GammQuerier<'a> {
    pub fn new(querier: cosmwasm_std::QuerierWrapper<'a, cosmwasm_std::Empty>) -> Self {
        Self { querier }
    }
    pub fn pools(
        &self,
        req: QueryPoolsRequest,
    ) -> Result<QueryPoolsResponse, cosmwasm_std::StdError> {
        req.query(self.querier)
    }
    pub fn num_pools(
        &self,
        req: QueryNumPoolsRequest,
    ) -> Result<QueryNumPoolsResponse, cosmwasm_std::StdError> {
        req.query(self.querier)
    }
    pub fn total_liquidity(
        &self,
        req: QueryTotalLiquidityRequest,
    ) -> Result<QueryTotalLiquidityResponse, cosmwasm_std::StdError> {
        req.query(self.querier)
    }
    pub fn pool(&self, req: QueryPoolRequest) -> Result<QueryPoolResponse, cosmwasm_std::StdError> {
        req.query(self.querier)
    }
    pub fn pool_params(
        &self,
        req: QueryPoolParamsRequest,
    ) -> Result<QueryPoolParamsResponse, cosmwasm_std::StdError> {
        req.query(self.querier)
    }
    pub fn total_pool_liquidity(
        &self,
        req: QueryTotalPoolLiquidityRequest,
    ) -> Result<QueryTotalPoolLiquidityResponse, cosmwasm_std::StdError> {
        req.query(self.querier)
    }
    pub fn total_shares(
        &self,
        req: QueryTotalSharesRequest,
    ) -> Result<QueryTotalSharesResponse, cosmwasm_std::StdError> {
        req.query(self.querier)
    }
    pub fn spot_price(
        &self,
        req: QuerySpotPriceRequest,
    ) -> Result<QuerySpotPriceResponse, cosmwasm_std::StdError> {
        req.query(self.querier)
    }
    pub fn estimate_swap_exact_amount_in(
        &self,
        req: QuerySwapExactAmountInRequest,
    ) -> Result<QuerySwapExactAmountInResponse, cosmwasm_std::StdError> {
        req.query(self.querier)
    }
    pub fn estimate_swap_exact_amount_out(
        &self,
        req: QuerySwapExactAmountOutRequest,
    ) -> Result<QuerySwapExactAmountOutResponse, cosmwasm_std::StdError> {
        req.query(self.querier)
    }
}
