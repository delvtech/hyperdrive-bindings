use ethers::types::U256;
use fixed_point::FixedPoint;
use hyperdrive_math::State;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    types::{JsPoolConfig, JsPoolInfo},
    utils::set_panic_hook,
};

/// Gets the curve fee paid by the trader when they open a short.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param shortAmount - The number of bonds to short
#[wasm_bindgen(skip_jsdoc)]
pub fn getOpenShortCurveFee(
    poolInfo: &JsPoolInfo,
    poolConfig: &JsPoolConfig,
    shortAmount: String,
) -> String {
    set_panic_hook();
    let state = State {
        info: poolInfo.into(),
        config: poolConfig.into(),
    };
    let short_amount = FixedPoint::from(U256::from_dec_str(&shortAmount).unwrap());
    let spot_price = state.get_spot_price();

    let result_fp = state.open_short_curve_fee(short_amount, spot_price);

    U256::from(result_fp).to_string()
}

// Get the governance fee paid by the trader when they open a short.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param shortAmount - The number of bonds to short
#[wasm_bindgen(skip_jsdoc)]
pub fn getOpenShortGovernanceFee(
    poolInfo: &JsPoolInfo,
    poolConfig: &JsPoolConfig,
    shortAmount: String,
) -> String {
    set_panic_hook();
    let state = State {
        info: poolInfo.into(),
        config: poolConfig.into(),
    };
    let short_amount = FixedPoint::from(U256::from_dec_str(&shortAmount).unwrap());
    let spot_price = state.get_spot_price();

    let result_fp = state.open_short_governance_fee(short_amount, spot_price);

    U256::from(result_fp).to_string()
}

/// Gets the curve fee paid by the trader when they close a short.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param shortAmount - The number of bonds to short
#[wasm_bindgen(skip_jsdoc)]
pub fn getCloseShortCurveFee(
    poolInfo: &JsPoolInfo,
    poolConfig: &JsPoolConfig,
    shortAmount: &str,
    maturityTime: &str,
    currentTime: &str,
) -> String {
    set_panic_hook();
    let state = State {
        info: poolInfo.into(),
        config: poolConfig.into(),
    };
    let short_amount = FixedPoint::from(U256::from_dec_str(shortAmount).unwrap());
    let normalized_time_remaining = state.time_remaining_scaled(
        U256::from_dec_str(currentTime).unwrap(),
        U256::from_dec_str(maturityTime).unwrap(),
    );

    let result_fp = state.close_short_curve_fee(short_amount, normalized_time_remaining);

    U256::from(result_fp).to_string()
}

/// Gets the flat fee paid by the trader when they close a short.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param shortAmount - The number of bonds to short
#[wasm_bindgen(skip_jsdoc)]
pub fn getCloseShortFlatFee(
    poolInfo: &JsPoolInfo,
    poolConfig: &JsPoolConfig,
    shortAmount: &str,
    maturityTime: &str,
    currentTime: &str,
) -> String {
    set_panic_hook();
    let state = State {
        info: poolInfo.into(),
        config: poolConfig.into(),
    };
    let short_amount = FixedPoint::from(U256::from_dec_str(&shortAmount).unwrap());
    let normalized_time_remaining = state.time_remaining_scaled(
        U256::from_dec_str(currentTime).unwrap(),
        U256::from_dec_str(maturityTime).unwrap(),
    );

    let result_fp = state.close_short_flat_fee(short_amount, normalized_time_remaining);

    U256::from(result_fp).to_string()
}
