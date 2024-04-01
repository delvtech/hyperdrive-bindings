use ethers::types::U256;
use hyperdrive_math::State;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    types::{JsPoolConfig, JsPoolInfo},
    utils::set_panic_hook,
};

/// Gets the amount of shares the trader will receive after fees for closing a
/// short
///
/// @param poolInfo - The current state of the pool
///
/// @param poolConfig - The pool's configuration
///
/// @param bondAmount - The number of short bonds to close
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
    bondAmount: &str,
    openVaultSharePrice: &str,
    closeVaultSharePrice: &str,
    normalizedTimeRemaining: &str,
) -> String {
    set_panic_hook();
    let state = State {
        info: poolInfo.into(),
        config: poolConfig.into(),
    };
    let bond_amount = U256::from_dec_str(bondAmount).unwrap();
    let open_vault_share_price = U256::from_dec_str(openVaultSharePrice).unwrap();
    let close_vault_share_price = U256::from_dec_str(closeVaultSharePrice).unwrap();
    let normalized_time_remaining = U256::from_dec_str(normalizedTimeRemaining).unwrap();

    let result_fp = state.calculate_close_short(
        bond_amount,
        open_vault_share_price,
        close_vault_share_price,
        normalized_time_remaining,
    );

    U256::from(result_fp).to_string()
}
