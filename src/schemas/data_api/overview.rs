use serde::Deserialize;


// Overview: MktCap Historical OHLCV


/// Overview: MktCap Historical OHLCV
#[derive(Deserialize, Debug)]
pub struct CCOverviewMktCapOHLCV {
    #[serde(rename = "UNIT")]
    /// The unit of the message.
    pub unit: String,
    #[serde(rename = "TIMESTAMP")]
    /// The timestamp in seconds of the beginning of the histo period.
    /// For minute it would be every minute at the beginning of the minute, for hour it would be the start of the hour and for daily it is 00:00 GMT.
    pub timestamp: i64,
    #[serde(rename = "TYPE")]
    /// Type of the message, this is 886.
    pub type_: String,
    #[serde(rename = "OPEN")]
    /// The sum of assets circulating mkt caps (circulating supply * price quoted in USD) that meet the inclusion criteria closest to the start of the time period.
    pub open: f64,
    #[serde(rename = "HIGH")]
    /// The highest sum of assets circulating mkt caps (circulating supply * price quoted in USD) that meet the inclusion criteria during the time period.
    pub high: f64,
    #[serde(rename = "LOW")]
    /// The lowest sum of assets circulating mkt caps (circulating supply * price quoted in USD) that meet the inclusion criteria during the time period.
    pub low: f64,
    #[serde(rename = "CLOSE")]
    /// The sum of assets circulating mkt caps (circulating supply * price quoted in USD) that meet the inclusion criteria closest to the end of the time period.
    pub close: f64,
    #[serde(rename = "TOP_TIER_VOLUME")]
    /// The sum of assets top-tier volumes (top-tier volume * price quoted in USD) that meet the inclusion criteria during the time period.
    pub top_tier_volume: f64,
}