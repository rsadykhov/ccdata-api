pub mod indices_and_reference_rates;
pub mod spot;
pub mod futures;
pub mod options;
pub mod derivatives_indices;
pub mod on_chain_dex;
pub mod on_chain_core;
pub mod asset;
pub mod news;
pub mod overview;


use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct CCPreviousAssetSymbol {
    #[serde(rename = "SYMBOL")]
    pub symbol: Option<String>,
    #[serde(rename = "SYMBOL_USAGE_START_DATE")]
    pub symbol_usage_start_date: Option<i64>,
    #[serde(rename = "SYMBOL_USAGE_END_DATE")]
    pub symbol_usage_end_date: Option<i64>,
    #[serde(rename = "DESCRIPTION")]
    pub description: Option<String>,
}


#[derive(Deserialize, Debug)]
pub struct CCAssetAlternativeId {
    #[serde(rename = "NAME")]
    pub name: Option<String>,
    #[serde(skip_deserializing)]
    #[serde(rename = "ID")]
    pub id: Option<String>,
}


#[derive(Deserialize, Debug)]
pub struct CCAssetIndustry {
    #[serde(rename = "ASSET_INDUSTRY")]
    pub asset_industry: Option<String>,
}


#[derive(Deserialize, Debug)]
pub struct CCSpecialAddress {
    #[serde(rename = "NAME")]
    pub name: Option<String>,
    #[serde(rename = "BLOCKCHAIN")]
    pub blockchain: Option<String>,
    #[serde(rename = "ADDRESS")]
    pub address: Option<String>,
    #[serde(rename = "DESCRIPTION")]
    pub description: Option<String>,
}


#[derive(Deserialize, Debug)]
pub struct CCInstrumentStatus {
    #[serde(rename = "ACTIVE")]
    pub active: i64,
    #[serde(rename = "IGNORED")]
    pub ignored: i64,
    #[serde(rename = "RETIRED")]
    pub retired: i64,
    #[serde(rename = "EXPIRED")]
    pub expired: i64,
    #[serde(rename = "undefined")]
    pub undefined: Option<i64>,
}