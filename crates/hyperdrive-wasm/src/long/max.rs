use ethers::types::{I256, U256};
use hyperdrive_math::State;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    types::{JsPoolConfig, JsPoolInfo},
    utils::set_panic_hook,
};

/// Gets the max long that can be opened given a budget.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param budget - The maximum amount of base tokens that can be spent
///
/// @param checkpointExposure - The exposure of the pool's current checkpoint
///
/// @param maybeMaxIterations - The maximum number of iterations to run the
/// binary search for
#[wasm_bindgen(skip_jsdoc)]
pub fn getMaxLong(
    poolInfo: &JsPoolInfo,
    poolConfig: &JsPoolConfig,
    budget: &str,
    checkpointExposure: &str,
    maybeMaxIterations: Option<u8>,
) -> String {
    set_panic_hook();
    let state = State {
        info: poolInfo.into(),
        config: poolConfig.into(),
    };
    let _budget = U256::from_dec_str(&budget).unwrap();
    let checkpoint_exposure: I256 = I256::from_dec_str(&checkpointExposure).unwrap();

    let result_fp = state.get_max_long(
        _budget,
        checkpoint_exposure,
        maybeMaxIterations.map(|x| x.into()),
    );

    U256::from(result_fp).to_string()
}
