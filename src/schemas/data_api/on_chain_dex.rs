use serde::Deserialize;
use crate::utils::Market;
use crate::schemas::data_api::CCInstrumentStatus;


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
    pub unit: String,
    #[serde(rename = "TIMESTAMP")]
    pub timestamp: i64,
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "MARKET")]
    pub market: String,
    #[serde(rename = "INSTRUMENT")]
    pub instrument: String,
    #[serde(rename = "MAPPED_INSTRUMENT")]
    pub mapped_instrument: String,
    #[serde(rename = "BASE")]
    pub base: String,
    #[serde(rename = "QUOTE")]
    pub quote: String,
    #[serde(rename = "BASE_ID")]
    pub base_id: i32,
    #[serde(rename = "QUOTE_ID")]
    pub quote_id: i32,
    #[serde(rename = "TRANSFORM_FUNCTION")]
    pub transform_function: String,
    #[serde(rename = "OPEN")]
    pub open: f64,
    #[serde(rename = "HIGH")]
    pub high: f64,
    #[serde(rename = "LOW")]
    pub low: f64,
    #[serde(rename = "CLOSE")]
    pub close: f64,
    #[serde(rename = "FIRST_SWAP_TIMESTAMP")]
    pub first_swap_timestamp: i64,
    #[serde(rename = "FIRST_SWAP_BLOCK")]
    pub first_swap_block: i64,
    #[serde(rename = "LAST_SWAP_TIMESTAMP")]
    pub last_swap_timestamp: i64,
    #[serde(rename = "LAST_SWAP_BLOCK")]
    pub last_swap_block: i64,
    #[serde(rename = "FIRST_SWAP_PRICE")]
    pub first_swap_price: f64,
    #[serde(rename = "HIGH_SWAP_PRICE")]
    pub high_swap_price: f64,
    #[serde(rename = "HIGH_SWAP_TIMESTAMP")]
    pub high_swap_timestamp: i64,
    #[serde(rename = "HIGH_SWAP_BLOCK")]
    pub high_swap_block: i64,
    #[serde(rename = "LOW_SWAP_PRICE")]
    pub low_swap_price: f64,
    #[serde(rename = "LOW_SWAP_TIMESTAMP")]
    pub low_swap_timestamp: i64,
    #[serde(rename = "LOW_SWAP_BLOCK")]
    pub low_swap_block: i64,
    #[serde(rename = "LAST_SWAP_PRICE")]
    pub last_swap_price: f64,
    #[serde(rename = "TOTAL_SWAPS")]
    pub total_swaps: i64,
    #[serde(rename = "TOTAL_SWAPS_BUY")]
    pub total_swaps_buy: i64,
    #[serde(rename = "TOTAL_SWAPS_SELL")]
    pub total_swaps_sell: i64,
    #[serde(rename = "TOTAL_SWAPS_UNKNOWN")]
    pub total_swaps_unknown: i64,
    #[serde(rename = "VOLUME")]
    pub volume: f64,
    #[serde(rename = "QUOTE_VOLUME")]
    pub quote_volume: f64,
    #[serde(rename = "VOLUME_BUY")]
    pub volume_buy: f64,
    #[serde(rename = "QUOTE_VOLUME_BUY")]
    pub quote_volume_buy: f64,
    #[serde(rename = "VOLUME_SELL")]
    pub volume_sell: f64,
    #[serde(rename = "QUOTE_VOLUME_SELL")]
    pub quote_volume_sell: f64,
    #[serde(rename = "VOLUME_UNKNOWN")]
    pub volume_unknown: String,
    #[serde(rename = "QUOTE_VOLUME_UNKNOWN")]
    pub quote_volume_unknown: String,
}


// On-Chain DEX: Market


/// On-Chain DEX: Market
#[derive(Deserialize, Debug)]
pub struct CCOCDEXMarkets {
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "EXCHANGE_STATUS")]
    pub exchange_status: String,
    #[serde(rename = "MAPPED_INSTRUMENTS_TOTAL")]
    pub mapped_instrument_total: i64,
    #[serde(rename = "UNMAPPED_INSTRUMENTS_TOTAL")]
    pub unmapped_instruments_total: i64,
    #[serde(rename = "INSTRUMENT_STATUS")]
    pub instrument_status: CCInstrumentStatus,
    #[serde(rename = "TOTAL_AMM_SWAPS_ONCHAIN")]
    pub total_amm_swaps_onchain: i64,
    #[serde(rename = "TOTAL_AMM_LIQUIDITY_UPDATES_ONCHAIN")]
    pub total_amm_liquidity_updates_onchain: i64,
    #[serde(rename = "HAS_ORDERBOOK_L2_MINUTE_SNAPSHOTS_ENABLED")]
    pub has_orderbook_l2_minute_snapshots_enabled: bool,
}