use serde::Deserialize;
use crate::schemas::data_api::{CCPreviousAssetSymbol, CCAssetAlternativeId, CCAssetIndustry, CCSpecialAddress};


// Asset: Full Asset Metadata


#[derive(Deserialize, Debug)]
pub struct CCConsensusMechanism {
    #[serde(rename = "NAME")]
    /// The type of consensus this blockhain / networks uses.
    pub name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CCConsensusAlgorithmType {
    #[serde(rename = "NAME")]
    /// The name of the algorithm this blockchain uses for the consensus mechanism.
    pub name: Option<String>,
    #[serde(rename = "DESCRIPTION")]
    /// A description for the algorithm type.
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CCHashingAlgorithmType {
    #[serde(rename = "NAME")]
    /// The types of algorithms this blockchain uses for hashing blocks, transactions etc.
    pub name: Option<String>,
}

/// Asset: Full Asset Metadata
#[derive(Deserialize, Debug)]
pub struct CCAssetMetadata {
    #[serde(rename = "ID")]
    /// The unique identifier for the asset entry.
    pub id: i32,
    #[serde(rename = "TYPE")]
    /// Type of the message.
    pub type_: String,
    #[serde(rename = "ID_LEGACY")]
    /// The legacy previous asset management system ID.
    pub id_legacy: Option<i32>,
    #[serde(rename = "ID_PARENT_ASSET")]
    /// This refers to the base, parent, or main asset to which a token is linked or pegged, signifying that the token acts as a representation of the parent asset.
    /// When a token loses its connection to a parent asset due to events such as hacks or the issuing entity's decision to not honor the peg—similar to how
    /// TerraUSD detached from its USD peg—the PARENT_ASSET_SYMBOL is removed because the token no longer serves as a true representation of the parent asset.
    /// In order to remove the parent we need clear communication from the company who is in charge of keeping the peg.
    /// We add add a plublic notice and the include the communication in the Other Document URLs.
    pub id_parent_asset: Option<i32>,
    #[serde(rename = "ID_ASSET_ISSUER")]
    /// This field identifies the original creator of the token.
    /// It provides essential information about the entity, individual or contract rules responsible for issuing the token initially and/or maintaining the supply.
    /// In the case of of bridged assets, this is the bridge operator and the parent will have its own issuer.
    /// You can go up the parent chain and figure out what counterparty risk you are exposed to when trading a specific asset.
    /// This clarification ensures that users can directly trace the origin of the token, understanding its issuance history and the primary issuer's credentials.
    pub id_asset_issuer: Option<i32>,
    #[serde(rename = "SYMBOL")]
    /// Internal mapped symbol for a specific asset.
    pub symbol: String,
    #[serde(rename = "URI")]
    /// The uri path that this asset will be found on / url-slug.
    pub uri: String,
    #[serde(rename = "ASSET_TYPE")]
    /// The asset class/type.
    pub asset_type: String,
    #[serde(rename = "ASSET_ISSUER_NAME")]
    /// This field identifies the original creator of the asset. It provides essential information about the entity
    ///  individual or contract rules responsible for issuing the asset initially and/or maintaining the supply.
    /// In the case of of bridged assets, this is the bridge operator and the parent will have its own issuer.
    /// You can go up the parent chain and figure out what counterparty risk you are exposed to when trading a specific asset.
    /// This clarification ensures that users can directly trace the origin of the asset, understanding its issuance history and the primary issuer's credentials.
    pub asset_issuer_name: Option<String>,
    #[serde(rename = "PARENT_ASSET_SYMBOL")]
    /// This refers to the base, parent, or main asset to which a token is linked or pegged, signifying that the token acts as a representation of the parent asset.
    /// When a token loses its connection to a parent asset due to events such as hacks or the issuing entity's decision to not honor the peg—similar to how
    /// TerraUSD detached from its USD peg—the PARENT_ASSET_SYMBOL is removed because the token no longer serves as a true representation of the parent asset.
    /// In order to remove the parent we need clear communication from the company who is in charge of keeping the peg.
    /// We add add a plublic notice and the include the communication in the Other Document URLs.
    pub parent_asset_symbol: Option<String>,
    #[serde(rename = "CREATED_ON")]
    /// Asset internal creation unix ts in our system.
    pub created_on: i64,
    #[serde(rename = "UPDATED_ON")]
    /// Asset internal last updated unix ts in our system.
    pub updated_on: i64,
    #[serde(rename = "PUBLIC_NOTICE")]
    /// A public notice for this asset.
    pub public_notice: Option<String>,
    #[serde(rename = "NAME")]
    /// The full name of the asset, e.g. Bitcoin.
    pub name: String,
    #[serde(rename = "LOGO_URL")]
    /// The image that appears when you see this asset.
    pub logo_url: String,
    #[serde(rename = "LAUNCH_DATE")]
    /// The launch date of the asset is indicated as (yyyy-mm-dd).
    /// However, if the asset was initially established as a token before being integrated into a blockchain,
    /// the launch date is reset to the creation of the first block when the blockchain is launched for the token.
    pub launch_date: i64,
    #[serde(rename = "PREVIOUS_ASSET_SYMBOLS")]
    /// A list of symbols that were previously associated with this asset.
    pub previous_asset_symbols: Option<Vec<CCPreviousAssetSymbol>>,
    #[serde(rename = "ASSET_ALTERNATIVE_IDS")]
    /// Alternative data platforms that also support this asset with their specific asset id.
    pub asset_alternative_ids: Option<Vec<CCAssetAlternativeId>>,
    #[serde(rename = "ASSET_DESCRIPTION_SNIPPET")]
    /// The shortest form description text only for this asset. This is a lot more limited than the summary. Generally this is a one or maximum two sentences.
    pub asset_description_snippet: Option<String>,
    #[serde(rename = "ASSET_DECIMAL_POINTS")]
    /// The total decimal places this asset can be divided into. E.g. 8 for BTC (1 Satoshi), 18 for ETH (1 Wei).
    /// Generally blockchains store all units as integers and this is the number you need to divide the lowest unit of accounting by to get the common unit
    /// of measure used for the asset.
    pub asset_decimal_points: Option<i32>,
    #[serde(rename = "SUPPLY_MAX")]
    /// The maximum number of asset parts (coins/tokens) that will ever be issued (supply_circulating + supply_burnt + supply_locked + all supply that
    /// has not been issued yet but is planned to be issued in the future). For assets (coins/tokens) that have infinite supply, we use -1.
    pub supply_max: f64,
    #[serde(rename = "SUPPLY_ISSUED")]
    /// The number of asset parts (coins/tokens) that have been issued so far. (supply_circulating + supply_locked + supply_burnt).
    pub supply_issued: Option<f64>,
    #[serde(rename = "SUPPLY_TOTAL")]
    /// The number of asset parts (coins/tokens) that have been issued so far excluding burnt tokens. (supply_circulating + supply_locked).
    pub supply_total: Option<f64>,
    #[serde(rename = "SUPPLY_CIRCULATING")]
    /// Also referred to as free float or public float. The number of asset parts (coins/tokens) that are available to be traded and
    /// it excludes burnt supply and locked supply.
    pub supply_circulating: Option<f64>,
    #[serde(rename = "SUPPLY_FUTURE")]
    /// The number of asset parts (coins/tokens) that are planned to be issued in the future. (supply_max - supply_issued).
    pub supply_future: f64,
    #[serde(rename = "SUPPLY_LOCKED")]
    /// The number of asset parts (coins/tokens) that are currently not transferable until certain conditions are met.
    /// Locked supply is generally held by team members, DAOs, foundations, bridges, stakers, liquidity pools, etc.
    pub supply_locked: Option<f64>,
    #[serde(rename = "SUPPY_BURNT")]
    /// The number of asset parts (coins/tokens) that have been sent to addresses/locations that are no longer accessible.
    /// They are permanently removed from the circulating supply on purpose, this does not include lost tokens sent to wallets that do not exist or
    /// sent to wallets that users no longer have access to, the address of burnt tokens is determined by the project team.
    pub supply_burnt: Option<f64>,
    #[serde(rename = "SUPPLY_STAKED")]
    /// The current number of asset parts (coins/tokens) that are locked as part of PoS and PoS partial chains.
    pub supply_staked: Option<f64>,
    #[serde(rename = "LAST_BLOCK_MINT")]
    /// New asset parts (coins/tokens) created in the most recently issued block.
    pub last_block_mint: Option<f64>,
    #[serde(rename = "LAST_BLOCK_BURN")]
    /// The total amount of asset parts (coins/tokens) that were taken out of circulation in the most recently issued block.
    pub last_block_burn: Option<f64>,
    #[serde(rename = "BURN_ADDRESSES")]
    /// The list of addresses that are considered burn addresses for this asset.
    pub burn_addresses: Option<Vec<CCSpecialAddress>>,
    #[serde(rename = "LOCKED_ADDRESSES")]
    /// The list of addresses that are considered locked addresses for this asset.
    pub locked_addresses: Option<Vec<CCSpecialAddress>>,
    #[serde(rename = "ASSET_INDUSTRIES")]
    /// The asset industries that asset operates in.
    pub asset_industries: Option<Vec<CCAssetIndustry>>,
    #[serde(rename = "CONSENSUS_MECHANISMS")]
    /// The types of consensus mechanisms this network / blockchain / token uses. This is a list of the fault-tolerant mechanism families that
    /// are used to achieve the necessary agreement on a single data value or a single state of the network among distributed processes or multi-agent systems.
    pub consensus_mechanisms: Option<Vec<CCConsensusMechanism>>,
    #[serde(rename = "CONSENSUS_ALGORITHM_TYPES")]
    /// The types of consensus algorithms this blockchain uses. This is dependent on the consensus mechanisms used by the network / blockchain / token.
    /// For blockchains that are proof of work this would in most cases have the mining algorithm. For hybrid assets,
    /// this would be a list of mining and proof of stake and other algorithm used for reaching consensus.
    pub consensus_algorithm_types: Option<Vec<CCConsensusAlgorithmType>>,
    #[serde(rename = "HASHING_ALGORITHM_TYPES")]
    /// The types of algorithms this blockchain uses for hashing blocks, transactions etc.
    pub hashing_algorithm_types: Option<Vec<CCHashingAlgorithmType>>,
}


// Asset: Events


/// Asset: Events
#[derive(Deserialize, Debug)]
pub struct CCAssetEvent {
    #[serde(rename = "TYPE")]
    /// Type of the message.
    pub type_: String,
    #[serde(rename = "ID")]
    /// The unique identifier for the asset event entry.
    pub id: i32,
    #[serde(rename = "ASSET_ID")]
    /// The unique identifier for the asset the event entry is associated to.
    pub asset_id: i32,
    #[serde(rename = "EVENT_TYPE")]
    /// The event type, one of the following: SECURITY-INCIDENT, HARD-FORK, SOFT-FORK, TICKER-CHANGE, MIGRATION, SUPPLY-BURN, SUPPLY-LOCK, SPLIT,
    /// REVERSE-SPLIT, TOKEN-LISTING, TOKEN-DELISTING, NETWORK-CONTROL-CHANGE, OTHER
    pub event_type: String,
    #[serde(rename = "ANNOUNCED_ON")]
    /// The unix timestamp when we were notified of the event.
    pub announced_on: i64,
    #[serde(rename = "IMPLEMENTATION_START_DATE")]
    /// The unix timestamp when the event was triggered.
    pub implementation_start_date: i64,
    #[serde(rename = "IMPLEMENTATION_END_DATE")]
    /// The unix timestamp when the event finished.
    pub implementation_end_date: Option<i64>,
    #[serde(rename = "NAME")]
    /// Name text.
    pub name: String,
    #[serde(rename = "DESCRIPTION")]
    /// Description text.
    pub description: String,
    #[serde(rename = "CREATED_ON")]
    /// Asset Event internal creation unix ts in our system.
    pub created_on: i64,
    #[serde(rename = "UPDATED_ON")]
    /// Asset Event internal last updated unix ts in our system.
    pub updated_on: Option<i64>,
}


// Asset: Historical: Code Repository Metrics Day


#[derive(Deserialize, Debug)]
pub struct CCAssetCodeRepository {
    #[serde(rename = "URL")]
    /// The URL of the code repository used to retrieve social metrics.
    pub url: String,
    #[serde(rename = "CONTRIBUTORS")]
    /// The number of contributors to the code repository.
    pub contributors: i32,
    #[serde(rename = "FORKS")]
    /// The number of forks of the code repository.
    pub forks: i32,
    #[serde(rename = "STARS")]
    /// The number of stars received by the code repository.
    pub stars: i32,
    #[serde(rename = "SUBSCRIBERS")]
    /// The number of subscribers to the code repository.
    pub subscribers: i32,
    #[serde(rename = "OPEN_ISSUES")]
    /// The number of open issues in the code repository.
    pub open_issues: Option<i32>,
    #[serde(rename = "CLOSED_ISSUES")]
    /// The number of closed issues in the code repository.
    pub closed_issues: i32,
    #[serde(rename = "OPEN_PULL_REQUESTS")]
    /// The number of open pull requests in the code repository.
    pub open_pull_requests: i32,
    #[serde(rename = "CLOSED_PULL_REQUESTS")]
    /// The number of closed pull requests in the code repository.
    pub closed_pull_requests: i32,
}

/// Asset: Historical: Code Repository Metrics Day
#[derive(Deserialize, Debug)]
pub struct CCAssetCodeRepoMetrics {
    #[serde(rename = "UNIT")]
    /// The unit of the historical period update: HOUR for hour and DAY for day.
    pub unit: String,
    #[serde(rename = "TIMESTAMP")]
    /// The timestamp in seconds of the histo period, for hour it would be start of the hour and for daily it is 00:00 GMT/UTC.
    pub timestamp: i64,
    #[serde(rename = "TYPE")]
    /// The type of the message.
    pub type_: String,
    #[serde(rename = "ASSET_ID")]
    /// The unique identifier for the asset.
    pub asset_id: i32,
    #[serde(rename = "ASSET_SYMBOL")]
    /// Internal mapped symbol for a specific asset.
    pub asset_symbol: String,
    #[serde(rename = "TOTAL_CONTRIBUTORS")]
    /// The total number of contributors across all code repositories associated with this asset.
    /// A contributor is a users who has contributed to a project by making changes or improvements to the project's codebase,
    /// having their changes accepted and merged, and is acknowledged for their contributions in the project's development history.
    pub total_contributors: i32,
    #[serde(rename = "TOTAL_FORKS")]
    /// The total number of forks across all code repositories associated with this asset.
    /// A fork in GitHub is a feature that allows users to create a personal copy of another repository, enabling them to modify,
    /// experiment with, or contribute to the project without affecting the original work.
    pub total_forks: i32,
    #[serde(rename = "TOTAL_STARS")]
    /// The total number of stars received across all code repositories associated with this asset.
    /// A star is when a GitHub user bookmarks or wants to show appreciation for a particular repository,
    /// much like a "like" or "favorite" function on other social media platforms.
    pub total_stars: i32,
    #[serde(rename = "TOTAL_SUBSCRIBERS")]
    /// The total number of subscribers across all code repositories associated with this asset.
    /// A subscriber is a user who has chosen to receive notifications for updates or changes made to a specific repository,
    /// allowing them to closely follow the development and discussions of a project.
    pub total_subscribers: i32,
    #[serde(rename = "TOTAL_OPEN_ISSUES")]
    /// The total number of open issues across all code repositories associated with this asset.
    /// An open issue is a reported problem, suggestion, or task related to a repository that has not been resolved or closed yet,
    /// providing a platform for users to track and discuss the ongoing development and improvements of a project.
    pub total_open_issues: Option<i32>,
    #[serde(rename = "TOTAL_CLOSED_ISSUES")]
    /// The total number of closed issues across all code repositories associated with this asset.
    /// A closed issue is a reported problem, suggestion, or task related to a repository that have been resolved or deemed no longer relevant,
    /// providing a record of past challenges and solutions within the project development.
    pub total_closed_issues: i32,
    #[serde(rename = "TOTAL_OPEN_PULL_REQUESTS")]
    /// The total number of open pull requests across all code repositories associated with this asset.
    /// An open pull request on GitHub is a proposed change to a repository's codebase that is open for review and discussion,
    // offering contributors an opportunity to improve the project and maintainers to accept, reject, or request changes before
    /// integrating the proposed modifications.
    pub total_open_pull_requests: i32,
    #[serde(rename = "TOTAL_CLOSED_PULL_REQUESTS")]
    /// The total number of closed pull requests across all code repositories associated with this asset.
    /// A closed pull request is a proposed change to a repository's codebase that has been either accepted and merged into the codebase,
    /// rejected by the maintainers, or withdrawn by the submitter, providing a record of changes that were suggested and their outcomes in
    /// the project's development history.
    pub total_closed_pull_requests: i32,
    #[serde(rename = "CODE_REPOSITORIES")]
    /// An array with all the data for each code repository used to calculate the total stats.
    pub code_repositories: Vec<CCAssetCodeRepository>,
}


// Asset: Historical Discord


#[derive(Deserialize, Debug)]
pub struct CCAssetDiscordServer {
    #[serde(rename = "URL")]
    /// The URL of the Discord server used to retrieve social metrics.
    pub url: Option<String>,
    #[serde(rename = "NAME")]
    /// The name of the Discord server.
    pub name: Option<String>,
    #[serde(rename = "TOTAL_MEMBERS")]
    /// The total number of users/members in this Discord server.
    pub total_members: Option<i32>,
    #[serde(rename = "CURRENT_ACTIVE_USERS")]
    /// The number of online users in this Discord server.
    pub total_closed_pull_requests: Option<i32>,
    #[serde(rename = "PREMIUM_SUBSCRIBERS")]
    /// The number of premium subscribers in this Discord server.
    pub premium_subscribers: Option<i32>,
}

/// Asset: Historical Discord
#[derive(Deserialize, Debug)]
pub struct CCAssetDiscord {
    #[serde(rename = "UNIT")]
    /// The unit of the historical period update: HOUR for hour and DAY for day.
    pub unit: String,
    #[serde(rename = "TIMESTAMP")]
    /// The timestamp in seconds of the histo period, for hour it would be start of the hour and for daily it is 00:00 GMT/UTC.
    pub timestamp: i64,
    #[serde(rename = "TYPE")]
    /// The type of the message.
    pub type_: String,
    #[serde(rename = "ASSET_ID")]
    /// The unique identifier for the asset.
    pub asset_id: i32,
    #[serde(rename = "ASSET_SYMBOL")]
    /// Symbol for a specific asset.
    pub asset_symbol: String,
    #[serde(rename = "TOTAL_MEMBERS")]
    /// The total number of users/members in this Discord server.
    pub total_members: Option<i32>,
    #[serde(rename = "TOTAL_CURRENT_ACTIVE_USERS")]
    /// The number of online users in this Discord server.
    pub total_current_active_users: Option<i32>,
    #[serde(rename = "TOTAL_PREMIUM_SUBSCRIBERS")]
    /// The number of premium subscribers in this Discord server.
    pub total_premium_subscribers: Option<i32>,
    #[serde(rename = "DISCORD_SERVERS")]
    /// An array with all the data for each Discord server used to calculate the total stats.
    pub discord_servers: Option<Vec<CCAssetDiscordServer>>,
}


// Asset: Historical Reddit


#[derive(Deserialize, Debug)]
pub struct CCAssetSubreddit {
    #[serde(rename = "URL")]
    /// The URL of the Subreddit used to retrieve social metrics.
    pub url: Option<String>,
    #[serde(rename = "NAME")]
    /// The name of the subreddit.
    pub name: Option<String>,
    #[serde(rename = "CURRENT_ACTIVE_USERS")]
    /// The number of currently active users in the subreddit.
    pub current_active_users: Option<i32>,
    #[serde(rename = "AVERAGE_POSTS_PER_DAY")]
    /// The average number of posts per day in the subreddit.
    pub average_posts_per_day: Option<f32>,
    #[serde(rename = "AVERAGE_POSTS_PER_HOUR")]
    /// The average number of posts per hour in the subreddit.
    pub average_posts_per_hour: Option<f32>,
    #[serde(rename = "AVERAGE_COMMENTS_PER_DAY")]
    /// The average number of comments per day in the subreddit.
    pub average_comments_per_day: Option<f32>,
    #[serde(rename = "AVERAGE_COMMENTS_PER_HOUR")]
    /// The average number of comments per hour in the subreddit.
    pub average_comments_per_hour: Option<f32>,
    #[serde(rename = "SUBSCRIBERS")]
    /// The number of subscribers to the subreddit.
    pub subscribers: Option<i32>,
}

/// Asset: Historical Reddit
#[derive(Deserialize, Debug)]
pub struct CCAssetReddit {
    #[serde(rename = "UNIT")]
    /// The unit of the historical period update: HOUR for hour and DAY for day.
    pub unit: String,
    #[serde(rename = "TIMESTAMP")]
    /// The timestamp in seconds of the histo period, for hour it would be start of the hour and for daily it is 00:00 GMT/UTC.
    pub timestamp: i64,
    #[serde(rename = "TYPE")]
    /// The type of the message.
    pub type_: String,
    #[serde(rename = "ASSET_ID")]
    /// The unique identifier for the asset.
    pub asset_id: i32,
    #[serde(rename = "ASSET_SYMBOL")]
    /// Internal mapped symbol for a specific asset.
    pub asset_symbol: String,
    #[serde(rename = "TOTAL_SUBSCRIBERS")]
    /// The number of subscribers to the subreddit.
    pub total_subscribers: i32,
    #[serde(rename = "TOTAL_ACTIVE_USERS")]
    /// The number of currently active users in the subreddit.
    pub total_active_users: i32,
    #[serde(rename = "TOTAL_AVERAGE_POSTS_PER_DAY")]
    /// The average number of posts per day in the subreddit.
    pub total_average_posts_per_day: f64,
    #[serde(rename = "TOTAL_AVERAGE_POSTS_PER_HOUR")]
    /// The average number of posts per hour in the subreddit.
    pub total_average_posts_per_hour: f64,
    #[serde(rename = "TOTAL_AVERAGE_COMMENTS_PER_DAY")]
    /// The average number of comments per day in the subreddit.
    pub total_average_comments_per_day: f64,
    #[serde(rename = "TOTAL_AVERAGE_COMMENTS_PER_HOUR")]
    /// The average number of comments per hour in the subreddit.
    pub total_average_comments_per_hour: f64,
    #[serde(rename = "SUBREDDITS")]
    /// An array with all the data for each Subreddit used to calculate the total stats.
    pub subreddits: Option<Vec<CCAssetSubreddit>>,
}


// Asset: Historical Telegram


#[derive(Deserialize, Debug)]
pub struct CCAssetTelegramGroup {
    #[serde(rename = "URL")]
    /// The URL of the Telegram group used to retrieve social metrics.
    pub url: String,
    #[serde(rename = "NAME")]
    /// The name of the Telegram group.
    pub name: String,
    #[serde(rename = "USERNAME")]
    /// The username of the Telegram group.
    pub username: String,
    #[serde(rename = "MEMBERS")]
    /// The total number of users/members in this Telegram group.
    pub members: i32,
}

/// Asset: Historical Telegram
#[derive(Deserialize, Debug)]
pub struct CCAssetTelegram {
    #[serde(rename = "UNIT")]
    /// The unit of the historical period update: HOUR for hour and DAY for day.
    pub unit: String,
    #[serde(rename = "TIMESTAMP")]
    /// The timestamp in seconds of the histo period, for hour it would be start of the hour and for daily it is 00:00 GMT/UTC.
    pub timestamp: i64,
    #[serde(rename = "TYPE")]
    /// The type of the message.
    pub type_: String,
    #[serde(rename = "ASSET_ID")]
    /// The unique identifier for the asset.
    pub asset_id: i32,
    #[serde(rename = "ASSET_SYMBOL")]
    /// Internal mapped symbol for a specific asset.
    pub asset_symbol: String,
    #[serde(rename = "TOTAL_MEMBERS")]
    /// The total number of users/members in this Telegram group.
    pub total_members: i32,
    #[serde(rename = "TELEGRAM_GROUPS")]
    /// An array with all the data for each Telegram group used to calculate the total stats.
    pub telegram_groups: Vec<CCAssetTelegramGroup>,
}


// Asset: Historical X (Twitter)


#[derive(Deserialize, Debug)]
pub struct CCAssetTwitterAccount {
    #[serde(rename = "URL")]
    /// The URL of the X account used to retrieve social metrics.
    pub url: Option<String>,
    #[serde(rename = "NAME")]
    /// The name of the X account.
    pub name: Option<String>,
    #[serde(rename = "USERNAME")]
    /// The username of the X account.
    pub username: Option<String>,
    #[serde(rename = "VERIFIED")]
    /// The verification status of the X account.
    pub verified: Option<bool>,
    #[serde(rename = "VERIFIED_TYPE")]
    /// The verification type of the X account.
    pub verifyied_type: Option<String>,
    #[serde(rename = "FOLLOWING")]
    /// The number of accounts followed by this X account.
    pub following: Option<i32>,
    #[serde(rename = "FOLLOWERS")]
    /// The number of followers of this X account.
    pub followers: Option<i32>,
    #[serde(rename = "FAVOURITES")]
    /// The number of tweets favorited by this X account.
    pub favourites: Option<i32>,
    #[serde(rename = "LISTS")]
    /// The number of lists this X account is a member of.
    pub lists: Option<i32>,
    #[serde(rename = "STATUSES")]
    /// The number of tweets and retweets made by this X account.
    pub statuses: Option<i32>,
}


/// Asset: Historical X (Twitter)
#[derive(Deserialize, Debug)]
pub struct CCAssetTwitter {
    #[serde(rename = "UNIT")]
    /// The unit of the historical period update: HOUR for hour and DAY for day.
    pub unit: String,
    #[serde(rename = "TIMESTAMP")]
    /// The timestamp in seconds of the histo period, for hour it would be start of the hour and for daily it is 00:00 GMT/UTC.
    pub timestamp: i64,
    #[serde(rename = "TYPE")]
    /// The type of the message.
    pub type_: String,
    #[serde(rename = "ASSET_ID")]
    /// The unique identifier for the asset.
    pub asset_id: i32,
    #[serde(rename = "ASSET_SYMBOL")]
    /// Internal mapped symbol for a specific asset.
    pub asset_symbol: String,
    #[serde(rename = "TOTAL_FOLLOWING")]
    /// The total number of accounts followed by this X account.
    pub total_following: i32,
    #[serde(rename = "TOTAL_FOLLOWERS")]
    /// The total number of followers of this X account.
    pub total_followers: i32,
    #[serde(rename = "TOTAL_FAVOURITES")]
    /// The total number of tweets favorited by this X account.
    pub total_favourites: i32,
    #[serde(rename = "TOTAL_LISTS")]
    /// The total number of lists this X account is a member of.
    pub total_lists: i32,
    #[serde(rename = "TOTAL_STATUSES")]
    /// The total number of tweets and retweets made by this X account.
    pub total_statuses: i32,
    #[serde(rename = "TWITTER_ACCOUNTS")]
    /// An array with all the data for each X account used to calculate the total stats.
    pub twitter_accounts: Option<Vec<CCAssetTwitterAccount>>,
}