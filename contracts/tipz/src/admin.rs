//! Admin operations for the Tipz contract.
//!
//! - Contract initialization
//! - Fee management
//! - Admin role transfer

use soroban_sdk::{Address, Env};

use crate::credit;
use crate::errors::ContractError;
use crate::events;
use crate::storage::{self, DataKey};

/// Initialize the contract. Can only be called once.
pub fn initialize(
    env: &Env,
    admin: &Address,
    fee_collector: &Address,
    fee_bps: u32,
    native_token: &Address,
) -> Result<(), ContractError> {
    if storage::is_initialized(env) {
        return Err(ContractError::AlreadyInitialized);
    }

    if fee_bps > 1000 {
        return Err(ContractError::InvalidFee);
    }

    storage::set_initialized(env);
    storage::set_admin(env, admin);
    storage::set_fee_collector(env, fee_collector);
    storage::set_fee_bps(env, fee_bps);
    storage::set_native_token(env, native_token);
    env.storage()
        .instance()
        .set(&DataKey::TotalCreators, &0_u32);
    env.storage().instance().set(&DataKey::TipCount, &0_u32);
    env.storage()
        .instance()
        .set(&DataKey::TotalTipsVolume, &0_i128);
    env.storage()
        .instance()
        .set(&DataKey::TotalFeesCollected, &0_i128);

    Ok(())
}

/// Update a creator's X metrics. Admin only.
pub fn update_x_metrics(
    env: &Env,
    caller: &Address,
    creator: &Address,
    x_followers: u32,
    x_engagement_avg: u32,
) -> Result<(), ContractError> {
    if !storage::is_initialized(env) {
        return Err(ContractError::NotInitialized);
    }

    let admin = storage::get_admin(env);
    if caller != &admin {
        return Err(ContractError::NotAuthorized);
    }

    admin.require_auth();

    if !storage::has_profile(env, creator) {
        return Err(ContractError::NotRegistered);
    }

    let mut profile = storage::get_profile(env, creator);
    let old_score = profile.credit_score;
    let now = env.ledger().timestamp();

    profile.x_followers = x_followers;
    profile.x_engagement_avg = x_engagement_avg;
    profile.updated_at = now;

    let new_score = credit::calculate_credit_score(&profile, now);
    profile.credit_score = new_score;
    storage::set_profile(env, &profile);

    if old_score != new_score {
        events::emit_credit_score_updated(env, creator, old_score, new_score);
    }

    Ok(())
}

// TODO: Implement set_fee, set_fee_collector, set_admin in issues #20, #21, #22
