use serde::Deserialize;


// Blockchain Data: Available Coin List


/// Blockchain Data: Available Coin List
#[derive(Deserialize, Debug)]
pub struct CCAvailableCoinList{
    pub id: i32,
    pub symbol: String,
    pub partner_symbol: String,
    pub data_available_from: i64,
}


// Blockchain Data: Historical Daily


/// Blockchain Data: Historical Daily
#[derive(Deserialize, Debug)]
pub struct CCHistoricalDaily {
    pub id: i32,
    pub time: i64,
    pub symbol: String,
    pub zero_balance_addresses_all_time: i64,
    pub unique_addresses_all_time: i64,
    pub new_addresses: i64,
    pub active_addresses: i64,
    pub transaction_count: i64,
    pub transaction_count_all_time: i64,
    pub large_transaction_count: i64,
    pub average_transaction_value: f64,
    pub block_height: i64,
    pub hashrate: f64,
    pub difficulty: f64,
    pub block_time: f64,
    pub block_size: f64,
}


// Blockchain Data: Balance Distribution Daily


#[derive(Deserialize, Debug)]
pub struct CCSupplyBand {
    pub from: f64,
    pub to: f64,
    #[serde(rename = "totalVolume")]
    pub total_volume: f64,
    #[serde(rename = "addressesCount")]
    pub addresses_count: i64,
}

/// Blockchain Data: Balance Distribution Daily
#[derive(Deserialize, Debug)]
pub struct CCBalanceDistribution {
    pub id: i32,
    pub symbol: String,
    pub partner_symbol: String,
    pub time: i64,
    pub balance_distribution: Vec<CCSupplyBand>,
}