use ethers::core::types::{I256, U256};
use fixed_point::FixedPoint;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::PyErr;

use hyperdrive_math::{
    calculate_effective_share_reserves as rs_calculate_effective_share_reserves,
    calculate_initial_bond_reserves as rs_calculate_initial_bond_reserves,
    calculate_time_stretch as rs_calculate_time_stretch,
};

#[pyfunction]
pub fn calculate_initial_bond_reserves(
    effective_share_reserves: &str,
    initial_vault_share_price: &str,
    apr: &str,
    position_duration: &str,
    time_stretch: &str,
) -> PyResult<String> {
    let effective_share_reserves_fp =
        FixedPoint::from(U256::from_dec_str(effective_share_reserves).map_err(|_| {
            PyErr::new::<PyValueError, _>(
                "Failed to convert effective_share_reserves string to U256",
            )
        })?);
    let initial_vault_share_price_fp =
        FixedPoint::from(U256::from_dec_str(initial_vault_share_price).map_err(|_| {
            PyErr::new::<PyValueError, _>(
                "Failed to convert initial_vault_share_price string to U256",
            )
        })?);
    let apr_fp = FixedPoint::from(
        U256::from_dec_str(apr)
            .map_err(|_| PyErr::new::<PyValueError, _>("Failed to convert apr string to U256"))?,
    );
    let position_duration_fp =
        FixedPoint::from(U256::from_dec_str(position_duration).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert position_duration string to U256")
        })?);
    let time_stretch_fp = FixedPoint::from(U256::from_dec_str(time_stretch).map_err(|_| {
        PyErr::new::<PyValueError, _>("Failed to convert time_stretch string to U256")
    })?);
    let result_fp = rs_calculate_initial_bond_reserves(
        effective_share_reserves_fp,
        initial_vault_share_price_fp,
        apr_fp,
        position_duration_fp,
        time_stretch_fp,
    );
    let result = U256::from(result_fp).to_string();
    return Ok(result);
}

#[pyfunction]
pub fn calculate_effective_share_reserves(
    share_reserves: &str,
    share_adjustment: &str,
) -> PyResult<String> {
    let share_reserves_fp = FixedPoint::from(U256::from_dec_str(share_reserves).map_err(|_| {
        PyErr::new::<PyValueError, _>("Failed to convert share_reserves string to U256")
    })?);
    let share_adjustment_i = I256::from_dec_str(share_adjustment).map_err(|_| {
        PyErr::new::<PyValueError, _>("Failed to convert share_adjustment string to I256")
    })?;
    let result_fp = rs_calculate_effective_share_reserves(share_reserves_fp, share_adjustment_i);
    let result = U256::from(result_fp).to_string();
    return Ok(result);
}

#[pyfunction]
pub fn calculate_time_stretch(rate: &str, position_duration: &str) -> PyResult<String> {
    let rate_fp = FixedPoint::from(
        U256::from_dec_str(rate)
            .map_err(|_| PyErr::new::<PyValueError, _>("Failed to convert rate string to U256"))?,
    );
    let position_duration_fp = FixedPoint::from(
        U256::from_dec_str(position_duration)
            .map_err(|_| PyErr::new::<PyValueError, _>("Failed to convert rate string to U256"))?,
    );
    let result_fp = rs_calculate_time_stretch(rate_fp, position_duration_fp);
    let result = U256::from(result_fp).to_string();
    return Ok(result);
}
