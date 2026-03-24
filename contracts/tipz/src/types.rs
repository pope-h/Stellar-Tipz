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
    /// X post count (set by admin)
    pub x_posts: u32,
    /// X reply count (set by admin)
    pub x_replies: u32,
    /// Credit score (0-1000)
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

/// Individual tip record.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Tip {
    /// Tipper's address
    pub from: Address,
    /// Creator's address
    pub to: Address,
    /// Tip amount in stroops
    pub amount: i128,
    /// Optional message (0-280 chars)
    pub message: String,
    /// Ledger timestamp
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
