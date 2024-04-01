use ethers::types::U256;
use fixed_point::FixedPoint;
use hyperdrive_math::State;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    types::{JsPoolConfig, JsPoolInfo},
    utils::set_panic_hook,
};

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
