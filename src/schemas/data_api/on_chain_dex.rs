use serde::Deserialize;
use crate::utils::Market;
use crate::schemas::data_api::CCInstrumentStatus;


/// The exchange to obtain data from.
pub enum CCOCDEXMarket {
    BALANCERV2,
    CURVE,
    PANCAKESWAPV2,
    PANCAKESWAPV3,
    SUSHISWAPV2,
    SUSHISWAPV3,
    UNISWAPV1,
    UNISWAPV2,
    UNISWAPV3,
}

impl Market for CCOCDEXMarket {
    /// Converts enum value to `String`.
    fn to_string(&self) -> String {
        match self {
            CCOCDEXMarket::BALANCERV2 => String::from("balancerv2"),
            CCOCDEXMarket::CURVE => String::from("curve"),
            CCOCDEXMarket::PANCAKESWAPV2 => String::from("pancakeswapv2"),
            CCOCDEXMarket::PANCAKESWAPV3 => String::from("pancakeswapv3"),
            CCOCDEXMarket::SUSHISWAPV2 => String::from("sushiswapv2"),
            CCOCDEXMarket::SUSHISWAPV3 => String::from("sushiswapv3"),
            CCOCDEXMarket::UNISWAPV1 => String::from("uniswapv1"),
            CCOCDEXMarket::UNISWAPV2 => String::from("uniswapv2"),
            CCOCDEXMarket::UNISWAPV3 => String::from("uniswapv3"),
        }
    }
}


// On-Cahin DEX: Historical OHLCV+


/// On-Chain DEX: Historical OHLCV+
#[derive(Deserialize, Debug)]
pub struct CCOCDEXOHLCV {
    #[serde(rename = "UNIT")]
    /// The unit of the historical period update: MINUTE for minute, HOUR for hour and DAY for day.
    pub unit: String,
    #[serde(rename = "TIMESTAMP")]
    /// The timestamp in seconds of the beginning of the histo period.
    /// For minute it would be every minute at the beginning of the minute, for hour it would be the start of the hour and for daily it is 00:00 GMT.
    pub timestamp: i64,
    #[serde(rename = "TYPE")]
    /// The internal type of the message.
    pub type_: String,
    #[serde(rename = "MARKET")]
    /// The market / exchange under consideration (uniswapv2, uniswapv3 etc.).
    pub market: String,
    #[serde(rename = "INSTRUMENT")]
    /// The unmapped instrument address and the CHAIN_ASSET ID separated by underscore (e.g. 0xe83c76c50033a5396d21ceff9fa192e2550d10ca_2, etc.).
    pub instrument: String,
    #[serde(rename = "MAPPED_INSTRUMENT")]
    /// The instrument id based on asset symbols, as derived from our mapping rules. This is the same as the INSTRUMENT and will not change.
    pub mapped_instrument: String,
    #[serde(rename = "BASE")]
    /// Represents the base asset or coin symbol, commonly known as the ticker (e.g., BTC). This symbol may change in cases of asset rebranding.
    /// Applicable only to instruments with a mapping.
    pub base: String,
    #[serde(rename = "QUOTE")]
    /// Represents the quote asset or counter coin symbol, commonly known as the ticker (e.g., USD). This symbol may change in cases of asset rebranding.
    /// Applicable only to instruments with a mapping.
    pub quote: String,
    #[serde(rename = "BASE_ID")]
    /// Represents the internal CCData ID for the base asset or coin (e.g., 1 for BTC). This ID is unique and immutable, ensuring consistent identification.
    /// Applicable only to instruments with a mapping.
    pub base_id: i32,
    #[serde(rename = "QUOTE_ID")]
    /// Represents the internal CCData ID for the quote asset or counter coin (e.g., 5 for USD). This ID is unique and immutable, ensuring consistent identification.
    /// Applicable only to instruments with a mapping.
    pub quote_id: i32,
    #[serde(rename = "TRANSFORM_FUNCTION")]
    /// The transform function (or list of functions concatenated by _AND_). This is the function we apply when we do mapping to change values into easier human
    /// readable ones and to make sure the mapped direction BASE - QUOTE is constant accross all chains and instruments.
    pub transform_function: String,
    #[serde(rename = "OPEN")]
    /// The open price for the historical period, this is based on the closest swap before the period start.
    pub open: f64,
    #[serde(rename = "HIGH")]
    /// The highest swap price in the time period. If there were no swaps in the time period, the open price will be given.
    pub high: f64,
    #[serde(rename = "LOW")]
    /// The lowest swap price in the time period. If there were no swaps in the time period, the open price will be given.
    pub low: f64,
    #[serde(rename = "CLOSE")]
    /// The price of the last swap in this time period. If there were no swaps in the time period, the open price will be given.
    pub close: f64,
    #[serde(rename = "FIRST_SWAP_TIMESTAMP")]
    /// The timestamp, in seconds, of the first swap in this time period. This is only available when there is at least one swap in the time period.
    pub first_swap_timestamp: i64,
    #[serde(rename = "FIRST_SWAP_BLOCK")]
    /// The block of the first swap in the time period. This is only available when there is at least one swap in the time period.
    pub first_swap_block: i64,
    #[serde(rename = "LAST_SWAP_TIMESTAMP")]
    /// The timestamp, in seconds, of the last swap in this time period. This is only available when there is at least one swap in the time period.
    pub last_swap_timestamp: i64,
    #[serde(rename = "LAST_SWAP_BLOCK")]
    /// The block of the last swap in the time period. This is only available when there is at least one swap in the time period.
    pub last_swap_block: i64,
    #[serde(rename = "FIRST_SWAP_PRICE")]
    /// The price of the first swap in the time period. This is only available when there is at least one swap in the time period.
    pub first_swap_price: f64,
    #[serde(rename = "HIGH_SWAP_PRICE")]
    /// The highest value of the swaps in the time period. This is only available when there is at least one swap in the time period.
    pub high_swap_price: f64,
    #[serde(rename = "HIGH_SWAP_TIMESTAMP")]
    /// The timestamp, in seconds, of the highest swap in the time period. This is only available when there is at least one swap in the time period.
    pub high_swap_timestamp: i64,
    #[serde(rename = "HIGH_SWAP_BLOCK")]
    /// The block of the highest swap in the time period. This is only available when there is at least one swap in the time period.
    pub high_swap_block: i64,
    #[serde(rename = "LOW_SWAP_PRICE")]
    /// The lowest value of the swaps in the time period. This is only available when there is at least one swap in the time period.
    pub low_swap_price: f64,
    #[serde(rename = "LOW_SWAP_TIMESTAMP")]
    /// The timestamp, in seconds, of the lowest swap in the time period. This is only available when there is at least one swap in the time period.
    pub low_swap_timestamp: i64,
    #[serde(rename = "LOW_SWAP_BLOCK")]
    /// The block of the lowest swap in the time period. This is only available when there is at least one swap in the time period.
    pub low_swap_block: i64,
    #[serde(rename = "LAST_SWAP_PRICE")]
    /// The last swap price in the time period. This is only available when there is at least one swap in the time period.
    pub last_swap_price: f64,
    #[serde(rename = "TOTAL_SWAPS")]
    /// The total number of swaps seen in this time period. If there were no swaps in the time period, 0 will be returned.
    pub total_swaps: i64,
    #[serde(rename = "TOTAL_SWAPS_BUY")]
    /// The total number of BUY swaps seen in this time period. If there were no swaps in the time period, 0 will be returned.
    pub total_swaps_buy: i64,
    #[serde(rename = "TOTAL_SWAPS_SELL")]
    /// The total number of SELL swaps seen in this time period. If there were no swaps in the time period, 0 will be returned."
    pub total_swaps_sell: i64,
    #[serde(rename = "TOTAL_SWAPS_UNKNOWN")]
    /// The total number of UNKNOWN swaps seen in this time period. If there were no swaps in the time period, 0 will be returned.
    pub total_swaps_unknown: i64,
    #[serde(rename = "VOLUME")]
    /// The sum of all the swap volumes in the from asset (base symbol / coin) for the time period. If there were no swaps in the time period, 0 will be returned.
    pub volume: f64,
    #[serde(rename = "QUOTE_VOLUME")]
    /// The sum of all the swap volumes in the To asset (quote/counter symbol/coin) for the time period. If there were no swaps in the time period,
    /// 0 will be returned.
    pub quote_volume: f64,
    #[serde(rename = "VOLUME_BUY")]
    /// The sum of all the BUY swap volumes in the from asset (base symbol / coin) for the time period.
    pub volume_buy: f64,
    #[serde(rename = "QUOTE_VOLUME_BUY")]
    /// The sum of all the BUY swap volumes in the to asset (quote/counter symbol/coin) for the time period.
    pub quote_volume_buy: f64,
    #[serde(rename = "VOLUME_SELL")]
    /// The sum of all the SELL swap volumes in the from asset (base symbol / coin) for the time period.
    pub volume_sell: f64,
    #[serde(rename = "QUOTE_VOLUME_SELL")]
    /// The sum of all the SELL swap volumes in the To asset (quote/counter symbol/coin) for the time period.
    pub quote_volume_sell: f64,
    #[serde(rename = "VOLUME_UNKNOWN")]
    /// The sum of all the UNKNOWN swap volumes in the from asset (base symbol / coin) for the time period.
    pub volume_unknown: String,
    #[serde(rename = "QUOTE_VOLUME_UNKNOWN")]
    /// The sum of all the UNKNOWN swap volumes in the To asset (quote/counter symbol/coin) for the time period.
    pub quote_volume_unknown: String,
}


// On-Chain DEX: Market


/// On-Chain DEX: Market
#[derive(Deserialize, Debug)]
pub struct CCOCDEXMarkets {
    #[serde(rename = "TYPE")]
    /// Type of the message.
    pub type_: String,
    #[serde(rename = "EXCHANGE_STATUS")]
    /// The status of the echange. We only poll / stream / connect to the ACTIVE ones, for the RETIRED ones we no longer query for data.
    pub exchange_status: String,
    #[serde(rename = "MAPPED_INSTRUMENTS_TOTAL")]
    /// The total number of instruments that have been verified by our mapping team and have been properly assigned with a base, quote, mapping function, 
    /// nd other necessary fields. This is done to ensure that pairs like XXBTZUSD are accurately mapped to BTC-USD and that the pair refers
    /// to the correct assets rather than using the same asset id to represent different assets.
    pub mapped_instrument_total: i64,
    #[serde(rename = "UNMAPPED_INSTRUMENTS_TOTAL")]
    /// The number of instruments that have not yet been verified by our mapping team.
    pub unmapped_instruments_total: i64,
    #[serde(rename = "INSTRUMENT_STATUS")]
    /// An object with the total number of instrument for each of the available instrument statuses.
    pub instrument_status: CCInstrumentStatus,
    #[serde(rename = "TOTAL_AMM_SWAPS_ONCHAIN")]
    /// The total number of defi swap trades this exchange has processed.
    pub total_amm_swaps_onchain: i64,
    #[serde(rename = "TOTAL_AMM_LIQUIDITY_UPDATES_ONCHAIN")]
    /// The total number of defi liquidity updates this exchange has processed.
    pub total_amm_liquidity_updates_onchain: i64,
    #[serde(rename = "HAS_ORDERBOOK_L2_MINUTE_SNAPSHOTS_ENABLED")]
    pub has_orderbook_l2_minute_snapshots_enabled: bool,
}