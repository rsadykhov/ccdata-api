use serde::Deserialize;
use crate::utils::Market;


/// The index family to obtain data from.
pub enum CCIndicesMarket {
    CADLI,
    CCIX,
    CCXRP,
    CCXRPPERP,
}

impl Market for CCIndicesMarket {
    /// Converts enum value to `String`.
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
    /// The unit of the historical period update: MINUTE for minute, HOUR for hour and DAY for day.
    pub unit: String,
    #[serde(rename = "TIMESTAMP")]
    /// The timestamp in seconds of the histo period, for minute it would be every minute at the beginning of the minute,
    /// for hour it would be start of the hour and for daily it is 00:00 GMT/UTC
    pub timestamp: i64,
    #[serde(rename = "TYPE")]
    /// Type of the message.
    pub type_: String,
    #[serde(rename = "MARKET")]
    /// The index family - a group of indices sharing common characteristics like methodology, type of securities, geographical region, sector,
    /// or company size. It's a crucial categorization in data analysis, aiding in performance comparison of different indices and understanding
    /// broader market trends.
    pub market: String,
    #[serde(rename = "INSTRUMENT")]
    /// The specific financial asset pair that an index is tracking in unmapped format. In most cases this is a combiation of the base and quote assets of the pair.
    pub instrument: String,
    #[serde(rename = "OPEN")]
    /// The initial value (price) of an index at market opening on a trading period. It's a significant data point used to understand the initial market sentiment,
    /// calculate various technical indicators, and for visual representation in charts to identify trends or patterns over time.
    pub open: f64,
    #[serde(rename = "HIGH")]
    /// The maximum value an index (price) reaches during a specific trading period. It's a significant data point used to understand the index's potential
    /// and volatility, calculate the range of daily movement, and for visual representation in charts to identify trends or patterns over time.
    pub high: f64,
    #[serde(rename = "LOW")]
    /// The minimum value (price) an index reaches during a specific trading period. It's a vital data point used to understand the index's volatility and risk,
    /// calculate the range of daily movement, and for visual representation in charts to identify trends or patterns over time.
    pub low: f64,
    #[serde(rename = "CLOSE")]
    /// The last value (price) of an index at the end of a trading period. It's a critical data point used for performance comparison,
    /// calculations like daily return or volatility, and for visual representation in charts to identify trends or patterns over time.
    pub close: f64,
    #[serde(rename = "FIRST_MESSAGE_TIMESTAMP")]
    /// The timestamp in seconds of the initial index update in the time period (only available when we have at least one index update in the time period).
    pub first_message_timestamp: i64,
    #[serde(rename = "LAST_MESSAGE_TIMESTAMP")]
    /// The timestamp in seconds of the last index update in the time period (only available when we have at least one index update in the time period).
    pub last_message_timestamp: i64,
    #[serde(rename = "FIRST_MESSAGE_VALUE")]
    /// The open value (price) based on the inital index update in the time period (only available when we have at least one index update in the time period).
    pub first_message_value: f64,
    #[serde(rename = "HIGH_MESSAGE_VALUE")]
    /// The maximum value an index (price) based on all the index updates in the time period (only available when we have at least one index update
    /// in the time period).
    pub high_message_value: f64,
    #[serde(rename = "HIGH_MESSAGE_TIMESTAMP")]
    /// The timestamp in seconds of the maximum value an index (price) based on all the index updates in the time period (only available when we have at
    /// least one index update in the time period).
    pub high_message_timestamp: i64,
    #[serde(rename = "LOW_MESSAGE_VALUE")]
    /// The minimum value an index (price) based on all the index updates in the time period (only available when we have at least one index update
    /// in the time period).
    pub low_message_value: f64,
    #[serde(rename = "LOW_MESSAGE_TIMESTAMP")]
    /// The timestamp in seconds of the the minimum value an index (price) based on all the index updates in the time period (only available when we have at
    /// least one index update in the time period).
    pub low_message_timestamp: i64,
    #[serde(rename = "LAST_MESSAGE_VALUE")]
    /// The last value (price) of an index based on the last index update in the time period (only available when we have at least one index update
    /// in the time period).
    pub last_message_value: f64,
    #[serde(rename = "TOTAL_INDEX_UPDATES")]
    /// The total number of message updates seen in this time period (0 when there no messages in the time period).
    pub total_index_updates: i32,
    #[serde(rename = "VOLUME")]
    /// The total number of base asset parts traded for the index instrument in the time period.
    /// It's a critical metric that provides insights into market liquidity and activity level. High volumes indicate high investor interest and liquidity,
    /// while low volumes suggest the opposite.
    pub volume: f64,
    #[serde(rename = "VOLUME_TOP_TIER")]
    /// The total number of top tier base asset parts traded for the index instrument in the time period.
    /// It's a critical metric that provides insights into market liquidity and activity level. High volumes indicate high investor interest and liquidity,
    /// while low volumes suggest the opposite.
    pub volume_top_tier: f64,
    #[serde(rename = "VOLUME_DIRECT")]
    /// The total number of direct base asset parts traded for the index instrument in the time period.
    /// It's a critical metric that provides insights into market liquidity and activity level. High volumes indicate high investor interest and liquidity,
    /// while low volumes suggest the opposite.
    pub volume_direct: f64,
    #[serde(rename = "VOLUME_TOP_TIER_DIRECT")]
    /// The total number of top tier direct base asset parts traded for the index instrument in the time period.
    /// It's a critical metric that provides insights into market liquidity and activity level. High volumes indicate high investor interest and liquidity,
    /// while low volumes suggest the opposite.
    pub volume_top_tier_direct: f64,
    #[serde(rename = "QUOTE_VOLUME")]
    /// The total number of quote (counter) asset parts traded for the index instrument in the time period.
    /// This offers insight into market activity and liquidity and is used widely in numerical analysis and data visualization.
    pub quote: f64,
    #[serde(rename = "QUOTE_VOLUME_TOP_TIER")]
    /// The total number of top tier quote (counter) asset parts traded for the index instrument in the time period.
    /// This offers insight into market activity and liquidity and is used widely in numerical analysis and data visualization.
    pub quote_top_tier: f64,
    #[serde(rename = "QUOTE_VOLUME_DIRECT")]
    /// The total number of direct quote (counter) asset parts traded for the index instrument in the time period.
    /// This offers insight into market activity and liquidity and is used widely in numerical analysis and data visualization.
    pub quote_direct: f64,
    #[serde(rename = "QUOTE_VOLUME_TOP_TIER_DIRECT")]
    /// The total number of top tier direct quote (counter) asset parts traded for the index instrument in the time period.
    /// This offers insight into market activity and liquidity and is used widely in numerical analysis and data visualization.
    pub quote_top_tier_direct: f64,
}