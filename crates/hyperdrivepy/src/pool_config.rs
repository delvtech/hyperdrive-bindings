use crate::{
    extract_address_from_attr, extract_bytes32_from_attr, extract_fees_from_attr,
    extract_u256_from_attr,
};
use hyperdrive_wrappers::wrappers::ihyperdrive::PoolConfig;
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
        let vault_shares_token= extract_address_from_attr(ob, "vaultSharesToken")?;
        let linker_factory = extract_address_from_attr(ob, "linkerFactory")?;
        let linker_code_hash = extract_bytes32_from_attr(ob, "linkerCodeHash")?;
        let initial_vault_share_price = extract_u256_from_attr(ob, "initialVaultSharePrice")?;
        let minimum_share_reserves = extract_u256_from_attr(ob, "minimumShareReserves")?;
        let minimum_transaction_amount = extract_u256_from_attr(ob, "minimumTransactionAmount")?;
        let position_duration = extract_u256_from_attr(ob, "positionDuration")?;
        let checkpoint_duration = extract_u256_from_attr(ob, "checkpointDuration")?;
        let time_stretch = extract_u256_from_attr(ob, "timeStretch")?;
        let governance = extract_address_from_attr(ob, "governance")?;
        let fee_collector = extract_address_from_attr(ob, "feeCollector")?;
        let sweep_collector = extract_address_from_attr(ob, "sweepCollector")?;
        let fees = extract_fees_from_attr(ob, "fees")?;

        let pool_config = PoolConfig {
            base_token,
            vault_shares_token,
            linker_factory,
            linker_code_hash,
            initial_vault_share_price,
            minimum_share_reserves,
            minimum_transaction_amount,
            position_duration,
            checkpoint_duration,
            time_stretch,
            governance,
            fee_collector,
            sweep_collector,
            fees,
        };

        Ok(PyPoolConfig::new(pool_config))
    }
}
