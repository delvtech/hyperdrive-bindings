use ethers::types::U256;
use fixed_point::FixedPoint;
use hyperdrive_math::State;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    types::{JsPoolConfig, JsPoolInfo},
    utils::set_panic_hook,
};

/// Gets the curve fee paid by traders when they open a long.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param baseAmount - The amount of base tokens to spend
#[wasm_bindgen(skip_jsdoc)]
pub fn getOpenLongCurveFee(
    poolInfo: &JsPoolInfo,
    poolConfig: &JsPoolConfig,
    baseAmount: String,
) -> String {
    set_panic_hook();
    let state = State {
        info: poolInfo.into(),
        config: poolConfig.into(),
    };

    let base_amount = FixedPoint::from(U256::from_dec_str(&baseAmount).unwrap());

    let result_fp = state.open_long_curve_fees(base_amount);

    U256::from(result_fp).to_string()
}

/// Gets the governance fee paid by traders when they open a long.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param baseAmount - The amount of base tokens to spend
#[wasm_bindgen(skip_jsdoc)]
pub fn getOpenLongGovernanceFee(
    poolInfo: &JsPoolInfo,
    poolConfig: &JsPoolConfig,
    baseAmount: &str,
) -> String {
    set_panic_hook();
    let state = State {
        info: poolInfo.into(),
        config: poolConfig.into(),
    };
    let base_amount = FixedPoint::from(U256::from_dec_str(baseAmount).unwrap());

    let result_fp = state.open_long_governance_fee(base_amount);

    U256::from(result_fp).to_string()
}

/// Gets the curve fee paid by traders when they close a long.
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
pub fn getCloseLongCurveFee(
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

    let bond_amount = U256::from_dec_str(bondAmount).unwrap();
    let normalized_time_remaining = state.time_remaining_scaled(
        U256::from_dec_str(currentTime).unwrap(),
        U256::from_dec_str(maturityTime).unwrap(),
    );

    let result_fp = state.close_long_curve_fee(bond_amount.into(), normalized_time_remaining);

    U256::from(result_fp).to_string()
}

/// Gets the flat fee paid by traders when they close a long.
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
pub fn getCloseLongFlatFee(
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

    let bond_amount = U256::from_dec_str(bondAmount).unwrap();
    let normalized_time_remaining = state.time_remaining_scaled(
        U256::from_dec_str(currentTime).unwrap(),
        U256::from_dec_str(maturityTime).unwrap(),
    );

    let result_fp = state.close_long_flat_fee(bond_amount.into(), normalized_time_remaining);

    U256::from(result_fp).to_string()
}
