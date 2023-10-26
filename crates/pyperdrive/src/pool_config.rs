use crate::{extract_address_from_attr, extract_fees_from_attr, extract_u256_from_attr};
use hyperdrive_wrappers::wrappers::i_hyperdrive::PoolConfig;
use pyo3::prelude::*;

pub struct PyPoolConfig {
    pub pool_config: PoolConfig,
}

impl PyPoolConfig {
    pub(crate) fn new(pool_config: PoolConfig) -> Self {
        PyPoolConfig { pool_config }
    }
}

impl FromPyObject<'_> for PyPoolConfig {
    fn extract(ob: &PyAny) -> PyResult<Self> {
        let base_token = extract_address_from_attr(ob, "baseToken")?;
        let initial_share_price = extract_u256_from_attr(ob, "initialSharePrice")?;
        let minimum_share_reserves = extract_u256_from_attr(ob, "minimumShareReserves")?;
        let minimum_transaction_amount = extract_u256_from_attr(ob, "minimumTransactionAmount")?;
        let position_duration = extract_u256_from_attr(ob, "positionDuration")?;
        let checkpoint_duration = extract_u256_from_attr(ob, "checkpointDuration")?;
        let time_stretch = extract_u256_from_attr(ob, "timeStretch")?;
        let governance = extract_address_from_attr(ob, "governance")?;
        let fee_collector = extract_address_from_attr(ob, "feeCollector")?;
        let fees = extract_fees_from_attr(ob, "fees")?;
        let oracle_size = extract_u256_from_attr(ob, "oracleSize")?;
        let update_gap = extract_u256_from_attr(ob, "updateGap")?;

        let pool_config = PoolConfig {
            base_token,
            initial_share_price,
            minimum_share_reserves,
            minimum_transaction_amount,
            position_duration,
            checkpoint_duration,
            time_stretch,
            governance,
            fee_collector,
            fees,
            oracle_size,
            update_gap,
        };

        Ok(PyPoolConfig::new(pool_config))
    }
}
