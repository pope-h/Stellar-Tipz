//! Tipping logic for the Tipz contract.
//!
//! Handles:
//! - send_tip: Transfer XLM from tipper → contract, credit creator balance
//! - withdraw_tips: Transfer from contract → creator, deduct fee

use soroban_sdk::{token, Address, Env, String};

use crate::errors::ContractError;
use crate::events::emit_tip_sent;
use crate::storage::DataKey;
use crate::types::{Profile, Tip};

/// Send an XLM tip from `tipper` to a registered `creator`.
pub fn send_tip(
    env: &Env,
    tipper: &Address,
    creator: &Address,
    amount: i128,
    message: &String,
) -> Result<(), ContractError> {
    // 1. Require tipper authorization
    tipper.require_auth();

    // 2. Validate creator is registered
    if !env
        .storage()
        .persistent()
        .has(&DataKey::Profile(creator.clone()))
    {
        return Err(ContractError::NotRegistered);
    }

    // 3. Validate tipper != creator
    if tipper == creator {
        return Err(ContractError::CannotTipSelf);
    }

    // 4. Validate amount > 0
    if amount <= 0 {
        return Err(ContractError::InvalidAmount);
    }

    // 5. Validate message length ≤ 280 chars
    if message.len() > 280 {
        return Err(ContractError::MessageTooLong);
    }

    // 6. Transfer XLM from tipper to contract via the Stellar Asset Contract (SAC)
    let native_token: Address = env
        .storage()
        .instance()
        .get(&DataKey::NativeToken)
        .ok_or(ContractError::NotInitialized)?;
    let token_client = token::Client::new(env, &native_token);
    let contract_address = env.current_contract_address();
    token_client.transfer(tipper, &contract_address, &amount);

    // 7. Credit amount to creator's balance
    let mut profile: Profile = env
        .storage()
        .persistent()
        .get(&DataKey::Profile(creator.clone()))
        .unwrap();
    profile.balance += amount;
    profile.total_tips_received += amount;
    profile.total_tips_count += 1;
    env.storage()
        .persistent()
        .set(&DataKey::Profile(creator.clone()), &profile);

    // 8. Create Tip record and store in temporary storage
    let tip_count: u32 = env
        .storage()
        .instance()
        .get(&DataKey::TipCount)
        .unwrap_or(0);
    let tip = Tip {
        from: tipper.clone(),
        to: creator.clone(),
        amount,
        message: message.clone(),
        timestamp: env.ledger().timestamp(),
    };
    env.storage()
        .temporary()
        .set(&DataKey::Tip(tip_count), &tip);

    // 9. Increment global TipCount and TotalTipsVolume
    env.storage()
        .instance()
        .set(&DataKey::TipCount, &(tip_count + 1));
    let total_volume: i128 = env
        .storage()
        .instance()
        .get(&DataKey::TotalTipsVolume)
        .unwrap_or(0);
    env.storage()
        .instance()
        .set(&DataKey::TotalTipsVolume, &(total_volume + amount));

    // 10. Emit TipSent event
    emit_tip_sent(env, tipper, creator, amount);

    Ok(())
}

// TODO: Implement withdraw_tips in issue #10
