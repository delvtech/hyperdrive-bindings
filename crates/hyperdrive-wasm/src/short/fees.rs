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
///
/// @param spotPrice - The spot price of the pool
#[wasm_bindgen(skip_jsdoc)]
pub fn getOpenShortCurveFee(
    poolInfo: &JsPoolInfo,
    poolConfig: &JsPoolConfig,
    shortAmount: String,
    spotPrice: String,
) -> String {
    set_panic_hook();
    let state = State {
        info: poolInfo.into(),
        config: poolConfig.into(),
    };
    let _shortAmount = FixedPoint::from(U256::from_dec_str(&shortAmount).unwrap());
    let _spotPrice = FixedPoint::from(U256::from_dec_str(&spotPrice).unwrap());

    let result_fp = state.open_short_curve_fee(_shortAmount, _spotPrice);

    U256::from(result_fp).to_string()
}
