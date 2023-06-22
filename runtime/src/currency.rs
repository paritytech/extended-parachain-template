use crate::Balance;

pub const MICROUNIT: Balance = 1_000_000;
pub const MILLIUNIT: Balance = 1_000 * MICROUNIT; // 1_000_000_000
pub const UNIT: Balance = 1_000 * MILLIUNIT;

pub const EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

pub const fn deposit(items: u32, bytes: u32) -> Balance {
	(items as Balance * 20 * UNIT + (bytes as Balance) * 100 * MICROUNIT) / 100
}
