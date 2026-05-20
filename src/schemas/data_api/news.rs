use std::fmt::Display;
use serde::{Serialize, Deserialize};


#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
/// List of statuses used to filter articles based on their source integration state.
/// Current allowed values are: ACTIVE, INACTIVE. An ACTIVE status indicates that the source's integration, such as an RSS feed,
/// is currently operational and actively providing data. An INACTIVE status is assigned to sources that have either blocked access,
/// disabled their integration mechanisms, or ceased operations. More statuses may be added in the future as source integration scenarios evolve.
pub enum NewsStatus {
    #[default]
    ACTIVE,
    INACTIVE,
}

impl Display for NewsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
            Self::ACTIVE => write!(f, "ACTIVE"),
            Self::INACTIVE => write!(f, "INACTIVE")
        } 
    }
}


#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
/// The preferred language for the sources or articles - English (EN), Espanol (ES), Turkish (TR), French (FR), Japanese (JP), Portuguese (PT).
pub enum NewsLang {
    #[default]
    EN,
    ES,
    TR,
    FR,
    JP,
    PT,
}

impl Display for NewsLang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EN => write!(f, "EN"),
            Self::ES => write!(f, "ES"),
            Self::TR => write!(f, "TR"),
            Self::FR => write!(f, "FR"),
            Self::JP => write!(f, "JP"),
            Self::PT => write!(f, "PT"),
        }
    }
}


// News: Latest Articles


#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
/// Get articles from specific sources based on their keys. If left empty it will just return news from ACTIVE sources for the selected language,
/// if you want to get inactive sources as well, please pass them in alongside the active ones in this array.
pub enum NewsSourceID {
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

impl Display for NewsSourceID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CoinDesk => write!(f, "coindesk"),
            Self::CoinTelegraph => write!(f, "cointelegraph"),
            Self::BitcoinMagazine => write!(f, "bitcoinmagazine"),
            Self::CryptoGlobe => write!(f, "cryptoglobe"),
            Self::CoinGape => write!(f, "coingape"),
            Self::Blockworks => write!(f, "blockworks"),
            Self::TheDailyHodl => write!(f, "dailyhodl"),
            Self::CryptoSlate => write!(f, "cryptoslate"),
            Self::CryptoPotato => write!(f, "cryptopotato"),
            Self::Decrypt => write!(f, "decrypt"),
            Self::CryptoBriefing => write!(f, "cryptobriefing"),
            Self::TheBlock => write!(f, "theblock"),
            Self::BitcoinDotCom => write!(f, "bitcoin.com"),
            Self::NewsBTC => write!(f, "newsbtc"),
            Self::UToday => write!(f, "utoday"),
            Self::Bitcoinist => write!(f, "bitcoinist"),
            Self::Coinpedia => write!(f, "coinpedia"),
            Self::Cryptonomist => write!(f, "cryptonomist"),
            Self::CryptoNewsReview => write!(f, "cryptonewsreview"),
            Self::CCData => write!(f, "ccdata"),
            Self::Cryptoknowmics => write!(f, "cryptokowmics"),
            Self::CCN => write!(f, "ccn"),
            Self::FinanceMagnates => write!(f, "financemagnates"),
            Self::ETHNewsDotCom => write!(f, "ethnews.com"),
            Self::CryptoVest => write!(f, "cryptovest"),
            Self::CryptoInsider => write!(f, "cryptoinsider"),
            Self::HuobiBlog => write!(f, "huobi"),
            Self::CoinSpeaker => write!(f, "coinspeaker"),
            Self::CoinJoker => write!(f, "coinjoker"),
            Self::NintyNineBitcoins => write!(f, "99bitcoins"),
            Self::Cointelligence => write!(f, "cointelligence"),
            Self::OKXInsights => write!(f, "okexinsights"),
            Self::CryptoCoreMedia => write!(f, "cryptocoremedia"),            
            Self::Bitcoinerx => write!(f, "bitcoinerx"),
            Self::AMBCrypto => write!(f, "ambcrypto"),
            Self::Coinpaprika => write!(f, "coinpaprika"),
            Self::LiveBitcoinNews => write!(f, "livebitcoinnews"),
            Self::CryptoCompare => write!(f, "cryptocompare"),
            Self::BitDegree => write!(f, "bitdegree"),
            Self::TheCoinRepublic => write!(f, "coinrepublic"),
            Self::Chaindd => write!(f, "chaindd"),
            Self::Chaintimes => write!(f, "chaintimes"),
            Self::TheCoinRise => write!(f, "thecoinrise"),
            Self::CryptoNewsZ => write!(f, "cryptonewsz"),
            Self::YahooFinanceBitcoin => write!(f, "yahoofinance"),
            Self::VauldInsights => write!(f, "vauld_insights"),
            Self::ZyCrypto => write!(f, "zycrypto"),
            Self::KrakenBlog => write!(f, "krakenblog"),
            Self::Coincu => write!(f, "coincu"),
            Self::DailyCoin => write!(f, "dailycoin"),
            Self::TrustNodes => write!(f, "trustnodes"),
            Self::Coinnounce => write!(f, "coinnounce"),
            Self::CoinEdition => write!(f, "coinquora"),
            Self::BitcoinSistemi => write!(f, "bitcoinsistemi"),
            Self::TheNewsCrypto => write!(f, "thenewscrypto"),
            Self::ForbesDigitalAssets => write!(f, "forbes"),
            Self::Cryptonews => write!(f, "cryptonews"),
            Self::TimesNext => write!(f, "timesnext"),
            Self::EthereumWorldNews => write!(f, "ethereumworldnews"),
            Self::CryptoCoinDotNews => write!(f, "cryptocoinnews"),
            Self::BTCPulse => write!(f, "btcpulse"),
            Self::BloombergCrypto => write!(f, "bloomberg_crypto_"),
            Self::CoinOtag => write!(f, "coinotag"),
            Self::CryptoDotNews => write!(f, "crypto_news"),
            Self::Chainwire => write!(f, "chainwire"),
            Self::CryptoIntelligence => write!(f, "cryptointelligence"),
            Self::Coinpaper => write!(f, "coinpaper"),
            Self::BitfinexBlog => write!(f, "bitfinexblog"),
            Self::TheCryptoBasic => write!(f, "thecryptobasic"),
            Self::NFTDotNews => write!(f, "nft_news"),
            Self::Blokt => write!(f, "blokt"),
            Self::BitcoinWorld => write!(f, "bitcoinworld"),
            Self::CryptoDaily => write!(f, "cryptodaily"),
            Self::TimesTabloid => write!(f, "timestabloid"),
            Self::CoinTurkNews => write!(f, "cointurken"),
            Self::Invezz => write!(f, "invezz"),
            Self::SeekingAlpha => write!(f, "seekingalpha"),
            Self::Finbold => write!(f, "finbold"),
            Self::FinancialTimesCrypto => write!(f, "financial_times_"),
            Self::Cryptopolitan => write!(f, "cryptopolitan"),
            Self::NullTx => write!(f, "themerkle"),
            Self::TipRanks => write!(f, "tipranks"),
            Self::TheDefiant => write!(f, "thedefiant"),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CategoryData {
    #[serde(rename = "TYPE")]
    /// Type of the message.
    pub type_: String,
    #[serde(rename = "ID")]
    /// The unique identifier for the news category entry.
    pub id: i32,
    #[serde(rename = "NAME")]
    /// The name of the news category.
    pub name: String,
    #[serde(rename = "CATEGORY")]
    pub category: String,
}

/// News: Latest Articles
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NewsLatestArticle {
    #[serde(rename = "TYPE")]
    /// Type of the message.
    pub type_: String,
    #[serde(rename = "ID")]
    /// The unique identifier for the article entry.
    pub id: i32,
    #[serde(rename = "GUID")]
    /// The Global Unique Identifier (GUID) of the article. A GUID is a unique reference number used as an identifier in computer systems.
    /// The "Article guid" is unique for every article and serves as its distinct identifier.
    pub guid: String,
    #[serde(rename = "PUBLISHED_ON")]
    /// The unix timestamp when the article was first published. This information is useful for understanding the timeline of the content,
    /// sorting articles by date, or determining the recency of the information.
    pub published_on: i64,
    #[serde(rename = "IMAGE_URL")]
    /// The URL, or web address, of the article's associated or featured image. This URL can be used to retrieve the image for display,
    /// providing a visual context for the article content. The URL points directly to the location where the image file is stored online.
    pub image_url: String,
    #[serde(rename = "TITLE")]
    /// The heading or title of a specific article. It's a text string that gives a concise description of the article's content.
    pub title: String,
    #[serde(rename = "URL")]
    /// The web address that directs to the specific content or article on a source website. It's a unique URL used for directly
    /// linking to the article or for fetching additional data from the article page.
    pub url: String,
    #[serde(rename = "SOURCE_ID")]
    /// The unique identifier for the source of the article or content. The "SOURCE_ID" allows for easy tracking and categorization of articles
    /// based on their origin, facilitating analysis by source, or fetching additional content from the same source.
    pub source_id: i32,
    #[serde(rename = "BODY")]
    /// The main textual content of the article. It includes the substance of the article but it it generally very limited since sources want
    /// clients to visit their website. This is where the primary information of the article is found.
    pub body: String,
    #[serde(rename = "KEYWORDS")]
    /// A list of words or phrases that are relevant to the content of the article. These keywords are given by the source and serve as
    /// a summary of the main themes, topics, or subjects covered in the article.
    pub keywords: String,
    #[serde(rename = "LANG")]
    /// The article Preferred language - English (EN), Portuguese (PT), Espanol (ES), Turkish (TR), French (FR).
    pub lang: String,
    #[serde(rename = "UPVOTES")]
    /// The number of upvotes this article has.
    pub upvotes: i32,
    #[serde(rename = "DOWNVOTES")]
    /// The number of downvotes this article has.
    pub downvotes: i32,
    #[serde(rename = "SCORE")]
    /// The score of this article.
    pub score: i32,
    #[serde(rename = "SENTIMENT")]
    /// The sentiment polarity of this article. We compute this using ChatGPT.
    pub sentiment: String,
    #[serde(rename = "STATUS")]
    /// The status for the Article. Allowed values: ACTIVE, DELETED.
    pub status: String,
    #[serde(rename = "CREATED_ON")]
    /// Article internal creation unix ts in our system.
    pub created_on: i64,
    #[serde(rename = "UPDATED_ON")]
    /// Article internal last updated unix ts in our system.
    pub updated_on: Option<i64>,
    #[serde(rename = "SOURCE_DATA")]
    /// The news source data of this article.
    pub source_data: NewsSource,
    #[serde(rename = "CATEGORY_DATA")]
    /// An array of categories this article belongs to.
    pub category_date: Vec<CategoryData>,
}



// News: Sources


#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
/// Specifies the type of integration used by the news source. Current allowed values are RSS, API, and TWITTER.
/// 'RSS' indicates a source that distributes content via RSS feeds.
/// 'API' refers to sources that provide data through a standardized programming interface.
/// 'TWITTER' represents sources that disseminate information directly through Twitter.
/// This parameter helps in selecting the method through which news content is retrieved.
pub enum NewsSourceType {
    #[default]
    RSS,
    API,
    TWITTER,
}

impl Display for NewsSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RSS => write!(f, "RSS"),
            Self::API => write!(f, "API"),
            Self::TWITTER => write!(f, "TWITTER"),
        }
    }
}


/// News: Sources
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NewsSource {
    #[serde(rename = "TYPE")]
    /// Type of the message.
    pub type_: String,
    #[serde(rename = "ID")]
    /// The unique identifier for the news source entry.
    pub id: i32,
    #[serde(rename = "SOURCE_KEY")]
    /// The unique key for a news source.
    pub source_key: String,
    #[serde(rename = "NAME")]
    /// The name of the news source".
    pub name: String,
    #[serde(rename = "IMAGE_URL")]
    /// The image url for the article source.
    pub image_url: Option<String>,
    #[serde(rename = "URL")]
    /// The URL of the news source.
    pub url: String,
    #[serde(rename = "LANG")]
    /// The Article Source Preferred language - English (EN), Portuguese (PT), Espanol (ES), Turkish (TR), French (FR).
    pub lang: String,
    #[serde(rename = "SOURCE_TYPE")]
    /// RSS, API, TWITTER.
    pub source_type: String,
    #[serde(rename = "LAUNCH_DATE")]
    /// The launch date of the source is indicated as (yyyy-mm-dd).
    pub launch_date: Option<i64>,
    #[serde(rename = "SORT_ORDER")]
    /// Internal sort order of the source to define the field overwrites.
    pub sort_order: i32,
    #[serde(rename = "BENCHMARK_SCORE")]
    pub benchmark_score: i32,
    #[serde(rename = "STATUS")]
    /// The status for the Article Source entry. Allowed values: ACTIVE, INACTIVE".
    pub status: String,
    #[serde(rename = "LAST_UPDATED_TS")]
    /// The last script update timestamp for this article source.
    pub last_updated_ts: i64,
    #[serde(rename = "CREATED_ON")]
    /// Article Source internal creation unix ts in our system.
    pub created_on: i64,
    #[serde(rename = "UPDATED_ON")]
    /// Article Source internal last updated unix ts in our system.
    pub updated_on: Option<i64>,
}


// News: Categories


/// The filters for the news category.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CategoryFilter {
    #[serde(rename = "INCLUDED_WORDS")]
    /// Words related or included in news category.
    pub included_words: Option<Vec<String>>,
    #[serde(rename = "INCLUDED_PHRASES")]
    /// Phrases that should be included in news category.
    pub included_phrases: Option<Vec<String>>,
    #[serde(rename = "EXCLUDED_PHRASES")]
    /// Phrases to should be excluded from news category.
    pub excluded_phrases: Option<Vec<String>>,
}


/// News: Categories
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NewsCategory {
    #[serde(rename = "TYPE")]
    /// Type of the message.
    pub type_: String,
    #[serde(rename = "ID")]
    /// The unique identifier for the news category entry.
    pub id: i32,
    #[serde(rename = "NAME")]
    /// The name of the news category.
    pub name: String,
    #[serde(rename = "FILTER")]
    /// The filters for the news category.
    pub filter: Option<CategoryFilter>,
    #[serde(rename = "STATUS")]
    /// The status for the News category entry. Allowed values: ACTIVE, INACTIVE.
    pub status: String,
    #[serde(rename = "CREATED_ON")]
    /// News category internal creation unix ts in our system
    pub created_on: i64,
    #[serde(rename = "UPDATED_ON")]
    /// News category internal last updated unix ts in our system.
    pub updated_on: Option<i64>,
}