//! Tests for tip record storage with temporary TTL (issue #10).
//!
//! Test cases covered:
//! - Successful tip (balance updates, tip record created)
//! - Tip to unregistered creator → NotRegistered
//! - Tip amount = 0 → InvalidAmount
//! - Tip to self → CannotTipSelf
//! - Message length validation
//! - Multiple tips accumulate correctly
//! - Global stats update after tip

#![cfg(test)]

use soroban_sdk::{testutils::Address as _, token, Address, Env, String};

use crate::errors::ContractError;
use crate::storage::DataKey;
use crate::types::{Profile, Tip};
use crate::TipzContract;
use crate::TipzContractClient;

/// Helper: set up a test environment with the contract initialized
/// and a registered creator profile.
fn setup_env() -> (Env, TipzContractClient<'static>, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    // Register the Tipz contract
    let contract_id = env.register_contract(None, TipzContract);
    let client = TipzContractClient::new(&env, &contract_id);

    // Register a Stellar Asset Contract for native XLM
    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_address = token_contract.address();
    let token_admin_client = token::StellarAssetClient::new(&env, &token_address);

    // Initialize the contract
    let admin = Address::generate(&env);
    let fee_collector = Address::generate(&env);
    client.initialize(&admin, &fee_collector, &200, &token_address);

    // Create a registered creator profile
    let creator = Address::generate(&env);
    let now = env.ledger().timestamp();
    let profile = Profile {
        owner: creator.clone(),
        username: String::from_str(&env, "alice"),
        display_name: String::from_str(&env, "Alice"),
        bio: String::from_str(&env, "Hello!"),
        image_url: String::from_str(&env, ""),
        x_handle: String::from_str(&env, "alice_x"),
        x_followers: 0,
        x_engagement_avg: 0,
        credit_score: 0,
        total_tips_received: 0,
        total_tips_count: 0,
        balance: 0,
        registered_at: now,
        updated_at: now,
    };
    env.as_contract(&contract_id, || {
        env.storage()
            .persistent()
            .set(&DataKey::Profile(creator.clone()), &profile);
    });

    // Create a tipper with funds
    let tipper = Address::generate(&env);
    token_admin_client.mint(&tipper, &100_000_000_000); // 10,000 XLM

    (env, client, contract_id, tipper, creator)
}

#[test]
fn test_send_tip_success() {
    let (env, client, contract_id, tipper, creator) = setup_env();

    let message = String::from_str(&env, "Great work!");
    let amount: i128 = 10_000_000; // 1 XLM

    client.send_tip(&tipper, &creator, &amount, &message);

    // Verify creator's profile was updated
    env.as_contract(&contract_id, || {
        let profile: Profile = env
            .storage()
            .persistent()
            .get(&DataKey::Profile(creator.clone()))
            .unwrap();
        assert_eq!(profile.balance, amount);
        assert_eq!(profile.total_tips_received, amount);
        assert_eq!(profile.total_tips_count, 1);
    });

    // Verify tip record was created in temporary storage
    env.as_contract(&contract_id, || {
        let tip: Tip = env.storage().temporary().get(&DataKey::Tip(0)).unwrap();
        assert_eq!(tip.tipper, tipper);
        assert_eq!(tip.creator, creator);
        assert_eq!(tip.amount, amount);
    });

    // Verify global stats were updated
    env.as_contract(&contract_id, || {
        let tip_count: u32 = env.storage().instance().get(&DataKey::TipCount).unwrap();
        assert_eq!(tip_count, 1);
        let total_volume: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalTipsVolume)
            .unwrap();
        assert_eq!(total_volume, amount);
    });
}

#[test]
fn test_send_tip_not_registered() {
    let (env, client, _contract_id, tipper, _creator) = setup_env();

    let unregistered = Address::generate(&env);
    let message = String::from_str(&env, "Hello");

    let result = client.try_send_tip(&tipper, &unregistered, &10_000_000, &message);
    assert_eq!(result, Err(Ok(ContractError::NotRegistered)));
}

#[test]
fn test_send_tip_cannot_tip_self() {
    let (env, client, contract_id, _tipper, _creator) = setup_env();

    // Register a self-tipper as a creator
    let self_tipper = Address::generate(&env);
    let now = env.ledger().timestamp();
    let profile = Profile {
        owner: self_tipper.clone(),
        username: String::from_str(&env, "bob"),
        display_name: String::from_str(&env, "Bob"),
        bio: String::from_str(&env, ""),
        image_url: String::from_str(&env, ""),
        x_handle: String::from_str(&env, ""),
        x_followers: 0,
        x_engagement_avg: 0,
        credit_score: 0,
        total_tips_received: 0,
        total_tips_count: 0,
        balance: 0,
        registered_at: now,
        updated_at: now,
    };
    env.as_contract(&contract_id, || {
        env.storage()
            .persistent()
            .set(&DataKey::Profile(self_tipper.clone()), &profile);
    });

    let message = String::from_str(&env, "Self tip");
    let result = client.try_send_tip(&self_tipper, &self_tipper, &10_000_000, &message);
    assert_eq!(result, Err(Ok(ContractError::CannotTipSelf)));
}

#[test]
fn test_send_tip_invalid_amount_zero() {
    let (env, client, _contract_id, tipper, creator) = setup_env();

    let message = String::from_str(&env, "Zero tip");
    let result = client.try_send_tip(&tipper, &creator, &0, &message);
    assert_eq!(result, Err(Ok(ContractError::InvalidAmount)));
}

#[test]
fn test_send_tip_invalid_amount_negative() {
    let (env, client, _contract_id, tipper, creator) = setup_env();

    let message = String::from_str(&env, "Negative tip");
    let result = client.try_send_tip(&tipper, &creator, &-1, &message);
    assert_eq!(result, Err(Ok(ContractError::InvalidAmount)));
}

#[test]
fn test_send_tip_message_too_long() {
    let (env, client, _contract_id, tipper, creator) = setup_env();

    // Create a message longer than 280 characters
    let long_msg = String::from_str(
        &env,
        "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
    );
    let result = client.try_send_tip(&tipper, &creator, &10_000_000, &long_msg);
    assert_eq!(result, Err(Ok(ContractError::MessageTooLong)));
}

#[test]
fn test_send_tip_multiple_tips_accumulate() {
    let (env, client, contract_id, tipper, creator) = setup_env();

    let message = String::from_str(&env, "Tip!");
    let amount: i128 = 5_000_000;

    // Send 3 tips
    client.send_tip(&tipper, &creator, &amount, &message);
    client.send_tip(&tipper, &creator, &amount, &message);
    client.send_tip(&tipper, &creator, &amount, &message);

    // Verify accumulated balance and counts
    env.as_contract(&contract_id, || {
        let profile: Profile = env
            .storage()
            .persistent()
            .get(&DataKey::Profile(creator.clone()))
            .unwrap();
        assert_eq!(profile.balance, amount * 3);
        assert_eq!(profile.total_tips_received, amount * 3);
        assert_eq!(profile.total_tips_count, 3);
    });

    // Verify global stats
    env.as_contract(&contract_id, || {
        let tip_count: u32 = env.storage().instance().get(&DataKey::TipCount).unwrap();
        assert_eq!(tip_count, 3);
        let total_volume: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalTipsVolume)
            .unwrap();
        assert_eq!(total_volume, amount * 3);
    });

    // Verify each tip record exists
    env.as_contract(&contract_id, || {
        for i in 0..3u32 {
            let tip: Tip = env.storage().temporary().get(&DataKey::Tip(i)).unwrap();
            assert_eq!(tip.amount, amount);
        }
    });
}

#[test]
fn test_send_tip_empty_message_allowed() {
    let (env, client, contract_id, tipper, creator) = setup_env();

    let message = String::from_str(&env, "");
    let amount: i128 = 10_000_000;

    client.send_tip(&tipper, &creator, &amount, &message);

    env.as_contract(&contract_id, || {
        let profile: Profile = env
            .storage()
            .persistent()
            .get(&DataKey::Profile(creator.clone()))
            .unwrap();
        assert_eq!(profile.balance, amount);
    });
}
