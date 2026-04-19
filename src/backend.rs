use std::{env::var, collections::HashMap};
use dotenv::dotenv;
use crate::error::Error;
use crate::{Unit, APIEndpoint};
use crate::schemas::{self as sh, CoinDeskResponse};
use crate::schemas::min_api;
use crate::schemas::data_api::indices_and_reference_rates::{IndicesMarket, IndicesOHLCV};
use crate::schemas::data_api::spot::{SpotMarket, SpotInstrumentStatus, SpotOHLCV, SpotInstrumentMetdata, SpotMarkets,
                                     SpotMarketsInstruments};
use crate::schemas::data_api::futures::{FuturesMarket, FuturesOHLCV, FuturesInstrumentMetadata, FuturesMarkets};
use crate::schemas::data_api::options::{OptionsMarket, OptionsOHLCV, OptionsInstrumentMetadata, OptionsMarkets};
use crate::schemas::data_api::derivatives_indices::{DerIndicesMarket, DerIndicesOHLCV, DerIndicesMarkets};
use crate::schemas::data_api::on_chain_dex::{OCDEXMarket, OCDEXOHLCV, OCDEXMarkets};
use crate::schemas::data_api::on_chain_core::{OCCoreETHBlock, OCCoreAssetByChain, OCCoreAssetByAddress, OCCoreSupply};
use crate::schemas::data_api::asset::{AssetMetadata, AssetEvent, AssetCodeRepoMetrics, AssetDiscord, AssetReddit, AssetTelegram, AssetTwitter};
use crate::schemas::data_api::news::{NewsStatus, NewsLang, NewsSourceID, NewsLatestArticle, NewsSourceType, NewsSource, NewsCategory};
use crate::schemas::data_api::overview::OverviewMktCapOHLCV;
use crate::utils::{Param, call_api_endpoint};


/// API data collection backend.
pub struct CoinDesk {
    /// CoinDesk API key
    api_key: Option<String>,
}

impl CoinDesk {
    /// Creates a new backend for data collection.
    pub fn new() -> Self {
        Self { api_key: None }
    }

    /// Returns the refernce to the defined API key.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ccdata_api::CoinDesk;
    ///
    /// let mut backend: CoinDesk = CoinDesk::new();
    /// // Provide API key as the environment variable called API_KEY
    /// backend.build(&"API_KEY").unwrap();
    ///
    /// println!("{}", backend.api_key().unwrap());
    /// ```
    pub fn api_key(&self) -> Result<&str, Error> {
        match &self.api_key {
            Some(v) => Ok(v),
            None => Err(Error::NoAPIKey),
        }
    }

    /// Updates the API key.
    ///
    /// # Input
    /// - `new_api_key`: New API key that will be used by the backend to send requests to CoinDesk API endpoints
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ccdata_api::CoinDesk;
    ///
    /// let mut backend: CoinDesk = CoinDesk::new();
    ///
    /// let new_api_key: String = String::from("xxxxxxx");
    /// backend.update_api_key(new_api_key);
    ///
    /// assert_eq!(backend.api_key().unwrap(), "xxxxxxx");
    /// ```
    pub fn update_api_key(&mut self, new_api_key: String) -> () {
        self.api_key = Some(new_api_key);
    }

    /// Initiates the API data collection backend with the API key stored in the environment variable.
    ///
    /// # Input
    /// -`api_key_env_var`: Name of the environment variable in the local `.env` file that stores the CoinDesk API key
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ccdata_api::CoinDesk;
    ///
    /// let mut backend: CoinDesk = CoinDesk::new();
    /// // Provide API key as the environment variable called API_KEY
    /// backend.build(&"API_KEY").unwrap();
    ///
    /// println!("{}", backend.api_key().unwrap());
    /// ```
    pub fn build(&mut self, api_key_env_var: &str) -> Result<(), Error> {
        dotenv()?;
        Ok(self.update_api_key(var(api_key_env_var)?))
    }

    #[deprecated(since="1.0.6", note="Deprecated by CoinDesk")]
    /// # Balance Distribution Daily (Blockchain Data)
    /// Returns the daily balance distribution history for Bitcoin.
    ///
    /// # Description (CoinDesk Documentation)
    /// Powered by IntoTheBlock, an intelligence company that leverages machine learning and advanced statistics to extract intelligent signals for crypto-assets.
    /// 
    /// Retrieves the balance distribution for a specified asset over a specified time range at a daily interval.
    /// Only data for BTC (Bitcoin) is currently available.
    ///
    /// You can only use this endpoint with a valid api_key. Retrieve the daily wallet balance distribution data for the requested coin,
    /// back through time for the number of points as specifed by the limit. Timestamp values are based on 00:00 GMT time.
    ///
    /// # Input
    /// - `to_timestamp`: Final timestamp up to which the data will be extracted
    /// - `limit`: Maximum number of datapoints per API endpoint call
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::CoinDesk;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let limit: usize = 2000;
    ///     let balance_distribution = backend.get_balance_distribution(None, Some(limit)).await.unwrap();
    ///     assert!(balance_distribution.data.unwrap().data.unwrap().len() <= limit);
    ///
    /// }
    /// ```
    pub async fn get_balance_distribution(&self, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<sh::CCMinResponse<sh::CCMinWrapper<Vec<min_api::BalanceDistribution>>>, Error> {
        call_api_endpoint::<sh::CCMinResponse<sh::CCMinWrapper<Vec<min_api::BalanceDistribution>>>>(
            self.api_key()?,
            APIEndpoint::BalanceDistribution, Unit::NA,
            vec![Param::Symbol { v: "BTC", }, Param::Limit { v: limit, }, Param::ToTs { v: to_timestamp, }],
            None
        ).await
    }

    /// # Historical OHLCV+ \[Day, Hour, Minute\] (Indices & Ref. Rates)
    /// Returns historical OHLCV data for a given instrument.
    ///
    /// # Description (CoinDesk Documentation)
    /// This endpoint is meticulously designed to provide historical candlestick data for various indices, captured at one-day intervals.
    /// The data encompasses crucial metrics such as OPEN, HIGH, LOW, CLOSE, VOLUME and additional trading-derived values (OHLCV+),
    /// offering a comprehensive view of an index's historical performance. This information is essential for conducting in-depth market analyses and making
    /// informed decisions based on past trends.
    ///
    /// # Input
    /// - `instument`: Instrument symbol
    /// - `to_timestamp`: Final timestamp up to which the data will be extracted
    /// - `limit`: Maximum number of datapoints per API endpoint call
    /// - `market`: Market name
    /// - `unit`: Unit of the interval between successive data points
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::{CoinDesk, Unit, IndicesMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let market: IndicesMarket = IndicesMarket::CADLI;
    ///     let limit: usize = 2000;
    ///     let ohlcv = backend.get_indices_ohlcv("BTC-USD", None, Some(limit), market, Unit::Day).await.unwrap();
    ///     assert_eq!(ohlcv.data.unwrap().len(), limit);
    ///
    /// }
    /// ```
    pub async fn get_indices_ohlcv(&self, instrument: &str, to_timestamp: Option<i64>, limit: Option<usize>, market: IndicesMarket, unit: Unit) -> Result<CoinDeskResponse<Vec<IndicesOHLCV>>, Error> {
        call_api_endpoint::<CoinDeskResponse<Vec<IndicesOHLCV>>>(
            self.api_key()?,
            APIEndpoint::IndicesOHLCV, unit,
            vec![Param::Instrument { v: instrument, }, Param::Limit { v: limit, }, Param::ToTimestamp { v: to_timestamp, }, Param::Market { v: market.to_string(), }],
            None
        ).await
    }

    /// # Historical OHLCV+ \[Day, Hour, Minute\] (Spot)
    /// Returns historical OHLCV data for a given instrument.
    ///
    /// # Description (CoinDesk Documentation)
    /// This endpoint delivers daily aggregated candlestick data for specific cryptocurrency instruments across selected exchanges.
    /// It offers vital trading metrics, including open, high, low, close (OHLC) prices, and trading volumes, both in base and quote currencies.
    /// This data is key for understanding historical price movements and market behavior, allowing for detailed analysis of trading patterns and trends over time.
    ///
    /// # Input
    /// - `instument`: Instrument symbol
    /// - `to_timestamp`: Final timestamp up to which the data will be extracted
    /// - `limit`: Maximum number of datapoints per API endpoint call
    /// - `market`: Market name
    /// - `unit`: Unit of the interval between successive data points
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::{CoinDesk, Unit, SpotMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let market: SpotMarket = SpotMarket::KRAKEN;
    ///     let limit: usize = 2000;
    ///     let ohlcv = backend.get_spot_ohlcv("BTC-USD", None, Some(limit), market, Unit::Day).await.unwrap();
    ///     assert_eq!(ohlcv.data.unwrap().len(), limit);
    ///
    /// }
    /// ```
    pub async fn get_spot_ohlcv(&self, instrument: &str, to_timestamp: Option<i64>, limit: Option<usize>, market: SpotMarket, unit: Unit) -> Result<CoinDeskResponse<Vec<SpotOHLCV>>, Error> {
        call_api_endpoint::<CoinDeskResponse<Vec<SpotOHLCV>>>(
            self.api_key()?,
            APIEndpoint::SpotOHLCV, unit,
            vec![Param::Instrument { v: instrument, }, Param::Limit { v: limit, }, Param::ToTimestamp { v: to_timestamp, }, Param::Market { v: market.to_string(), }],
            None
        ).await
    }

    /// # Instrument Metadata (Spot)
    /// Returns metadata for a given instrument.
    ///
    /// # Description (CoinDesk Documentation)
    /// This endpoint, specific to the Spot segment of the API, delivers vital metadata about financial instruments traded on specified exchanges,
    /// focusing solely on non-price related information. This endpoint is crucial for internal use, offering a comprehensive dataset that includes mappings,
    /// operational statuses, and historical data (first seen/last seen timestamps) about each instrument.
    ///
    /// # Input
    /// - `instruments`: List of instrument symbols
    /// - `market`: Market name
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::{CoinDesk, SpotMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let instruments: Vec<String> = vec![String::from("BTC-USD"), String::from("ETH-USD")];
    ///     let market: SpotMarket = SpotMarket::KRAKEN;
    ///     let instrument_metadata = backend.get_spot_instrument_metadata(&instruments, market).await.unwrap();
    ///     assert_eq!(instrument_metadata.data.unwrap().len(), 2);
    ///
    /// }
    /// ```
    pub async fn get_spot_instrument_metadata(&self, instruments: &Vec<String>, market: SpotMarket) -> Result<CoinDeskResponse<HashMap<String, SpotInstrumentMetdata>>, Error> {
        call_api_endpoint::<CoinDeskResponse<HashMap<String, SpotInstrumentMetdata>>>(
            self.api_key()?,
            APIEndpoint::SpotInstrumentMetadata, Unit::NA,
            vec![Param::Instruments { v: instruments, }, Param::Market { v: market.to_string(), }],
            None
        ).await
    }

    /// # Markets (Spot) - V2
    /// Returns metadata about a given market.
    /// 
    /// Description (CoinDesk Documentation)
    /// This endpoint provides comprehensive information about various cryptocurrency spot markets,
    /// featuring extensive exchange metadata and operational details. By specifying a markets list through the "markets" parameter,
    /// users can retrieve details about specific markets, such as its trading pairs, volume, operational status,
    /// and comprehensive static metadata including exchange status, launch dates, supported trading types, orderbook/trade integration status,
    /// benchmark scores, and resource links. If no specific markets are indicated, the endpoint delivers data on all available markets.
    /// 
    /// # Input
    /// - `markets`: Markets' names
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use ccdata_api::{CoinDesk, SpotMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let markets: Vec<SpotMarket> = vec![SpotMarket::KRAKEN];
    ///     let markets = backend.get_spot_markets_v2(markets).await.unwrap();
    ///     assert_eq!(markets.data.unwrap().get("kraken").unwrap().exchange_status, String::from("ACTIVE"));
    ///
    /// }
    /// ```
    pub async fn get_spot_markets_v2(&self, markets: Vec<SpotMarket>) -> Result<CoinDeskResponse<HashMap<String, SpotMarkets>>, Error> {
        let v: Vec<String> = markets.iter().map(|v| v.to_string() ).collect();
        call_api_endpoint::<CoinDeskResponse<HashMap<String, SpotMarkets>>>(
            self.api_key()?,
            APIEndpoint::SpotMarketsV2, Unit::NA,
            vec![Param::Markets { v, }],
            None
        ).await
    }

    /// # Markets + Instruments \[Mapped\] (Spot)
    /// Returns a map of given instruments across a market.
    ///
    /// # Description (CoinDesk Documentation)
    /// This endpoint retrieves a comprehensive dictionary of mapped instruments across one or more spot markets, filtered by a specified state or status.
    /// Each entry in the dictionary uses the instrument ID—standardized by the mapping team—as the key, ensuring consistency and ease of reference.
    /// This endpoint is particularly valuable for users needing precise and standardized information on trading instruments, facilitating the tracking,
    // comparison, and analysis of different instruments within and across various spot markets. It is ideal for applications that depend on uniform identifiers
    /// to integrate and interpret market data effectively.
    ///
    /// # Input
    /// - `instruments`: List of instrument symbols
    /// - `market`: Market name
    /// - `instrument_status`: Status of the instrument (e.g., `ACTIVE`, `EXPIRED`)
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::{CoinDesk, SpotMarket, SpotInstrumentStatus};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let instruments: Vec<String> = vec![String::from("BTC-USD"), String::from("ETH-USD")];
    ///     let market: SpotMarket = SpotMarket::KRAKEN;
    ///     let instrument_status: SpotInstrumentStatus = SpotInstrumentStatus::ACTIVE;
    ///     let markets_instruments = backend.get_spot_markets_instruments(&instruments, market, instrument_status).await.unwrap();
    ///     assert_eq!(markets_instruments.data.unwrap().get("kraken").unwrap().instruments.len(), 2);
    ///
    /// }
    /// ```
    pub async fn get_spot_markets_instruments(&self, instruments: &Vec<String>, market: SpotMarket, instrument_status: SpotInstrumentStatus) -> Result<CoinDeskResponse<HashMap<String, SpotMarketsInstruments>>, Error> {
        call_api_endpoint::<CoinDeskResponse<HashMap<String, SpotMarketsInstruments>>>(
            self.api_key()?,
            APIEndpoint::SpotMarketsInstruments, Unit::NA,
            vec![Param::Instruments { v: instruments, }, Param::Market { v: market.to_string(), }, Param::InstrumentStatus { v: instrument_status, }],
            None
        ).await
    }

    /// # Historical OHLCV+ \[Day, Hour, Minute\] (Futures)
    /// Returns historical OHLCV data for a given instrument.
    ///
    /// # Description (CoinDesk Documentation)
    /// This endpoint offers daily aggregated candlestick data for specific futures instruments on designated exchanges.
    /// It provides crucial trading data points such as open, high, low, close prices (OHLC), and volumes, vital for traders and analysts aiming to
    /// understand historical price movements and market behavior over specific periods. The flexibility of this endpoint is enhanced by supporting a range
    /// of parameters to tailor the data retrieval to specific needs, such as market selection, instrument details, and aggregation customization.
    /// This makes it a highly adaptable tool for historical data analysis in the context of futures markets.
    ///
    /// # Input
    /// - `instrument`: Instrument symbol
    /// - `to_timestamp`: Final timestamp up to which the data will be extracted
    /// - `limit`: Maximum number of datapoints per API endpoint call
    /// - `market`: Market name
    /// - `unit`: Unit of the interval between successive data points
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::{CoinDesk, Unit, FuturesMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let market: FuturesMarket = FuturesMarket::BINANCE;
    ///     let limit: usize = 2000;
    ///     let ohlcv = backend.get_futures_ohlcv("BTC-USDT-VANILLA-PERPETUAL", None, Some(limit), market, Unit::Day).await.unwrap();
    ///     assert!(ohlcv.data.unwrap().len() <= limit)
    ///
    /// }
    /// ```
    pub async fn get_futures_ohlcv(&self, instrument: &str, to_timestamp: Option<i64>, limit: Option<usize>, market: FuturesMarket, unit: Unit) -> Result<CoinDeskResponse<Vec<FuturesOHLCV>>, Error> {
        call_api_endpoint::<CoinDeskResponse<Vec<FuturesOHLCV>>>(
            self.api_key()?,
            APIEndpoint::FuturesOHLCV, unit,
            vec![Param::Instrument { v: instrument, }, Param::Limit { v: limit, }, Param::ToTimestamp { v: to_timestamp, }, Param::Market { v: market.to_string(), }],
            None
        ).await
    }

    /// # Instrument Metadata (Futures)
    /// Returns metadata for a given instrument.
    /// 
    /// # Description (CoinDesk Documentation)
    /// This endpoint is specifically tailored for the Futures segment of the API, providing essential metadata about futures instruments
    /// traded on various exchanges. It delivers critical non-price related information, including mappings, operational statuses,
    /// and historical data points such as the first and last seen timestamps for each instrument. Unlike the Markets + Instruments endpoint,
    /// which offers a streamlined subset of data, the Futures Instrument Metadata endpoint is designed for comprehensive internal analysis
    /// and integration, ensuring that organizations have detailed insights necessary for effective management and evaluation of futures
    /// trading instruments.
    /// 
    /// # Input
    /// - `instruments`: List of instrument symbols
    /// - `market`: Market name
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// 
    /// use ccdata_api::{CoinDesk, FuturesMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let instruments: Vec<String> = vec![String::from("BTCUSD_PERP"), String::from("ETH-USDT-VANILLA-PERPETUAL")];
    ///     let market: FuturesMarket = FuturesMarket::BINANCE;
    ///     let futures_metadata = backend.get_futures_instrument_metadata(&instruments, market).await.unwrap();
    ///     assert_eq!(futures_metadata.data.unwrap().len(), 2);
    ///
    /// }
    /// ```
    pub async fn get_futures_instrument_metadata(&self, instruments: &Vec<String>, market: FuturesMarket) -> Result<CoinDeskResponse<HashMap<String, FuturesInstrumentMetadata>>, Error> {
        call_api_endpoint::<CoinDeskResponse<HashMap<String, FuturesInstrumentMetadata>>>(
            self.api_key()?,
            APIEndpoint::FuturesInstrumentMetadata, Unit::NA,
            vec![Param::Instruments { v: instruments, }, Param::Market { v: market.to_string(), }],
            None
        ).await
    }

    /// # Markets (Futures)
    /// Returns metadata about a given market.
    ///
    /// # Description (CoinDesk Documentation)
    /// This endpoint provides comprehensive information about various cryptocurrency futures markets,
    /// featuring extensive exchange metadata and derivatives-specific operational details.
    /// By specifying a markets through the "markets" parameter, users can retrieve details about specific futures markets,
    /// such as their available contracts, leverage options, margin requirements, settlement mechanisms, operational status,
    /// and comprehensive static metadata including exchange status, launch dates, supported contract types, orderbook/trade integration status,
    /// benchmark scores, and resource links. If no specific markets are indicated, the endpoint delivers data on all available futures markets.
    ///
    /// # Input
    /// - `markets`: Markets' names
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::{CoinDesk, FuturesMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let markets: Vec<FuturesMarket> = vec![FuturesMarket::BINANCE];
    ///     let markets = backend.get_futures_markets_v2(markets).await.unwrap();
    ///     assert_eq!(markets.data.unwrap().get("binance").unwrap().exchange_status, String::from("ACTIVE"));
    ///
    /// }
    /// ```
    pub async fn get_futures_markets_v2(&self, markets: Vec<FuturesMarket>) -> Result<CoinDeskResponse<HashMap<String, FuturesMarkets>>, Error> {
        let v: Vec<String> = markets.iter().map(|v| v.to_string() ).collect();
        call_api_endpoint::<CoinDeskResponse<HashMap<String, FuturesMarkets>>>(
            self.api_key()?,
            APIEndpoint::FuturesMarketsV2, Unit::NA,
            vec![Param::Markets { v, }],
            None
        ).await
    }  

    /// # Historical OHLCV+ \[Day, Hour, Minute\] (Options)
    /// Returns historical OHLCV data for a given instrument.
    ///
    /// # Description (CoinDesk Documentation)
    /// The Options Historical OHLCV+ Day endpoint provides historical OHLCV (open, high, low, close, volume) data for specified options instruments
    /// on a chosen exchange, aggregated on a daily basis. This API endpoint delivers comprehensive historical data, enabling users to perform detailed
    /// analysis of options market performance over time. It is essential for long-term market analysis, backtesting strategies, and informed
    /// decision-making based on historical trends.
    ///
    /// # Input
    /// - `instrument`: Instrument symbol
    /// - `to_timestamp`: Final timestamp up to which the data will be extracted
    /// - `limit`: Maximum number of datapoints per API endpoint call
    /// - `market`: Market name
    /// - `unit`: Unit of the interval between successive data points
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::{CoinDesk, Unit, OptionsMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let market: OptionsMarket = OptionsMarket::OKEX;
    ///     let limit: usize = 2000;
    ///     let ohlcv = backend.get_options_ohlcv("BTC-USD-20241227-15000-P", None, Some(limit), market, Unit::Day).await.unwrap();
    ///     assert!(ohlcv.data.unwrap().len() <= limit);
    ///
    /// }
    /// ```
    pub async fn get_options_ohlcv(&self, instrument: &str, to_timestamp: Option<i64>, limit: Option<usize>, market: OptionsMarket, unit: Unit) -> Result<CoinDeskResponse<Vec<OptionsOHLCV>>, Error> {
        call_api_endpoint::<CoinDeskResponse<Vec<OptionsOHLCV>>>(
            self.api_key()?,
            APIEndpoint::OptionsOHLCV, unit,
            vec![Param::Instrument { v: instrument, }, Param::Limit { v: limit, }, Param::ToTimestamp { v: to_timestamp, }, Param::Market { v: market.to_string(), }],
            None
        ).await
    }

    /// # Instrument Metadata (Options)
    /// Returns metadata for a given instrument.
    /// 
    /// # Description
    /// The Options Instrument Metadata endpoint provides detailed metadata about options instruments across various exchanges.
    /// This API endpoint delivers extensive information, including mappings, operational statuses, underlying assets, expiration dates,
    /// strike prices, and other relevant data. Essential for users who need comprehensive and standardized details on options instruments,
    /// it supports accurate tracking, analysis, and integration of options data within financial systems.
    /// 
    /// # Input
    /// - `instruments`: List of instrument symbols
    /// - `market`: Market name
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// 
    /// use ccdata_api::{CoinDesk, OptionsMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let instruments: Vec<String> = vec![String::from("BTC-29NOV24-25000-P"), String::from("ETH-31JAN25-2500-P")];
    ///     let market: OptionsMarket = OptionsMarket::DERIBIT;
    ///     let options_metadata = backend.get_options_instrument_metadata(&instruments, market).await.unwrap();
    ///     assert_eq!(options_metadata.data.unwrap().len(), 2);
    ///
    /// }
    /// ```
    pub async fn get_options_instrument_metadata(&self, instruments: &Vec<String>, market: OptionsMarket) -> Result<CoinDeskResponse<HashMap<String, OptionsInstrumentMetadata>>, Error> {
        call_api_endpoint::<CoinDeskResponse<HashMap<String, OptionsInstrumentMetadata>>>(
            self.api_key()?,
            APIEndpoint::OptionsInstrumentMetadata, Unit::NA,
            vec![Param::Instruments { v: instruments, }, Param::Market { v: market.to_string(), }],
            None
        ).await
    }

    /// # Markets (Options)
    /// Returns metadata about a given market.
    ///
    /// # Description (CoinDesk Documentation)
    /// This endpoint provides comprehensive information about various cryptocurrency options markets,
    /// featuring extensive exchange metadata and derivatives-specific operational details.
    /// By specifying markets through the "markets" parameter, users can retrieve details about specific options markets,
    /// such as their available option chains, strike price intervals, expiration schedules, settlement mechanisms,
    /// implied volatility calculations, operational status, and comprehensive static metadata including exchange status, launch dates,
    /// supported option types, orderbook/trade integration status, benchmark scores, and resource links. If no specific markets are indicated,
    /// the endpoint delivers data on all available options markets.
    ///
    /// # Input
    /// - `markets`: Markets' names
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::{CoinDesk, OptionsMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let markets: Vec<OptionsMarket> = vec![OptionsMarket::DERIBIT];
    ///     let markets = backend.get_options_markets_v2(markets).await.unwrap();
    ///     assert_eq!(markets.data.unwrap().get("deribit").unwrap().exchange_status, String::from("ACTIVE"));
    ///
    /// }
    /// ```
    pub async fn get_options_markets_v2(&self, markets: Vec<OptionsMarket>) -> Result<CoinDeskResponse<HashMap<String, OptionsMarkets>>, Error> {
        let v: Vec<String> = markets.iter().map(|v| v.to_string() ).collect();
        call_api_endpoint::<CoinDeskResponse<HashMap<String, OptionsMarkets>>>(
            self.api_key()?,
            APIEndpoint::OptionsMarketsV2, Unit::NA,
            vec![Param::Markets { v, }],
            None
        ).await
    }

    /// # Historical OHLCV+ \[Day, Hour, Minute\] (Derivatives Indices)
    /// Returns historical OHLCV data for a given instrument.
    ///
    /// # Description (CoinDesk Documentation)
    /// The Derivatives Index Historical OHLC+ Day endpoint provides historical OHLC (open, high, low, close) data for specified index instruments
    /// on a selected market. This API endpoint delivers daily aggregated index metrics, offering a comprehensive view of historical performance.
    /// Ideal for long-term analysis, it supports thorough market research and historical data examination, essential for informed decision-making
    /// and strategic planning.
    ///
    /// # Input
    /// - `instrument`: Instrument symbol
    /// - `to_timestamp`: Final timestamp up to which the data will be extracted
    /// - `limit`: Maximum number of datapoints per API endpoint call
    /// - `market`: Market name
    /// - `unit`: Unit of the interval between successive data points
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::{CoinDesk, Unit, DerIndicesMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let market: DerIndicesMarket = DerIndicesMarket::BINANCE;
    ///     let limit: usize = 2000;
    ///     let ohlcv = backend.get_der_indices_ohlcv("BTCUSDT", None, Some(limit), market, Unit::Day).await.unwrap();
    ///     assert!(ohlcv.data.unwrap().len() <= limit);
    ///
    /// }
    /// ```
    pub async fn get_der_indices_ohlcv(&self, instrument: &str, to_timestamp: Option<i64>, limit: Option<usize>, market: DerIndicesMarket, unit: Unit) -> Result<CoinDeskResponse<Vec<DerIndicesOHLCV>>, Error> {
        call_api_endpoint::<CoinDeskResponse<Vec<DerIndicesOHLCV>>>(
            self.api_key()?,
            APIEndpoint::DerIndicesOHLCV, unit,
            vec![Param::Instrument { v: instrument, }, Param::Limit { v: limit, }, Param::ToTimestamp { v: to_timestamp, }, Param::Market { v: market.to_string(), }],
            None
        ).await
    }

    /// # Markets (Derivatives Indices)
    /// Returns metadata about a given market.
    ///
    /// # Description (CoinDesk Documentation)
    /// This endpoint provides comprehensive information about various derivatives index markets,
    /// featuring extensive exchange metadata and index-specific operational details. By specifying a markets through the "markets" parameter,
    /// users can retrieve details about specific derivatives index markets, such as its available index instruments, volume metrics,
    /// operational status, benchmark scores, integration configurations, and comprehensive static metadata including exchange status,
    /// launch dates, supported index types, data polling/streaming capabilities, and resource links. If no specific markets are indicated,
    /// the endpoint delivers data on all available derivatives index markets.
    ///
    /// # Input
    /// - `markets`: Markets' names
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::{CoinDesk, DerIndicesMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let markets: Vec<DerIndicesMarket> = vec![DerIndicesMarket::KRAKEN];
    ///     let markets = backend.get_der_indices_markets_v2(markets).await.unwrap();
    ///     assert_eq!(markets.data.unwrap().get("kraken").unwrap().exchange_status, String::from("ACTIVE"));
    ///
    /// }
    /// ```
    pub async fn get_der_indices_markets_v2(&self, markets: Vec<DerIndicesMarket>) -> Result<CoinDeskResponse<HashMap<String, DerIndicesMarkets>>, Error> {
        let v: Vec<String> = markets.iter().map(|v| v.to_string() ).collect();
        call_api_endpoint::<CoinDeskResponse<HashMap<String, DerIndicesMarkets>>>(
            self.api_key()?,
            APIEndpoint::DerIndicesMarketsV2, Unit::NA,
            vec![Param::Markets { v, }],
            None
        ).await
    }

    /// # Historical OHLCV+ (Swap) \[Day, Hour, Minute\] (On-Chain DEX)
    /// Returns historical OHLCV data for a given instrument.
    ///
    /// # Description (CoinDesk Documentation)
    /// The On-Chain AMM Historical OHLCV+ (Swap) Day endpoint retrieves daily aggregated candlestick data for AMM swap transactions.
    /// This data includes open, high, low, and close prices (OHLC), as well as trading volumes in both base and quote currencies for a selected instrument
    /// on a specified exchange. This endpoint is essential for traders and analysts looking to understand historical price movements and market behavior
    /// over specific periods.
    ///
    /// # Input
    /// - `instrument`: Instrument symbol
    /// - `to_timestamp`: Final timestamp up to which the data will be extracted
    /// - `limit`: Maximum number of datapoints per API endpoint call
    /// - `market`: Market name
    /// - `unit`: Unit of the interval between successive data points
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::{CoinDesk, Unit, OCDEXMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let market: OCDEXMarket = OCDEXMarket::UNISWAPV2;
    ///     let limit: usize = 2000;
    ///     let ohlcv = backend.get_ocdex_ohlcv("0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852_2", None, Some(limit), market, Unit::Day).await.unwrap();
    ///     assert!(ohlcv.data.unwrap().len() <= limit);
    ///
    /// }
    /// ```
    pub async fn get_ocdex_ohlcv(&self, instrument: &str, to_timestamp: Option<i64>, limit: Option<usize>, market: OCDEXMarket, unit: Unit) -> Result<CoinDeskResponse<Vec<OCDEXOHLCV>>, Error> {
        call_api_endpoint::<CoinDeskResponse<Vec<OCDEXOHLCV>>>(
            self.api_key()?,
            APIEndpoint::OCDEXOHLCV, unit,
            vec![Param::Instrument { v: instrument, }, Param::Limit { v: limit, }, Param::ToTimestamp { v: to_timestamp, }, Param::Market { v: market.to_string(), }],
            None
        ).await
    }

    /// # Markets (On-Chain DEX)
    /// Returns metadata about a given market.
    ///
    /// # Description (CoinDesk Documentation)
    /// This endpoint provides comprehensive information about various on-chain decentralized exchange markets, 
    ///featuring extensive exchange metadata and DEX-specific operational details. By specifying a market through the "markets" parameter,
    /// users can retrieve details about specific on-chain markets, such as its supported blockchain networks, smart contract addresses,
    /// protocol type (AMM, order book, hybrid), governance structure, launch dates, audit information, operational status,
    /// and comprehensive static metadata including exchange status, supported token standards, integration configurations, benchmark scores,
    /// and resource links. If no specific markets are indicated, the endpoint delivers data on all available on-chain markets.
    ///
    /// # Input
    /// - `markets`: Markets' names
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::{CoinDesk, OCDEXMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let markets: Vec<OCDEXMarket> = vec![OCDEXMarket::UNISWAPV2];
    ///     let markets = backend.get_ocdex_markets_v2(markets).await.unwrap();
    ///     assert_eq!(markets.data.unwrap().get("uniswapv2").unwrap().exchange_status, String::from("ACTIVE"));
    ///
    /// }
    /// ```
    pub async fn get_ocdex_markets_v2(&self, markets: Vec<OCDEXMarket>) -> Result<CoinDeskResponse<HashMap<String, OCDEXMarkets>>, Error> {
        let v: Vec<String> = markets.iter().map(|v| v.to_string() ).collect();
        call_api_endpoint::<CoinDeskResponse<HashMap<String, OCDEXMarkets>>>(
            self.api_key()?,
            APIEndpoint::OCDEXMarketsV2, Unit::NA,
            vec![Param::Markets { v, }],
            None
        ).await
    }

    /// # ETH Blocks \[Full Processed\] (On-Chain Core)
    /// Returns the processed data for a given block.
    ///
    /// # Description (CoinDesk Documentation)
    /// The On-Chain ETH: Block Full Processed endpoint delivers exhaustive details on a specific Ethereum block in a meticulously processed format,
    /// complete with detailed explanations for each field. This endpoint is crucial for developers, researchers, and users seeking the most in-depth
    /// and intelligible data concerning an Ethereum block. By utilizing the 'groups' parameter, users can unlock the full spectrum of data,
    /// including general block information as well as detailed transaction data. This encompasses logs, traces, and blobs, offering a holistic view
    /// of the block's activities. Access this wealth of information through the URL structure: /onchain/{version}/block/{asset_id}.
    /// Passing -1 or leaving the block_number parameter empty will retrieve the latest available block. Alternatively, using a negative value for block_number
    /// will return the data relative to the block that is currently being produce: -1 for the latest available block, -2 for the block before that, and so on,
    /// allowing users to access past blocks with ease.
    ///
    /// # Input
    /// - `block_number`: Block number on the blockchain
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::CoinDesk;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let eth_block = backend.get_occore_eth_block(19501436).await.unwrap();
    ///     assert_eq!(eth_block.data.unwrap().symbol, String::from("ETH"));;
    ///
    /// }
    /// ```
    pub async fn get_occore_eth_block(&self, block_number: i64) -> Result<CoinDeskResponse<OCCoreETHBlock>, Error> {
        call_api_endpoint::<CoinDeskResponse<OCCoreETHBlock>>(
            self.api_key()?,
            APIEndpoint::OCCoreETHBlocks, Unit::NA,
            vec![Param::OCCoreBlockNumber { v: block_number, }],
            None
        ).await
    }

    /// # Assets Summary By Chain (On-Chain Core)
    /// Returns a summary of assets on a given chain.
    ///
    /// # Description (CoinDesk Documentation)
    /// The On-Chain Assets Summary By Chain endpoint retrieves a comprehensive summary of chain asset information for a specified blockchain,
    /// identified by its chain symbol. This endpoint provides detailed summaries of the supported assets on the given chain,
    /// including a complete list of all assets available. It is invaluable for users needing an overarching view of the asset landscape within a specific
    /// blockchain, offering insights into the diversity and characteristics of the assets supported by that chain.
    ///
    /// # Input
    /// - `chain_asset`: Chain asset symbol
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::CoinDesk;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let assets_by_chain = backend.get_occore_assets_by_chain("ETH").await.unwrap();
    ///     assert_eq!(assets_by_chain.data.unwrap().chain_asset_summary.symbol, String::from("ETH"));
    ///
    /// }
    /// ```
    pub async fn get_occore_assets_by_chain(&self, chain_asset: &str) -> Result<CoinDeskResponse<OCCoreAssetByChain>, Error> {
        call_api_endpoint::<CoinDeskResponse<OCCoreAssetByChain>>(
            self.api_key()?,
            APIEndpoint::OCCoreAssetsByChain, Unit::NA,
            vec![Param::ChainAsset { v: chain_asset, }],
            None
        ).await
    }

    /// # Asset By Address Lookup (On-Chain Core)
    /// Returns a summary of a smart contract on a given chain.
    ///
    /// # Description (CoinDesk Documentation)
    /// The On-Chain Asset By Address Lookup endpoint retrieves comprehensive asset information for a specific asset identified by its smart contract address
    /// and associated blockchain asset. This API endpoint is particularly useful for obtaining details about assets represented by tokens on various blockchain
    /// platforms. By specifying the smart contract address and blockchain asset, users can access detailed metadata about the asset, including its name, symbol,
    /// total supply, and other pertinent information. This endpoint is essential for developers, researchers, and users who need accurate and detailed asset
    /// information for blockchain-based applications, analysis, or integration.
    ///
    /// # Input
    /// - `chain_asset`: Chain asset symbol
    /// - `address`: Blockchain address
    /// - `quote_asset`: Asset to quote data in
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::CoinDesk;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let address: String = String::from("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
    ///     let quote_asset: String = String::from("USD");
    ///     let asset_by_address = backend.get_occore_asset_by_address("ETH", &address, &quote_asset).await.unwrap();
    ///     assert_eq!(asset_by_address.data.unwrap().parent_asset_symbol.unwrap(), String::from("ETH"));
    ///
    /// }
    /// ```
    pub async fn get_occore_asset_by_address(&self, chain_asset: &str, address: &str, quote_asset: &str) -> Result<CoinDeskResponse<OCCoreAssetByAddress>, Error> {
        call_api_endpoint::<CoinDeskResponse<OCCoreAssetByAddress>>(
            self.api_key()?,
            APIEndpoint::OCCoreAssetByAddress, Unit::NA,
            vec![Param::ChainAsset { v: chain_asset, }, Param::OCCoreAddress { v: address, }, Param::OCCoreQuoteAsset { v: quote_asset, }],
            None
        ).await
    }

    /// # Historical Supply Day (On-Chain Core)
    /// Returns supply history for a given asset.
    ///
    /// # Description (CoinDesk Documentation)
    /// The On-Chain Historical Supply Day endpoint retrieves comprehensive historical supply data for various digital assets identified by either their
    /// CoinDesk asset ID or unique asset symbol. This endpoint offers a detailed view of an asset's supply dynamics on a daily basis, providing insights
    /// into circulating supply, total issued supply, staked supply, burnt tokens, and more.
    ///
    /// # Input
    /// - `asset`: Asset symbol
    /// - `to_timestamp`: Final timestamp up to which the data will be extracted
    /// - `limit`: Maximum number of datapoints per API endpoint call
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::CoinDesk;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let limit: usize = 2000;
    ///     let historical_supply = backend.get_occore_supply("BTC", None, Some(limit)).await.unwrap();
    ///     assert!(historical_supply.data.unwrap().len() <= limit);
    ///
    /// }
    /// ```
    pub async fn get_occore_supply(&self, asset: &str, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<CoinDeskResponse<Vec<OCCoreSupply>>, Error> {
        call_api_endpoint::<CoinDeskResponse<Vec<OCCoreSupply>>>(
            self.api_key()?,
            APIEndpoint::OCCoreSupply, Unit::NA,
            vec![Param::Asset { v: asset, }, Param::ToTimestamp { v: to_timestamp, }, Param::Limit { v: limit, }],
            None
        ).await
    }

    /// # Full Asset Metadata (Asset) - V2
    /// 
    /// # Description (CoinDesk Documentation)
    /// The Full Asset Metadata endpoint returns an object that provides detailed and comprehensive information about multiple
    /// cryptocurrency assets in response to a request. Each asset can be identified by its CoinDesk asset ID, unique asset symbol,
    /// or asset URI, ensuring users receive a broad dataset covering all requested assets. This object consolidates extensive data related
    /// to asset description, classification, blockchain properties, social metrics, token sale information, and equity sale details for each
    /// asset—facilitating in-depth analysis, development, and research.
    /// 
    /// # Input
    /// - `assets`: List of asset symbols
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// 
    /// use ccdata_api::CoinDesk;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    /// 
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    /// 
    ///     let assets: Vec<String> = vec![String::from("ETH"), String::from("BTC")];
    ///     let metadata = backend.get_asset_metadata_v2(assets).await.unwrap();
    ///     assert_eq!(metadata.data.as_ref().unwrap().get("ETH").unwrap().name, String::from("Ethereum"));
    ///     assert_eq!(metadata.data.unwrap().get("BTC").unwrap().name, String::from("Bitcoin"));
    /// 
    /// }
    /// ```
    pub async fn get_asset_metadata_v2(&self, assets: Vec<String>) -> Result<CoinDeskResponse<HashMap<String, AssetMetadata>>, Error> {
        call_api_endpoint::<CoinDeskResponse<HashMap<String, AssetMetadata>>>(
            self.api_key()?,
            APIEndpoint::AssetMetadataV2, Unit::NA,
            vec![Param::Assets { v: assets, }],
            None
        ).await
    }

    /// # Significant Asset Events (Asset)
    /// Returns a list of significant events for a given asset.
    ///
    /// # Description (CoinDesk Documentation)
    /// The Asset Events endpoint retrieves an array of significant events related to digital assets, such as security incidents, rebrandings, blockchain forks,
    /// and other impactful developments. Events are returned in chronological order, with the most recent events appearing first.
    ///
    /// # Input
    /// - `asset`: Asset symbol
    /// - `to_timestamp`: Final timestamp up to which the data will be extracted
    /// - `limit`: Maximum number of datapoints per API endpoint call
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::CoinDesk;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let limit: usize = 100;
    ///     let events = backend.get_asset_events("ETH", None, Some(limit)).await.unwrap();
    ///     assert!(events.data.unwrap().len() <= limit);
    ///
    /// }
    /// ```
    pub async fn get_asset_events(&self, asset: &str, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<CoinDeskResponse<Vec<AssetEvent>>, Error> {
        call_api_endpoint::<CoinDeskResponse<Vec<AssetEvent>>>(
            self.api_key()?,
            APIEndpoint::AssetEvents, Unit::NA,
            vec![Param::Asset { v: asset, }, Param::ToTimestamp { v: to_timestamp, }, Param::Limit { v: limit, }],
            None
        ).await
    }

    /// # Historical Social \[Code Repository Day\] (Asset)
    /// Returns daily code repository metadata for a given asset.
    ///
    /// # Description (CoinDesk Documentation)
    /// The Historical Social Metrics Code Repository Day endpoint provides an in-depth, daily snapshot of a digital asset's code repositories.
    /// It is invaluable for gauging developer activity, community engagement, and the asset's overall health.
    ///
    /// # Input
    /// - `asset`: Asset symbol
    /// - `to_timestamp`: Final timestamp up to which the data will be extracted
    /// - `limit`: Maximum number of datapoints per API endpoint call
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::CoinDesk;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let limit: usize = 2000;
    ///     let code_repo = backend.get_asset_code_repo("ETH", None, Some(limit)).await.unwrap();
    ///     assert_eq!(code_repo.data.unwrap().len(), limit);
    ///
    /// }
    /// ```
    pub async fn get_asset_code_repo(&self, asset: &str, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<CoinDeskResponse<Vec<AssetCodeRepoMetrics>>, Error> {
        call_api_endpoint::<CoinDeskResponse<Vec<AssetCodeRepoMetrics>>>(
            self.api_key()?,
            APIEndpoint::AssetCodeRepo, Unit::NA,
            vec![Param::Asset { v: asset, }, Param::ToTimestamp{ v: to_timestamp, }, Param::Limit { v: limit, }],
            None
        ).await
    }

    /// # Historical Social \[Discord Day\] (Asset)
    /// Returns daily Discord metadata for a given asset.
    ///
    /// # Description (CoinDesk Documentation)
    /// The Historical Social Metrics Discord Days endpoint aggregates detailed daily metrics from all Discord servers related to a specific digital asset,
    /// offering a multifaceted view into community engagement and the asset's standing within Discord communities.
    ///
    /// # Input
    /// - `asset`: Asset symbol
    /// - `to_timestamp`: Final timestamp up to which the data will be extracted
    /// - `limit`: Maximum number of datapoints per API endpoint call
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::CoinDesk;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let limit: usize = 2000;
    ///     let discord = backend.get_asset_discord("ETH", None, Some(limit)).await.unwrap();
    ///     assert_eq!(discord.data.unwrap().len(), limit);
    ///
    /// }
    /// ```
    pub async fn get_asset_discord(&self, asset: &str, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<CoinDeskResponse<Vec<AssetDiscord>>, Error> {
        call_api_endpoint::<CoinDeskResponse<Vec<AssetDiscord>>>(
            self.api_key()?,
            APIEndpoint::AssetDiscord, Unit::NA,
            vec![Param::Asset { v: asset, }, Param::ToTimestamp { v: to_timestamp, }, Param::Limit { v: limit, }],
            None
        ).await
    }

    /// # Historical Social \[Reddit Day\] (Asset)
    /// Returns daily Reddit metadata for a given asset.
    ///
    /// # Description (CoinDesk Documentation)
    /// The Reddit Historical Daily Metrics endpoint aggregates key performance indicators from all the subreddits related to a specific digital asset,
    /// providing a comprehensive understanding of the asset's footprint on Reddit—a critical channel for community engagement and public sentiment
    /// in the digital asset industry.
    ///
    /// # Input
    /// - `asset`: Asset symbol
    /// - `to_timestamp`: Final timestamp up to which the data will be extracted
    /// - `limit`: Maximum number of datapoints per API endpoint call
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::CoinDesk;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let limit: usize = 2000;
    ///     let reddit = backend.get_asset_reddit("ETH", None, Some(limit)).await.unwrap();
    ///     assert_eq!(reddit.data.unwrap().len(), limit);
    ///
    /// }
    /// ```
    pub async fn get_asset_reddit(&self, asset: &str, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<CoinDeskResponse<Vec<AssetReddit>>, Error> {
        call_api_endpoint::<CoinDeskResponse<Vec<AssetReddit>>>(
            self.api_key()?,
            APIEndpoint::AssetReddit, Unit::NA,
            vec![Param::Asset { v: asset, }, Param::ToTimestamp { v: to_timestamp, }, Param::Limit { v: limit, }],
            None
        ).await
    }

    /// # Historical Social \[Telegram Day\] (Asset)
    /// Returns daily Telegram metadata for a given asset.
    ///
    /// # Description (CoinDesk Documentation)
    /// The Telegram Historical Daily Metrics endpoint collates essential data points across all Telegram groups affiliated with a particular cryptocurrency asset.
    /// Telegram often serves as a primary hub for real-time community engagement, announcements, and discussions in the crypto ecosystem.
    ///
    /// # Input
    /// - `asset`: Asset symbol
    /// - `to_timestamp`: Final timestamp up to which the data will be extracted
    /// - `limit`: Maximum number of datapoints per API endpoint call
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::CoinDesk;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let limit: usize = 2000;
    ///     let telegram = backend.get_asset_telegram("SOL", None, Some(limit)).await.unwrap();
    ///     assert_eq!(telegram.data.unwrap().len(), limit);
    ///
    /// }
    /// ```
    pub async fn get_asset_telegram(&self, asset: &str, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<CoinDeskResponse<Vec<AssetTelegram>>, Error> {
        call_api_endpoint::<CoinDeskResponse<Vec<AssetTelegram>>>(
            self.api_key()?,
            APIEndpoint::AssetTelegram, Unit::NA,
            vec![Param::Asset { v: asset, }, Param::ToTimestamp { v: to_timestamp, }, Param::Limit { v: limit, }],
            None
        ).await
    }

    /// # Historical Social \[X (Twitter) Day\] (Asset)
    /// Returns daily X (Twitter) metadata for a given asset.
    ///
    /// # Description (CoinDesk Documentation)
    /// The X (Twitter) Historical Daily Metrics endpoint aggregates essential metrics from all X (Twitter) accounts associated with a specific cryptocurrency asset.
    /// X (Twitter) is a key platform for real-time updates, announcements, and community engagement in the digital asset industry.
    ///
    /// # Input
    /// - `asset`: Asset symbol
    /// - `to_timestamp`: Final timestamp up to which the data will be extracted
    /// - `limit`: Maximum number of datapoints per API endpoint call
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::CoinDesk;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let limit: usize = 2000;
    ///     let twitter = backend.get_asset_twitter("SOL", None, Some(limit)).await.unwrap();
    ///     assert_eq!(twitter.data.unwrap().len(), limit)
    ///
    /// }
    /// ```
    pub async fn get_asset_twitter(&self, asset: &str, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<CoinDeskResponse<Vec<AssetTwitter>>, Error> {
        call_api_endpoint::<CoinDeskResponse<Vec<AssetTwitter>>>(
            self.api_key()?,
            APIEndpoint::AssetTwitter, Unit::NA,
            vec![Param::Asset { v: asset, }, Param::ToTimestamp { v: to_timestamp, }, Param::Limit { v: limit, }],
            None
        ).await
    }

    /// # Latest Articles (News)
    /// Returns a list of latest articles.
    ///
    /// # Description (CoinDesk Documentation)
    /// The Latest Articles endpoint serves as the pulse of the crypto news landscape, providing users with instant access to the most recent articles across
    /// the industry. By drawing from a wide array of reputable sources, this endpoint curates a fresh, real-time stream of information, insights,
    /// and developments, ensuring that users remain at the forefront of crypto news narratives. Whether you are an investor, enthusiast, or industry professional,
    /// this endpoint delivers a comprehensive and up-to-the-minute news digest, placing you at the heart of the ever-evolving crypto conversation.
    ///
    /// # Input
    /// - `language`: Language of the news
    /// - `source_id`: Source ID of the news stream
    /// - `categories`: List of news categories
    /// - `exclude_categories`: List of news categories to exclude
    /// - `to_timestamp`: Final timestamp up to which the data will be extracted
    /// - `limit`: Maximum number of datapoints per API endpoint call
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::{CoinDesk, NewsLang, NewsSourceID};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let language: NewsLang = NewsLang::EN;
    ///     let source_id: NewsSourceID = NewsSourceID::ForbesDigitalAssets;
    ///     let limit: usize = 100;
    ///     let articles = backend.get_news_latest_articles(language, source_id, None, None, None, Some(limit)).await.unwrap();
    ///     assert_eq!(articles.data.unwrap().len(), limit);
    ///
    /// }
    /// ```
    pub async fn get_news_latest_articles(&self, language: NewsLang, source_id: NewsSourceID, categories: Option<Vec<String>>,
                                          exclude_categories: Option<Vec<String>>, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<CoinDeskResponse<Vec<NewsLatestArticle>>, Error> {
        call_api_endpoint::<CoinDeskResponse<Vec<NewsLatestArticle>>>(
            self.api_key()?,
            APIEndpoint::NewsLatestArticles, Unit::NA,
            vec![
                Param::NewsLanguage { v: language, }, Param::NewsSourceID { v: source_id, }, Param::NewsCategories { v: categories, },
                Param::NewsExcludeCategories { v: exclude_categories, }, Param::ToTimestamp { v: to_timestamp, }, Param::Limit { v: limit, },
            ],
            None
        ).await
    }

    /// # Sources (News)
    /// Returns a list of news sources.
    ///
    /// # Description (CoinDesk Documentation)
    /// The News Sources endpoint offers a comprehensive listing of all news sources available through our API. This endpoint is crucial for users who need
    /// to identify and access a diverse array of reputable news outlets, blogs, and information platforms. It ensures that users can explore and select from
    /// a curated list of trusted industry voices, supporting a variety of applications in research, news aggregation, and content curation.
    ///
    /// # Input
    /// - `language`: Language of the news
    /// - `source_type`: Type of news stream
    /// - `status`: Status of the news stream (e.g., ACTIVE, INACTIVE)
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::{CoinDesk, NewsLang, NewsSourceType, NewsStatus};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let language: NewsLang = NewsLang::EN;
    ///     let source_type: NewsSourceType = NewsSourceType::RSS;
    ///     let status: NewsStatus = NewsStatus::ACTIVE;
    ///     let sources = backend.get_news_sources(language, source_type, status).await.unwrap();
    ///     assert_eq!(sources.data.unwrap()[0].source_type, String::from("RSS"));
    ///
    /// }
    /// ```
    pub async fn get_news_sources(&self, language: NewsLang, source_type: NewsSourceType, status: NewsStatus) -> Result<CoinDeskResponse<Vec<NewsSource>>, Error> {
        call_api_endpoint::<CoinDeskResponse<Vec<NewsSource>>>(
            self.api_key()?,
            APIEndpoint::NewsSources, Unit::NA,
            vec![Param::NewsLanguage { v: language, }, Param::NewsSourceType { v: source_type, }, Param::NewsStatus { v: status, }],
            None
        ).await
    }

    /// # Categories (News)
    /// Return  a list of news categories.
    ///
    /// # Description (CoinDesk Documentation)
    /// The News Categories List endpoint is designed to provide a straightforward listing of all news categories available through our API.
    /// This endpoint is essential for users looking to identify the broad spectrum of topics covered by our news sources, ranging from market trends
    /// and technological advances to regulatory changes and cultural events. By offering a clear overview of these categories, it facilitates users
    /// in understanding and navigating the thematic organization of news content.
    ///
    /// # Input
    /// - `status`: Status of the news stream (e.g., ACTIVE, INACTIVE)
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::{CoinDesk, NewsStatus};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let status: NewsStatus = NewsStatus::ACTIVE;
    ///     let categories = backend.get_news_categories(status).await.unwrap();
    ///     assert_eq!(categories.data.unwrap()[0].status, String::from("ACTIVE"));
    ///
    /// }
    /// ```
    pub async fn get_news_categories(&self, status: NewsStatus) -> Result<CoinDeskResponse<Vec<NewsCategory>>, Error> {
        call_api_endpoint::<CoinDeskResponse<Vec<NewsCategory>>>(
            self.api_key()?,
            APIEndpoint::NewsCategories, Unit::NA,
            vec![Param::NewsStatus { v: status, }],
            None
        ).await
    }

    /// # MktCap Historical OHLCV \[All Assets Day\] (Overview)
    /// Returns daily historical OHLCV data for all assets.
    ///
    /// # Description (CoinDesk Documentation)
    /// Built as an overview for the digital asset industry, this endpoint presents a thorough historical daily overview of market capitalisation
    /// for digital assets that meet the volume and listing criteria. Users can explore day-by-day total market cap figures, along with daily open-high-low values
    /// and top-tier trading volumes, for the entire digital asset industry. It's a fundamental tool for analysts, researchers, and investors looking
    /// to understand market trends, trace asset performance over time, and make data-driven decisions rooted in historical contexts.
    ///
    /// # Input
    /// - `to_timestamp`: Final timestamp up to which the data will be extracted
    /// - `limit`: Maximum number of datapoints per API endpoint call
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::CoinDesk;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CoinDesk = CoinDesk::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let limit: usize = 2000;
    ///     let mktcap = backend.get_overview_mktcap_ohlcv(None, Some(limit)).await.unwrap();
    ///     assert_eq!(mktcap.data.unwrap().len(), limit);
    ///
    /// }
    /// ```
    pub async fn get_overview_mktcap_ohlcv(&self, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<CoinDeskResponse<Vec<OverviewMktCapOHLCV>>, Error> {
        call_api_endpoint::<CoinDeskResponse<Vec<OverviewMktCapOHLCV>>>(
            self.api_key()?,
            APIEndpoint::OverviewMktCapOHLCV, Unit::NA,
            vec![Param::ToTimestamp { v: to_timestamp, }, Param::Limit { v: limit, }],
            None
        ).await
    }
}



#[cfg(test)]
mod tests {
    use crate::backend::CoinDesk;

    #[tokio::test]
    async fn unit_test_backend() -> () {
        let mut backend: CoinDesk = CoinDesk::new();
        backend.build(&"API_KEY").unwrap();
        assert!(backend.api_key != None);
    }
}