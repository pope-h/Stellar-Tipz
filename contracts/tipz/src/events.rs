//! Event emission helpers for the Tipz contract.
//!
//! Every on-chain action that mutates meaningful state should emit an event so
//! that off-chain indexers can follow contract activity without replaying every
//! transaction.
//!
//! ## Naming convention
//! Topic tuple  → `(Symbol,)`          – identifies the event type
//! Data tuple   → `(field, field, …)`  – the payload
//!
//! ADD THE THREE FUNCTIONS BELOW to your existing events.rs file.
//! The functions already present in your file (emit_credit_score_updated,
//! emit_x_metrics_batch_skipped, etc.) remain unchanged.

use soroban_sdk::{symbol_short, Address, Env};

// ── Existing helpers (keep whatever you already have) ────────────────────────
// pub fn emit_credit_score_updated(...)  { ... }
// pub fn emit_x_metrics_batch_skipped(...) { ... }
// ... etc.

// ── New helpers required by this issue ───────────────────────────────────────

/// Emitted by `set_fee` when the platform fee is changed.
///
/// Topics : `("FeeUpdated",)`
/// Data   : `(old_bps: u32, new_bps: u32)`
pub fn emit_fee_updated(env: &Env, old_bps: u32, new_bps: u32) {
    env.events()
        .publish((symbol_short!("FeeUpdate"),), (old_bps, new_bps));
}

/// Emitted by `set_fee_collector` when the fee-receiving address changes.
///
/// Topics : `("FeeColl",)`
/// Data   : `(new_collector: Address,)`
pub fn emit_fee_collector_updated(env: &Env, new_collector: &Address) {
    env.events()
        .publish((symbol_short!("FeeColl"),), (new_collector.clone(),));
}

/// Emitted by `set_admin` when the admin role is transferred.
///
/// Topics : `("AdminChg",)`
/// Data   : `(old_admin: Address, new_admin: Address)`
pub fn emit_admin_changed(env: &Env, old_admin: &Address, new_admin: &Address) {
    env.events().publish(
        (symbol_short!("AdminChg"),),
        (old_admin.clone(), new_admin.clone()),
    );
}