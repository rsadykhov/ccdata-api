use std::{io, error::Error, env::var, collections::HashMap};
use dotenv::dotenv;
use crate::{CCUnit, CCAPIEndpoint};
use crate::schemas::{self as sh, CCDataResponse};
use crate::schemas::min_api;
use crate::schemas::data_api::indices_and_reference_rates::{CCIndicesMarket, CCIndicesOHLCV};
use crate::schemas::data_api::spot::{CCSpotMarket, CCSpotInstrumentStatus, CCSpotOHLCV, CCSpotInstrumentMetdata, CCSpotMarkets,
                                     CCSpotMarketsInstruments};
use crate::schemas::data_api::futures::{CCFuturesMarket, CCFuturesOHLCV, CCFuturesMarkets};
use crate::schemas::data_api::options::{CCOptionsMarket, CCOptionsOHLCV, CCOptionsMarkets};
use crate::schemas::data_api::derivatives_indices::{CCDerIndicesMarket, CCDerIndicesOHLCV, CCDerIndicesMarkets};
use crate::schemas::data_api::on_chain_dex::{CCOCDEXMarket, CCOCDEXOHLCV, CCOCDEXMarkets};
use crate::schemas::data_api::on_chain_core::{CCOCCoreETHBlock, CCOCCoreAssetByChain, CCOCCoreAssetByAddress, CCOCCoreSupply};
use crate::schemas::data_api::asset::{CCAssetMetadata, CCAssetEvent, CCAssetCodeRepoMetrics, CCAssetDiscord, CCAssetReddit, CCAssetTelegram, CCAssetTwitter};
use crate::schemas::data_api::news::{CCNewsStatus, CCNewsLang, CCNewsSourceID, CCNewsLatestArticle, CCNewsSourceType, CCNewsSource, CCNewsCategory};
use crate::schemas::data_api::overview::CCOverviewMktCapOHLCV;
use crate::utils::{Market, Param, call_api_endpoint};


/// API data collection backend.
pub struct CCData {
    /// CCData API key
    api_key: Option<String>,
}

impl CCData {
    /// Creates a new CCData backend for data collection.
    pub fn new() -> Self {
        CCData { api_key: None }
    }

    /// Returns the refernce to the defined API key.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ccdata_api::CCData;
    ///
    /// let mut backend: CCData = CCData::new();
    /// // Provide API key as the environment variable called API_KEY
    /// backend.build(&"API_KEY").unwrap();
    ///
    /// println!("{}", backend.api_key().unwrap());
    /// ```
    pub fn api_key(&self) -> Result<&String, io::Error> {
        match &self.api_key {
            Some(v) => Ok(v),
            None => {
                Err(io::Error::new(io::ErrorKind::InvalidInput, "CCData: No API key is defined."))
            },
        }
    }

    /// Updates the API key.
    ///
    /// # Input
    /// - `new_api_key`: New API key that will be used by the backend to send requests to CCData API endpoints
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ccdata_api::CCData;
    ///
    /// let mut backend: CCData = CCData::new();
    ///
    /// let new_api_key: String = String::from("xxxxxxx");
    /// backend.update_api_key(new_api_key);
    ///
    /// assert_eq!(backend.api_key().unwrap(), &String::from("xxxxxxx"));
    /// ```
    pub fn update_api_key(&mut self, new_api_key: String) -> () {
        self.api_key = Some(new_api_key);
    }

    /// Initiates the API data collection backend with the API key stored in the environment variable.
    ///
    /// # Input
    /// -`api_key_env_var`: Name of the environment variable in the local `.env` file that stores the CCData API key
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ccdata_api::CCData;
    ///
    /// let mut backend: CCData = CCData::new();
    /// // Provide API key as the environment variable called API_KEY
    /// backend.build(&"API_KEY").unwrap();
    ///
    /// println!("{}", backend.api_key().unwrap());
    /// ```
    pub fn build(&mut self, api_key_env_var: &str) -> Result<(), io::Error> {
        match dotenv() {
            Ok(_b) => (),
            Err(e) => {
                return Err(io::Error::new(io::ErrorKind::NotFound, e.to_string()));
            },
        }
        match var(api_key_env_var) {
            Ok(k) => self.update_api_key(k),
            Err(e) => return Err(io::Error::new(io::ErrorKind::NotFound, e.to_string())),
        }
        Ok(())
    }

    /// # Available Coin List (Blockchain Data)
    /// Returns a list of all coins that CCData have data for.
    ///
    /// # Description (CCData Documentation)
    /// Powered by IntoTheBlock, an intelligence company that leverages machine learning and advanced statistics to extract intelligent signals for crypto-assets.
    ///
    /// You can only use this endpoint with a valid api_key. Returns a list of all coins for which we currently get blockchain data from IntoTheBlock.
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::CCData;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let available_coin_list = backend.get_available_coin_list().await.unwrap();
    ///     assert!(0 < available_coin_list.data.unwrap().len());
    ///
    /// }
    /// ```
    pub async fn get_available_coin_list(&self) -> Result<sh::CCMinResponse<HashMap<String, min_api::CCAvailableCoinList>>, Box<dyn Error>> {
        call_api_endpoint::<sh::CCMinResponse<HashMap<String, min_api::CCAvailableCoinList>>>(
            self.api_key()?,
            CCAPIEndpoint::AvailableCoinList, CCUnit::NA,
            vec![], None
        ).await
    }

    /// # Historical Daily (Blockchain Data)
    /// Returns the historical data for a given symbol.
    ///
    /// # Description (CCData Documentation)
    /// Powered by IntoTheBlock, an intelligence company that leverages machine learning and advanced statistics to extract intelligent signals for crypto-assets.
    /// Full description of the return fields available here.
    ///
    /// You can only use this endpoint with a valid api_key. Retrieve the daily aggregated blockchain data for the requested coin,
    /// back through time for the number of points as specifed by the limit. Timestamp values are based on 00:00 GMT time.
    ///
    /// # Input
    /// - `symbol`: Asset symbol
    /// - `to_timestamp`: Final timestamp up to which the data will be extracted
    /// - `limit`: Maximum number of datapoints per API endpoint call
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::CCData;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let limit: usize = 2000;
    ///     let historical_daily = backend.get_historical_daily(&String::from("ETH"), None, Some(limit)).await.unwrap();
    ///     assert_eq!(historical_daily.data.unwrap().data.unwrap().len(), limit);
    ///
    /// }
    /// ```
    pub async fn get_historical_daily(&self, symbol: &String, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<sh::CCMinResponse<sh::CCMinWrapper<Vec<min_api::CCHistoricalDaily>>>, Box<dyn Error>> {
        call_api_endpoint::<sh::CCMinResponse<sh::CCMinWrapper<Vec<min_api::CCHistoricalDaily>>>>(
            self.api_key()?,
            CCAPIEndpoint::HistoricalDaily, CCUnit::NA,
            vec![Param::Symbol{v: symbol}, Param::Limit{v: limit}, Param::ToTs{v: to_timestamp}], None
        ).await
    }

    /// # Balance Distribution Daily (Blockchain Data)
    /// Returns the daily balance distribution history for Bitcoin.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::CCData;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let limit: usize = 2000;
    ///     let balance_distribution = backend.get_balance_distribution(None, Some(limit)).await.unwrap();
    ///     assert!(balance_distribution.data.unwrap().data.unwrap().len() <= limit);
    ///
    /// }
    /// ```
    pub async fn get_balance_distribution(&self, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<sh::CCMinResponse<sh::CCMinWrapper<Vec<min_api::CCBalanceDistribution>>>, Box<dyn Error>> {
        call_api_endpoint::<sh::CCMinResponse<sh::CCMinWrapper<Vec<min_api::CCBalanceDistribution>>>>(
            self.api_key()?,
            CCAPIEndpoint::BalanceDistribution, CCUnit::NA,
            vec![Param::Symbol{v: &String::from("BTC")}, Param::Limit{v: limit}, Param::ToTs{v: to_timestamp}], None
        ).await
    }

    /// # Historical OHLCV+ \[Day, Hour, Minute\] (Indices & Ref. Rates)
    /// Returns historical OHLCV data for a given instrument.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::{CCData, CCUnit, CCIndicesMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let market: CCIndicesMarket = CCIndicesMarket::CADLI;
    ///     let limit: usize = 2000;
    ///     let ohlcv = backend.get_indices_ohlcv(&String::from("BTC-USD"), None, Some(limit), market, CCUnit::Day).await.unwrap();
    ///     assert_eq!(ohlcv.data.unwrap().len(), limit);
    ///
    /// }
    /// ```
    pub async fn get_indices_ohlcv(&self, instrument: &String, to_timestamp: Option<i64>, limit: Option<usize>, market: CCIndicesMarket, unit: CCUnit) -> Result<CCDataResponse<Vec<CCIndicesOHLCV>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<Vec<CCIndicesOHLCV>>>(
            self.api_key()?,
            CCAPIEndpoint::IndicesOHLCV, unit,
            vec![Param::Instrument{v: instrument}, Param::Limit{v: limit}, Param::ToTimestamp{v: to_timestamp}, Param::Market{v: market.to_string()}], None
        ).await
    }

    /// # Historical OHLCV+ \[Day, Hour, Minute\] (Spot)
    /// Returns historical OHLCV data for a given instrument.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::{CCData, CCUnit, CCSpotMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let market: CCSpotMarket = CCSpotMarket::KRAKEN;
    ///     let limit: usize = 2000;
    ///     let ohlcv = backend.get_spot_ohlcv(&String::from("BTC-USD"), None, Some(limit), market, CCUnit::Day).await.unwrap();
    ///     assert_eq!(ohlcv.data.unwrap().len(), limit);
    ///
    /// }
    /// ```
    pub async fn get_spot_ohlcv(&self, instrument: &String, to_timestamp: Option<i64>, limit: Option<usize>, market: CCSpotMarket, unit: CCUnit) -> Result<CCDataResponse<Vec<CCSpotOHLCV>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<Vec<CCSpotOHLCV>>>(
            self.api_key()?,
            CCAPIEndpoint::SpotOHLCV, unit,
            vec![Param::Instrument{v: instrument}, Param::Limit{v: limit}, Param::ToTimestamp{v: to_timestamp}, Param::Market{v: market.to_string()}], None
        ).await
    }

    /// # Instrument Metadata (Spot)
    /// Returns metadata for a given instrument.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::{CCData, CCSpotMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let instruments: Vec<String> = vec![String::from("BTC-USD"), String::from("ETH-USD")];
    ///     let market: CCSpotMarket = CCSpotMarket::KRAKEN;
    ///     let instrument_metadata = backend.get_spot_instrument_metadata(&instruments, market).await.unwrap();
    ///     assert_eq!(instrument_metadata.data.unwrap().len(), 2);
    ///
    /// }
    /// ```
    pub async fn get_spot_instrument_metadata(&self, instruments: &Vec<String>, market: CCSpotMarket) -> Result<CCDataResponse<HashMap<String, CCSpotInstrumentMetdata>>, Box< dyn Error>> {
        call_api_endpoint::<CCDataResponse<HashMap<String, CCSpotInstrumentMetdata>>>(
            self.api_key()?,
            CCAPIEndpoint::SpotInstrumentMetadata, CCUnit::NA,
            vec![Param::Instruments{v: instruments}, Param::Market{v: market.to_string()}], None
        ).await
    }

    /// # Markets (Spot)
    /// Returns metadata about a given market.
    ///
    /// # Description (CCData Documentation)
    /// This endpoint provides comprehensive information about various cryptocurrency spot markets. By specifying a market through the "market" parameter,
    /// users can retrieve details about a specific market, such as its trading pairs, volume, operational status, and other relevant metadata.
    /// If no specific market is indicated, the endpoint delivers data on all available markets. This functionality is essential for users looking to explore
    /// and compare the characteristics and trading conditions of different cryptocurrency exchanges or market segments, assisting in market analysis,
    /// strategic planning, and decision-making.
    ///
    /// # Input
    /// - `market`: Market name
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::{CCData, CCSpotMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let market: CCSpotMarket = CCSpotMarket::KRAKEN;
    ///     let markets = backend.get_spot_markets(market).await.unwrap();
    ///     assert_eq!(markets.data.unwrap().get("kraken").unwrap().exchange_status, String::from("ACTIVE"));
    ///
    /// }
    /// ```
    pub async fn get_spot_markets(&self, market: CCSpotMarket) -> Result<CCDataResponse<HashMap<String, CCSpotMarkets>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<HashMap<String, CCSpotMarkets>>>(
            self.api_key()?,
            CCAPIEndpoint::SpotMarkets, CCUnit::NA,
            vec![Param::Market{v: market.to_string()}], None
        ).await
    }

    /// # Markets + Instruments \[Mapped\] (Spot)
    /// Returns a map of given instruments across a market.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::{CCData, CCSpotMarket, CCSpotInstrumentStatus};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let instruments: Vec<String> = vec![String::from("BTC-USD"), String::from("ETH-USD")];
    ///     let market: CCSpotMarket = CCSpotMarket::KRAKEN;
    ///     let instrument_status: CCSpotInstrumentStatus = CCSpotInstrumentStatus::ACTIVE;
    ///     let markets_instruments = backend.get_spot_markets_instruments(&instruments, market, instrument_status).await.unwrap();
    ///     assert_eq!(markets_instruments.data.unwrap().get("kraken").unwrap().instruments.len(), 2);
    ///
    /// }
    /// ```
    pub async fn get_spot_markets_instruments(&self, instruments: &Vec<String>, market: CCSpotMarket, instrument_status: CCSpotInstrumentStatus) -> Result<CCDataResponse<HashMap<String, CCSpotMarketsInstruments>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<HashMap<String, CCSpotMarketsInstruments>>>(
            self.api_key()?,
            CCAPIEndpoint::SpotMarketsInstruments, CCUnit::NA,
            vec![Param::Instruments{v: instruments}, Param::Market{v: market.to_string()}, Param::InstrumentStatus {v: instrument_status}], None
        ).await
    }

    /// # Historical OHLCV+ \[Day, Hour, Minute\] (Futures)
    /// Returns historical OHLCV data for a given instrument.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::{CCData, CCUnit, CCFuturesMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let market: CCFuturesMarket = CCFuturesMarket::BINANCE;
    ///     let limit: usize = 2000;
    ///     let ohlcv = backend.get_futures_ohlcv(&String::from("BTC-USDT-VANILLA-PERPETUAL"), None, Some(limit), market, CCUnit::Day).await.unwrap();
    ///     assert!(ohlcv.data.unwrap().len() <= limit)
    ///
    /// }
    /// ```
    pub async fn get_futures_ohlcv(&self, instrument: &String, to_timestamp: Option<i64>, limit: Option<usize>, market: CCFuturesMarket, unit: CCUnit) -> Result<CCDataResponse<Vec<CCFuturesOHLCV>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<Vec<CCFuturesOHLCV>>>(
            self.api_key()?,
            CCAPIEndpoint::FuturesOHLCV, unit,
            vec![Param::Instrument{v: instrument}, Param::Limit{v: limit}, Param::ToTimestamp{v: to_timestamp}, Param::Market{v: market.to_string()}],
            Some(String::from("&groups=ID,MAPPING,OHLC,TRADE,VOLUME,MAPPING_ADVANCED,OHLC_TRADE"))
        ).await
    }

    /// # Markets (Futures)
    /// Returns metadata about a given market.
    ///
    /// # Description (CCData Documentation)
    /// This endpoint provides comprehensive information about various cryptocurrency futures markets. By utilizing the "market" parameter,
    /// users can access detailed data about specific futures markets, including trading pairs, volume, operational status, and additional relevant metadata.
    /// If no specific market is specified, the endpoint returns information on all available futures markets. This capability is crucial for users who wish
    /// to explore and analyze the characteristics and trading conditions of different cryptocurrency futures exchanges or market segments,
    /// aiding in market analysis, strategic planning, and decision-making.
    ///
    /// # Input
    /// - `market`: Market name
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::{CCData, CCFuturesMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let market: CCFuturesMarket = CCFuturesMarket::BINANCE;
    ///     let markets = backend.get_futures_markets(market).await.unwrap();
    ///     assert_eq!(markets.data.unwrap().get("binance").unwrap().exchange_status, String::from("ACTIVE"));
    ///
    /// }
    /// ```
    pub async fn get_futures_markets(&self, market: CCFuturesMarket) -> Result<CCDataResponse<HashMap<String, CCFuturesMarkets>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<HashMap<String, CCFuturesMarkets>>>(
            self.api_key()?,
            CCAPIEndpoint::FuturesMarkets, CCUnit::NA,
            vec![Param::Market{v: market.to_string()}], None
        ).await
    }  

    /// # Historical OHLCV+ \[Day, Hour, Minute\] (Options)
    /// Returns historical OHLCV data for a given instrument.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::{CCData, CCUnit, CCOptionsMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let market: CCOptionsMarket = CCOptionsMarket::OKEX;
    ///     let limit: usize = 2000;
    ///     let ohlcv = backend.get_options_ohlcv(&String::from("BTC-USD-20241227-15000-P"), None, Some(limit), market, CCUnit::Day).await.unwrap();
    ///     assert!(ohlcv.data.unwrap().len() <= limit);
    ///
    /// }
    /// ```
    pub async fn get_options_ohlcv(&self, instrument: &String, to_timestamp: Option<i64>, limit: Option<usize>, market: CCOptionsMarket, unit: CCUnit) -> Result<CCDataResponse<Vec<CCOptionsOHLCV>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<Vec<CCOptionsOHLCV>>>(
            self.api_key()?,
            CCAPIEndpoint::OptionsOHLCV, unit,
            vec![Param::Instrument{v: instrument}, Param::Limit{v: limit}, Param::ToTimestamp{v: to_timestamp}, Param::Market{v: market.to_string()}],
            Some(String::from("&groups=ID,MAPPING,OHLC,TRADE,VOLUME,MAPPING_ADVANCED,OHLC_TRADE"))
        ).await
    }

    /// # Markets (Options)
    /// Returns metadata about a given market.
    ///
    /// # Description (CCData Documentation)
    /// This endpoint provides comprehensive information about the various options markets integrated by our platform. By utilizing the "market" parameter,
    /// users can access detailed data about specific options markets, including available instruments, trading volume, operational status, and additional
    /// relevant metadata. If no specific market is specified, the endpoint returns information on all integrated options markets.
    /// This capability is crucial for users who wish to explore and analyze the characteristics and trading conditions of different options exchanges
    /// or market segments, aiding in market analysis, strategic planning, and decision-making.
    ///
    /// # Input
    /// - `market`: Market name
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::{CCData, CCOptionsMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let market: CCOptionsMarket = CCOptionsMarket::DERIBIT;
    ///     let markets = backend.get_options_markets(market).await.unwrap();
    ///     assert_eq!(markets.data.unwrap().get("deribit").unwrap().exchange_status, String::from("ACTIVE"));
    ///
    /// }
    /// ```
    pub async fn get_options_markets(&self, market: CCOptionsMarket) -> Result<CCDataResponse<HashMap<String, CCOptionsMarkets>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<HashMap<String, CCOptionsMarkets>>>(
            self.api_key()?,
            CCAPIEndpoint::OptionsMarkets, CCUnit::NA,
            vec![Param::Market{v: market.to_string()}], None
        ).await
    }

    /// # Historical OHLCV+ \[Day, Hour, Minute\] (Derivatives Indices)
    /// Returns historical OHLCV data for a given instrument.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::{CCData, CCUnit, CCDerIndicesMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let market: CCDerIndicesMarket = CCDerIndicesMarket::BINANCE;
    ///     let limit: usize = 2000;
    ///     let ohlcv = backend.get_der_indices_ohlcv(&String::from("BTCUSDT"), None, Some(limit), market, CCUnit::Day).await.unwrap();
    ///     assert!(ohlcv.data.unwrap().len() <= limit);
    ///
    /// }
    /// ```
    pub async fn get_der_indices_ohlcv(&self, instrument: &String, to_timestamp: Option<i64>, limit: Option<usize>, market: CCDerIndicesMarket, unit: CCUnit) -> Result<CCDataResponse<Vec<CCDerIndicesOHLCV>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<Vec<CCDerIndicesOHLCV>>>(
            self.api_key()?,
            CCAPIEndpoint::DerIndicesOHLCV, unit,
            vec![Param::Instrument{v: instrument}, Param::Limit{v: limit}, Param::ToTimestamp{v: to_timestamp}, Param::Market{v: market.to_string()}],
            Some(String::from("&groups=ID,OHLC,OHLC_MESSAGE,MESSAGE,MAPPING,MAPPING_ADVANCED"))
        ).await
    }

    /// # Markets (Derivatives Indices)
    /// Returns metadata about a given market.
    ///
    /// # Description (CCData Documentation)
    /// This endpoint provides comprehensive information about various derivatives index markets. By specifying a market through the "market" parameter,
    /// users can retrieve details about a specific derivatives market, such as its index instruments, volume, operational status, and other relevant metadata.
    /// If no specific market is indicated, the endpoint delivers data on all available markets. This functionality is essential for users looking to explore
    /// and compare the characteristics and trading conditions of different derivatives exchanges or market segments, assisting in market analysis,
    /// strategic planning, and decision-making.
    ///
    /// # Input
    /// - `market`: Market name
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::{CCData, CCDerIndicesMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let market: CCDerIndicesMarket = CCDerIndicesMarket::KRAKEN;
    ///     let markets = backend.get_der_indices_markets(market).await.unwrap();
    ///     assert_eq!(markets.data.unwrap().get("kraken").unwrap().exchange_status, String::from("ACTIVE"));
    ///
    /// }
    /// ```
    pub async fn get_der_indices_markets(&self, market: CCDerIndicesMarket) -> Result<CCDataResponse<HashMap<String, CCDerIndicesMarkets>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<HashMap<String, CCDerIndicesMarkets>>>(
            self.api_key()?,
            CCAPIEndpoint::DerIndicesMarkets, CCUnit::NA,
            vec![Param::Market{v: market.to_string()}], None
        ).await
    }

    /// # Historical OHLCV+ (Swap) \[Day, Hour, Minute\] (On-Chain DEX)
    /// Returns historical OHLCV data for a given instrument.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::{CCData, CCUnit, CCOCDEXMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let market: CCOCDEXMarket = CCOCDEXMarket::UNISWAPV2;
    ///     let limit: usize = 2000;
    ///     let ohlcv = backend.get_ocdex_ohlcv(&String::from("0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852_2"), None, Some(limit), market, CCUnit::Day).await.unwrap();
    ///     assert!(ohlcv.data.unwrap().len() <= limit);
    ///
    /// }
    /// ```
    pub async fn get_ocdex_ohlcv(&self, instrument: &String, to_timestamp: Option<i64>, limit: Option<usize>, market: CCOCDEXMarket, unit: CCUnit) -> Result<CCDataResponse<Vec<CCOCDEXOHLCV>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<Vec<CCOCDEXOHLCV>>>(
            self.api_key()?,
            CCAPIEndpoint::OCDEXOHLCV, unit,
            vec![Param::Instrument{v: instrument}, Param::Limit{v: limit}, Param::ToTimestamp{v: to_timestamp}, Param::Market{v: market.to_string()}],
            Some(String::from("&groups=ID,MAPPING,MAPPING_ADVANCED,OHLC,OHLC_SWAP,SWAP,VOLUME"))
        ).await
    }

    /// # Markets (On-Chain DEX)
    /// Returns metadata about a given market.
    ///
    /// # Description (CCData Documentation)
    /// The On-Chain Markets endpoint offers comprehensive information about various decentralized exchange (DEX) markets within the blockchain ecosystem.
    /// By specifying a market through the "market" parameter, users can retrieve detailed information about a specific on-chain market, such as its trading pairs,
    /// liquidity, operational status, and other relevant metadata. If no specific market is indicated, the endpoint delivers data on all available on-chain markets.
    /// This functionality is essential for users looking to explore and compare the characteristics and trading conditions of different decentralized exchanges
    /// or market segments, aiding in market analysis, strategic planning, and decision-making.
    ///
    /// # Input
    /// - `market`: Market name
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::{CCData, CCOCDEXMarket};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let market: CCOCDEXMarket = CCOCDEXMarket::UNISWAPV2;
    ///     let markets = backend.get_ocdex_markets(market).await.unwrap();
    ///     assert_eq!(markets.data.unwrap().get("uniswapv2").unwrap().exchange_status, String::from("ACTIVE"));
    ///
    /// }
    /// ```
    pub async fn get_ocdex_markets(&self, market: CCOCDEXMarket) -> Result<CCDataResponse<HashMap<String, CCOCDEXMarkets>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<HashMap<String, CCOCDEXMarkets>>>(
            self.api_key()?,
            CCAPIEndpoint::OCDEXMarkets, CCUnit::NA,
            vec![Param::Market{v: market.to_string()}], None
        ).await
    }

    /// # ETH Blocks \[Full Processed\] (On-Chain Core)
    /// Returns the processed data for a given block.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::CCData;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let eth_block = backend.get_occore_eth_block(19501436).await.unwrap();
    ///     assert_eq!(eth_block.data.unwrap().symbol, String::from("ETH"));;
    ///
    /// }
    /// ```
    pub async fn get_occore_eth_block(&self, block_number: i64) -> Result<CCDataResponse<CCOCCoreETHBlock>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<CCOCCoreETHBlock>>(
            self.api_key()?,
            CCAPIEndpoint::OCCoreETHBlocks, CCUnit::NA,
            vec![Param::OCCoreBlockNumber{v: block_number}],
            Some(String::from("&groups=ID,METADATA,TRANSACTIONS,ORPHAN_TRACES,UNCLES,WITHDRAWALS"))
        ).await
    }

    /// # Assets Summary By Chain (On-Chain Core)
    /// Returns a summary of assets on a given chain.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::CCData;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let assets_by_chain = backend.get_occore_assets_by_chain(&String::from("ETH")).await.unwrap();
    ///     assert_eq!(assets_by_chain.data.unwrap().chain_asset_summary.symbol, String::from("ETH"));
    ///
    /// }
    /// ```
    pub async fn get_occore_assets_by_chain(&self, chain_asset: &String) -> Result<CCDataResponse<CCOCCoreAssetByChain>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<CCOCCoreAssetByChain>>(
            self.api_key()?,
            CCAPIEndpoint::OCCoreAssetsByChain, CCUnit::NA,
            vec![Param::ChainAsset{v: chain_asset}],
            Some(String::from("&asset_lookup_priority=SYMBOL"))
        ).await
    }

    /// # Asset By Address Lookup (On-Chain Core)
    /// Returns a summary of a smart contract on a given chain.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::CCData;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let address: String = String::from("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
    ///     let quote_asset: String = String::from("USD");
    ///     let asset_by_address = backend.get_occore_asset_by_address(&String::from("ETH"), &address, &quote_asset).await.unwrap();
    ///     assert_eq!(asset_by_address.data.unwrap().parent_asset_symbol.unwrap(), String::from("ETH"));
    ///
    /// }
    /// ```
    pub async fn get_occore_asset_by_address(&self, chain_asset: &String, address: &String, quote_asset: &String) -> Result<CCDataResponse<CCOCCoreAssetByAddress>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<CCOCCoreAssetByAddress>>(
            self.api_key()?,
            CCAPIEndpoint::OCCoreAssetByAddress, CCUnit::NA,
            vec![Param::ChainAsset{v: chain_asset}, Param::OCCoreAddress{v: address}, Param::OCCoreQuoteAsset{v: quote_asset}],
            Some(String::from("&asset_lookup_priority=SYMBOL&groups=ID,BASIC,SUPPORTED_PLATFORMS,SECURITY_METRICS,SUPPLY,SUPPLY_ADDRESSES,ASSET_TYPE_SPECIFIC_METRICS,RESOURCE_LINKS,CLASSIFICATION,PRICE,MKT_CAP,VOLUME,CHANGE,TOPLIST_RANK,DESCRIPTION,DESCRIPTION_SUMMARY,CONTACT,SEO"))
        ).await
    }

    /// # Historical Supply Day (On-Chain Core)
    /// Returns supply history for a given asset.
    ///
    /// # Description (CCData Documentation)
    /// The On-Chain Historical Supply Day endpoint retrieves comprehensive historical supply data for various digital assets identified by either their
    /// CCData asset ID or unique asset symbol. This endpoint offers a detailed view of an asset's supply dynamics on a daily basis, providing insights
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
    /// use ccdata_api::CCData;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let limit: usize = 2000;
    ///     let historical_supply = backend.get_occore_supply(&String::from("BTC"), None, Some(limit)).await.unwrap();
    ///     assert_eq!(historical_supply.data.unwrap().len(), limit);
    ///
    /// }
    /// ```
    pub async fn get_occore_supply(&self, asset: &String, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<CCDataResponse<Vec<CCOCCoreSupply>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<Vec<CCOCCoreSupply>>>(
            self.api_key()?,
            CCAPIEndpoint::OCCoreSupply, CCUnit::NA,
            vec![Param::Asset{v: asset}, Param::ToTimestamp{v: to_timestamp}, Param::Limit{v: limit}], None
        ).await
    }

    /// # Full Asset Metadata (Asset)
    /// Returns a summary of metadata for a given asset.
    ///
    /// # Description (CCData Documentation)
    /// The Full Asset Metadata endpoint provides detailed and comprehensive information about any cryptocurrency asset identified by its CCData asset ID,
    /// unique asset symbol, or asset URI. This includes extensive data on asset description, classification, blockchain properties, social metrics,
    /// token sale information, and equity sale details—all consolidated into a single response to facilitate in-depth analysis, development, and research.
    ///
    /// # Input
    /// - `asset`: Asset symbol
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use ccdata_api::CCData;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let metadata = backend.get_asset_metadata(&String::from("ETH")).await.unwrap();
    ///     assert_eq!(metadata.data.unwrap().name, String::from("Ethereum"));
    ///
    /// }
    /// ```
    pub async fn get_asset_metadata(&self, asset: &String) -> Result<CCDataResponse<CCAssetMetadata>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<CCAssetMetadata>>(
            self.api_key()?,
            CCAPIEndpoint::AssetMetadata, CCUnit::NA,
            vec![Param::Asset{v: asset}],
            Some(String::from("&asset_lookup_priority=SYMBOL&quote_asset=USD&groups=ID,BASIC,SUPPLY,SUPPLY_ADDRESSES,CLASSIFICATION"))
        ).await
    }

    /// # Significant Asset Events (Asset)
    /// Returns a list of significant events for a given asset.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::CCData;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let limit: usize = 100;
    ///     let events = backend.get_asset_events(&String::from("ETH"), None, Some(limit)).await.unwrap();
    ///     assert!(events.data.unwrap().len() <= limit);
    ///
    /// }
    /// ```
    pub async fn get_asset_events(&self, asset: &String, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<CCDataResponse<Vec<CCAssetEvent>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<Vec<CCAssetEvent>>>(
            self.api_key()?,
            CCAPIEndpoint::AssetEvents, CCUnit::NA,
            vec![Param::Asset{v: asset}, Param::ToTimestamp{v: to_timestamp}, Param::Limit{v: limit}], None
        ).await
    }

    /// # Historical Social \[Code Repository Day\] (Asset)
    /// Returns daily code repository metadata for a given asset.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::CCData;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let limit: usize = 2000;
    ///     let code_repo = backend.get_asset_code_repo(&String::from("ETH"), None, Some(limit)).await.unwrap();
    ///     assert_eq!(code_repo.data.unwrap().len(), limit);
    ///
    /// }
    /// ```
    pub async fn get_asset_code_repo(&self, asset: &String, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<CCDataResponse<Vec<CCAssetCodeRepoMetrics>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<Vec<CCAssetCodeRepoMetrics>>>(
            self.api_key()?,
            CCAPIEndpoint::AssetCodeRepo, CCUnit::NA,
            vec![Param::Asset{v: asset}, Param::ToTimestamp{v: to_timestamp}, Param::Limit{v: limit}],
            Some(String::from("&groups=ID,GENERAL,ACTIVITY,SOURCE"))
        ).await
    }

    /// # Historical Social \[Discord Day\] (Asset)
    /// Returns daily Discord metadata for a given asset.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::CCData;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let limit: usize = 2000;
    ///     let discord = backend.get_asset_discord(&String::from("ETH"), None, Some(limit)).await.unwrap();
    ///     assert_eq!(discord.data.unwrap().len(), limit);
    ///
    /// }
    /// ```
    pub async fn get_asset_discord(&self, asset: &String, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<CCDataResponse<Vec<CCAssetDiscord>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<Vec<CCAssetDiscord>>>(
            self.api_key()?,
            CCAPIEndpoint::AssetDiscord, CCUnit::NA,
            vec![Param::Asset{v: asset}, Param::ToTimestamp{v: to_timestamp}, Param::Limit{v: limit}],
            Some(String::from("&groups=ID,GENERAL,ACTIVITY,SOURCE"))
        ).await
    }

    /// # Historical Social \[Reddit Day\] (Asset)
    /// Returns daily Reddit metadata for a given asset.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::CCData;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let limit: usize = 2000;
    ///     let reddit = backend.get_asset_reddit(&String::from("ETH"), None, Some(limit)).await.unwrap();
    ///     assert_eq!(reddit.data.unwrap().len(), limit);
    ///
    /// }
    /// ```
    pub async fn get_asset_reddit(&self, asset: &String, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<CCDataResponse<Vec<CCAssetReddit>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<Vec<CCAssetReddit>>>(
            self.api_key()?,
            CCAPIEndpoint::AssetReddit, CCUnit::NA,
            vec![Param::Asset{v: asset}, Param::ToTimestamp{v: to_timestamp}, Param::Limit{v: limit}],
            Some(String::from("&groups=ID,GENERAL,ACTIVITY,SOURCE"))
        ).await
    }

    /// # Historical Social \[Telegram Day\] (Asset)
    /// Returns daily Telegram metadata for a given asset.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::CCData;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let limit: usize = 2000;
    ///     let telegram = backend.get_asset_telegram(&String::from("SOL"), None, Some(limit)).await.unwrap();
    ///     assert_eq!(telegram.data.unwrap().len(), limit);
    ///
    /// }
    /// ```
    pub async fn get_asset_telegram(&self, asset: &String, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<CCDataResponse<Vec<CCAssetTelegram>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<Vec<CCAssetTelegram>>>(
            self.api_key()?,
            CCAPIEndpoint::AssetTelegram, CCUnit::NA,
            vec![Param::Asset{v: asset}, Param::ToTimestamp{v: to_timestamp}, Param::Limit{v: limit}],
            Some(String::from("&groups=ID,GENERAL,SOURCE"))
        ).await
    }

    /// # Historical Social \[X (Twitter) Day\] (Asset)
    /// Returns daily X (Twitter) metadata for a given asset.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::CCData;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let limit: usize = 2000;
    ///     let twitter = backend.get_asset_twitter(&String::from("SOL"), None, Some(limit)).await.unwrap();
    ///     assert_eq!(twitter.data.unwrap().len(), limit)
    ///
    /// }
    /// ```
    pub async fn get_asset_twitter(&self, asset: &String, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<CCDataResponse<Vec<CCAssetTwitter>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<Vec<CCAssetTwitter>>>(
            self.api_key()?,
            CCAPIEndpoint::AssetTwitter, CCUnit::NA,
            vec![Param::Asset{v: asset}, Param::ToTimestamp{v: to_timestamp}, Param::Limit{v: limit}],
            Some(String::from("&groups=ID,GENERAL,ACTIVITY,SOURCE"))
        ).await
    }

    /// # Latest Articles (News)
    /// Returns a list of latest articles.
    ///
    /// # Description (CCData Description)
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
    /// use ccdata_api::{CCData, CCNewsLang, CCNewsSourceID};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let language: CCNewsLang = CCNewsLang::EN;
    ///     let source_id: CCNewsSourceID = CCNewsSourceID::ForbesDigitalAssets;
    ///     let limit: usize = 100;
    ///     let articles = backend.get_news_latest_articles(language, source_id, None, None, None, Some(limit)).await.unwrap();
    ///     assert_eq!(articles.data.unwrap().len(), limit);
    ///
    /// }
    /// ```
    pub async fn get_news_latest_articles(&self, language: CCNewsLang, source_id: CCNewsSourceID, categories: Option<Vec<String>>,
                                          exclude_categories: Option<Vec<String>>, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<CCDataResponse<Vec<CCNewsLatestArticle>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<Vec<CCNewsLatestArticle>>>(
            self.api_key()?,
            CCAPIEndpoint::NewsLatestArticles, CCUnit::NA,
            vec![
                Param::NewsLanguage{v: language}, Param::NewsSourceID{v: source_id}, Param::NewsCategories{v: categories},
                Param::NewsExcludeCategories{v: exclude_categories}, Param::ToTimestamp{v: to_timestamp}, Param::Limit{v: limit},
            ],
            None
        ).await
    }

    /// # Sources (News)
    /// Returns a list of news sources.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::{CCData, CCNewsLang, CCNewsSourceType, CCNewsStatus};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let language: CCNewsLang = CCNewsLang::EN;
    ///     let source_type: CCNewsSourceType = CCNewsSourceType::RSS;
    ///     let status: CCNewsStatus = CCNewsStatus::ACTIVE;
    ///     let sources = backend.get_news_sources(language, source_type, status).await.unwrap();
    ///     assert_eq!(sources.data.unwrap()[0].source_type, String::from("RSS"));
    ///
    /// }
    /// ```
    pub async fn get_news_sources(&self, language: CCNewsLang, source_type: CCNewsSourceType, status: CCNewsStatus) -> Result<CCDataResponse<Vec<CCNewsSource>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<Vec<CCNewsSource>>>(
            self.api_key()?,
            CCAPIEndpoint::NewsSources, CCUnit::NA,
            vec![Param::NewsLanguage{v: language}, Param::NewsSourceType{v: source_type}, Param::NewsStatus{v: status}],
            None
        ).await
    }

    /// # Categories (News)
    /// Return  a list of news categories.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::{CCData, CCNewsStatus};
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let status: CCNewsStatus = CCNewsStatus::ACTIVE;
    ///     let categories = backend.get_news_categories(status).await.unwrap();
    ///     assert_eq!(categories.data.unwrap()[0].status, String::from("ACTIVE"));
    ///
    /// }
    /// ```
    pub async fn get_news_categories(&self, status: CCNewsStatus) -> Result<CCDataResponse<Vec<CCNewsCategory>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<Vec<CCNewsCategory>>>(
            self.api_key()?,
            CCAPIEndpoint::NewsCategories, CCUnit::NA,
            vec![Param::NewsStatus{v: status}],
            None
        ).await
    }

    /// # MktCap Historical OHLCV \[All Assets Day\] (Overview)
    /// Returns daily historical OHLCV data for all assets.
    ///
    /// # Description (CCData Documentation)
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
    /// use ccdata_api::CCData;
    ///
    /// #[tokio::main]
    /// async fn main() -> () {
    ///
    ///     let mut backend: CCData = CCData::new();
    ///     // Provide API key as the environment variable called API_KEY
    ///     backend.build(&"API_KEY").unwrap();
    ///
    ///     let limit: usize = 2000;
    ///     let mktcap = backend.get_overview_mktcap_ohlcv(None, Some(limit)).await.unwrap();
    ///     assert_eq!(mktcap.data.unwrap().len(), limit);
    ///
    /// }
    /// ```
    pub async fn get_overview_mktcap_ohlcv(&self, to_timestamp: Option<i64>, limit: Option<usize>) -> Result<CCDataResponse<Vec<CCOverviewMktCapOHLCV>>, Box<dyn Error>> {
        call_api_endpoint::<CCDataResponse<Vec<CCOverviewMktCapOHLCV>>>(
            self.api_key()?,
            CCAPIEndpoint::OverviewMktCapOHLCV, CCUnit::NA,
            vec![Param::ToTimestamp{v: to_timestamp}, Param::Limit{v: limit}],
            Some(String::from("&groups=ID,OHLC,VOLUME"))
        ).await
    }
}



#[cfg(test)]
mod tests {
    use crate::backend::CCData;

    #[tokio::test]
    async fn unit_test_backend() -> () {
        let mut backend: CCData = CCData::new();
        backend.build(&"API_KEY").unwrap();
        assert!(backend.api_key != None);
    }
}