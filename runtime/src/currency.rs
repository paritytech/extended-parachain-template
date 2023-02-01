use super::Balance;

pub const MICROUNIT: Balance = 1_000_000;
pub const MILLIUNIT: Balance = 1_000 * MICROUNIT;
pub const UNIT: Balance = 1_000 * MILLIUNIT;

pub const EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

pub const fn deposit(items: u32, bytes: u32) -> Balance {
    // https://github.com/paritytech/cumulus/blob/master/parachains/runtimes/assets/statemint/src/constants.rs#L28
    (items as Balance * 20 * UNIT + (bytes as Balance) * 100 * MICROUNIT) / 100
}
