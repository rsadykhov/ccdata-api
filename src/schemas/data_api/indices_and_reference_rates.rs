use serde::Deserialize;
use crate::utils::Market;


pub enum CCIndicesMarket {
    CADLI,
    CCIX,
    CCXRP,
    CCXRPPERP,
}

impl Market for CCIndicesMarket {
    fn to_string(&self) -> String {
        match self {
            CCIndicesMarket::CADLI => String::from("cadli"),
            CCIndicesMarket::CCIX => String::from("ccix"),
            CCIndicesMarket::CCXRP => String::from("CCXRP"),
            CCIndicesMarket::CCXRPPERP => String::from("CCXRPPERP"),
        }
    }
}


// Indices & Reference Rates: Historical OHLCV+


/// Indices & Reference Rates: Historical OHLCV+
#[derive(Deserialize, Debug)]
pub struct CCIndicesOHLCV {
    #[serde(rename = "UNIT")]
    pub unit: String,
    #[serde(rename = "TIMESTAMP")]
    pub timestamp: i64,
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "MARKET")]
    pub market: String,
    #[serde(rename = "INSTRUMENT")]
    pub instrument: String,
    #[serde(rename = "OPEN")]
    pub open: f64,
    #[serde(rename = "HIGH")]
    pub high: f64,
    #[serde(rename = "LOW")]
    pub low: f64,
    #[serde(rename = "CLOSE")]
    pub close: f64,
    #[serde(rename = "FIRST_MESSAGE_TIMESTAMP")]
    pub first_message_timestamp: i64,
    #[serde(rename = "FIRST_MESSAGE_VALUE")]
    pub first_message_value: f64,
    #[serde(rename = "LAST_MESSAGE_TIMESTAMP")]
    pub last_message_timestamp: i64,
    #[serde(rename = "LAST_MESSAGE_VALUE")]
    pub last_message_value: f64,
    #[serde(rename = "HIGH_MESSAGE_TIMESTAMP")]
    pub high_message_timestamp: i64,
    #[serde(rename = "HIGH_MESSAGE_VALUE")]
    pub high_message_value: f64,
    #[serde(rename = "LOW_MESSAGE_TIMESTAMP")]
    pub low_message_timestamp: i64,
    #[serde(rename = "LOW_MESSAGE_VALUE")]
    pub low_message_value: f64,
    #[serde(rename = "TOTAL_INDEX_UPDATES")]
    pub total_index_updates: i64,
    #[serde(rename = "VOLUME")]
    pub volume: f64,
    #[serde(rename = "VOLUME_TOP_TIER")]
    pub volume_top_tier: f64,
    #[serde(rename = "VOLUME_DIRECT")]
    pub volume_direct: f64,
    #[serde(rename = "VOLUME_TOP_TIER_DIRECT")]
    pub volume_top_tier_direct: f64,
    #[serde(rename = "QUOTE_VOLUME")]
    pub quote: f64,
    #[serde(rename = "QUOTE_VOLUME_TOP_TIER")]
    pub quote_top_tier: f64,
    #[serde(rename = "QUOTE_VOLUME_DIRECT")]
    pub quote_direct: f64,
    #[serde(rename = "QUOTE_VOLUME_TOP_TIER_DIRECT")]
    pub quote_top_tier_direct: f64,
}