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

#[pyclass(module = "pyperdrive", name = "HyperdriveState")]
pub struct HyperdriveState {
    pub state: State,
}

impl HyperdriveState {
    pub(crate) fn new(state: State) -> Self {
        HyperdriveState { state }
    }

    pub(crate) fn new_from_pool(pool_config: &PyAny, pool_info: &PyAny) -> Self {
        let rust_pool_config = match PyPoolConfig::extract(pool_config) {
            Ok(py_pool_config) => py_pool_config.pool_config,
            Err(err) => {
                panic!("Error extracting pool config: {:?}", err);
            }
        };
        let rust_pool_info = match PyPoolInfo::extract(pool_info) {
            Ok(py_pool_info) => py_pool_info.pool_info,
            Err(err) => {
                // Handle the error, e.g., printing an error message or panicking
                panic!("Error extracting pool info: {:?}", err);
            }
        };
        let state = State::new(rust_pool_config, rust_pool_info);
        HyperdriveState::new(state)
    }
}

impl From<State> for HyperdriveState {
    fn from(state: State) -> Self {
        HyperdriveState::new(state)
    }
}

impl From<(&PyAny, &PyAny)> for HyperdriveState {
    fn from(args: (&PyAny, &PyAny)) -> Self {
        HyperdriveState::new_from_pool(args.0, args.1)
    }
}

pub struct PyPoolConfig {
    pub pool_config: PoolConfig,
}

// Helper function to extract U256 values from Python object attributes
fn extract_u256_from_attr(ob: &PyAny, attr: &str) -> PyResult<U256> {
    let value_str: String = ob.getattr(attr)?.extract()?;
    U256::from_dec_str(&value_str)
        .map_err(|e| PyErr::new::<PyValueError, _>(format!("Invalid U256 for {}: {}", attr, e)))
}

// Helper function to extract I256 values from Python object attributes
fn extract_i256_from_attr(ob: &PyAny, attr: &str) -> PyResult<I256> {
    let value_str: String = ob.getattr(attr)?.extract()?;
    I256::from_dec_str(&value_str)
        .map_err(|e| PyErr::new::<PyValueError, _>(format!("Invalid I256 for {}: {}", attr, e)))
}

// Helper function to extract Ethereum Address values from Python object attributes
fn extract_address_from_attr(ob: &PyAny, attr: &str) -> PyResult<Address> {
    let address_str: String = ob.getattr(attr)?.extract()?;
    address_str.parse::<Address>().map_err(|e| {
        PyErr::new::<PyValueError, _>(format!("Invalid Ethereum address for {}: {}", attr, e))
    })
}

fn extract_fees_from_attr(ob: &PyAny, attr: &str) -> PyResult<Fees> {
    let fees_obj = ob.getattr(attr)?;

    let curve = extract_u256_from_attr(&fees_obj, "curve")?;
    let flat = extract_u256_from_attr(&fees_obj, "flat")?;
    let governance = extract_u256_from_attr(&fees_obj, "governance")?;

    Ok(Fees {
        curve,
        flat,
        governance,
    })
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
        let fees = extract_fees_from_attr(ob, "Fees")?;
        let oracle_size = extract_u256_from_attr(ob, "oracleSize")?;
        let update_gap = extract_u256_from_attr(ob, "updateGap")?;

        return Ok(PyPoolConfig {
            pool_config: PoolConfig {
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
            },
        });
    }
}

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
        let bond_reserves = extract_u256_from_attr(ob, "bondReserves")?;
        let lp_total_supply = extract_u256_from_attr(ob, "lpTotalSupply")?;
        let share_price = extract_u256_from_attr(ob, "sharePrice")?;
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
            bond_reserves,
            lp_total_supply,
            share_price,
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

#[pymethods]
impl HyperdriveState {
    #[new]
    pub fn __init__(pool_config: &PyAny, pool_info: &PyAny) -> PyResult<Self> {
        let rust_pool_config = PyPoolConfig::extract(pool_config)?.pool_config;
        let rust_pool_info = PyPoolInfo::extract(pool_info)?.pool_info;
        let state = State::new(rust_pool_config, rust_pool_info);
        Ok(HyperdriveState::new(state))
    }

    pub fn get_spot_price(&self) -> PyResult<String> {
        let result_fp = self.state.get_spot_price();
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn get_spot_rate(&self) -> PyResult<String> {
        let result_fp = self.state.get_spot_rate();
        let result = U256::from(result_fp).to_string();
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

    pub fn to_checkpoint(&self, time: &str) -> PyResult<String> {
        let time_int = U256::from_dec_str(time)
            .map_err(|_| PyErr::new::<PyValueError, _>("Failed to convert time string to U256"))?;
        let result_int = self.state.to_checkpoint(time_int);
        let result = result_int.to_string();
        return Ok(result);
    }

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

/// Get the spot price for a Hyperdrive market with the given pool state
#[pyfunction]
fn get_spot_price(pool_config: &PyAny, pool_info: &PyAny) -> PyResult<String> {
    let hyperdrive_state: HyperdriveState = (pool_config, pool_info).into();
    return hyperdrive_state.get_spot_price();
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

/// Get the share reserves after subtracting the adjustment used for
/// A pyO3 wrapper for the hyperdrie_math crate.
/// The Hyperdrive State struct will be exposed with the following methods:
///   - get_spot_price
///   - get_max_long
///   - get_max_short
#[pymodule]
#[pyo3(name = "pyperdrive")]
fn pyperdrive(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<HyperdriveState>()?;
    m.add_function(wrap_pyfunction!(get_spot_price, m)?)?;
    m.add_function(wrap_pyfunction!(get_max_long, m)?)?;
    m.add_function(wrap_pyfunction!(get_max_short, m)?)?;
    m.add_function(wrap_pyfunction!(get_out_for_in, m)?)?;
    m.add_function(wrap_pyfunction!(get_out_for_in_safe, m)?)?;
    m.add_function(wrap_pyfunction!(get_in_for_out, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_bonds_given_shares_and_rate, m)?)?;
    m.add_function(wrap_pyfunction!(get_effective_share_reserves, m)?)?;
    Ok(())
}
