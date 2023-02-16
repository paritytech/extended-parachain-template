use super::Balance;
use frame_support::weights::constants::WEIGHT_REF_TIME_PER_SECOND;

/// Current approximation of the gas per second consumption
pub const GAS_PER_SECOND: u64 = 160_000_000;

/// Approximate ratio of the amount of Weight per Gas.
/// u64 works for approximations because Weight is a very small unit compared to gas.
pub const WEIGHT_PER_GAS: u64 = WEIGHT_REF_TIME_PER_SECOND / GAS_PER_SECOND;

pub const WEI: Balance = 1;
pub const KILOWEI: Balance = 1_000 * WEI;
pub const MEGAWEI: Balance = 1_000 * KILOWEI;
pub const GIGAWEI: Balance = 1_000 * MEGAWEI;
