//! Admin operations for the Tipz contract.
//!
//! - Contract initialization
//! - Fee management
//! - Admin role transfer

use soroban_sdk::{Address, Env};

use crate::errors::ContractError;
use crate::storage::DataKey;

/// Initialize the contract. Can only be called once.
pub fn initialize(
    env: &Env,
    admin: &Address,
    fee_collector: &Address,
    fee_bps: u32,
    native_token: &Address,
) -> Result<(), ContractError> {
    // Check not already initialized
    if env.storage().instance().has(&DataKey::Initialized) {
        return Err(ContractError::AlreadyInitialized);
    }

    // Validate fee
    if fee_bps > 1000 {
        return Err(ContractError::InvalidFee);
    }

    // Store initial config
    env.storage().instance().set(&DataKey::Initialized, &true);
    env.storage().instance().set(&DataKey::Admin, admin);
    env.storage()
        .instance()
        .set(&DataKey::FeeCollector, fee_collector);
    env.storage().instance().set(&DataKey::FeePercent, &fee_bps);
    env.storage()
        .instance()
        .set(&DataKey::TotalFeesCollected, &0_i128);
    env.storage()
        .instance()
        .set(&DataKey::TotalCreators, &0_u32);
    env.storage().instance().set(&DataKey::TipCount, &0_u32);
    env.storage()
        .instance()
        .set(&DataKey::TotalTipsVolume, &0_i128);
    env.storage()
        .instance()
        .set(&DataKey::NativeToken, native_token);

    Ok(())
}

// TODO: Implement set_fee, set_fee_collector, set_admin in issues #20, #21, #22
