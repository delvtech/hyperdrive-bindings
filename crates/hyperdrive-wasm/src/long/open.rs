use ethers::types::U256;
use fixed_point::FixedPoint;
use hyperdrive_math::State;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    types::{JsPoolConfig, JsPoolInfo},
    utils::set_panic_hook,
};

/// Gets the long amount that will be opened for a given base amount.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param baseAmount - The amount of base tokens to open a long for
#[wasm_bindgen(skip_jsdoc)]
pub fn calcOpenLong(
    poolInfo: &JsPoolInfo,
    poolConfig: &JsPoolConfig,
    baseAmount: String,
) -> String {
    set_panic_hook();
    let state = State {
        info: poolInfo.into(),
        config: poolConfig.into(),
    };
    let base_amount: FixedPoint = FixedPoint::from(U256::from_dec_str(&baseAmount).unwrap());

    let result_fp = state.calculate_open_long(base_amount);

    U256::from(result_fp).to_string()
}

/// Gets the spot price after opening the short on the YieldSpace curve and
/// before calculating the fees.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param baseAmount - The amount of base to spend
#[wasm_bindgen(skip_jsdoc)]
pub fn calcSpotPriceAfterLong(
    poolInfo: &JsPoolInfo,
    poolConfig: &JsPoolConfig,
    baseAmount: String,
) -> String {
    set_panic_hook();
    let state = State {
        info: poolInfo.into(),
        config: poolConfig.into(),
    };
    let _baseAmount = FixedPoint::from(U256::from_dec_str(&baseAmount).unwrap());

    let result_fp = state.get_spot_price_after_long(_baseAmount);

    U256::from(result_fp).to_string()
}
