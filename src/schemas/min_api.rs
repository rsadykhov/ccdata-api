use serde::{Serialize, Deserialize};


// Blockchain Data: Balance Distribution Daily


#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SupplyBand {
    pub from: f64,
    pub to: f64,
    #[serde(rename = "totalVolume")]
    pub total_volume: f64,
    #[serde(rename = "addressesCount")]
    pub addresses_count: i64,
}

/// Blockchain Data: Balance Distribution Daily
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BalanceDistribution {
    pub id: i32,
    pub symbol: String,
    pub partner_symbol: String,
    pub time: i64,
    pub balance_distribution: Vec<SupplyBand>,
}