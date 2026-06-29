#![no_std]

use soroban_sdk::{contracttype, Address, Env, Symbol};

use axionvera_state::{
    GovernanceState, RewardState, StakingState, StateError, TreasuryState, VaultState,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
enum DataKey {
    VaultState,
    StakingState,
    RewardState,
    TreasuryState,
    GovernanceState(Symbol),
}

fn extend(e: &Env) {
    e.storage().instance().extend_ttl(518_400, 518_400);
}

pub fn get_vault_state(e: &Env) -> VaultState {
    e.storage()
        .instance()
        .get(&DataKey::VaultState)
        .unwrap_or(VaultState::Uninitialized)
}

pub fn set_vault_state(e: &Env, new_state: VaultState, _caller: Address) -> Result<VaultState, StateError> {
    let current = get_vault_state(e);
    current.transition(new_state)?;
    e.storage().instance().set(&DataKey::VaultState, &new_state);
    extend(e);
    Ok(new_state)
}

pub fn get_staking_state(e: &Env) -> StakingState {
    e.storage()
        .instance()
        .get(&DataKey::StakingState)
        .unwrap_or(StakingState::Uninitialized)
}

pub fn set_staking_state(e: &Env, new_state: StakingState, _caller: Address) -> Result<StakingState, StateError> {
    let current = get_staking_state(e);
    current.transition(new_state)?;
    e.storage().instance().set(&DataKey::StakingState, &new_state);
    extend(e);
    Ok(new_state)
}

pub fn get_reward_state(e: &Env) -> RewardState {
    e.storage()
        .instance()
        .get(&DataKey::RewardState)
        .unwrap_or(RewardState::Idle)
}

pub fn set_reward_state(e: &Env, new_state: RewardState, _caller: Address) -> Result<RewardState, StateError> {
    let current = get_reward_state(e);
    current.transition(new_state)?;
    e.storage().instance().set(&DataKey::RewardState, &new_state);
    extend(e);
    Ok(new_state)
}

pub fn get_treasury_state(e: &Env) -> TreasuryState {
    e.storage()
        .instance()
        .get(&DataKey::TreasuryState)
        .unwrap_or(TreasuryState::Normal)
}

pub fn set_treasury_state(e: &Env, new_state: TreasuryState, _caller: Address) -> Result<TreasuryState, StateError> {
    let current = get_treasury_state(e);
    current.transition(new_state)?;
    e.storage().instance().set(&DataKey::TreasuryState, &new_state);
    extend(e);
    Ok(new_state)
}

pub fn get_governance_state(e: &Env, proposal_id: Symbol) -> GovernanceState {
    e.storage()
        .instance()
        .get(&DataKey::GovernanceState(proposal_id))
        .unwrap_or(GovernanceState::Draft)
}

pub fn set_governance_state(
    e: &Env,
    proposal_id: Symbol,
    new_state: GovernanceState,
    _caller: Address,
) -> Result<GovernanceState, StateError> {
    let current = get_governance_state(e, proposal_id.clone());
    current.transition(new_state)?;
    e.storage().instance().set(&DataKey::GovernanceState(proposal_id), &new_state);
    extend(e);
    Ok(new_state)
}
