use ethers::core::types::{Address, I256, U256};
use fixed_point::FixedPoint;
use hyperdrive_math::Asset;
use hyperdrive_wrappers::wrappers::i_hyperdrive::{Fees, PoolConfig, PoolInfo};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::PyErr;

use hyperdrive_math::{
    calculate_bonds_given_shares_and_rate as rs_calculate_bonds_given_shares_and_rate,
    get_effective_share_reserves as rs_get_effective_share_reserves,
    get_time_stretch as rs_get_time_stretch, State, YieldSpace,
};


/// /// /// ///
use ethers::core::types::{Address, I256, U256};
use fixed_point::FixedPoint;
use hyperdrive_math::hyperdrive_math::State;
use hyperdrive_math::Asset;
use hyperdrive_wrappers::wrappers::i_hyperdrive::{Fees, PoolConfig, PoolInfo};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::PyErr;

impl From<pool_config: &PyAny, pool_info: &PyAny> for State {

/// Get the spot price for a Hyperdrive market with the given pool state
#[pyfunction]
fn get_spot_price(pool_config: &PyAny, pool_info: &PyAny) -> PyResult<String> {
    let hyperdrive_state: HyperdriveState = (pool_config, pool_info).into();
    return hyperdrive_state.get_spot_price();
}

/// Get the spot rate (fixed rate) for a Hyperdrive market with the given pool state
#[pyfunction]
fn get_spot_rate(pool_config: &PyAny, pool_info: &PyAny) -> PyResult<String> {
    let hyperdrive_state: HyperdriveState = (pool_config, pool_info).into();
    return hyperdrive_state.get_spot_rate();
}

/// Get the the amount out of an asset for a corresponding amount in.
#[pyfunction]
fn get_out_for_in(
    pool_config: &PyAny,
    pool_info: &PyAny,
    amount_in: &str,
    shares_in: bool,
) -> PyResult<String> {
    let hyperdrive_state: HyperdriveState = (pool_config, pool_info).into();
    return hyperdrive_state.get_out_for_in(amount_in, shares_in);
}

/// Get the the amount out of an asset for a corresponding amount in.  Safe means it will return a
/// status instead of panicking.
#[pyfunction]
fn get_out_for_in_safe(
    pool_config: &PyAny,
    pool_info: &PyAny,
    amount_in: &str,
    shares_in: bool,
) -> PyResult<String> {
    let hyperdrive_state: HyperdriveState = (pool_config, pool_info).into();
    return hyperdrive_state.get_out_for_in_safe(amount_in, shares_in);
}

/// Get the the amount in of an asset for a corresponding amount out.
#[pyfunction]
fn get_in_for_out(
    pool_config: &PyAny,
    pool_info: &PyAny,
    amount_out: &str,
    shares_out: bool,
) -> PyResult<String> {
    let hyperdrive_state: HyperdriveState = (pool_config, pool_info).into();
    return hyperdrive_state.get_out_for_in(amount_out, shares_out);
}

/// Get the max long for a Hyperdrive market with the given pool state
#[pyfunction]
fn get_max_long(
    pool_config: &PyAny,
    pool_info: &PyAny,
    budget: &str,
    checkpoint_exposure: &str,
    maybe_max_iterations: Option<usize>,
) -> PyResult<String> {
    let hyperdrive_state: HyperdriveState = (pool_config, pool_info).into();
    return hyperdrive_state.get_max_long(budget, checkpoint_exposure, maybe_max_iterations);
}

/// Get the max short for a Hyperdrive market with the given pool state
#[pyfunction]
fn get_max_short(
    pool_config: &PyAny,
    pool_info: &PyAny,
    budget: &str,
    open_share_price: &str,
    checkpoint_exposure: &str,
    maybe_conservative_price: Option<&str>,
    maybe_max_iterations: Option<usize>,
) -> PyResult<String> {
    let hyperdrive_state: HyperdriveState = (pool_config, pool_info).into();
    return hyperdrive_state.get_max_short(
        budget,
        open_share_price,
        checkpoint_exposure,
        maybe_conservative_price,
        maybe_max_iterations,
    );
}

/// Get the amount of bonds required for a given pool's share reserves and spot rate
#[pyfunction]
fn calculate_bonds_given_shares_and_rate(
    effective_share_reserves: &str,
    initial_share_price: &str,
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
    let initial_share_price_fp =
        FixedPoint::from(U256::from_dec_str(initial_share_price).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert initial_share_price string to U256")
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
    let result_fp = rs_calculate_bonds_given_shares_and_rate(
        effective_share_reserves_fp,
        initial_share_price_fp,
        apr_fp,
        position_duration_fp,
        time_stretch_fp,
    );
    let result = U256::from(result_fp).to_string();
    return Ok(result);
}

/// Get the effective share reserves given share reserves and share adjustments
#[pyfunction]
fn get_effective_share_reserves(share_reserves: &str, share_adjustment: &str) -> PyResult<String> {
    let share_reserves_fp = FixedPoint::from(U256::from_dec_str(share_reserves).map_err(|_| {
        PyErr::new::<PyValueError, _>("Failed to convert share_reserves string to U256")
    })?);
    let share_adjustment_i = I256::from_dec_str(share_adjustment).map_err(|_| {
        PyErr::new::<PyValueError, _>("Failed to convert share_adjustment string to I256")
    })?;
    let result_fp = rs_get_effective_share_reserves(share_reserves_fp, share_adjustment_i);
    let result = U256::from(result_fp).to_string();
    return Ok(result);
}

/// Get the time stretch given a market rate
#[pyfunction]
fn get_time_stretch(rate: &str) -> PyResult<String> {
    let rate_fp = FixedPoint::from(
        U256::from_dec_str(rate)
            .map_err(|_| PyErr::new::<PyValueError, _>("Failed to convert rate string to U256"))?,
    );
    let result_fp = rs_get_time_stretch(rate_fp);
    let result = U256::from(result_fp).to_string();
    return Ok(result);
}

/// Get the share reserves after subtracting the adjustment used for
/// A pyO3 wrapper for the hyperdrie_math crate.
#[pymodule]
#[pyo3(name = "pyperdrive")]
fn pyperdrive(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_spot_price, m)?)?;
    m.add_function(wrap_pyfunction!(get_spot_rate, m)?)?;
    m.add_function(wrap_pyfunction!(get_max_long, m)?)?;
    m.add_function(wrap_pyfunction!(get_max_short, m)?)?;
    m.add_function(wrap_pyfunction!(get_out_for_in, m)?)?;
    m.add_function(wrap_pyfunction!(get_out_for_in_safe, m)?)?;
    m.add_function(wrap_pyfunction!(get_in_for_out, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_bonds_given_shares_and_rate, m)?)?;
    m.add_function(wrap_pyfunction!(get_effective_share_reserves, m)?)?;
    m.add_function(wrap_pyfunction!(get_time_stretch, m)?)?;
    Ok(())
}
