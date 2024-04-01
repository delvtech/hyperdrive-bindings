use ethers::types::U256;
use fixed_point::FixedPoint;
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
/// @param maturityTime - The maturity timestamp of the short (in seconds)
///
/// @param currentTime - The current timestamp (in seconds)
#[wasm_bindgen(skip_jsdoc)]
pub fn calcCloseLong(
    poolInfo: &JsPoolInfo,
    poolConfig: &JsPoolConfig,
    bondAmount: &str,
    maturityTime: &str,
    currentTime: &str,
) -> String {
    set_panic_hook();
    let state = State {
        info: poolInfo.into(),
        config: poolConfig.into(),
    };
    let bond_amount = FixedPoint::from(U256::from_dec_str(bondAmount).unwrap());
    let normalized_time_remaining = state.time_remaining_scaled(
        U256::from_dec_str(currentTime).unwrap(),
        U256::from_dec_str(maturityTime).unwrap(),
    );

    let result_fp = state.calculate_close_long(bond_amount, normalized_time_remaining);

    U256::from(result_fp).to_string()
}
