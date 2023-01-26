use super::Balance;
use frame_support::weights::constants::WEIGHT_PER_SECOND;

pub const MILLICENTS: Balance = 1_000_000_000;
pub const CENTS: Balance = 1_000 * MILLICENTS; // assume this is worth about a cent.
pub const DOLLARS: Balance = 100 * CENTS;

pub const fn _deposit(items: u32, bytes: u32) -> Balance {
    items as Balance * 15 * CENTS + (bytes as Balance) * 6 * CENTS
}

/// Current approximation of the gas per second consumption
pub const GAS_PER_SECOND: u64 = 160_000_000;

/// Approximate ratio of the amount of Weight per Gas.
/// u64 works for approximations because Weight is a very small unit compared to gas.
pub const WEIGHT_PER_GAS: u64 = WEIGHT_PER_SECOND.ref_time() / GAS_PER_SECOND;


pub const WEI: Balance = 1;
pub const KILOWEI: Balance = 1_000 * WEI;
pub const MEGAWEI: Balance = 1_000 * KILOWEI;
pub const GIGAWEI: Balance = 1_000 * MEGAWEI;