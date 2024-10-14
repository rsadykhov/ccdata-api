use serde::Deserialize;
use crate::schemas::data_api::{CCPreviousAssetSymbol, CCAssetAlternativeId, CCAssetIndustry, CCSpecialAddress};


// Asset: Full Asset Metadata


#[derive(Deserialize, Debug)]
pub struct CCConsensusMechanism {
    #[serde(rename = "NAME")]
    pub name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CCConsensusAlgorithmType {
    #[serde(rename = "NAME")]
    pub name: Option<String>,
    #[serde(rename = "DESCRIPTION")]
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CCHashingAlgorithmType {
    #[serde(rename = "NAME")]
    pub name: Option<String>,
}

/// Asset: Full Asset Metadata
#[derive(Deserialize, Debug)]
pub struct CCAssetMetadata {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "ID_LEGACY")]
    pub id_legacy: Option<i32>,
    #[serde(rename = "ID_PARENT_ASSET")]
    pub id_parent_asset: Option<i32>,
    #[serde(rename = "ID_ASSET_ISSUER")]
    pub id_asset_issuer: Option<i32>,
    #[serde(rename = "SYMBOL")]
    pub symbol: String,
    #[serde(rename = "URI")]
    pub uri: String,
    #[serde(rename = "ASSET_TYPE")]
    pub asset_type: String,
    #[serde(rename = "ASSET_ISSUER_NAME")]
    pub asset_issuer_name: Option<String>,
    #[serde(rename = "PARENT_ASSET_SYMBOL")]
    pub parent_asset_symbol: Option<String>,
    #[serde(rename = "CREATED_ON")]
    pub created_on: i64,
    #[serde(rename = "UPDATED_ON")]
    pub updated_on: i64,
    #[serde(rename = "PUBLIC_NOTICE")]
    pub public_notice: Option<String>,
    #[serde(rename = "NAME")]
    pub name: String,
    #[serde(rename = "LOGO_URL")]
    pub logo_url: String,
    #[serde(rename = "LAUNCH_DATE")]
    pub launch_date: i64,
    #[serde(rename = "PREVIOUS_ASSET_SYMBOLS")]
    pub previous_asset_symbols: Option<Vec<CCPreviousAssetSymbol>>,
    #[serde(rename = "ASSET_ALTERNATIVE_IDS")]
    pub asset_alternative_ids: Option<Vec<CCAssetAlternativeId>>,
    #[serde(rename = "ASSET_DESCRIPTION_SNIPPET")]
    pub asset_description_snippet: Option<String>,
    #[serde(rename = "ASSET_DECIMAL_POINTS")]
    pub asset_decimal_points: Option<i32>,
    #[serde(rename = "SUPPLY_MAX")]
    pub supply_max: f64,
    #[serde(rename = "SUPPLY_ISSUED")]
    pub supply_issued: Option<f64>,
    #[serde(rename = "SUPPLY_TOTAL")]
    pub supply_total: Option<f64>,
    #[serde(rename = "SUPPLY_CIRCULATING")]
    pub supply_circulating: Option<f64>,
    #[serde(rename = "SUPPLY_FUTURE")]
    pub supply_future: f64,
    #[serde(rename = "SUPPLY_LOCKED")]
    pub supply_locked: Option<f64>,
    #[serde(rename = "SUPPY_BURNT")]
    pub supply_burnt: Option<f64>,
    #[serde(rename = "SUPPLY_STAKED")]
    pub supply_staked: Option<f64>,
    #[serde(rename = "LAST_BLOCK_MINT")]
    pub last_block_mint: Option<f64>,
    #[serde(rename = "LAST_BLOCK_BURN")]
    pub last_block_burn: Option<f64>,
    #[serde(rename = "BURN_ADDRESSES")]
    pub burn_addresses: Option<Vec<CCSpecialAddress>>,
    #[serde(rename = "LOCKED_ADDRESSES")]
    pub locked_addresses: Option<Vec<CCSpecialAddress>>,
    #[serde(rename = "ASSET_INDUSTRIES")]
    pub asset_industries: Option<Vec<CCAssetIndustry>>,
    #[serde(rename = "CONSENSUS_MECHANISMS")]
    pub consensus_mechanisms: Option<Vec<CCConsensusMechanism>>,
    #[serde(rename = "CONSENSUS_ALGORITHM_TYPES")]
    pub consensus_algorithm_types: Option<Vec<CCConsensusAlgorithmType>>,
    #[serde(rename = "HASHING_ALGORITHM_TYPES")]
    pub hashing_algorithm_types: Option<Vec<CCHashingAlgorithmType>>,
}


// Asset: Events


/// Asset: Events
#[derive(Deserialize, Debug)]
pub struct CCAssetEvent {
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "ASSET_ID")]
    pub asset_id: i32,
    #[serde(rename = "EVENT_TYPE")]
    pub event_type: String,
    #[serde(rename = "ANNOUNCED_ON")]
    pub announced_on: i64,
    #[serde(rename = "IMPLEMENTATION_START_DATE")]
    pub implementation_start_date: i64,
    #[serde(rename = "IMPLEMENTATION_END_DATE")]
    pub implementation_end_date: Option<i64>,
    #[serde(rename = "NAME")]
    pub name: String,
    #[serde(rename = "DESCRIPTION")]
    pub description: String,
    #[serde(rename = "CREATED_ON")]
    pub created_on: i64,
    #[serde(rename = "UPDATED_ON")]
    pub updated_on: Option<i64>,
}


// Asset: Historical: Code Repository Metrics Day


#[derive(Deserialize, Debug)]
pub struct CCAssetCodeRepository {
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "CONTRIBUTORS")]
    pub contributors: i32,
    #[serde(rename = "FORKS")]
    pub forks: i32,
    #[serde(rename = "STARS")]
    pub stars: i32,
    #[serde(rename = "SUBSCRIBERS")]
    pub subscribers: i32,
    #[serde(rename = "OPEN_ISSUES")]
    pub open_issues: Option<i32>,
    #[serde(rename = "CLOSED_ISSUES")]
    pub closed_issues: i32,
    #[serde(rename = "OPEN_PULL_REQUESTS")]
    pub open_pull_requests: i32,
    #[serde(rename = "CLOSED_PULL_REQUESTS")]
    pub closed_pull_requests: i32,
}

/// Asset: Historical: Code Repository Metrics Day
#[derive(Deserialize, Debug)]
pub struct CCAssetCodeRepoMetrics {
    #[serde(rename = "UNIT")]
    pub unit: String,
    #[serde(rename = "TIMESTAMP")]
    pub timestamp: i64,
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "ASSET_ID")]
    pub asset_id: i32,
    #[serde(rename = "ASSET_SYMBOL")]
    pub asset_symbol: String,
    #[serde(rename = "TOTAL_CONTRIBUTORS")]
    pub total_contributors: i32,
    #[serde(rename = "TOTAL_FORKS")]
    pub total_forks: i32,
    #[serde(rename = "TOTAL_STARS")]
    pub total_stars: i32,
    #[serde(rename = "TOTAL_SUBSCRIBERS")]
    pub total_subscribers: i32,
    #[serde(rename = "TOTAL_OPEN_ISSUES")]
    pub total_open_issues: Option<i32>,
    #[serde(rename = "TOTAL_CLOSED_ISSUES")]
    pub total_closed_issues: i32,
    #[serde(rename = "TOTAL_OPEN_PULL_REQUESTS")]
    pub total_open_pull_requests: i32,
    #[serde(rename = "TOTAL_CLOSED_PULL_REQUESTS")]
    pub total_closed_pull_requests: i32,
    #[serde(rename = "CODE_REPOSITORIES")]
    pub code_repositories: Vec<CCAssetCodeRepository>,
}


// Asset: Historical Discord


#[derive(Deserialize, Debug)]
pub struct CCAssetDiscordServer {
    #[serde(rename = "URL")]
    pub url: Option<String>,
    #[serde(rename = "NAME")]
    pub name: Option<String>,
    #[serde(rename = "TOTAL_MEMBERS")]
    pub total_members: Option<i32>,
    #[serde(rename = "CURRENT_ACTIVE_USERS")]
    pub total_closed_pull_requests: Option<i32>,
    #[serde(rename = "PREMIUM_SUBSCRIBERS")]
    pub premium_subscribers: Option<i32>,
}

/// Asset: Historical Discord
#[derive(Deserialize, Debug)]
pub struct CCAssetDiscord {
    #[serde(rename = "UNIT")]
    pub unit: String,
    #[serde(rename = "TIMESTAMP")]
    pub timestamp: i64,
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "ASSET_ID")]
    pub asset_id: i32,
    #[serde(rename = "ASSET_SYMBOL")]
    pub asset_symbol: String,
    #[serde(rename = "TOTAL_MEMBERS")]
    pub total_members: Option<i32>,
    #[serde(rename = "TOTAL_CURRENT_ACTIVE_USERS")]
    pub total_current_active_users: Option<i32>,
    #[serde(rename = "TOTAL_PREMIUM_SUBSCRIBERS")]
    pub total_premium_subscribers: Option<i32>,
    #[serde(rename = "DISCORD_SERVERS")]
    pub discord_servers: Option<Vec<CCAssetDiscordServer>>,
}


// Asset: Historical Reddit


#[derive(Deserialize, Debug)]
pub struct CCAssetSubreddit {
    #[serde(rename = "URL")]
    pub url: Option<String>,
    #[serde(rename = "NAME")]
    pub name: Option<String>,
    #[serde(rename = "CURRENT_ACTIVE_USERS")]
    pub current_active_users: Option<i32>,
    #[serde(rename = "AVERAGE_POSTS_PER_DAY")]
    pub average_posts_per_day: Option<f32>,
    #[serde(rename = "AVERAGE_POSTS_PER_HOUR")]
    pub average_posts_per_hour: Option<f32>,
    #[serde(rename = "AVERAGE_COMMENTS_PER_DAY")]
    pub average_comments_per_day: Option<f32>,
    #[serde(rename = "AVERAGE_COMMENTS_PER_HOUR")]
    pub average_comments_per_hour: Option<f32>,
    #[serde(rename = "SUBSCRIBERS")]
    pub subscribers: Option<i32>,
}

/// Asset: Historical Reddit
#[derive(Deserialize, Debug)]
pub struct CCAssetReddit {
    #[serde(rename = "UNIT")]
    pub unit: String,
    #[serde(rename = "TIMESTAMP")]
    pub timestamp: i64,
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "ASSET_ID")]
    pub asset_id: i32,
    #[serde(rename = "ASSET_SYMBOL")]
    pub asset_symbol: String,
    #[serde(rename = "TOTAL_SUBSCRIBERS")]
    pub total_subscribers: i32,
    #[serde(rename = "TOTAL_ACTIVE_USERS")]
    pub total_active_users: i32,
    #[serde(rename = "TOTAL_AVERAGE_POSTS_PER_DAY")]
    pub total_average_posts_per_day: f64,
    #[serde(rename = "TOTAL_AVERAGE_POSTS_PER_HOUR")]
    pub total_average_posts_per_hour: f64,
    #[serde(rename = "TOTAL_AVERAGE_COMMENTS_PER_DAY")]
    pub total_average_comments_per_day: f64,
    #[serde(rename = "TOTAL_AVERAGE_COMMENTS_PER_HOUR")]
    pub total_average_comments_per_hour: f64,
    #[serde(rename = "SUBREDDITS")]
    pub subreddits: Option<Vec<CCAssetSubreddit>>,
}


// Asset: Historical Telegram


#[derive(Deserialize, Debug)]
pub struct CCAssetTelegramGroup {
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "NAME")]
    pub name: String,
    #[serde(rename = "USERNAME")]
    pub username: String,
    #[serde(rename = "MEMBERS")]
    pub members: i32,
}

/// Asset: Historical Telegram
#[derive(Deserialize, Debug)]
pub struct CCAssetTelegram {
    #[serde(rename = "UNIT")]
    pub unit: String,
    #[serde(rename = "TIMESTAMP")]
    pub timestamp: i64,
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "ASSET_ID")]
    pub asset_id: i32,
    #[serde(rename = "ASSET_SYMBOL")]
    pub asset_symbol: String,
    #[serde(rename = "TOTAL_MEMBERS")]
    pub total_members: i32,
    #[serde(rename = "TELEGRAM_GROUPS")]
    pub telegram_groups: Vec<CCAssetTelegramGroup>,
}


// Asset: Historical X (Twitter)


#[derive(Deserialize, Debug)]
pub struct CCAssetTwitterAccount {
    #[serde(rename = "URL")]
    pub url: Option<String>,
    #[serde(rename = "NAME")]
    pub name: Option<String>,
    #[serde(rename = "USERNAME")]
    pub username: Option<String>,
    #[serde(rename = "VERIFIED")]
    pub verified: Option<bool>,
    #[serde(rename = "VERIFIED_TYPE")]
    pub verifyied_type: Option<String>,
    #[serde(rename = "FOLLOWING")]
    pub following: Option<i32>,
    #[serde(rename = "FOLLOWERS")]
    pub followers: Option<i32>,
    #[serde(rename = "FAVOURITES")]
    pub favourites: Option<i32>,
    #[serde(rename = "LISTS")]
    pub lists: Option<i32>,
    #[serde(rename = "STATUSES")]
    pub statuses: Option<i32>,
}


/// Asset: Historical X (Twitter)
#[derive(Deserialize, Debug)]
pub struct CCAssetTwitter {
    #[serde(rename = "UNIT")]
    pub unit: String,
    #[serde(rename = "TIMESTAMP")]
    pub timestamp: i64,
    #[serde(rename = "TYPE")]
    pub type_: String,
    #[serde(rename = "ASSET_ID")]
    pub asset_id: i32,
    #[serde(rename = "ASSET_SYMBOL")]
    pub asset_symbol: String,
    #[serde(rename = "TOTAL_FOLLOWING")]
    pub total_following: i32,
    #[serde(rename = "TOTAL_FOLLOWERS")]
    pub total_followers: i32,
    #[serde(rename = "TOTAL_FAVOURITES")]
    pub total_favourites: i32,
    #[serde(rename = "TOTAL_LISTS")]
    pub total_lists: i32,
    #[serde(rename = "TOTAL_STATUSES")]
    pub total_statuses: i32,
    #[serde(rename = "TWITTER_ACCOUNTS")]
    pub twitter_accounts: Option<Vec<CCAssetTwitterAccount>>,
}