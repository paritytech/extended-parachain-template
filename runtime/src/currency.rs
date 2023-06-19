use crate::Balance;

pub const MICROUNIT: Balance = 1_000_000;
pub const MILLIUNIT: Balance = 1_000 * MICROUNIT; // 1_000_000_000
pub const UNIT: Balance = 1_000 * MILLIUNIT;
// Defined in polkadot_runtime_constants::currency::CENTS as 100_000_000_u128
// https://github.com/paritytech/polkadot/blob/47c8c36d6e3f90da77ebccb8b672e4fb4f33c84c/runtime/polkadot/constants/src/lib.rs#L33
pub const CENTS: Balance = MILLIUNIT / 10;

pub const EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

pub const fn deposit(items: u32, bytes: u32) -> Balance {
	(items as Balance * 20 * UNIT + (bytes as Balance) * 100 * MICROUNIT) / 100
}
