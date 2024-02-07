use crate::{extract_i256_from_attr, extract_u256_from_attr};
use hyperdrive_wrappers::wrappers::ihyperdrive::PoolInfo;
use pyo3::prelude::*;

pub struct PyPoolInfo {
    pub pool_info: PoolInfo,
}

impl PyPoolInfo {
    pub(crate) fn new(pool_info: PoolInfo) -> Self {
        PyPoolInfo { pool_info }
    }
}

impl FromPyObject<'_> for PyPoolInfo {
    fn extract(ob: &PyAny) -> PyResult<Self> {
        let share_reserves = extract_u256_from_attr(ob, "shareReserves")?;
        let share_adjustment = extract_i256_from_attr(ob, "shareAdjustment")?;
        let zombie_base_proceeds = extract_u256_from_attr(ob, "zombieBaseProceeds")?;
        let zombie_share_reserves = extract_u256_from_attr(ob, "zombieShareReserves")?;
        let bond_reserves = extract_u256_from_attr(ob, "bondReserves")?;
        let lp_total_supply = extract_u256_from_attr(ob, "lpTotalSupply")?;
        let vault_share_price = extract_u256_from_attr(ob, "vaultSharePrice")?;
        let longs_outstanding = extract_u256_from_attr(ob, "longsOutstanding")?;
        let long_average_maturity_time = extract_u256_from_attr(ob, "longAverageMaturityTime")?;
        let shorts_outstanding = extract_u256_from_attr(ob, "shortsOutstanding")?;
        let short_average_maturity_time = extract_u256_from_attr(ob, "shortAverageMaturityTime")?;
        let withdrawal_shares_ready_to_withdraw =
            extract_u256_from_attr(ob, "withdrawalSharesReadyToWithdraw")?;
        let withdrawal_shares_proceeds = extract_u256_from_attr(ob, "withdrawalSharesProceeds")?;
        let lp_share_price = extract_u256_from_attr(ob, "lpSharePrice")?;
        let long_exposure = extract_u256_from_attr(ob, "longExposure")?;

        let pool_info = PoolInfo {
            share_reserves,
            share_adjustment,
            zombie_base_proceeds,
            zombie_share_reserves,
            bond_reserves,
            lp_total_supply,
            vault_share_price,
            longs_outstanding,
            long_average_maturity_time,
            shorts_outstanding,
            short_average_maturity_time,
            withdrawal_shares_ready_to_withdraw,
            withdrawal_shares_proceeds,
            lp_share_price,
            long_exposure,
        };

        Ok(PyPoolInfo::new(pool_info))
    }
}
