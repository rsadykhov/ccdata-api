use serde::Deserialize;


/// List of statuses used to filter articles based on their source integration state.
/// Current allowed values are: ACTIVE, INACTIVE. An ACTIVE status indicates that the source's integration, such as an RSS feed,
/// is currently operational and actively providing data. An INACTIVE status is assigned to sources that have either blocked access,
/// disabled their integration mechanisms, or ceased operations. More statuses may be added in the future as source integration scenarios evolve.
pub enum CCNewsStatus {
    ACTIVE,
    INACTIVE,
}

impl CCNewsStatus {
    /// Converts enum value to `String`
    pub fn to_string(&self) -> String {
        match self {
            CCNewsStatus::ACTIVE => String::from("ACTIVE"),
            CCNewsStatus::INACTIVE => String::from("INACTIVE")
        }
    }
}


/// The preferred language for the sources or articles - English (EN), Espanol (ES), Turkish (TR), French (FR), Japanese (JP), Portuguese (PT).
pub enum CCNewsLang {
    EN,
    ES,
    TR,
    FR,
    JP,
    PT,
}

impl CCNewsLang {
    /// Converts enum value to `String`.
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


/// Get articles from specific sources based on their keys. If left empty it will just return news from ACTIVE sources for the selected language,
/// if you want to get inactive sources as well, please pass them in alongside the active ones in this array.
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
    /// Converts enum value to `String`.
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
#[derive(Deserialize, Debug)]
pub struct CCNewsLatestArticle {
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
    pub updated_on: i64,
    #[serde(rename = "SOURCE_DATA")]
    /// The news source data of this article.
    pub source_data: CCNewsSource,
    #[serde(rename = "CATEGORY_DATA")]
    /// An array of categories this article belongs to.
    pub category_date: Vec<CCCategoryData>,
}



// News: Sources

/// Specifies the type of integration used by the news source. Current allowed values are RSS, API, and TWITTER.
/// 'RSS' indicates a source that distributes content via RSS feeds.
/// 'API' refers to sources that provide data through a standardized programming interface.
/// 'TWITTER' represents sources that disseminate information directly through Twitter.
/// This parameter helps in selecting the method through which news content is retrieved.
pub enum CCNewsSourceType {
    RSS,
    API,
    TWITTER,
}

impl CCNewsSourceType {
    /// Converts enum value to `String`.
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
    pub image_url: String,
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
    pub updated_on: i64,
}


// News: Categories


/// The filters for the news category.
#[derive(Deserialize, Debug)]
pub struct CCCategoryFilter {
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
#[derive(Deserialize, Debug)]
pub struct CCNewsCategory {
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
    pub filter: Option<CCCategoryFilter>,
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