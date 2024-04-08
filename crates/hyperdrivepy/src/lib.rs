mod hyperdrive_state;
mod hyperdrive_state_methods;
mod hyperdrive_utils;
mod pool_config;
mod pool_info;
mod utils;

use pyo3::prelude::*;

use hyperdrive_state::HyperdriveState;
pub use hyperdrive_state_methods::*;
pub use hyperdrive_utils::{
    calculate_effective_share_reserves, calculate_initial_bond_reserves, calculate_time_stretch,
};
pub use pool_config::PyPoolConfig;
pub use pool_info::PyPoolInfo;

/// Get the share reserves after subtracting the adjustment used for
/// A pyO3 wrapper for the hyperdrive_math crate.
#[pymodule]
#[pyo3(name = "hyperdrivepy")]
fn hyperdrivepy(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<HyperdriveState>()?;
    m.add_function(wrap_pyfunction!(calculate_initial_bond_reserves, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_effective_share_reserves, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_time_stretch, m)?)?;
    Ok(())
}
