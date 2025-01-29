use std::collections::HashMap;
use ccdata_api::{CCUnit, CCData};
use ccdata_api::schemas::{self as sh, CCDataResponse};
use ccdata_api::schemas::min_api;
use ccdata_api::{CCIndicesMarket, CCIndicesOHLCV};
use ccdata_api::{CCSpotMarket, CCSpotInstrumentStatus, CCSpotOHLCV, CCSpotInstrumentMetdata, CCSpotMarkets, CCSpotMarketsInstruments};
use ccdata_api::{CCFuturesMarket, CCFuturesOHLCV, CCFuturesMarkets};
use ccdata_api::{CCOptionsMarket, CCOptionsOHLCV, CCOptionsMarkets};
use ccdata_api::{CCDerIndicesMarket, CCDerIndicesOHLCV, CCDerIndicesMarkets};
use ccdata_api::{CCOCDEXMarket, CCOCDEXOHLCV, CCOCDEXMarkets};
use ccdata_api::{CCOCCoreETHBlock, CCOCCoreAssetByChain, CCOCCoreAssetByAddress, CCOCCoreSupply};
use ccdata_api::{CCAssetMetadata, CCAssetEvent, CCAssetCodeRepoMetrics, CCAssetDiscord, CCAssetReddit, CCAssetTelegram, CCAssetTwitter};
use ccdata_api::{CCNewsStatus, CCNewsLang, CCNewsSourceID, CCNewsLatestArticle, CCNewsSourceType, CCNewsSource, CCNewsCategory};
use ccdata_api::CCOverviewMktCapOHLCV;


// Min-API Integration Tests


#[tokio::test]
async fn test_get_available_coin_list() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let available_coin_list: sh::CCMinResponse<HashMap<String, min_api::CCAvailableCoinList>> = backend.get_available_coin_list().await.unwrap();
    assert!(0 < available_coin_list.data.unwrap().len());
}


#[tokio::test]
async fn test_get_historical_daily() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let limit: usize = 2000;
    let historical_daily: sh::CCMinResponse<sh::CCMinWrapper<Vec<min_api::CCHistoricalDaily>>> = backend.get_historical_daily(&String::from("ETH"), None, Some(limit)).await.unwrap();
    assert_eq!(historical_daily.data.unwrap().data.unwrap().len(), limit);
}


#[tokio::test]
async fn test_get_balance_distribution() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let limit: usize = 2000;
    let balance_distribution: sh::CCMinResponse<sh::CCMinWrapper<Vec<min_api::CCBalanceDistribution>>> = backend.get_balance_distribution(None, Some(limit)).await.unwrap();
    assert!(balance_distribution.data.unwrap().data.unwrap().len() <= limit);
}


// Data-API Integration Tests


// Indices & Reference Rates


#[tokio::test]
async fn test_get_indices_ohlcv() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let market: CCIndicesMarket = CCIndicesMarket::CADLI;
    let limit: usize = 2000;
    let ohlcv: CCDataResponse<Vec<CCIndicesOHLCV>> = backend.get_indices_ohlcv(&String::from("BTC-USD"), None, Some(limit), market, CCUnit::Day).await.unwrap();
    assert_eq!(ohlcv.data.unwrap().len(), limit);
}


// Spot


#[tokio::test]
async fn test_get_spot_ohlcv() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let market: CCSpotMarket = CCSpotMarket::KRAKEN;
    let limit: usize = 2000;
    let ohlcv: sh::CCDataResponse<Vec<CCSpotOHLCV>> = backend.get_spot_ohlcv(&String::from("BTC-USD"), None, Some(limit), market, CCUnit::Day).await.unwrap();
    assert_eq!(ohlcv.data.unwrap().len(), limit);
}


#[tokio::test]
async fn test_get_spot_instrument_metadata() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let instruments: Vec<String> = vec![String::from("BTC-USD"), String::from("ETH-USD")];
    let market: CCSpotMarket = CCSpotMarket::KRAKEN;
    let instrument_metadata: CCDataResponse<HashMap<String, CCSpotInstrumentMetdata>> = backend.get_spot_instrument_metadata(&instruments, market).await.unwrap();
    assert_eq!(instrument_metadata.data.unwrap().len(), 2);
}


#[tokio::test]
async fn test_get_spot_markets() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let market: CCSpotMarket = CCSpotMarket::KRAKEN;
    let markets: CCDataResponse<HashMap<String, CCSpotMarkets>> = backend.get_spot_markets(market).await.unwrap();
    assert_eq!(markets.data.unwrap().get("kraken").unwrap().exchange_status, String::from("ACTIVE"));
}


#[tokio::test]
async fn test_get_spot_markets_instruments() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let instruments: Vec<String> = vec![String::from("BTC-USD"), String::from("ETH-USD")];
    let market: CCSpotMarket = CCSpotMarket::KRAKEN;
    let instrument_status: CCSpotInstrumentStatus = CCSpotInstrumentStatus::ACTIVE;
    let markets_instruments: CCDataResponse<HashMap<String, CCSpotMarketsInstruments>> = backend.get_spot_markets_instruments(&instruments, market, instrument_status).await.unwrap();
    assert_eq!(markets_instruments.data.unwrap().get("kraken").unwrap().instruments.len(), 2);
}


// Futures


#[tokio::test]
async fn test_get_futures_ohlcv() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let market: CCFuturesMarket = CCFuturesMarket::BINANCE;
    let limit: usize = 2000;
    let ohlcv: CCDataResponse<Vec<CCFuturesOHLCV>> = backend.get_futures_ohlcv(&String::from("BTC-USDT-VANILLA-PERPETUAL"), None, Some(limit), market, CCUnit::Day).await.unwrap();
    assert!(ohlcv.data.unwrap().len() <= limit);
}


#[tokio::test]
async fn test_get_futures_markets() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let market: CCFuturesMarket = CCFuturesMarket::BINANCE;
    let markets: CCDataResponse<HashMap<String, CCFuturesMarkets>> = backend.get_futures_markets(market).await.unwrap();
    assert_eq!(markets.data.unwrap().get("binance").unwrap().exchange_status, String::from("ACTIVE"));
}


// Options


#[tokio::test]
async fn test_get_options_ohlcv() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let market: CCOptionsMarket = CCOptionsMarket::OKEX;
    let limit: usize = 2000;
    let ohlcv: CCDataResponse<Vec<CCOptionsOHLCV>> = backend.get_options_ohlcv(&String::from("BTC-USD-20241227-15000-P"), None, Some(limit), market, CCUnit::Day).await.unwrap();
    assert!(ohlcv.data.unwrap().len() <= limit);
}


#[tokio::test]
async fn test_get_options_markets() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let market: CCOptionsMarket = CCOptionsMarket::DERIBIT;
    let markets: CCDataResponse<HashMap<String, CCOptionsMarkets>> = backend.get_options_markets(market).await.unwrap();
    assert_eq!(markets.data.unwrap().get("deribit").unwrap().exchange_status, String::from("ACTIVE"));
}


// Derivatives Indices


#[tokio::test]
async fn test_get_der_indices_ohlcv() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let market: CCDerIndicesMarket = CCDerIndicesMarket::BINANCE;
    let limit: usize = 2000;
    let ohlcv: CCDataResponse<Vec<CCDerIndicesOHLCV>> = backend.get_der_indices_ohlcv(&String::from("BTCUSDT"), None, Some(limit), market, CCUnit::Day).await.unwrap();
    assert!(ohlcv.data.unwrap().len() <= limit);
}


#[tokio::test]
async fn test_get_der_indices_markets() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let market: CCDerIndicesMarket = CCDerIndicesMarket::KRAKEN;
    let markets: CCDataResponse<HashMap<String, CCDerIndicesMarkets>> = backend.get_der_indices_markets(market).await.unwrap();
    assert_eq!(markets.data.unwrap().get("kraken").unwrap().exchange_status, String::from("ACTIVE"));
}


// On-Chain DEX


#[tokio::test]
async fn test_get_ocdex_ohlcv() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let market: CCOCDEXMarket = CCOCDEXMarket::UNISWAPV2;
    let limit: usize = 2000;
    let ohlcv: CCDataResponse<Vec<CCOCDEXOHLCV>> = backend.get_ocdex_ohlcv(&String::from("0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852_2"), None, Some(limit), market, CCUnit::Day).await.unwrap();
    assert!(ohlcv.data.unwrap().len() <= limit);
}


#[tokio::test]
async fn test_get_ocdex_markets() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let market: CCOCDEXMarket = CCOCDEXMarket::UNISWAPV2;
    let markets: CCDataResponse<HashMap<String, CCOCDEXMarkets>> = backend.get_ocdex_markets(market).await.unwrap();
    assert_eq!(markets.data.unwrap().get("uniswapv2").unwrap().exchange_status, String::from("ACTIVE"));
}


// On-Chain Core


#[tokio::test]
async fn test_get_occore_eth_block() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let eth_block: CCDataResponse<CCOCCoreETHBlock> = backend.get_occore_eth_block(19501436).await.unwrap();
    assert_eq!(eth_block.data.unwrap().symbol, String::from("ETH"));
}


#[tokio::test]
async fn test_get_occore_assets_by_chain() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let assets_by_chain: CCDataResponse<CCOCCoreAssetByChain> = backend.get_occore_assets_by_chain(&String::from("ETH")).await.unwrap();
    assert_eq!(assets_by_chain.data.unwrap().chain_asset_summary.symbol, String::from("ETH"));
}


#[tokio::test]
async fn test_get_occore_asset_by_address() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let address: String = String::from("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
    let quote_asset: String = String::from("USD");
    let asset_by_address: CCDataResponse<CCOCCoreAssetByAddress> = backend.get_occore_asset_by_address(&String::from("ETH"), &address, &quote_asset).await.unwrap();
    assert_eq!(asset_by_address.data.unwrap().parent_asset_symbol.unwrap(), String::from("ETH"));
}


#[tokio::test]
async fn test_get_occore_supply() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let limit: usize = 2000;
    let historical_supply: CCDataResponse<Vec<CCOCCoreSupply>> = backend.get_occore_supply(&String::from("BTC"), None, Some(limit)).await.unwrap();
    assert_eq!(historical_supply.data.unwrap().len(), limit);
}


// Asset


#[tokio::test]
async fn test_get_asset_metadata() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let metadata: CCDataResponse<CCAssetMetadata> = backend.get_asset_metadata(&String::from("ETH")).await.unwrap();
    assert_eq!(metadata.data.unwrap().name, String::from("Ethereum"));
}


#[tokio::test]
async fn test_get_asset_events() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let limit: usize = 100;
    let events: CCDataResponse<Vec<CCAssetEvent>> = backend.get_asset_events(&String::from("ETH"), None, Some(limit)).await.unwrap();
    assert!(events.data.unwrap().len() <= limit);
}


#[tokio::test]
async fn test_get_asset_code_repo() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let limit: usize = 2000;
    let code_repo: CCDataResponse<Vec<CCAssetCodeRepoMetrics>> = backend.get_asset_code_repo(&String::from("ETH"), None, Some(limit)).await.unwrap();
    assert_eq!(code_repo.data.unwrap().len(), limit);
}


#[tokio::test]
async fn test_get_asset_discord() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let limit: usize = 2000;
    let discord: CCDataResponse<Vec<CCAssetDiscord>> = backend.get_asset_discord(&String::from("ETH"), None, Some(limit)).await.unwrap();
    assert_eq!(discord.data.unwrap().len(), limit);
}


#[tokio::test]
async fn test_get_asset_reddit() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let limit: usize = 2000;
    let reddit: CCDataResponse<Vec<CCAssetReddit>> = backend.get_asset_reddit(&String::from("ETH"), None, Some(limit)).await.unwrap();
    assert_eq!(reddit.data.unwrap().len(), limit);
}


#[tokio::test]
async fn test_get_asset_telegram() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let limit: usize = 2000;
    let telegram: CCDataResponse<Vec<CCAssetTelegram>> = backend.get_asset_telegram(&String::from("SOL"), None, Some(limit)).await.unwrap();
    assert_eq!(telegram.data.unwrap().len(), limit);
}


#[tokio::test]
async fn test_get_asset_twitter() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let limit: usize = 2000;
    let twitter: CCDataResponse<Vec<CCAssetTwitter>> = backend.get_asset_twitter(&String::from("SOL"), None, Some(limit)).await.unwrap();
    assert_eq!(twitter.data.unwrap().len(), limit);
}


// News


#[tokio::test]
async fn test_get_news_latest_articles() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let language: CCNewsLang = CCNewsLang::EN;
    let source_id: CCNewsSourceID = CCNewsSourceID::ForbesDigitalAssets;
    let limit: usize = 100;
    let articles: CCDataResponse<Vec<CCNewsLatestArticle>> = backend.get_news_latest_articles(language, source_id, None, None, None, Some(limit)).await.unwrap();
    assert_eq!(articles.data.unwrap().len(), limit);
}


#[tokio::test]
async fn test_get_news_sources() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let language: CCNewsLang = CCNewsLang::EN;
    let source_type: CCNewsSourceType = CCNewsSourceType::RSS;
    let status: CCNewsStatus = CCNewsStatus::ACTIVE;
    let sources: CCDataResponse<Vec<CCNewsSource>> = backend.get_news_sources(language, source_type, status).await.unwrap();
    assert_eq!(sources.data.unwrap()[0].source_type, String::from("RSS"));
}


#[tokio::test]
async fn test_get_news_categories() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let status: CCNewsStatus = CCNewsStatus::ACTIVE;
    let categories: CCDataResponse<Vec<CCNewsCategory>> = backend.get_news_categories(status).await.unwrap();
    assert_eq!(categories.data.unwrap()[0].status, String::from("ACTIVE"));
}


// Overview


#[tokio::test]
async fn test_get_overview_mktcap_ohlcv() -> () {
    let mut backend: CCData = CCData::new();
    backend.build(&"API_KEY").unwrap();
    let limit: usize = 2000;
    let mktcap: CCDataResponse<Vec<CCOverviewMktCapOHLCV>> = backend.get_overview_mktcap_ohlcv(None, Some(limit)).await.unwrap();
    assert_eq!(mktcap.data.unwrap().len(), limit);
}