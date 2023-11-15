use pyo3::prelude::*;

use crate::{PyPoolConfig, PyPoolInfo};
use hyperdrive_math::State;

#[pyclass(module = "hyperdrivepy", name = "HyperdriveState")]
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
