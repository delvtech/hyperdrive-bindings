#![allow(non_snake_case)]

mod long;
mod short;
mod types;
mod utils;

use ethers::types::U256;
use hyperdrive_math::State;
use types::{JsPoolConfig, JsPoolInfo};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

/// Gets the pool's spot price, i.e. the price to open a long of 1.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
#[wasm_bindgen(skip_jsdoc)]
pub fn getSpotPrice(poolInfo: &JsPoolInfo, poolConfig: &JsPoolConfig) -> String {
    set_panic_hook();
    let state = State {
        config: poolConfig.into(),
        info: poolInfo.into(),
    };
    let result_fp = state.get_spot_price();
    U256::from(result_fp).to_string()
}

/// Gets the pool's fixed APR, i.e. the fixed rate a user locks in when they
/// open a long.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
#[wasm_bindgen(skip_jsdoc)]
pub fn getSpotRate(poolInfo: &JsPoolInfo, poolConfig: &JsPoolConfig) -> String {
    set_panic_hook();
    let state = State {
        info: poolInfo.into(),
        config: poolConfig.into(),
    };
    let result_fp = state.get_spot_rate();
    U256::from(result_fp).to_string()
}
