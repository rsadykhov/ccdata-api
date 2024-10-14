use serde::Deserialize;


// Overview: MktCap Historical OHLCV


/// Overview: MktCap Historical OHLCV
#[derive(Deserialize, Debug)]
pub struct CCOverviewMktCapOHLCV {
    #[serde(rename = "UNIT")]
    pub unit: String,
    #[serde(rename = "TIMESTAMP")]
    pub timestamp: i64,
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "OPEN")]
    pub open: f64,
    #[serde(rename = "HIGH")]
    pub high: f64,
    #[serde(rename = "LOW")]
    pub low: f64,
    #[serde(rename = "CLOSE")]
    pub close: f64,
    #[serde(rename = "TOP_TIER_VOLUME")]
    pub top_tier_volume: f64,
}