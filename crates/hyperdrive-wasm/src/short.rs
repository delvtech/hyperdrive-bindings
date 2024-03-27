use crate::utils::set_panic_hook;
use crate::{JsPoolConfig, JsPoolInfo};
use ethers::types::{I256, U256};
use fixed_point::FixedPoint;
use hyperdrive_math::State;
use wasm_bindgen::prelude::*;

/// Gets the amount of base the trader will need to deposit for a short of a given size.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param shortAmount - The amount of bonds to short
///
/// @param openVaultSharePrice - The vault share price at the start of the checkpoint
#[wasm_bindgen(skip_jsdoc)]
pub fn calcOpenShort(
    poolInfo: &JsPoolInfo,
    poolConfig: &JsPoolConfig,
    shortAmount: String,
    openVaultSharePrice: String,
) -> String {
    set_panic_hook();
    let state = State {
        info: poolInfo.into(),
        config: poolConfig.into(),
    };
    let short_amount = FixedPoint::from(U256::from_dec_str(&shortAmount).unwrap());

    let spot_price = state.get_spot_price();

    let openVaultSharePriceFixedPoint =
        FixedPoint::from(U256::from_dec_str(&openVaultSharePrice).unwrap());

    let result_fp = state
        .calculate_open_short(short_amount, spot_price, openVaultSharePriceFixedPoint)
        .unwrap();

    U256::from(result_fp).to_string()
}

/// Get the max amount of longs that can be shorted given the current state of
/// the pool.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param openVaultSharePrice - The open share price of the pool's current
/// checkpoint
///
/// @param checkpointExposure - The exposure of the pool's current checkpoint
///
/// @param maybeConservativePrice - A lower bound on the realized price that the
/// short will pay. This is used to help the algorithm converge faster in real
/// world situations. If this is `None`, then we'll use the theoretical worst
/// case realized price.
///
/// @param maybeMaxIterations - The maximum number of iterations to run the
/// binary search for
#[wasm_bindgen(skip_jsdoc)]
pub fn getMaxShort(
    poolInfo: &JsPoolInfo,
    poolConfig: &JsPoolConfig,
    budget: String,
    openVaultSharePrice: String,
    checkpointExposure: String,
    maybeConservativePrice: Option<String>,
    maybeMaxIterations: Option<u8>,
) -> String {
    set_panic_hook();
    let state = State {
        info: poolInfo.into(),
        config: poolConfig.into(),
    };
    let _budget = U256::from_dec_str(&budget).unwrap();
    let checkpoint_exposure: I256 = I256::from_dec_str(&checkpointExposure).unwrap();
    let open_vault_share_price: I256 =
        I256::from_raw(U256::from_dec_str(&openVaultSharePrice).unwrap());

    let _maybe_conservative_price: Option<FixedPoint> = maybeConservativePrice
        .as_ref()
        .map(|price_str| FixedPoint::from(U256::from_dec_str(price_str).unwrap()));

    let result_fp = state.get_max_short(
        _budget,
        open_vault_share_price,
        checkpoint_exposure,
        _maybe_conservative_price,
        maybeMaxIterations.map(|x| x.into()),
    );

    U256::from(result_fp).to_string()
}

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
pub fn getOpenShortCurveFees(
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

/// Gets the spot price after opening the short on the YieldSpace curve and
/// before calculating the fees.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param bondAmount - The number of bonds to short
#[wasm_bindgen(skip_jsdoc)]
pub fn calcSpotPriceAfterShort(
    poolInfo: &JsPoolInfo,
    poolConfig: &JsPoolConfig,
    bondAmount: String,
) -> String {
    set_panic_hook();
    let state = State {
        info: poolInfo.into(),
        config: poolConfig.into(),
    };
    let _bondAmount = FixedPoint::from(U256::from_dec_str(&bondAmount).unwrap());

    let result_fp = state.get_spot_price_after_short(_bondAmount);

    U256::from(result_fp).to_string()
}

/// Gets the amount of shares the trader will receive after fees for closing a
/// short
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param bondAmount - The amount of bonds to close
///
/// @param openVaultSharePrice - The vault share price at the checkpoint when the position was opened
///
/// @param closeVaultSharePrice - The current vault share price, or if the position has matured, the vault share price from the closing checkpoint
///
/// @param normalizedTimeRemaining - 0 for mature bonds, 1 for not matured bonds
#[wasm_bindgen(skip_jsdoc)]
pub fn calcCloseShort(
    poolInfo: &JsPoolInfo,
    poolConfig: &JsPoolConfig,
    bondAmount: String,
    openVaultSharePrice: String,
    closeVaultSharePrice: String,
    normalizedTimeRemaining: String,
) -> String {
    set_panic_hook();
    let state = State {
        info: poolInfo.into(),
        config: poolConfig.into(),
    };
    let _bondAmount = U256::from_dec_str(&bondAmount).unwrap();
    let _openVaultSharePrice = U256::from_dec_str(&openVaultSharePrice).unwrap();
    let _closeVaultSharePrice = U256::from_dec_str(&closeVaultSharePrice).unwrap();
    let _normalizedTimeRemaining = U256::from_dec_str(&normalizedTimeRemaining).unwrap();

    let result_fp = state.calculate_close_short(
        _bondAmount,
        _openVaultSharePrice,
        _closeVaultSharePrice,
        _normalizedTimeRemaining,
    );

    U256::from(result_fp).to_string()
}
