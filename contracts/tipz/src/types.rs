//! Data types for the Tipz contract.

use soroban_sdk::{contracttype, Address, String};

/// Creator profile stored on-chain.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Profile {
    /// Stellar address of the creator
    pub owner: Address,
    /// Unique username (lowercase, alphanumeric + underscore, 3-32 chars)
    pub username: String,
    /// Display name (1-64 chars)
    pub display_name: String,
    /// Short bio (0-280 chars)
    pub bio: String,
    /// Profile image URL or IPFS CID (0-256 chars)
    pub image_url: String,
    /// X (Twitter) handle (0-32 chars)
    pub x_handle: String,
    /// X follower count (set by admin)
    pub x_followers: u32,
    /// Average X engagement per post (set by admin)
    pub x_engagement_avg: u32,
    /// Credit score (0-100)
    pub credit_score: u32,
    /// Lifetime tips received (in stroops)
    pub total_tips_received: i128,
    /// Number of tips received
    pub total_tips_count: u32,
    /// Current withdrawable balance (in stroops)
    pub balance: i128,
    /// Ledger timestamp of registration
    pub registered_at: u64,
    /// Last profile update timestamp
    pub updated_at: u64,
}

/// Individual tip record stored in temporary storage with a TTL of ~7 days.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Tip {
    /// Unique tip ID (monotonically increasing global counter)
    pub id: u32,
    /// Address that sent the tip
    pub tipper: Address,
    /// Address of the creator who received the tip
    pub creator: Address,
    /// Tip amount in stroops
    pub amount: i128,
    /// Optional message (0-280 chars)
    pub message: String,
    /// Ledger timestamp at the time the tip was sent
    pub timestamp: u64,
}

/// Leaderboard entry for top creators.
#[contracttype]
#[derive(Clone, Debug)]
pub struct LeaderboardEntry {
    /// Creator's address
    pub address: Address,
    /// Creator's username
    pub username: String,
    /// Lifetime tips received
    pub total_tips_received: i128,
    /// Current credit score
    pub credit_score: u32,
}

/// Credit tier derived from a creator's on-chain credit score (0–100).
///
/// | Tier    | Score range | Description                         |
/// |---------|-------------|-------------------------------------|
/// | New     | 0 – 19      | No activity yet                     |
/// | Bronze  | 20 – 39     | Early-stage creator                 |
/// | Silver  | 40 – 59     | Default for newly registered profiles|
/// | Gold    | 60 – 79     | Established creator                  |
/// | Diamond | 80 – 100    | Elite creator                        |
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum CreditTier {
    New,
    Bronze,
    Silver,
    Gold,
    Diamond,
}

/// Global contract statistics.
#[contracttype]
#[derive(Clone, Debug)]
pub struct ContractStats {
    /// Total registered creators
    pub total_creators: u32,
    /// Total tips sent (count)
    pub total_tips_count: u32,
    /// Total tip volume in stroops
    pub total_tips_volume: i128,
    /// Total fees collected in stroops
    pub total_fees_collected: i128,
    /// Current fee in basis points
    pub fee_bps: u32,
}
