use crate::utils::set_panic_hook;
use crate::{JsPoolConfig, JsPoolInfo};
use ethers::types::{I256, U256};
use fixed_point::FixedPoint;
use hyperdrive_math::State;
use wasm_bindgen::prelude::*;

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
    budget: String,
    checkpointExposure: String,
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

/// Gets the curve fee paid by longs for a given base amount.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param baseAmount - The amount of base tokens to spend
#[wasm_bindgen(skip_jsdoc)]
pub fn getOpenLongCurveFees(
    poolInfo: &JsPoolInfo,
    poolConfig: &JsPoolConfig,
    baseAmount: String,
) -> String {
    set_panic_hook();
    let state = State {
        info: poolInfo.into(),
        config: poolConfig.into(),
    };

    let _baseAmount: FixedPoint = FixedPoint::from(U256::from_dec_str(&baseAmount).unwrap());

    let result_fp = state.open_long_curve_fees(_baseAmount);

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
    let _baseAmount: FixedPoint = FixedPoint::from(U256::from_dec_str(&baseAmount).unwrap());

    let result_fp = state.get_spot_price_after_long(_baseAmount);

    U256::from(result_fp).to_string()
}

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