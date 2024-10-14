use serde::Deserialize;
use crate::schemas::data_api::{CCPreviousAssetSymbol, CCAssetAlternativeId, CCAssetIndustry, CCSpecialAddress};


#[derive(Deserialize, Debug)]
pub struct CCOCCoreSupportedPlatforms {
    #[serde(rename = "BLOCKCHAIN")]
    pub blockchain: Option<String>,
    #[serde(rename = "BRIDGE_OPERATOR")]
    pub bridge_oprator: Option<String>,
    #[serde(rename = "TOKEN_STANDARD")]
    pub token_standard: Option<String>,
    #[serde(rename = "SMART_CONTRACT_ADDRESS")]
    pub smart_contract_address: Option<String>,
    #[serde(rename = "EXPLORER_URL")]
    pub explorer_url: Option<String>,
    #[serde(rename = "DECIMALS")]
    pub decimals: Option<i32>,
    #[serde(rename = "IS_ASSET_ISSUER")]
    pub is_asset_issuer: Option<bool>,
    #[serde(rename = "TRADING_AS")]
    pub trading_as: Option<String>,
    #[serde(rename = "LAUNCH_DATE")]
    pub launch_date: Option<i64>,
}


// On-Chain Core: ETH Blocks#


#[derive(Deserialize, Debug)]
pub struct CCOCCoreETHTrace {
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "TRACE_TYPE")]
    pub trace_type: Option<String>,
    #[serde(rename = "ADDRESS")]
    pub address: Option<Vec<i32>>,
    #[serde(rename = "ACTION_FROM")]
    pub action_from: Option<String>,
    #[serde(rename = "ACTION_CALL_TYPE")]
    pub action_call_type: Option<String>,
    #[serde(rename = "ACTION_GAS")]
    pub action_gas: Option<String>,
    #[serde(rename = "ACTION_INIT")]
    pub action_init: Option<String>,
    #[serde(rename = "ACTION_INPUT")]
    pub action_input: Option<String>,
    #[serde(rename = "ACTION_TO")]
    pub action_to: Option<String>,
    #[serde(rename = "ACTION_VALUE")]
    pub action_value: Option<String>,
    #[serde(rename = "ACTION_AUTHOR")]
    pub action_author: Option<String>,
    #[serde(rename = "ACTION_REWARD_TYPE")]
    pub action_reward_type: Option<String>,
    #[serde(rename = "ACTION_ADDRESS")]
    pub action_address: Option<String>,
    #[serde(rename = "ACTION_REFUND_ADDRESS")]
    pub action_refund_address: Option<String>,
    #[serde(rename = "ACTION_BALANCE")]
    pub action_balance: Option<String>,
    #[serde(rename = "RESULT_ADDRESS")]
    pub result_address: Option<String>,
    #[serde(rename = "RESULT_CODE")]
    pub result_code: Option<String>,
    #[serde(rename = "RESULT_GAS_USED")]
    pub result_gas_used: Option<String>,
    #[serde(rename = "RESULT_OUTPUT")]
    pub result_output: Option<String>,
    #[serde(rename = "SUBTRACES")]
    pub subtraces: Option<i32>,
    #[serde(rename = "ERROR")]
    pub error: Option<String>,
    #[serde(rename = "STATUS")]
    pub status: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CCOCCoreETHMetadata {
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "NUMBER")]
    pub number: i64,
    #[serde(rename = "TIMESTAMP")]
    pub timestamp: i64,
    #[serde(rename = "HASH")]
    pub hash: String,
    #[serde(rename = "PARENT_HASH")]
    pub parent_hash: String,
    #[serde(rename = "NONCE")]
    pub nonce: String,
    #[serde(rename = "SHA3_UNCLES")]
    pub sha3_uncles: String,
    #[serde(rename = "LOGS_BLOOM")]
    pub logs_bloom: String,
    #[serde(rename = "TRANSACTIONS_ROOT")]
    pub transaction_root: String,
    #[serde(rename = "STATE_ROOT")]
    pub state_root: String,
    #[serde(rename = "MERKLE_ROOT")]
    pub merkle_root: Option<String>,
    #[serde(rename = "RECEIPTS_ROOT")]
    pub receipts_root: String,
    #[serde(rename = "MINER")]
    pub miner: String,
    #[serde(rename = "MIX_HASH")]
    pub mix_hash: String,
    #[serde(rename = "DIFFICULTY")]
    pub difficulty: i64,
    #[serde(rename = "TOTAL_DIFFICULTY")]
    pub total_difficulty: String,
    #[serde(rename = "CHAIN_WORK")]
    pub chain_work: Option<String>,
    #[serde(rename = "SIZE")]
    pub size: i64,
    #[serde(rename = "WEIGHT")]
    pub weight: Option<i64>,
    #[serde(rename = "BLOCK_TIME")]
    pub block_time: f64,
    #[serde(rename = "MEDIAN_TIME")]
    pub median_time: Option<f64>,
    #[serde(rename = "BLOB_GAS_USED")]
    pub blob_gas_used: i64,
    #[serde(rename = "EXCESS_BLOB_GAS")]
    pub excess_blob_gas: i64,
    #[serde(rename = "EXTRA_DATA")]
    pub extra_data: String,
    #[serde(rename = "GAS_LIMIT")]
    pub gas_limit: String,
    #[serde(rename = "GAS_USED")]
    pub gas_used: String,
    #[serde(rename = "TRANSACTION_COUNT")]
    pub transaction_count: i64,
    #[serde(rename = "BASE_FEE_PER_GAS")]
    pub base_fee_per_gas: String,
    #[serde(rename = "WITHDRAWALS_ROOT")]
    pub withdrawals_root: String,
}

#[derive(Deserialize, Debug)]
pub struct CCOCCoreETHTransactionAccessList {
    #[serde(rename = "TYPE")]
    pub type_: Option<String>,
    #[serde(rename = "ADDRESS")]
    pub address: Option<String>,
    #[serde(rename = "Storage_KEYS")]
    pub storage_keys: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct CCOCCoreETHTransactionBlob {
    #[serde(rename = "TYPE")]
    pub type_: Option<String>,
    #[serde(rename = "INDEX")]
    pub index: Option<i32>,
    #[serde(rename = "VERSIONED_HASH")]
    pub version_hash: Option<String>,
    #[serde(rename = "KZG_COMMITMENT")]
    pub kzg_commitment: Option<String>,
    #[serde(rename = "KZG_PROOF")]
    pub kzg_proof: Option<String>,
    #[serde(rename = "KZG_COMMITMENT_INCLUSION_PROOF")]
    pub kzg_commitment_inclusion_proof: Option<Vec<String>>,
    #[serde(rename = "SIZE")]
    pub size: Option<i32>,
    #[serde(rename = "DATA")]
    pub data: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CCOCCoreETHTransactionLog {
    #[serde(rename = "TYPE")]
    pub type_: Option<String>,
    #[serde(rename = "ADDRESS")]
    pub address: Option<String>,
    #[serde(rename = "INDEX")]
    pub index: Option<i32>,
    #[serde(rename = "DATA")]
    pub data: Option<String>,
    #[serde(rename = "TOPICS")]
    pub topics: Option<Vec<String>>,
    #[serde(rename = "REMOVED")]
    pub removed: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct CCOCCoreETHTransaction {
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "HASH")]
    pub hash: String,
    #[serde(rename = "TRANSACTION_TYPE")]
    pub transaction_type: i32,
    #[serde(rename = "NONCE")]
    pub nonce: i64,
    #[serde(rename = "INDEX")]
    pub index: i32,
    #[serde(rename = "FROM_ADDRESS")]
    pub from_address: String,
    #[serde(rename = "TO_ADDRESS")]
    pub to_address: String,
    #[serde(rename = "VALUE")]
    pub value: String,
    #[serde(rename = "GAS")]
    pub gas: i64,
    #[serde(rename = "LOGS_BLOOM")]
    pub logs_bloom: Option<String>,
    #[serde(rename = "L1_GAS_USED")]
    pub l1_gas_used: Option<i32>,
    #[serde(rename = "L1_FEE")]
    pub l1_fee: Option<i32>,
    #[serde(rename = "L1_GAS_PRICE")]
    pub l1_gas_price: Option<i32>,
    #[serde(rename = "DEPOSIT_NONCE")]
    pub deposit_nonce: Option<i64>,
    #[serde(rename = "DEPOSIT_RECEIPT_VERSION")]
    pub deposit_receipt_version: Option<i32>,
    #[serde(rename = "GAS_PRICE")]
    pub gas_price: i64,
    #[serde(rename = "INPUT")]
    pub input: String,
    #[serde(rename = "RECEIPT_BLOB_GAS_PRICE")]
    pub receipt_blob_gas_price: Option<i64>,
    #[serde(rename = "RECEIPT_BLOB_GAS_USED")]
    pub receipt_blob_gas_used: Option<i64>,
    #[serde(rename = "RECEIPT_CUMULATIVE_GAS_USED")]
    pub receipt_cumulative_gas_used: Option<i64>,
    #[serde(rename = "RECEIPT_GAS_USED")]
    pub receipt_gas_used: Option<i64>,
    #[serde(rename = "RECEIPT_CONTACT_ADDRESS")]
    pub receipt_contact_address: Option<String>,
    #[serde(rename = "RECEIPT_ROOT")]
    pub receipt_root: Option<String>,
    #[serde(rename = "RECEIPT_STATUS")]
    pub receipt_status: i32,
    #[serde(rename = "MAX_FEE_PER_GAS")]
    pub max_fee_per_gas: Option<i64>,
    #[serde(rename = "MAX_PRIORITY_FEE_PER_GAS")]
    pub max_priority_fee_per_gas: Option<i64>,
    #[serde(rename = "MAX_FEE_PER_BLOB_GAS")]
    pub max_fee_per_blob_gas: Option<i64>,
    #[serde(rename = "RECEIPT_EFFECTIVE_GAS_PRICE")]
    pub receipt_effective_gas_price: Option<i64>,
    #[serde(rename = "ACCESS_LIST")]
    pub access_list: Option<Vec<CCOCCoreETHTransactionAccessList>>,
    #[serde(rename = "BLOBS")]
    pub blobs: Option<Vec<CCOCCoreETHTransactionBlob>>,
    #[serde(rename = "ECDSA_V")]
    pub ecdsa_v: Option<i32>,
    #[serde(rename = "ECDSA_R")]
    pub ecdsa_r: Option<String>,
    #[serde(rename = "ECDSA_S")]
    pub ecdsa_s: Option<String>,
    #[serde(rename = "Y_PARITY")]
    pub y_parity: Option<String>,
    #[serde(rename = "TRACES")]
    pub traces: Option<Vec<CCOCCoreETHTrace>>,
    #[serde(rename = "LOGS")]
    pub logs: Option<Vec<CCOCCoreETHTransactionLog>>,
    #[serde(rename = "HEX")]
    pub hex: Option<String>,
    #[serde(rename = "SOURCE_HASH")]
    pub source_hash: Option<String>,
    #[serde(rename = "MINT")]
    pub mint: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct CCOCCoreETHUncle {
    #[serde(rename = "TYPE")]
    pub type_: String,
}

#[derive(Deserialize, Debug)]
pub struct CCOCCoreETHWithdrawal {
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "INDEX")]
    pub index: i32,
    #[serde(rename = "VALIDATOR_INDEX")]
    pub validator_index: i32,
    #[serde(rename = "ADDRESS")]
    pub address: String,
    #[serde(rename = "AMOUNT")]
    pub amount: f64,
    #[serde(rename = "UNIT")]
    pub unit: String,
}

/// ON-Chain Core: ETH Blocks
#[derive(Deserialize, Debug)]
pub struct CCOCCoreETHBlock {
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "ASSET_ID")]
    pub asset_id: i32,
    #[serde(rename = "SYMBOL")]
    pub symbol: String,
    #[serde(rename = "PROVIDER_KEY")]
    pub provider_key: String,
    #[serde(rename = "CHAIN_ID")]
    pub chain_id: i32,
    #[serde(rename = "IS_PART_OF_REORG")]
    pub is_part_of_reorg: bool,
    #[serde(rename = "NUMBER")]
    pub number: i64,
    #[serde(rename = "TIMESTAMP")]
    pub timestamp: i64,
    #[serde(rename = "RECEIVED_TIMESTAMP")]
    pub received_timestamp: i64,
    #[serde(rename = "METADATA")]
    pub metadata: CCOCCoreETHMetadata,
    #[serde(rename = "TRANSACTIONS")]
    pub transactions: Option<Vec<CCOCCoreETHTransaction>>,
    #[serde(rename = "ORPHAN_TRACES")]
    pub orphan_traces: Option<Vec<CCOCCoreETHTrace>>,
    #[serde(rename = "UNCLES")]
    pub uncles: Option<Vec<CCOCCoreETHUncle>>,
    #[serde(rename = "WITHDRAWALS")]
    pub withdrawals: Option<Vec<CCOCCoreETHWithdrawal>>,
}


// On-Chain Core: Assets Summary By Chain


#[derive(Deserialize, Debug)]
pub struct CCChainAssetSummary {
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "SYMBOL")]
    pub symbol: String,
    #[serde(rename = "ASSET_TYPE")]
    pub asset_type: String,
    #[serde(rename = "NAME")]
    pub name: String,
    #[serde(rename = "LOGO_URL")]
    pub logo_url: String,
    #[serde(rename = "LAUNCH_DATE")]
    pub launch_date: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct CCSupportedAsset {
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "SYMBOL")]
    pub symbol: String,
    #[serde(rename = "ASSET_TYPE")]
    pub asset_type: String,
    #[serde(rename = "NAME")]
    pub name: String,
    #[serde(rename = "LOGO_URL")]
    pub logo_url: Option<String>,
    #[serde(rename = "LAUNCH_DATE")]
    pub launch_date: Option<i64>,
    #[serde(rename = "FILTERED_SUPPORTED_PLATFORMS")]
    pub filtered_supported_platforms: Option<Vec<CCOCCoreSupportedPlatforms>>,
}

/// On-Chain Core: Assets Summary By Chain
#[derive(Deserialize, Debug)]
pub struct CCOCCoreAssetByChain {
    #[serde(rename = "CHAIN_ASSET_SUMMARY")]
    pub chain_asset_summary: CCChainAssetSummary,
    #[serde(rename = "ASSETS_SUPPORTED")]
    pub assets_supported: Vec<CCSupportedAsset>,
}


// On-Chain Core: Asset by Address


#[derive(Deserialize, Debug)]
pub struct CCOCCoreSecurityMetric {
    #[serde(rename = "NAME")]
    pub name: String,
    #[serde(rename = "OVERALL_SCORE")]
    pub overall_score: f64,
    #[serde(rename = "OVERALL_RANK")]
    pub overall_rank: f64,
    #[serde(rename = "UPDATED_AT")]
    pub updated_at: i64,
}

#[derive(Deserialize, Debug)]
pub struct CCOCCoreReservesBreakdown {
    #[serde(rename = "RESERVE_TYPE")]
    pub reserve_type: String,
    #[serde(rename = "HOLDING_ADDRESSES")]
    pub holding_addresses: String,
    #[serde(rename = "PERCENTAGE")]
    pub percentage: f64,
    #[serde(rename = "DESCRIPTION")]
    pub description: String,
    #[serde(rename = "COMMENTS")]
    pub comments: String,
}

#[derive(Deserialize, Debug)]
pub struct CCOCCoreDocumentURLs {
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "VERSION")]
    pub version: i32,
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "COMMENT")]
    pub comment: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CCOCCorePriceConversionAsset {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "SYMBOL")]
    pub symbol: String,
    #[serde(rename = "ASSET_TYPE")]
    pub asset_type: String,
}

#[derive(Deserialize, Debug)]
pub struct CCOCCoreToplistRank {
    #[serde(rename = "CREATED_ON")]
    pub created_on: i64,
    #[serde(rename = "LAUNCH_DATE")]
    pub launch_date: i64,
    #[serde(rename = "PRICE_USD")]
    pub price_usd: Option<f64>,
    #[serde(rename = "CIRCULATING_MKT_CAP_USD")]
    pub circulating_mkt_cap_usd: Option<f64>,
    #[serde(rename = "TOTAL_MKT_CAP_USD")]
    pub total_mkt_cap_usd: Option<f64>,
    #[serde(rename = "SPOT_MOVING_24_HOUR_QUOTE_VOLUME_TOP_TIER_DIRECT_USD")]
    pub spot_moving_24_hour_quote_volume_top_tier_direct_usd: Option<f64>,
    #[serde(rename = "SPOT_MOVING_24_HOUR_QUOTE_VOLUME_DIRECT_USD")]
    pub spot_moving_24_hour_quote_volume_direct_usd: Option<f64>,
    #[serde(rename = "SPOT_MOVING_24_HOUR_QUOTE_VOLUME_TOP_TIER_USD")]
    pub spot_moving_24_hour_quote_volume_top_tier_usd: Option<f64>,
    #[serde(rename = "SPOT_MOVING_24_HOUR_QUOTE_VOLUME_USD")]
    pub spot_moving_24_hour_quote_volume_usd: Option<f64>,
    #[serde(rename = "SPOT_MOVING_24_HOUR_CHANGE_USD")]
    pub spot_moving_24_hour_change_usd: Option<f64>,
    #[serde(rename = "SPOT_MOVING_24_HOUR_CHANGE_PERCENTAGE_USD")]
    pub spot_moving_24_hour_change_percentage_usd: Option<f64>,
    #[serde(rename = "SPOT_MOVING_7_DAY_QUOTE_VOLUME_TOP_TIER_DIRECT_USD")]
    pub spot_moving_7_day_quote_volume_top_tier_direct_usd: Option<f64>,
    #[serde(rename = "SPOT_MOVING_7_DAY_QUOTE_VOLUME_DIRECT_USD")]
    pub spot_moving_7_day_quote_volume_direct_usd: Option<f64>,
    #[serde(rename = "SPOT_MOVING_7_DAY_QUOTE_VOLUME_TOP_TIER_USD")]
    pub spot_moving_7_day_quote_volume_top_tier_usd: Option<f64>,
    #[serde(rename = "SPOT_MOVING_7_DAY_QUOTE_VOLUME_USD")]
    pub spot_moving_7_day_quote_volume_usd: Option<f64>,
    #[serde(rename = "SPOT_MOVING_7_DAY_CHANGE_USD")]
    pub spot_moving_7_day_change_usd: Option<f64>,
    #[serde(rename = "SPOT_MOVING_7_DAY_CHANGE_PERCENTAGE_USD")]
    pub spot_moving_7_day_change_percentage_usd: Option<f64>,
    #[serde(rename = "SPOT_MOVING_30_DAY_QUOTE_VOLUME_TOP_TIER_DIRECT_USD")]
    pub spot_moving_30_day_quote_volume_top_tier_direct_usd: Option<f64>,
    #[serde(rename = "SPOT_MOVING_30_DAY_QUOTE_VOLUME_DIRECT_USD")]
    pub spot_moving_30_day_quote_volume_direct_usd: Option<f64>,
    #[serde(rename = "SPOT_MOVING_30_DAY_QUOTE_VOLUME_TOP_TIER_USD")]
    pub spot_moving_30_day_quote_volume_top_tier_usd: Option<f64>,
    #[serde(rename = "SPOT_MOVING_30_DAY_QUOTE_VOLUME_USD")]
    pub spot_moving_30_day_quote_volume_usd: Option<f64>,
    #[serde(rename = "SPOT_MOVING_30_DAY_CHANGE_USD")]
    pub spot_moving_30_day_change_usd: Option<f64>,
    #[serde(rename = "SPOT_MOVING_30_DAY_CHANGE_PERCENTAGE_USD")]
    pub spot_moving_30_day_change_percentage_usd: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct CCOCCoreProjectLeader {
    #[serde(rename = "LEADER_TYPE")]
    pub leader_type: String,
    #[serde(rename = "FULL_NAME")]
    pub fuill_name: String,
}

#[derive(Deserialize, Debug)]
pub struct CCOCCoreContactDetails {
    #[serde(rename = "CONTACT_TYPE")]
    pub contact_type: String,
    #[serde(rename = "CONTACT_MEDIUM")]
    pub contact_medium: String,
    #[serde(rename = "FULL_NAME")]
    pub full_name: String,
    #[serde(rename = "ADDRESS")]
    pub address: String,
    #[serde(rename = "COMMENTS")]
    pub comments: String,
}

/// On-Chain Core: Asset by Address
#[derive(Deserialize, Debug)]
pub struct CCOCCoreAssetByAddress {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "ID_LEGACY")]
    pub id_legacy: i32,
    #[serde(rename = "ID_PARENT_ASSET")]
    pub id_parent_asset: i32,
    #[serde(rename = "IS_ASSET_ISSUER")]
    pub id_asset_issuer: Option<i32>,
    #[serde(rename = "SYMBOL")]
    pub symbol: String,
    #[serde(rename = "URI")]
    pub uri: String,
    #[serde(rename = "ASSET_TYPE")]
    pub asset_type: String,
    #[serde(rename = "ASSET_ISSUER_NAME")]
    pub asset_issuer_name: Option<String>,
    #[serde(rename = "PARENT_ASSET_SYMBOL")]
    pub parent_asset_symbol: Option<String>,
    #[serde(rename = "ROOT_ASSET_ID")]
    pub root_asset_id: i32,
    #[serde(rename = "ROOT_ASSET_SYMBOL")]
    pub root_asset_symbol: String,
    #[serde(rename = "ROOT_ASSET_TYPE")]
    pub root_asset_type: String,
    #[serde(rename = "CREATED_ON")]
    pub created_on: i64,
    #[serde(rename = "UPDATED_ON")]
    pub updated_on: i64,
    #[serde(rename = "PUBLIC_NOTICE")]
    pub public_notice: Option<String>,
    #[serde(rename = "NAME")]
    pub name: String,
    #[serde(rename = "LOGO_URL")]
    pub logo_url: String,
    #[serde(rename = "LAUNCH_DATE")]
    pub launch_date: i64,
    #[serde(rename = "PERIOUS_ASSET_SYMBOLS")]
    pub previous_asset_symbols: Option<Vec<CCPreviousAssetSymbol>>,
    #[serde(rename = "ASSET_ALTERNATIVE_IDS")]
    pub asset_laternative_ids: Option<Vec<CCAssetAlternativeId>>,
    #[serde(rename = "ASSET_DESCRIPTION_SNIPPET")]
    pub asset_description_snippet: Option<String>,
    #[serde(rename = "SUPPORTED_PLATFORMS")]
    pub supported_platforms: Option<Vec<CCOCCoreSupportedPlatforms>>,
    #[serde(rename = "ASSET_SECURITY_METRICS")]
    pub asset_security_metrics: Option<Vec<CCOCCoreSecurityMetric>>,
    #[serde(rename = "SUPPLY_MAX")]
    pub supply_max: f64,
    #[serde(rename = "SUPPLY_ISSUED")]
    pub supply_issued: Option<f64>,
    #[serde(rename = "SUPPLY_TOTAL")]
    pub supply_total: Option<f64>,
    #[serde(rename = "SUPPLY_CIRCULATING")]
    pub supply_circulating: Option<f64>,
    #[serde(rename = "SUPPLY_FUTURE")]
    pub supply_future: f64,
    #[serde(rename = "SUPPLY_LOCKED")]
    pub supply_locked: Option<f64>,
    #[serde(rename = "SUPPY_BURNT")]
    pub supply_burnt: Option<f64>,
    #[serde(rename = "SUPPLY_STAKED")]
    pub supply_staked: Option<f64>,
    #[serde(rename = "BURN_ADDRESSES")]
    pub burn_addresses: Option<Vec<CCSpecialAddress>>,
    #[serde(rename = "LOCKED_ADDRESSES")]
    pub locked_addresses: Option<Vec<CCSpecialAddress>>,
    #[serde(rename = "RESERVES_BREAKDOWN")]
    pub reserves_breakdown: Option<Vec<CCOCCoreReservesBreakdown>>,
    #[serde(rename = "WEBSITE_URL")]
    pub website_url: Option<String>,
    #[serde(rename = "BLOG_URL")]
    pub blog_url: Option<String>,
    #[serde(rename = "WHITE_PAPER_URL")]
    pub white_paper_url: Option<String>,
    #[serde(rename = "OTHER_DOCUMENT_URLS")]
    pub other_document_urls: Option<Vec<CCOCCoreDocumentURLs>>,
    #[serde(rename = "ASSET_INDUSTRIES")]
    pub asset_industries: Option<Vec<CCAssetIndustry>>,
    #[serde(rename = "PRICE_USD")]
    pub price_usd: f64,
    #[serde(rename = "PRICE_USD_SOURCE")]
    pub price_usd_source: String,
    #[serde(rename = "PRICE_USD_LAST_UPDATE_TS")]
    pub price_usd_last_update_ts: i64,
    #[serde(rename = "PRICE_CONVERSION_ASSET")]
    pub price_conversion_asset: Option<CCOCCorePriceConversionAsset>,
    #[serde(rename = "PRICE_CONVERSION_RATE")]
    pub price_conversion_rate: Option<f64>,
    #[serde(rename = "PRICE_CONVERSION_VALUE")]
    pub price_conversion_value: Option<f64>,
    #[serde(rename = "PRICE_CONVERSION_SOURCE")]
    pub price_conversion_source: Option<String>,
    #[serde(rename = "PRICE_CONVERSION_LAST_UPDATE_TS")]
    pub price_conversion_last_update_ts: Option<i64>,
    #[serde(rename = "MKT_CAP_PENALTY")]
    pub mkt_cap_penalty: Option<f64>,
    #[serde(rename = "CIRCULATING_MKT_CAP_USD")]
    pub circulating_mkt_cap_usd: f64,
    #[serde(rename = "TOTAL_MKT_CAP_USD")]
    pub total_mkt_cap_usd: f64,
    #[serde(rename = "CIRCULATING_MKT_CAP_CONVERSION")]
    pub circulating_mkt_cap_conversion: Option<f64>,
    #[serde(rename = "TOTAL_MKT_CAP_CONVERSION")]
    pub total_mkt_cap_conversion: Option<f64>,
    #[serde(rename = "SPOT_MOVING_24_HOUR_QUOTE_VOLUME_TOP_TIER_DIRECT_USD")]
    pub spot_moving_24_hour_quote_volume_top_tier_direct_usd: f64,
    #[serde(rename = "SPOT_MOVING_24_HOUR_QUOTE_VOLUME_DIRECT_USD")]
    pub spot_moving_24_hour_quote_volume_direct_usd: f64,
    #[serde(rename = "SPOT_MOVING_24_HOUR_QUOTE_VOLUME_TOP_TIER_USD")]
    pub spot_moving_24_hour_quote_volume_top_tier_usd: f64,
    #[serde(rename = "SPOT_MOVING_24_HOUR_QUOTE_VOLUME_USD")]
    pub spot_moving_24_hour_quote_volume_usd: f64,
    #[serde(rename = "SPOT_MOVING_24_HOUR_QUOTE_VOLUME_TOP_TIER_CONVERSION")]
    pub spot_moving_24_hour_quote_volume_top_tier_conversion: Option<f64>,
    #[serde(rename = "SPOT_MOVING_24_HOUR_QUOTE_VOLUME_CONVERSION")]
    pub spot_moving_24_hour_quote_volume_conversion: Option<f64>,
    #[serde(rename = "SPOT_MOVING_7_DAY_QUOTE_VOLUME_TOP_TIER_DIRECT_USD")]
    pub spot_moving_7_day_quote_volume_top_tier_direct_usd: f64,
    #[serde(rename = "SPOT_MOVING_7_DAY_QUOTE_VOLUME_DIRECT_USD")]
    pub spot_moving_7_day_quote_volume_direct_usd: f64,
    #[serde(rename = "SPOT_MOVING_7_DAY_QUOTE_VOLUME_TOP_TIER_USD")]
    pub spot_moving_7_day_quote_volume_top_tier_usd: f64,
    #[serde(rename = "SPOT_MOVING_7_DAY_QUOTE_VOLUME_USD")]
    pub spot_moving_7_day_quote_volume_usd: f64,
    #[serde(rename = "SPOT_MOVING_7_DAY_QUOTE_VOLUME_TOP_TIER_CONVERSION")]
    pub spot_moving_7_day_quote_volume_top_tier_conversion: Option<f64>,
    #[serde(rename = "SPOT_MOVING_7_DAY_QUOTE_VOLUME_CONVERSION")]
    pub spot_moving_7_day_quote_volume_conversion: Option<f64>,
    #[serde(rename = "SPOT_MOVING_30_DAY_QUOTE_VOLUME_TOP_TIER_DIRECT_USD")]
    pub spot_moving_30_day_quote_volume_top_tier_direct_usd: f64,
    #[serde(rename = "SPOT_MOVING_30_DAY_QUOTE_VOLUME_DIRECT_USD")]
    pub spot_moving_30_day_quote_volume_direct_usd: f64,
    #[serde(rename = "SPOT_MOVING_30_DAY_QUOTE_VOLUME_TOP_TIER_USD")]
    pub spot_moving_30_day_quote_volume_top_tier_usd: f64,
    #[serde(rename = "SPOT_MOVING_30_DAY_QUOTE_VOLUME_USD")]
    pub spot_moving_30_day_quote_volume_usd: f64,
    #[serde(rename = "SPOT_MOVING_30_DAY_QUOTE_VOLUME_TOP_TIER_CONVERSION")]
    pub spot_moving_30_day_quote_volume_top_tier_conversion: Option<f64>,
    #[serde(rename = "SPOT_MOVING_30_DAY_QUOTE_VOLUME_CONVERSION")]
    pub spot_moving_30_day_quote_volume_conversion: Option<f64>,
    #[serde(rename = "SPOT_MOVING_24_HOUR_CHANGE_USD")]
    pub spot_moving_24_hour_change_usd: f64,
    #[serde(rename = "SPOT_MOVING_24_HOUR_CHANGE_PERCENTAGE_USD")]
    pub spot_moving_24_hour_change_percentage_usd: f64,
    #[serde(rename = "SPOT_MOVING_24_HOUR_CHANGE_CONVERSION")]
    pub spot_moving_24_hour_change_conversion: Option<f64>,
    #[serde(rename = "SPOT_MOVING_24_HOUR_CHANGE_PERCENTAGE_CONVERSION")]
    pub spot_moving_24_hour_change_percentage_conversion: Option<f64>,
    #[serde(rename = "SPOT_MOVING_7_DAY_CHANGE_USD")]
    pub spot_moving_7_day_change_usd: f64,
    #[serde(rename = "SPOT_MOVING_7_DAY_CHANGE_PERCENTAGE_USD")]
    pub spot_moving_7_day_change_percentage_usd: f64,
    #[serde(rename = "SPOT_MOVING_7_DAY_CHANGE_CONVERSION")]
    pub spot_moving_7_day_change_conversion: Option<f64>,
    #[serde(rename = "SPOT_MOVING_7_DAY_CHANGE_PERCENTAGE_CONVERSION")]
    pub spot_moving_7_day_change_percentage_conversion: Option<f64>,
    #[serde(rename = "SPOT_MOVING_30_DAY_CHANGE_USD")]
    pub spot_moving_30_day_change_usd: f64,
    #[serde(rename = "SPOT_MOVING_30_DAY_CHANGE_PERCENTAGE_USD")]
    pub spot_moving_30_day_change_percentage_usd: f64,
    #[serde(rename = "SPOT_MOVING_30_DAY_CHANGE_CONVERSION")]
    pub spot_moving_30_day_change_conversion: Option<f64>,
    #[serde(rename = "SPOT_MOVING_30_DAY_CHANGE_PERCENTAGE_CONVERSION")]
    pub spot_moving_30_day_change_percentage_conversion: Option<f64>,
    #[serde(rename = "TOPLIST_BASE_RANK")]
    pub toplist_base_rank: Option<CCOCCoreToplistRank>,
    #[serde(rename = "ASSET_DESCRIPTION")]
    pub asset_description: String,
    #[serde(rename = "ASSET_DESCRIPTION_SUMMARY")]
    pub asset_description_summary: String,
    #[serde(rename = "PROJECT_LEADERS")]
    pub project_leaders: Option<Vec<CCOCCoreProjectLeader>>,
    #[serde(rename = "ASSOCIATED_CONTACT_DETAILS")]
    pub associated_contact_details: Option<CCOCCoreContactDetails>,
    #[serde(rename = "SEO_TITLE")]
    pub seo_title: String,
    #[serde(rename = "SEO_DESCRIPTION")]
    pub seo_description: String, 
}


// On-Chain Core: Historical Supply Day


/// On-Chain Core: Historical Supply Day
#[derive(Deserialize, Debug)]
pub struct CCOCCoreSupply {
    #[serde(rename = "UNIT")]
    pub unit: String,
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "ASSET_ID")]
    pub asset_id: i32,
    #[serde(rename = "SYMBOL")]
    pub symbol: String,
    #[serde(rename = "TIMESTAMP")]
    pub timestamp: i64,
    #[serde(rename = "SUPPLY_CIRCULATING")]
    pub supply_circulating: Option<f64>,
    #[serde(rename = "SUPPLY_TOTAL")]
    pub supply_total: Option<f64>,
    #[serde(rename = "SUPPLY_BURNT")]
    pub supply_burnt: Option<f64>,
    #[serde(rename = "SUPPLY_MAX")]
    pub supply_max: Option<f64>,
    #[serde(rename = "SUPPLY_STAKED")]
    pub supply_staked: Option<f64>,
    #[serde(rename = "SUPPLY_FUTURE")]
    pub supply_future: Option<f64>,
    #[serde(rename = "SUPPLY_ISSUED")]
    pub supply_issued: Option<f64>,
    #[serde(rename = "SUPPLY_LOCKED")]
    pub supply_locked: Option<f64>,
}