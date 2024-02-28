use crate::utils::set_panic_hook;
use crate::{JsPoolConfig, JsPoolInfo};
use ethers::types::{I256, U256};
use fixed_point::FixedPoint;
use fixed_point_macros::fixed;
use hyperdrive_math::State;
use wasm_bindgen::prelude::*;

/// Gets the amount of base the trader will need to deposit for a short of a given size.
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param shortAmount - The amount of bonds to short
#[wasm_bindgen(skip_jsdoc)]
pub fn calcOpenShort(
    poolInfo: &JsPoolInfo,
    poolConfig: &JsPoolConfig,
    shortAmount: String,
) -> String {
    set_panic_hook();
    let state = State {
        info: poolInfo.into(),
        config: poolConfig.into(),
    };
    let short_amount: FixedPoint = FixedPoint::from(U256::from_dec_str(&shortAmount).unwrap());

    let spot_price = state.get_spot_price();

    let result_fp = state
        .calculate_open_short(short_amount, spot_price, fixed!(0))
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
    let checkpoint_exposure: I256 =
        I256::from_raw(I256::from_dec_str(&checkpointExposure).unwrap());
    let open_vault_share_price: I256 = I256::from_raw(U256::from_dec_str(&openVaultSharePrice).unwrap());

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
