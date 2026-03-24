//! Storage keys and helpers for the Tipz contract.

use soroban_sdk::{contracttype, Address, String};

/// Storage key enum for all contract data.
#[contracttype]
pub enum DataKey {
    /// Contract admin address
    Admin,
    /// Withdrawal fee in basis points
    FeePercent,
    /// Address that receives fees
    FeeCollector,
    /// Lifetime fees collected
    TotalFeesCollected,
    /// Creator profile by address
    Profile(Address),
    /// Reverse lookup: username → address
    UsernameToAddress(String),
    /// Global tip counter
    TipCount,
    /// Individual tip record by index
    Tip(u32),
    /// Leaderboard (top creators)
    Leaderboard,
    /// Total registered creators
    TotalCreators,
    /// Lifetime tip volume
    TotalTipsVolume,
    /// Flag indicating contract is initialized
    Initialized,
    /// Native XLM token contract address (SAC)
    NativeToken,
}

// ──────────────────────────────────────────────
// Storage helper functions
// ──────────────────────────────────────────────
// TODO: Implement storage read/write helpers as needed by issues.
//
// Examples:
//   pub fn get_admin(env: &Env) -> Address { ... }
//   pub fn set_admin(env: &Env, admin: &Address) { ... }
//   pub fn has_profile(env: &Env, address: &Address) -> bool { ... }
//   pub fn get_profile(env: &Env, address: &Address) -> Profile { ... }
//   pub fn set_profile(env: &Env, profile: &Profile) { ... }
