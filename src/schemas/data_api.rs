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
    /// A symbol this asset was previously associated with.
    pub symbol: Option<String>,
    #[serde(rename = "SYMBOL_USAGE_START_DATE")]
    /// Indicates the date this past symbol started being used.
    pub symbol_usage_start_date: Option<i64>,
    #[serde(rename = "SYMBOL_USAGE_END_DATE")]
    /// Indicates the date this past symbol stopped being used.
    pub symbol_usage_end_date: Option<i64>,
    #[serde(rename = "DESCRIPTION")]
    /// A description for why this symbol existed or was changed.
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
    #[serde(rename = "JUSTIFICATION")]
    /// A justification for putting an asset in this industry.
    pub justification: Option<String>,
}


#[derive(Deserialize, Debug)]
pub struct CCSpecialAddress {
    #[serde(rename = "NAME")]
    /// The name of the address. Contract name or just the common name for this address.
    pub name: Option<String>,
    #[serde(rename = "BLOCKCHAIN")]
    /// The is linked to the asset representing a specific chain.
    pub blockchain: Option<String>,
    #[serde(rename = "ADDRESS")]
    /// The address of the smart contracts, external user accounts or other account.
    pub address: Option<String>,
    #[serde(rename = "DESCRIPTION")]
    /// A description for the address.
    pub description: Option<String>,
}


#[derive(Deserialize, Debug)]
pub struct CCInstrumentStatus {
    #[serde(rename = "ACTIVE")]
    /// The total number of instruments currently available on the market, which are considered active. An active instrument is defined as an instrument
    /// from which we retrieve data and have either already mapped or are planning to map.
    pub active: i64,
    #[serde(rename = "IGNORED")]
    /// The total number of instruments available on the market that are classified as ignored, meaning that we do not plan to map them.
    /// Ignored instruments are those from which we do retrieve data but do not have any intention to map.
    pub ignored: i64,
    #[serde(rename = "RETIRED")]
    /// The total number of instruments that are classified as retired, meaning that they are no longer actively traded on the market.
    /// These instruments have ceased trading, and as such, we do not retrieve data from them but we have mapped them already.
    pub retired: i64,
    #[serde(rename = "EXPIRED")]
    /// The total number of instruments that are classified as expired, meaning that they are mapped instruments that are no longer actively traded on the market.
    /// These expired instruments are typically futures or options instruments that have reached their expiration date and are no longer available for trading.
    /// While we have previously mapped these instruments, we do not retrieve any data from them since they are no longer actively traded.
    pub expired: i64,
    #[serde(rename = "undefined")]
    pub undefined: Option<i64>,
}