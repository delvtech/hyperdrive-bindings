use ethers::core::types::{Address, I256, U256};
use fixed_point::FixedPoint;
use hyperdrive_wrappers::wrappers::i_hyperdrive::{Fees, PoolConfig, PoolInfo};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::PyErr;

use hyperdrive_math::hyperdrive_math::State;

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
        let base_token = extract_address_from_attr(ob, "base_token")?;
        let initial_share_price = extract_u256_from_attr(ob, "initial_share_price")?;
        let minimum_share_reserves = extract_u256_from_attr(ob, "minimum_share_reserves")?;
        let position_duration = extract_u256_from_attr(ob, "position_duration")?;
        let checkpoint_duration = extract_u256_from_attr(ob, "checkpoint_duration")?;
        let time_stretch = extract_u256_from_attr(ob, "time_stretch")?;
        let governance = extract_address_from_attr(ob, "governance")?;
        let fees = extract_fees_from_attr(ob, "fees")?;
        let fee_collector = extract_address_from_attr(ob, "fee_collector")?;
        let oracle_size = extract_u256_from_attr(ob, "oracle_size")?;
        let update_gap = extract_u256_from_attr(ob, "update_gap")?;

        return Ok(PyPoolConfig {
            pool_config: PoolConfig {
                base_token,
                initial_share_price,
                minimum_share_reserves,
                position_duration,
                checkpoint_duration,
                time_stretch,
                governance,
                fees,
                fee_collector,
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
        let share_reserves = extract_u256_from_attr(ob, "share_reserves")?;
        let bond_reserves = extract_u256_from_attr(ob, "bond_reserves")?;
        let long_exposure = extract_i256_from_attr(ob, "long_exposure")?;
        let share_price = extract_u256_from_attr(ob, "share_price")?;
        let longs_outstanding = extract_u256_from_attr(ob, "longs_outstanding")?;
        let shorts_outstanding = extract_u256_from_attr(ob, "shorts_outstanding")?;
        let long_average_maturity_time = extract_u256_from_attr(ob, "long_average_maturity_time")?;
        let short_average_maturity_time =
            extract_u256_from_attr(ob, "short_average_maturity_time")?;
        let lp_total_supply = extract_u256_from_attr(ob, "lp_total_supply")?;
        let lp_share_price = extract_u256_from_attr(ob, "lp_share_price")?;
        let withdrawal_shares_proceeds = extract_u256_from_attr(ob, "withdrawal_shares_proceeds")?;
        let withdrawal_shares_ready_to_withdraw =
            extract_u256_from_attr(ob, "withdrawal_shares_ready_to_withdraw")?;

        let pool_info = PoolInfo {
            share_reserves,
            bond_reserves,
            long_exposure,
            share_price,
            longs_outstanding,
            shorts_outstanding,
            long_average_maturity_time,
            short_average_maturity_time,
            lp_total_supply,
            withdrawal_shares_ready_to_withdraw,
            withdrawal_shares_proceeds,
            lp_share_price,
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

    pub fn get_max_long(
        &self,
        budget: &str,
        maybe_max_iterations: Option<usize>,
    ) -> PyResult<String> {
        let budget_fp = FixedPoint::from(U256::from_dec_str(budget).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert budget string to U256")
        })?);
        let result_fp = self.state.get_max_long(budget_fp, maybe_max_iterations);
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn get_max_short(
        &self,
        budget: &str,
        open_share_price: &str,
        maybe_max_iterations: Option<usize>,
    ) -> PyResult<String> {
        let budget_fp = FixedPoint::from(U256::from_dec_str(budget).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert budget string to U256")
        })?);
        let open_share_price_fp =
            FixedPoint::from(U256::from_dec_str(open_share_price).map_err(|_| {
                PyErr::new::<PyValueError, _>("Failed to convert open_share_price string to U256")
            })?);
        let result_fp =
            self.state
                .get_max_short(budget_fp, open_share_price_fp, maybe_max_iterations);
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }
}

/// Get the spot price for a Hyperdrive market with the given pool state
#[pyfunction]
fn get_spot_price(pool_config: &PyAny, pool_info: &PyAny) -> PyResult<String> {
    let hyperdrive_state: HyperdriveState = (pool_config, pool_info).into();
    let spot_price = hyperdrive_state.get_spot_price()?;
    Ok(spot_price)
}

/// Get the max long for a Hyperdrive market with the given pool state
#[pyfunction]
fn get_max_long(
    pool_config: &PyAny,
    pool_info: &PyAny,
    budget: &str,
    maybe_max_iterations: Option<usize>,
) -> PyResult<String> {
    let hyperdrive_state: HyperdriveState = (pool_config, pool_info).into();
    let max_long = hyperdrive_state.get_max_long(budget, maybe_max_iterations)?;
    Ok(max_long)
}

/// Get the max long for a Hyperdrive market with the given pool state
#[pyfunction]
fn get_max_short(
    pool_config: &PyAny,
    pool_info: &PyAny,
    budget: &str,
    open_share_price: &str,
    maybe_max_iterations: Option<usize>,
) -> PyResult<String> {
    let hyperdrive_state: HyperdriveState = (pool_config, pool_info).into();
    let max_short = hyperdrive_state.get_max_short(budget, open_share_price, maybe_max_iterations)?;
    Ok(max_short)
}

/// A pyO3 wrapper for the hyperdrie_math crate.
/// The Hyperdrive State struct will be exposed with the following methods:
///   - get_spot_price
#[pymodule]
#[pyo3(name = "pyperdrive")]
fn pyperdrive(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<HyperdriveState>()?;
    m.add_function(wrap_pyfunction!(get_spot_price, m)?)?;
    m.add_function(wrap_pyfunction!(get_max_long, m)?)?;
    m.add_function(wrap_pyfunction!(get_max_short, m)?)?;
    Ok(())
}
