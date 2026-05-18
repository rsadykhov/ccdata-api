use std::collections::HashMap;
use ccdata_api::{Unit, CoinDesk};
use ccdata_api::schemas::{self as sh, CoinDeskResponse};
use ccdata_api::schemas::min_api;
use ccdata_api::{IndicesMarket, IndicesOHLCV};
use ccdata_api::{SpotMarket, SpotInstrumentStatus, SpotOHLCV, SpotInstrumentMetdata, SpotMarkets, SpotMarketsInstruments};
use ccdata_api::{FuturesMarket, FuturesOHLCV, FuturesInstrumentMetadata, FuturesMarkets};
use ccdata_api::{OptionsMarket, OptionsOHLCV, OptionsInstrumentMetadata, OptionsMarkets};
use ccdata_api::{DerIndicesMarket, DerIndicesOHLCV, DerIndicesMarkets};
use ccdata_api::{OCDEXMarket, OCDEXOHLCV, OCDEXMarkets};
use ccdata_api::{OCCoreETHBlock, OCCoreAssetByChain, OCCoreAssetByAddress, OCCoreSupply};
use ccdata_api::{AssetMetadata, AssetEvent, AssetCodeRepoMetrics, AssetDiscord, AssetReddit, AssetTelegram, AssetTwitter};
use ccdata_api::{NewsStatus, NewsLang, NewsSourceID, NewsLatestArticle, NewsSourceType, NewsSource, NewsCategory};
use ccdata_api::OverviewMktCapOHLCV;


// Legacy Integration Tests


#[tokio::test]
async fn test_get_balance_distribution() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let limit: usize = 2000;
    let balance_distribution: sh::CCMinResponse<sh::CCMinWrapper<Vec<min_api::BalanceDistribution>>> = backend.get_balance_distribution(None, Some(limit)).await.unwrap();
    assert!(balance_distribution.data.unwrap().data.unwrap().len() <= limit);
}


// Data-API Integration Tests


// Indices & Reference Rates


#[tokio::test]
async fn test_get_indices_ohlcv() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let market: IndicesMarket = IndicesMarket::CADLI;
    let limit: usize = 2000;
    let ohlcv: CoinDeskResponse<Vec<IndicesOHLCV>> = backend.get_indices_ohlcv("BTC-USD", None, Some(limit), market, Unit::Day).await.unwrap();
    assert_eq!(ohlcv.data.unwrap().len(), limit);
}


// Spot


#[tokio::test]
async fn test_get_spot_ohlcv() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let market: SpotMarket = SpotMarket::KRAKEN;
    let limit: usize = 2000;
    let ohlcv: sh::CoinDeskResponse<Vec<SpotOHLCV>> = backend.get_spot_ohlcv("BTC-USD", None, Some(limit), market, Unit::Day).await.unwrap();
    assert_eq!(ohlcv.data.unwrap().len(), limit);
}


#[tokio::test]
async fn test_get_spot_instrument_metadata() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let instruments: Vec<String> = vec![String::from("BTC-USD"), String::from("ETH-USD")];
    let market: SpotMarket = SpotMarket::KRAKEN;
    let instrument_metadata: CoinDeskResponse<HashMap<String, SpotInstrumentMetdata>> = backend.get_spot_instrument_metadata(&instruments, market).await.unwrap();
    assert_eq!(instrument_metadata.data.unwrap().len(), 2);
}


#[tokio::test]
async fn test_get_spot_markets_v2() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build("API_KEY").unwrap();
    let markets: Vec<SpotMarket> = vec![SpotMarket::KRAKEN];
    let markets: CoinDeskResponse<HashMap<String, SpotMarkets>> = backend.get_spot_markets_v2(markets).await.unwrap();
    assert_eq!(markets.data.unwrap().get("kraken").unwrap().exchange_status, String::from("ACTIVE"));
}


#[tokio::test]
async fn test_get_spot_markets_instruments() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let instruments: Vec<String> = vec![String::from("BTC-USD"), String::from("ETH-USD")];
    let market: SpotMarket = SpotMarket::KRAKEN;
    let instrument_status: SpotInstrumentStatus = SpotInstrumentStatus::ACTIVE;
    let markets_instruments: CoinDeskResponse<HashMap<String, SpotMarketsInstruments>> = backend.get_spot_markets_instruments(&instruments, market, instrument_status).await.unwrap();
    assert_eq!(markets_instruments.data.unwrap().get("kraken").unwrap().instruments.len(), 2);
}


// Futures


#[tokio::test]
async fn test_get_futures_ohlcv() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let market: FuturesMarket = FuturesMarket::BINANCE;
    let limit: usize = 2000;
    let ohlcv: CoinDeskResponse<Vec<FuturesOHLCV>> = backend.get_futures_ohlcv("BTC-USDT-VANILLA-PERPETUAL", None, Some(limit), market, Unit::Day).await.unwrap();
    assert!(ohlcv.data.unwrap().len() <= limit);
}


#[tokio::test]
async fn test_get_futures_instrument_metadata() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let instruments: Vec<String> = vec![String::from("BTCUSD_PERP"), String::from("ETH-USDT-VANILLA-PERPETUAL")];
    let market: FuturesMarket = FuturesMarket::BINANCE;
    let futures_metadata: CoinDeskResponse<HashMap<String, FuturesInstrumentMetadata>> = backend.get_futures_instrument_metadata(&instruments, market).await.unwrap();
    assert_eq!(futures_metadata.data.unwrap().len(), 2);
}


#[tokio::test]
async fn test_get_futures_markets_v2() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let markets: Vec<FuturesMarket> = vec![FuturesMarket::BINANCE];
    let markets: CoinDeskResponse<HashMap<String, FuturesMarkets>> = backend.get_futures_markets_v2(markets).await.unwrap();
    assert_eq!(markets.data.unwrap().get("binance").unwrap().exchange_status, String::from("ACTIVE"));
}


// Options


#[tokio::test]
async fn test_get_options_ohlcv() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let market: OptionsMarket = OptionsMarket::OKEX;
    let to_timestamp: Option<i64> = Some(1735084800);
    let limit: usize = 2000;
    let ohlcv: CoinDeskResponse<Vec<OptionsOHLCV>> = backend.get_options_ohlcv("BTC-USD-20241227-15000-P", to_timestamp, Some(limit), market, Unit::Day).await.unwrap();
    assert!(ohlcv.data.unwrap().len() <= limit);
}


#[tokio::test]
async fn test_get_options_instrument_metadata() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let instruments: Vec<String> = vec![String::from("BTC-29NOV24-25000-P"), String::from("ETH-31JAN25-2500-P")];
    let market: OptionsMarket = OptionsMarket::DERIBIT;
    let options_metadata: CoinDeskResponse<HashMap<String, OptionsInstrumentMetadata>> = backend.get_options_instrument_metadata(&instruments, market).await.unwrap();
    assert_eq!(options_metadata.data.unwrap().len(), 2);
}


#[tokio::test]
async fn test_get_options_markets_v2() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let markets: Vec<OptionsMarket> = vec![OptionsMarket::DERIBIT];
    let markets: CoinDeskResponse<HashMap<String, OptionsMarkets>> = backend.get_options_markets_v2(markets).await.unwrap();
    assert_eq!(markets.data.unwrap().get("deribit").unwrap().exchange_status, String::from("ACTIVE"));
}


// Derivatives Indices


#[tokio::test]
async fn test_get_der_indices_ohlcv() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let market: DerIndicesMarket = DerIndicesMarket::BINANCE;
    let limit: usize = 2000;
    let ohlcv: CoinDeskResponse<Vec<DerIndicesOHLCV>> = backend.get_der_indices_ohlcv("BTCUSDT", None, Some(limit), market, Unit::Day).await.unwrap();
    assert!(ohlcv.data.unwrap().len() <= limit);
}


#[tokio::test]
async fn test_get_der_indices_markets_v2() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let markets: Vec<DerIndicesMarket> = vec![DerIndicesMarket::KRAKEN];
    let markets: CoinDeskResponse<HashMap<String, DerIndicesMarkets>> = backend.get_der_indices_markets_v2(markets).await.unwrap();
    assert_eq!(markets.data.unwrap().get("kraken").unwrap().exchange_status, String::from("ACTIVE"));
}


// On-Chain DEX


#[tokio::test]
async fn test_get_ocdex_ohlcv() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let market: OCDEXMarket = OCDEXMarket::UNISWAPV2;
    let limit: usize = 2000;
    let ohlcv: CoinDeskResponse<Vec<OCDEXOHLCV>> = backend.get_ocdex_ohlcv("0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852_2", None, Some(limit), market, Unit::Day).await.unwrap();
    assert!(ohlcv.data.unwrap().len() <= limit);
}


#[tokio::test]
async fn test_get_ocdex_markets_v2() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let markets: Vec<OCDEXMarket> = vec![OCDEXMarket::UNISWAPV2];
    let markets: CoinDeskResponse<HashMap<String, OCDEXMarkets>> = backend.get_ocdex_markets_v2(markets).await.unwrap();
    assert_eq!(markets.data.unwrap().get("uniswapv2").unwrap().exchange_status, String::from("ACTIVE"));
}


// On-Chain Core


#[tokio::test]
async fn test_get_occore_eth_block() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let eth_block: CoinDeskResponse<OCCoreETHBlock> = backend.get_occore_eth_block(19501436).await.unwrap();
    assert_eq!(eth_block.data.unwrap().symbol, String::from("ETH"));
}


#[tokio::test]
async fn test_get_occore_assets_by_chain() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let assets_by_chain: CoinDeskResponse<OCCoreAssetByChain> = backend.get_occore_assets_by_chain("ETH").await.unwrap();
    assert_eq!(assets_by_chain.data.unwrap().chain_asset_summary.symbol, String::from("ETH"));
}


#[tokio::test]
async fn test_get_occore_asset_by_address() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let address: String = String::from("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
    let quote_asset: String = String::from("USD");
    let asset_by_address: CoinDeskResponse<OCCoreAssetByAddress> = backend.get_occore_asset_by_address("ETH", &address, &quote_asset).await.unwrap();
    assert_eq!(asset_by_address.data.unwrap().parent_asset_symbol.unwrap(), String::from("ETH"));
}


#[tokio::test]
async fn test_get_occore_supply() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let limit: usize = 2000;
    let historical_supply: CoinDeskResponse<Vec<OCCoreSupply>> = backend.get_occore_supply("BTC", None, Some(limit)).await.unwrap();
    assert!(historical_supply.data.unwrap().len() <= limit);
}


// Asset


#[tokio::test]
async fn test_get_asset_metadata_v2() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let assets: Vec<String> = vec![String::from("ETH"), String::from("BTC")];
    let metadata: CoinDeskResponse<HashMap<String, AssetMetadata>> = backend.get_asset_metadata_v2(assets).await.unwrap();
    assert_eq!(metadata.data.as_ref().unwrap().get("ETH").unwrap().name, String::from("Ethereum"));
    assert_eq!(metadata.data.unwrap().get("BTC").unwrap().name, String::from("Bitcoin"));
}


#[tokio::test]
async fn test_get_asset_events() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let limit: usize = 100;
    let events: CoinDeskResponse<Vec<AssetEvent>> = backend.get_asset_events("ETH", None, Some(limit)).await.unwrap();
    assert!(events.data.unwrap().len() <= limit);
}


#[tokio::test]
async fn test_get_asset_code_repo() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let limit: usize = 2000;
    let code_repo: CoinDeskResponse<Vec<AssetCodeRepoMetrics>> = backend.get_asset_code_repo("ETH", None, Some(limit)).await.unwrap();
    assert_eq!(code_repo.data.unwrap().len(), limit);
}


#[tokio::test]
async fn test_get_asset_discord() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let limit: usize = 2000;
    let discord: CoinDeskResponse<Vec<AssetDiscord>> = backend.get_asset_discord("ETH", None, Some(limit)).await.unwrap();
    assert!(discord.data.unwrap().len() <= limit);
}


#[tokio::test]
async fn test_get_asset_reddit() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let limit: usize = 2000;
    let reddit: CoinDeskResponse<Vec<AssetReddit>> = backend.get_asset_reddit("ETH", None, Some(limit)).await.unwrap();
    assert!(reddit.data.unwrap().len() <= limit);
}


#[tokio::test]
async fn test_get_asset_telegram() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let limit: usize = 2000;
    let telegram: CoinDeskResponse<Vec<AssetTelegram>> = backend.get_asset_telegram("SOL", None, Some(limit)).await.unwrap();
    assert!(telegram.data.unwrap().len() <= limit);
}


#[tokio::test]
async fn test_get_asset_twitter() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let limit: usize = 2000;
    let twitter: CoinDeskResponse<Vec<AssetTwitter>> = backend.get_asset_twitter("BTC", Some(1779119914), Some(limit)).await.unwrap();
    assert!(twitter.data.unwrap().len() <= limit);
}


// News


#[tokio::test]
async fn test_get_news_latest_articles() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let language: NewsLang = NewsLang::EN;
    let source_id: NewsSourceID = NewsSourceID::ForbesDigitalAssets;
    let limit: usize = 100;
    let articles: CoinDeskResponse<Vec<NewsLatestArticle>> = backend.get_news_latest_articles(language, source_id, None, None, None, Some(limit)).await.unwrap();
    assert_eq!(articles.data.unwrap().len(), limit);
}


#[tokio::test]
async fn test_get_news_sources() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let language: NewsLang = NewsLang::EN;
    let source_type: NewsSourceType = NewsSourceType::RSS;
    let status: NewsStatus = NewsStatus::ACTIVE;
    let sources: CoinDeskResponse<Vec<NewsSource>> = backend.get_news_sources(language, source_type, status).await.unwrap();
    assert_eq!(sources.data.unwrap()[0].source_type, String::from("RSS"));
}


#[tokio::test]
async fn test_get_news_categories() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let status: NewsStatus = NewsStatus::ACTIVE;
    let categories: CoinDeskResponse<Vec<NewsCategory>> = backend.get_news_categories(status).await.unwrap();
    assert_eq!(categories.data.unwrap()[0].status, String::from("ACTIVE"));
}


// Overview


#[tokio::test]
async fn test_get_overview_mktcap_ohlcv() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let limit: usize = 2000;
    let mktcap: CoinDeskResponse<Vec<OverviewMktCapOHLCV>> = backend.get_overview_mktcap_ohlcv(None, Some(limit)).await.unwrap();
    assert_eq!(mktcap.data.unwrap().len(), limit);
}


// Extra test


#[tokio::test]
async fn test_extra() -> () {
    let mut backend: CoinDesk = CoinDesk::new();
    backend.build(&"API_KEY").unwrap();
    let symbol: String = String::from("BTC");
    // let to_timestamp: Option<i64> = Some(1577145600); // 24 Dec 2019
    // let to_timestamp: Option<i64> = Some(1404342000); // 03 Jul 2014
    // let to_timestamp: Option<i64> = Some(1231545600); // 10 Jan 2009
    let to_timestamp: Option<i64> = Some(1231598000);
    let limit: Option<usize> = Some(2000);
    // let data = backend.get_asset_metadata(&symbol).await.unwrap();
    // let data = backend.get_historical_daily(&symbol, to_timestamp, limit).await.unwrap();
    // let data = backend.get_asset_code_repo(&symbol, to_timestamp, limit).await.unwrap();
    // let data = backend.get_occore_supply(&symbol, to_timestamp, limit).await.unwrap();
    let data = backend.get_spot_ohlcv(&format!("{}-USD", symbol), to_timestamp, limit, SpotMarket::KRAKEN, Unit::Day).await.unwrap();
    println!("{:?}", data);
}