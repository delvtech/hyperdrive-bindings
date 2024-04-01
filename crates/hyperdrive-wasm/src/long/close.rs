use ethers::types::U256;
use hyperdrive_math::State;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    types::{JsPoolConfig, JsPoolInfo},
    utils::set_panic_hook,
};

/// Gets the amount of shares the trader will receive after fees for closing a
/// long
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param bondAmount - The amount of bonds to close
///
/// @param normalizedTimeRemaining - 0 for mature bonds, 1 for not matured bonds
#[wasm_bindgen(skip_jsdoc)]
pub fn calcCloseLong(
    poolInfo: &JsPoolInfo,
    poolConfig: &JsPoolConfig,
    bondAmount: String,
    normalizedTimeRemaining: String,
) -> String {
    set_panic_hook();
    let state = State {
        info: poolInfo.into(),
        config: poolConfig.into(),
    };
    let _bondAmount = U256::from_dec_str(&bondAmount).unwrap();
    let _normalizedTimeRemaining = U256::from_dec_str(&normalizedTimeRemaining).unwrap();

    let result_fp = state.calculate_close_long(_bondAmount, _normalizedTimeRemaining);

    U256::from(result_fp).to_string()
}
