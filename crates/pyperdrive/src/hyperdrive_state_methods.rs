use ethers::core::types::{I256, U256};
use fixed_point::FixedPoint;
use hyperdrive_math::Asset;

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

    /// Gets the pool's solvency.
    pub fn get_solvency(&self) -> PyResult<String> {
        let result_fp = self.state.get_solvency();
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    /// Get the pool's spot price.
    pub fn get_spot_price(&self) -> PyResult<String> {
        let result_fp = self.state.get_spot_price();
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    /// Get the pool's spot rate.
    pub fn get_spot_rate(&self) -> PyResult<String> {
        let result_fp = self.state.get_spot_rate();
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    /// Get the long amount that will be opened for a given base amount.
    pub fn get_long_amount(&self, base_amount: &str) -> PyResult<String> {
        let base_amount_fp = FixedPoint::from(U256::from_dec_str(base_amount).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert base_amount string to U256")
        })?);
        let result_fp = self.state.get_long_amount(base_amount_fp);
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    /// Get the amount of base the trader will need to deposit for a short of a given size.
    pub fn get_short_deposit(
        &self,
        short_amount: &str,
        spot_price: &str,
        open_share_price: &str,
    ) -> PyResult<String> {
        let short_amount_fp = FixedPoint::from(U256::from_dec_str(short_amount).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert short_amount string to U256")
        })?);
        let spot_price_fp = FixedPoint::from(U256::from_dec_str(spot_price).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert spot_price string to U256")
        })?);
        let open_share_price_fp =
            FixedPoint::from(U256::from_dec_str(open_share_price).map_err(|_| {
                PyErr::new::<PyValueError, _>("Failed to convert open_share_price string to U256")
            })?);
        let result_fp =
            self.state
                .get_short_deposit(short_amount_fp, spot_price_fp, open_share_price_fp);
        let result = match result_fp {
            Some(result) => U256::from(result).to_string(),
            None => {
                return Err(PyErr::new::<PyValueError, _>(
                    "Failed to estimate the short deposit; short_principal is None ",
                ));
            }
        };
        return Ok(result);
    }

    /// Get amount out for a given amount in.
    pub fn get_out_for_in(&self, amount_in: &str, shares_in: bool) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert budget string to U256")
        })?);
        let asset = match shares_in {
            true => Asset::Shares(amount_in_fp),
            false => Asset::Bonds(amount_in_fp),
        };
        let result_fp = self.state.get_out_for_in(asset);
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    /// Get amount out for a given amount in.  Returns a python error instead of panicking.
    pub fn get_out_for_in_safe(&self, amount_in: &str, shares_in: bool) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert budget string to U256")
        })?);
        let asset = match shares_in {
            true => Asset::Shares(amount_in_fp),
            false => Asset::Bonds(amount_in_fp),
        };
        match self.state.get_out_for_in_safe(asset) {
            Some(result_fp) => Ok(U256::from(result_fp).to_string()),
            None => Err(PyErr::new::<PyValueError, _>(
                "get_out_for_in_safe returned None",
            )),
        }
    }

    /// Get amount in for a given amount out.
    pub fn get_in_for_out(&self, amount_out: &str, shares_out: bool) -> PyResult<String> {
        let amount_out_fp = FixedPoint::from(U256::from_dec_str(amount_out).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert budget string to U256")
        })?);
        let asset = match shares_out {
            true => Asset::Shares(amount_out_fp),
            false => Asset::Bonds(amount_out_fp),
        };
        let result_fp = self.state.get_out_for_in(asset);
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    /// Convert a timestamp to the checkpoint timestamp that it corresponds to.
    pub fn to_checkpoint(&self, time: &str) -> PyResult<String> {
        let time_int = U256::from_dec_str(time)
            .map_err(|_| PyErr::new::<PyValueError, _>("Failed to convert time string to U256"))?;
        let result_int = self.state.to_checkpoint(time_int);
        let result = result_int.to_string();
        return Ok(result);
    }

    /// Get the max long that can be opened given a budget.
    pub fn get_max_long(
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
                .get_max_long(budget_fp, checkpoint_exposure_i, maybe_max_iterations);
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    /// Get the max short that can be opened with the given budget.
    pub fn get_max_short(
        &self,
        budget: &str,
        open_share_price: &str,
        checkpoint_exposure: &str,
        maybe_conservative_price: Option<&str>,
        maybe_max_iterations: Option<usize>,
    ) -> PyResult<String> {
        let budget_fp = FixedPoint::from(U256::from_dec_str(budget).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert budget string to U256")
        })?);
        let open_share_price_fp =
            FixedPoint::from(U256::from_dec_str(open_share_price).map_err(|_| {
                PyErr::new::<PyValueError, _>("Failed to convert open_share_price string to U256")
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
        let result_fp = self.state.get_max_short(
            budget_fp,
            open_share_price_fp,
            checkpoint_exposure_i,
            maybe_conservative_price_fp,
            maybe_max_iterations,
        );
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }
}
