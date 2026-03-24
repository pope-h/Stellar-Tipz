//! Credit score calculation and tier classification for the Tipz contract.
//!
//! ## Score range
//! All scores are in the range **0 – 100**.  Newly registered profiles start
//! at the base score of **40** (bottom of the Silver tier) because they
//! haven't yet built up tips, X presence, or account age.
//!
//! ## Formula
//! ```text
//! score = BASE_SCORE
//!       + tip_sub  * 20 / 100   (0-20 pts — tip volume component)
//!       + x_sub    * 30 / 100   (0-30 pts — X metrics component)
//!       + age_sub  * 10 / 100   (0-10 pts — account age component)
//!
//! capped at 100
//! ```
//!
//! Each *sub-score* is independently capped at 100 before weighting:
//!
//! | Sub-score  | Formula                                            | Cap |
//! |------------|----------------------------------------------------|-----|
//! | `tip_sub`  | `total_tips_received (stroops) / 10_000_000`       | 100 |
//! | `x_sub`    | `min(followers/50, 50) + min((posts+replies×1.5)/10, 50)` | 100 |
//! | `age_sub`  | `age_in_days / 10`  (0 when age < 1 day)          | 100 |
//!
//! ## Tier boundaries
//! | Tier    | Range   |
//! |---------|---------|
//! | New     | 0 – 19  |
//! | Bronze  | 20 – 39 |
//! | Silver  | 40 – 59 |
//! | Gold    | 60 – 79 |
//! | Diamond | 80 – 100|

use soroban_sdk::{Address, Env};

use crate::errors::ContractError;
use crate::storage;
use crate::types::{CreditTier, Profile};

/// Base score awarded to every registered profile.
/// Places new creators in the Silver tier (40–59) by default.
pub const BASE_SCORE: u32 = 40;

/// Tip volume (in stroops) that yields the maximum tip sub-score of 100.
/// 1_000_000_000 stroops = 100 XLM.
const TIP_VOLUME_CAP: i128 = 1_000_000_000;

/// Stroops per XLM (used to normalise tip volume to 0–100).
const STROOPS_PER_XLM: i128 = 10_000_000;

/// Seconds in one day — the minimum account age for the age component to
/// contribute anything to the score.
const SECONDS_PER_DAY: u64 = 86_400;

/// Compute the credit score (0–100) for `profile` at the given `now` timestamp
/// (seconds since the Unix epoch, obtained from `env.ledger().timestamp()`).
///
/// # Edge-case behaviour
/// | Condition                                | Result                         |
/// |------------------------------------------|--------------------------------|
/// | `total_tips_received` == 0               | tip component = 0 → score = 40|
/// | `total_tips_received` in the billions    | tip sub-score capped at 100    |
/// | all X metric fields are 0                | X component = 0                |
/// | account age < 1 day                      | age component = 0              |
pub fn calculate_credit_score(profile: &Profile, now: u64) -> u32 {
    // ── Tip volume sub-score (0–100) ─────────────────────────────────────────
    // Clamp negative balances to 0 (defensive), then cap at TIP_VOLUME_CAP so
    // arbitrarily large values don't overflow the cast to u32.
    let tip_sub: u32 =
        (profile.total_tips_received.clamp(0, TIP_VOLUME_CAP) / STROOPS_PER_XLM) as u32;
    // tip_sub is guaranteed 0..=100

    // ── X metrics sub-score (0–100) ──────────────────────────────────────────
    // All three X fields at 0 → the whole component contributes 0.
    let x_sub: u32 = if profile.x_followers == 0 && profile.x_engagement_avg == 0 {
        0
    } else {
        // Followers half (0–50): saturates at 2 500 followers.
        let follower_part = (profile.x_followers / 50).min(50);

        // Engagement half (0–50): saturates at an average engagement of 500.
        let engagement_part = (profile.x_engagement_avg / 10).min(50);

        follower_part + engagement_part // 0..=100
    };

    // ── Account age sub-score (0–100) ─────────────────────────────────────────
    // Age component is 0 when account is less than one day old.
    // Reaches 100 at 1 000 days (~2.7 years).
    let age_sub: u32 =
        if now <= profile.registered_at || now - profile.registered_at < SECONDS_PER_DAY {
            0
        } else {
            let age_days = (now - profile.registered_at) / SECONDS_PER_DAY;
            (age_days as u32 / 10).min(100)
        };

    // ── Weighted contributions ────────────────────────────────────────────────
    let tip_pts = tip_sub * 20 / 100; // 0–20
    let x_pts = x_sub * 30 / 100; // 0–30
    let age_pts = age_sub * 10 / 100; // 0–10

    // Total is capped at 100.  Maximum possible: 40 + 20 + 30 + 10 = 100.
    (BASE_SCORE + tip_pts + x_pts + age_pts).min(100)
}

/// Map a credit score (0–100) to its [`CreditTier`].
///
/// Scores above 100 are treated as Diamond (the highest tier).
pub fn get_tier(score: u32) -> CreditTier {
    match score {
        0..=19 => CreditTier::New,
        20..=39 => CreditTier::Bronze,
        40..=59 => CreditTier::Silver,
        60..=79 => CreditTier::Gold,
        _ => CreditTier::Diamond, // 80–100 (and any value above 100)
    }
}

/// Load the profile for `address` from on-chain storage, compute its current
/// credit score, and return `(score, tier)`.
///
/// # Errors
/// Returns [`ContractError::NotRegistered`] when no profile exists for the
/// given address.
pub fn get_credit_tier(env: &Env, address: &Address) -> Result<(u32, CreditTier), ContractError> {
    if !storage::has_profile(env, address) {
        return Err(ContractError::NotRegistered);
    }

    let profile: Profile = storage::get_profile(env, address);

    let now = env.ledger().timestamp();
    let score = calculate_credit_score(&profile, now);
    let tier = get_tier(score);

    Ok((score, tier))
}
