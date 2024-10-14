use serde::Deserialize;


pub enum CCNewsStatus {
    ACTIVE,
    INACTIVE,
}

impl CCNewsStatus {
    pub fn to_string(&self) -> String {
        match self {
            CCNewsStatus::ACTIVE => String::from("ACTIVE"),
            CCNewsStatus::INACTIVE => String::from("INACTIVE")
        }
    }
}


pub enum CCNewsLang {
    EN,
    ES,
    TR,
    FR,
    JP,
    PT,
}

impl CCNewsLang {
    pub fn to_string(&self) -> String {
        match self {
            CCNewsLang::EN => String::from("EN"),
            CCNewsLang::ES => String::from("ES"),
            CCNewsLang::TR => String::from("TR"),
            CCNewsLang::FR => String::from("FR"),
            CCNewsLang::JP => String::from("JP"),
            CCNewsLang::PT => String::from("PT"),
        }
    }
}


// News: Latest Articles


pub enum CCNewsSourceID {
    CoinDesk,
    CoinTelegraph,
    BitcoinMagazine,
    CryptoGlobe,
    CoinGape,
    Blockworks,
    TheDailyHodl,
    CryptoSlate,
    CryptoPotato,
    Decrypt,
    CryptoBriefing,
    TheBlock,
    BitcoinDotCom,
    NewsBTC,
    UToday,
    Bitcoinist,
    Coinpedia,
    Cryptonomist,
    CryptoNewsReview,
    CCData,
    Cryptoknowmics,
    CCN,
    FinanceMagnates,
    ETHNewsDotCom,
    CryptoVest,
    CryptoInsider,
    HuobiBlog,
    CoinSpeaker,
    CoinJoker,
    NintyNineBitcoins,
    Cointelligence,
    OKXInsights,
    CryptoCoreMedia,
    Bitcoinerx,
    AMBCrypto,
    Coinpaprika,
    LiveBitcoinNews,
    CryptoCompare,
    BitDegree,
    TheCoinRepublic,
    Chaindd,
    Chaintimes,
    TheCoinRise,
    CryptoNewsZ,
    YahooFinanceBitcoin,
    VauldInsights,
    ZyCrypto,
    KrakenBlog,
    Coincu,
    DailyCoin,
    TrustNodes,
    Coinnounce,
    CoinEdition,
    BitcoinSistemi,
    TheNewsCrypto,
    ForbesDigitalAssets,
    Cryptonews,
    TimesNext,
    EthereumWorldNews,
    CryptoCoinDotNews,
    BTCPulse,
    BloombergCrypto,
    CoinOtag,
    CryptoDotNews,
    Chainwire,
    CryptoIntelligence,
    Coinpaper,
    BitfinexBlog,
    TheCryptoBasic,
    NFTDotNews,
    Blokt,
    BitcoinWorld,
    CryptoDaily,
    TimesTabloid,
    CoinTurkNews,
    Invezz,
    SeekingAlpha,
    Finbold,
    FinancialTimesCrypto,
    Cryptopolitan,
    NullTx,
    TipRanks,
    TheDefiant,
}

impl CCNewsSourceID {
    pub fn to_string(&self) -> String {
        match self {
            CCNewsSourceID::CoinDesk => String::from("coindesk"),
            CCNewsSourceID::CoinTelegraph => String::from("cointelegraph"),
            CCNewsSourceID::BitcoinMagazine => String::from("bitcoinmagazine"),
            CCNewsSourceID::CryptoGlobe => String::from("cryptoglobe"),
            CCNewsSourceID::CoinGape => String::from("coingape"),
            CCNewsSourceID::Blockworks => String::from("blockworks"),
            CCNewsSourceID::TheDailyHodl => String::from("dailyhodl"),
            CCNewsSourceID::CryptoSlate => String::from("cryptoslate"),
            CCNewsSourceID::CryptoPotato => String::from("cryptopotato"),
            CCNewsSourceID::Decrypt => String::from("decrypt"),
            CCNewsSourceID::CryptoBriefing => String::from("cryptobriefing"),
            CCNewsSourceID::TheBlock => String::from("theblock"),
            CCNewsSourceID::BitcoinDotCom => String::from("bitcoin.com"),
            CCNewsSourceID::NewsBTC => String::from("newsbtc"),
            CCNewsSourceID::UToday => String::from("utoday"),
            CCNewsSourceID::Bitcoinist => String::from("bitcoinist"),
            CCNewsSourceID::Coinpedia => String::from("coinpedia"),
            CCNewsSourceID::Cryptonomist => String::from("cryptonomist"),
            CCNewsSourceID::CryptoNewsReview => String::from("cryptonewsreview"),
            CCNewsSourceID::CCData => String::from("ccdata"),
            CCNewsSourceID::Cryptoknowmics => String::from("cryptokowmics"),
            CCNewsSourceID::CCN => String::from("ccn"),
            CCNewsSourceID::FinanceMagnates => String::from("financemagnates"),
            CCNewsSourceID::ETHNewsDotCom => String::from("ethnews.com"),
            CCNewsSourceID::CryptoVest => String::from("cryptovest"),
            CCNewsSourceID::CryptoInsider => String::from("cryptoinsider"),
            CCNewsSourceID::HuobiBlog => String::from("huobi"),
            CCNewsSourceID::CoinSpeaker => String::from("coinspeaker"),
            CCNewsSourceID::CoinJoker => String::from("coinjoker"),
            CCNewsSourceID::NintyNineBitcoins => String::from("99bitcoins"),
            CCNewsSourceID::Cointelligence => String::from("cointelligence"),
            CCNewsSourceID::OKXInsights => String::from("okexinsights"),
            CCNewsSourceID::CryptoCoreMedia => String::from("cryptocoremedia"),            
            CCNewsSourceID::Bitcoinerx => String::from("bitcoinerx"),
            CCNewsSourceID::AMBCrypto => String::from("ambcrypto"),
            CCNewsSourceID::Coinpaprika => String::from("coinpaprika"),
            CCNewsSourceID::LiveBitcoinNews => String::from("livebitcoinnews"),
            CCNewsSourceID::CryptoCompare => String::from("cryptocompare"),
            CCNewsSourceID::BitDegree => String::from("bitdegree"),
            CCNewsSourceID::TheCoinRepublic => String::from("coinrepublic"),
            CCNewsSourceID::Chaindd => String::from("chaindd"),
            CCNewsSourceID::Chaintimes => String::from("chaintimes"),
            CCNewsSourceID::TheCoinRise => String::from("thecoinrise"),
            CCNewsSourceID::CryptoNewsZ => String::from("cryptonewsz"),
            CCNewsSourceID::YahooFinanceBitcoin => String::from("yahoofinance"),
            CCNewsSourceID::VauldInsights => String::from("vauld_insights"),
            CCNewsSourceID::ZyCrypto => String::from("zycrypto"),
            CCNewsSourceID::KrakenBlog => String::from("krakenblog"),
            CCNewsSourceID::Coincu => String::from("coincu"),
            CCNewsSourceID::DailyCoin => String::from("dailycoin"),
            CCNewsSourceID::TrustNodes => String::from("trustnodes"),
            CCNewsSourceID::Coinnounce => String::from("coinnounce"),
            CCNewsSourceID::CoinEdition => String::from("coinquora"),
            CCNewsSourceID::BitcoinSistemi => String::from("bitcoinsistemi"),
            CCNewsSourceID::TheNewsCrypto => String::from("thenewscrypto"),
            CCNewsSourceID::ForbesDigitalAssets => String::from("forbes"),
            CCNewsSourceID::Cryptonews => String::from("cryptonews"),
            CCNewsSourceID::TimesNext => String::from("timesnext"),
            CCNewsSourceID::EthereumWorldNews => String::from("ethereumworldnews"),
            CCNewsSourceID::CryptoCoinDotNews => String::from("cryptocoinnews"),
            CCNewsSourceID::BTCPulse => String::from("btcpulse"),
            CCNewsSourceID::BloombergCrypto => String::from("bloomberg_crypto_"),
            CCNewsSourceID::CoinOtag => String::from("coinotag"),
            CCNewsSourceID::CryptoDotNews => String::from("crypto_news"),
            CCNewsSourceID::Chainwire => String::from("chainwire"),
            CCNewsSourceID::CryptoIntelligence => String::from("cryptointelligence"),
            CCNewsSourceID::Coinpaper => String::from("coinpaper"),
            CCNewsSourceID::BitfinexBlog => String::from("bitfinexblog"),
            CCNewsSourceID::TheCryptoBasic => String::from("thecryptobasic"),
            CCNewsSourceID::NFTDotNews => String::from("nft_news"),
            CCNewsSourceID::Blokt => String::from("blokt"),
            CCNewsSourceID::BitcoinWorld => String::from("bitcoinworld"),
            CCNewsSourceID::CryptoDaily => String::from("cryptodaily"),
            CCNewsSourceID::TimesTabloid => String::from("timestabloid"),
            CCNewsSourceID::CoinTurkNews => String::from("cointurken"),
            CCNewsSourceID::Invezz => String::from("invezz"),
            CCNewsSourceID::SeekingAlpha => String::from("seekingalpha"),
            CCNewsSourceID::Finbold => String::from("finbold"),
            CCNewsSourceID::FinancialTimesCrypto => String::from("financial_times_"),
            CCNewsSourceID::Cryptopolitan => String::from("cryptopolitan"),
            CCNewsSourceID::NullTx => String::from("themerkle"),
            CCNewsSourceID::TipRanks => String::from("tipranks"),
            CCNewsSourceID::TheDefiant => String::from("thedefiant"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CCCategoryData {
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "NAME")]
    pub name: String,
    #[serde(rename = "CATEGORY")]
    pub category: String,
}

/// News: Latest Articles
#[derive(Deserialize, Debug)]
pub struct CCNewsLatestArticle {
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "GUID")]
    pub guid: String,
    #[serde(rename = "PUBLISHED_ON")]
    pub published_on: i64,
    #[serde(rename = "IMAGE_URL")]
    pub image_url: String,
    #[serde(rename = "TITLE")]
    pub title: String,
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "SOURCE_ID")]
    pub source_id: i32,
    #[serde(rename = "BODY")]
    pub body: String,
    #[serde(rename = "KEYWORDS")]
    pub keywords: String,
    #[serde(rename = "LANG")]
    pub lang: String,
    #[serde(rename = "UPVOTES")]
    pub upvotes: i32,
    #[serde(rename = "DOWNVOTES")]
    pub downvotes: i32,
    #[serde(rename = "SCORE")]
    pub score: i32,
    #[serde(rename = "SENTIMENT")]
    pub sentiment: String,
    #[serde(rename = "STATUS")]
    pub status: String,
    #[serde(rename = "CREATED_ON")]
    pub created_on: i64,
    #[serde(rename = "UPDATED_ON")]
    pub updated_on: i64,
    #[serde(rename = "SOURCE_DATA")]
    pub source_data: CCNewsSource,
    #[serde(rename = "CATEGORY_DATA")]
    pub category_date: Vec<CCCategoryData>,
}



// News: Sources


pub enum CCNewsSourceType {
    RSS,
    API,
    TWITTER,
}

impl CCNewsSourceType {
    pub fn to_string(&self) -> String {
        match self {
            CCNewsSourceType::RSS => String::from("RSS"),
            CCNewsSourceType::API => String::from("API"),
            CCNewsSourceType::TWITTER => String::from("TWITTER"),
        }
    }
}


/// News: Sources
#[derive(Deserialize, Debug)]
pub struct CCNewsSource {
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "SOURCE_KEY")]
    pub source_key: String,
    #[serde(rename = "NAME")]
    pub name: String,
    #[serde(rename = "IMAGE_URL")]
    pub image_url: String,
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "LANG")]
    pub lang: String,
    #[serde(rename = "SOURCE_TYPE")]
    pub source_type: String,
    #[serde(rename = "LAUNCH_DATE")]
    pub launch_date: Option<i64>,
    #[serde(rename = "SORT_ORDER")]
    pub sort_order: i32,
    #[serde(rename = "BENCHMARK_SCORE")]
    pub benchmark_score: i32,
    #[serde(rename = "STATUS")]
    pub status: String,
    #[serde(rename = "LAST_UPDATED_TS")]
    pub last_updated_ts: i64,
    #[serde(rename = "CREATED_ON")]
    pub created_on: i64,
    #[serde(rename = "UPDATED_ON")]
    pub updated_on: i64,
}


// News: Categories


#[derive(Deserialize, Debug)]
pub struct CCCategoryFilter {
    #[serde(rename = "INCLUDED_WORDS")]
    pub included_words: Option<Vec<String>>,
    #[serde(rename = "INCLUDED_PHRASES")]
    pub included_phrases: Option<Vec<String>>,
    #[serde(rename = "EXCLUDED_PHRASES")]
    pub excluded_phrases: Option<Vec<String>>,
}


/// News: Categories
#[derive(Deserialize, Debug)]
pub struct CCNewsCategory {
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "NAME")]
    pub name: String,
    #[serde(rename = "FILTER")]
    pub filter: Option<CCCategoryFilter>,
    #[serde(rename = "STATUS")]
    pub status: String,
    #[serde(rename = "CREATED_ON")]
    pub created_on: i64,
    #[serde(rename = "UPDATED_ON")]
    pub updated_on: Option<i64>,
}