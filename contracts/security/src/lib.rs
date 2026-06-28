#![no_std]

use axionvera_auth::{AccessPolicy, PolicyViolation};
use soroban_sdk::Address;

pub type AddressResolver<Context> = fn(&Context) -> Address;
pub type PredicateResolver<Context> = fn(&Context) -> bool;

/// Validator that requires the selected address to authorize the call.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Authenticated<Context> {
    actor: AddressResolver<Context>,
}

impl<Context> Authenticated<Context> {
    pub const fn new(actor: AddressResolver<Context>) -> Self {
        Self { actor }
    }
}

impl<Context> AccessPolicy<Context> for Authenticated<Context> {
    fn enforce(&self, context: &Context) -> Result<(), PolicyViolation> {
        (self.actor)(context).require_auth();
        Ok(())
    }
}

/// Validator that requires two addresses in the context to match.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MatchAddress<Context> {
    actual: AddressResolver<Context>,
    expected: AddressResolver<Context>,
    violation: PolicyViolation,
}

impl<Context> MatchAddress<Context> {
    pub const fn new(
        actual: AddressResolver<Context>,
        expected: AddressResolver<Context>,
        violation: PolicyViolation,
    ) -> Self {
        Self {
            actual,
            expected,
            violation,
        }
    }
}

impl<Context> AccessPolicy<Context> for MatchAddress<Context> {
    fn enforce(&self, context: &Context) -> Result<(), PolicyViolation> {
        if (self.actual)(context) == (self.expected)(context) {
            Ok(())
        } else {
            Err(self.violation)
        }
    }
}

/// Validator that allows arbitrary predicates to participate in composed policies.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PredicatePolicy<Context> {
    predicate: PredicateResolver<Context>,
    violation: PolicyViolation,
}

impl<Context> PredicatePolicy<Context> {
    pub const fn new(
        predicate: PredicateResolver<Context>,
        violation: PolicyViolation,
    ) -> Self {
        Self {
            predicate,
            violation,
        }
    }
}

impl<Context> AccessPolicy<Context> for PredicatePolicy<Context> {
    fn enforce(&self, context: &Context) -> Result<(), PolicyViolation> {
        if (self.predicate)(context) {
            Ok(())
        } else {
            Err(self.violation)
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use axionvera_auth::AccessPolicy;
    use soroban_sdk::{
        testutils::Address as _,
        Address, Env,
    };

    use super::{Authenticated, MatchAddress, PolicyViolation, PredicatePolicy};

    #[derive(Clone)]
    struct TestContext {
        actor: Address,
        expected: Address,
        allowed: bool,
    }

    fn actor(context: &TestContext) -> Address {
        context.actor.clone()
    }

    fn expected(context: &TestContext) -> Address {
        context.expected.clone()
    }

    fn allowed(context: &TestContext) -> bool {
        context.allowed
    }

    #[test]
    fn composed_validators_accept_matching_actor() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let context = TestContext {
            actor: admin.clone(),
            expected: admin,
            allowed: true,
        };

        let policy = Authenticated::new(actor)
            .and(MatchAddress::new(
                actor,
                expected,
                PolicyViolation::AddressMismatch,
            ))
            .and(PredicatePolicy::new(
                allowed,
                PolicyViolation::PredicateFailed,
            ));

        assert_eq!(policy.enforce(&context), Ok(()));
    }

    #[test]
    fn composed_validators_reject_mismatched_actor() {
        let env = Env::default();
        env.mock_all_auths();

        let context = TestContext {
            actor: Address::generate(&env),
            expected: Address::generate(&env),
            allowed: true,
        };

        let policy = Authenticated::new(actor).and(MatchAddress::new(
            actor,
            expected,
            PolicyViolation::AddressMismatch,
        ));

        assert_eq!(policy.enforce(&context), Err(PolicyViolation::AddressMismatch));
    }
}
