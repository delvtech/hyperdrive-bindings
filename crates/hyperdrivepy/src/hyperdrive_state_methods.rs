use ethers::core::types::{I256, U256};
use fixed_point::FixedPoint;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::PyErr;

pub use crate::utils::*;
use crate::HyperdriveState;
pub use crate::PyPoolConfig;
pub use crate::PyPoolInfo;
use hyperdrive_math::State;
use hyperdrive_math::YieldSpace;

#[pymethods]
impl HyperdriveState {
    #[new]
    pub fn __init__(pool_config: &PyAny, pool_info: &PyAny) -> PyResult<Self> {
        let rust_pool_config = PyPoolConfig::extract(pool_config)?.pool_config;
        let rust_pool_info = PyPoolInfo::extract(pool_info)?.pool_info;
        let state = State::new(rust_pool_config, rust_pool_info);
        Ok(HyperdriveState::new(state))
    }

    pub fn calculate_max_spot_price(&self) -> PyResult<String> {
        let result_fp = self.state.calculate_max_spot_price();
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_spot_price_after_long(&self, long_amount: &str) -> PyResult<String> {
        let long_amount_fp = FixedPoint::from(U256::from_dec_str(long_amount).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert long_amount string to U256")
        })?);
        let result_fp = self
            .state
            .calculate_spot_price_after_long(long_amount_fp, None)
            .unwrap();
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_solvency(&self) -> PyResult<String> {
        let result_fp = self.state.calculate_solvency();
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_spot_price(&self) -> PyResult<String> {
        let result_fp = self.state.calculate_spot_price();
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_spot_rate(&self) -> PyResult<String> {
        let result_fp = self.state.calculate_spot_rate();
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_open_long(&self, base_amount: &str) -> PyResult<String> {
        let base_amount_fp = FixedPoint::from(U256::from_dec_str(base_amount).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert base_amount string to U256")
        })?);
        let result_fp = self.state.calculate_open_long(base_amount_fp).unwrap();
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_close_long(
        &self,
        bond_amount: &str,
        maturity_time: &str,
        current_time: &str,
    ) -> PyResult<String> {
        let bond_amount_fp = FixedPoint::from(U256::from_dec_str(bond_amount).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert bond_amount string to U256")
        })?);
        let maturity_time = U256::from_dec_str(maturity_time).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert maturity_time string to U256")
        })?;
        let current_time = U256::from_dec_str(current_time).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert current_time string to U256")
        })?;

        let result_fp =
            self.state
                .calculate_close_long(bond_amount_fp, maturity_time, current_time);
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_open_short(
        &self,
        short_amount: &str,
        spot_price: &str,
        open_vault_share_price: &str,
    ) -> PyResult<String> {
        let short_amount_fp = FixedPoint::from(U256::from_dec_str(short_amount).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert short_amount string to U256")
        })?);
        let spot_price_fp = FixedPoint::from(U256::from_dec_str(spot_price).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert spot_price string to U256")
        })?);
        let open_vault_share_price_fp =
            FixedPoint::from(U256::from_dec_str(open_vault_share_price).map_err(|_| {
                PyErr::new::<PyValueError, _>(
                    "Failed to convert open_vault_share_price string to U256",
                )
            })?);
        let result_fp = self
            .state
            .calculate_open_short(short_amount_fp, spot_price_fp, open_vault_share_price_fp)
            .unwrap();
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_close_short(
        &self,
        bond_amount: &str,
        open_vault_share_price: &str,
        close_vault_share_price: &str,
        maturity_time: &str,
        current_time: &str,
    ) -> PyResult<String> {
        let bond_amount_fp = FixedPoint::from(U256::from_dec_str(bond_amount).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert bond_amount string to U256")
        })?);
        let open_vault_share_price_fp =
            FixedPoint::from(U256::from_dec_str(open_vault_share_price).map_err(|_| {
                PyErr::new::<PyValueError, _>(
                    "Failed to convert open_vault_share_price string to U256",
                )
            })?);
        let close_vault_share_price_fp =
            FixedPoint::from(U256::from_dec_str(close_vault_share_price).map_err(|_| {
                PyErr::new::<PyValueError, _>(
                    "Failed to convert close_vault_share_price string to U256",
                )
            })?);
        let maturity_time = U256::from_dec_str(maturity_time).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert maturity_time string to U256")
        })?;
        let current_time = U256::from_dec_str(current_time).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert current_time string to U256")
        })?;
        let result_fp = self.state.calculate_close_short(
            bond_amount_fp,
            open_vault_share_price_fp,
            close_vault_share_price_fp,
            maturity_time,
            current_time,
        );
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_bonds_out_given_shares_in_down(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert amount_in string to U256")
        })?);
        let result_fp = self
            .state
            .calculate_bonds_out_given_shares_in_down(amount_in_fp);
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_shares_in_given_bonds_out_up(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert amount_in string to U256")
        })?);
        // We unwrap the error here to throw panic error if this fails
        let result_fp = self
            .state
            .calculate_shares_in_given_bonds_out_up_safe(amount_in_fp)
            .unwrap();
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_shares_in_given_bonds_out_down(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert amount_in string to U256")
        })?);
        let result_fp = self
            .state
            .calculate_shares_in_given_bonds_out_down(amount_in_fp);
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_shares_out_given_bonds_in_down(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert amount_in string to U256")
        })?);
        let result_fp = self
            .state
            .calculate_shares_out_given_bonds_in_down(amount_in_fp);
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn to_checkpoint(&self, time: &str) -> PyResult<String> {
        let time_int = U256::from_dec_str(time)
            .map_err(|_| PyErr::new::<PyValueError, _>("Failed to convert time string to U256"))?;
        let result_int = self.state.to_checkpoint(time_int);
        let result = result_int.to_string();
        return Ok(result);
    }

    pub fn calculate_max_long(
        &self,
        budget: &str,
        checkpoint_exposure: &str,
        maybe_max_iterations: Option<usize>,
    ) -> PyResult<String> {
        let budget_fp = FixedPoint::from(U256::from_dec_str(budget).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert budget string to U256")
        })?);
        let checkpoint_exposure_i = I256::from_dec_str(checkpoint_exposure).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert checkpoint_exposure string to I256")
        })?;
        let result_fp =
            self.state
                .calculate_max_long(budget_fp, checkpoint_exposure_i, maybe_max_iterations);
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_max_short(
        &self,
        budget: &str,
        open_vault_share_price: &str,
        checkpoint_exposure: &str,
        maybe_conservative_price: Option<&str>,
        maybe_max_iterations: Option<usize>,
    ) -> PyResult<String> {
        let budget_fp = FixedPoint::from(U256::from_dec_str(budget).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert budget string to U256")
        })?);
        let open_vault_share_price_fp =
            FixedPoint::from(U256::from_dec_str(open_vault_share_price).map_err(|_| {
                PyErr::new::<PyValueError, _>(
                    "Failed to convert open_vault_share_price string to U256",
                )
            })?);
        let checkpoint_exposure_i = I256::from_dec_str(checkpoint_exposure).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert checkpoint_exposure string to I256")
        })?;
        let maybe_conservative_price_fp = if let Some(conservative_price) = maybe_conservative_price
        {
            Some(FixedPoint::from(
                U256::from_dec_str(conservative_price).map_err(|_| {
                    PyErr::new::<PyValueError, _>(
                        "Failed to convert maybe_conservative_price string to U256",
                    )
                })?,
            ))
        } else {
            None
        };
        let result_fp = self.state.calculate_max_short(
            budget_fp,
            open_vault_share_price_fp,
            checkpoint_exposure_i,
            maybe_conservative_price_fp,
            maybe_max_iterations,
        );
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_present_value(&self, current_block_timestamp: &str) -> PyResult<String> {
        let current_block_timestamp_int =
            U256::from_dec_str(current_block_timestamp).map_err(|_| {
                PyErr::new::<PyValueError, _>(
                    "Failed to convert current_block_timestamp string to U256",
                )
            })?;
        let result_fp = self
            .state
            .calculate_present_value(current_block_timestamp_int);
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_idle_share_reserves_in_base(&self) -> PyResult<String> {
        let result_fp = self.state.calculate_idle_share_reserves_in_base();
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }
}
