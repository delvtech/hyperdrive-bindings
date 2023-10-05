use ethers::core::types::{Address, I256, U256};
use fixed_point::FixedPoint;
use hyperdrive_math::hyperdrive_math::State;
use hyperdrive_wrappers::wrappers::i_hyperdrive::{Fees, PoolConfig, PoolInfo};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn getMaxShort(
    _state: JsValue,
    _budget: String,
    _open_share_price: String,
    _maybe_conservative_price: Option<String>,
    _maybe_max_iterations: Option<u8>,
) -> String {
    let state = State::from(&serde_wasm_bindgen::from_value(_state).unwrap());
    let budget = U256::from_dec_str(&_budget).unwrap();
    let open_share_price: I256 = I256::from_raw(U256::from_dec_str(&_open_share_price).unwrap());

    let _maybe_conservative_price: Option<FixedPoint> = _maybe_conservative_price
        .as_ref()
        .map(|price_str| FixedPoint::from(U256::from_dec_str(price_str).unwrap()));

    state
        .get_max_short(
            budget,
            open_share_price,
            _maybe_conservative_price,
            _maybe_max_iterations.map(|x| x.into()),
        )
        .to_string()
}

#[wasm_bindgen]
pub fn getMaxLong(
    _state: JsValue,
    _budget: String,
    _checkpoint_exposure: String,
    _maybe_max_iterations: Option<u8>,
) -> String {
    let state = State::from(&serde_wasm_bindgen::from_value(_state).unwrap());
    let budget = U256::from_dec_str(&_budget).unwrap();
    let checkpoint_exposure: I256 = I256::from_dec_str(&_checkpoint_exposure).unwrap();

    state
        .get_max_long(
            budget,
            checkpoint_exposure,
            _maybe_max_iterations.map(|x| x.into()),
        )
        .to_string()
}

#[wasm_bindgen]
pub fn getSpotPrice(_state: JsValue) -> String {
    let state = State::from(&serde_wasm_bindgen::from_value(_state).unwrap());
    state.get_spot_price().to_string()
}

#[wasm_bindgen]
pub fn getSpotRate(_state: JsValue) -> String {
    let state = State::from(&serde_wasm_bindgen::from_value(_state).unwrap());
    state.get_spot_rate().to_string()
}

#[derive(Serialize, Deserialize)]
pub struct WasmFees {
    pub curve: String,
    pub flat: String,
    pub governance: String,
}

#[derive(Serialize, Deserialize)]
pub struct WasmPoolConfig {
    pub base_token: String,
    pub initial_share_price: String,
    pub minimum_share_reserves: String,
    pub minimum_transaction_amount: String,
    pub position_duration: String,
    pub checkpoint_duration: String,
    pub time_stretch: String,
    pub governance: String,
    pub fee_collector: String,
    pub fees: WasmFees,
    pub oracle_size: String,
    pub update_gap: String,
}

#[derive(Serialize, Deserialize)]
pub struct WasmPoolInfo {
    pub share_reserves: String,
    pub bond_reserves: String,
    pub lp_total_supply: String,
    pub share_price: String,
    pub longs_outstanding: String,
    pub long_average_maturity_time: String,
    pub shorts_outstanding: String,
    pub short_average_maturity_time: String,
    pub withdrawal_shares_ready_to_withdraw: String,
    pub withdrawal_shares_proceeds: String,
    pub lp_share_price: String,
    pub long_exposure: String,
}

#[derive(Serialize, Deserialize)]
pub struct WasmState {
    pub config: WasmPoolConfig,
    pub info: WasmPoolInfo,
}

impl From<&WasmState> for State {
    fn from(wasm_state: &WasmState) -> State {
        State {
            config: PoolConfig {
                base_token: Address::from_str(&wasm_state.config.base_token).unwrap(),
                governance: Address::from_str(&wasm_state.config.governance).unwrap(),
                fee_collector: Address::from_str(&wasm_state.config.fee_collector).unwrap(),
                fees: Fees {
                    curve: U256::from_dec_str(wasm_state.config.fees.curve.as_str()).unwrap(),
                    flat: U256::from_dec_str(wasm_state.config.fees.flat.as_str()).unwrap(),
                    governance: U256::from_dec_str(wasm_state.config.fees.governance.as_str())
                        .unwrap(),
                },
                initial_share_price: U256::from_dec_str(
                    wasm_state.config.initial_share_price.as_str(),
                )
                .unwrap(),
                minimum_share_reserves: U256::from_dec_str(
                    wasm_state.config.minimum_share_reserves.as_str(),
                )
                .unwrap(),
                minimum_transaction_amount: U256::from_dec_str(
                    wasm_state.config.minimum_transaction_amount.as_str(),
                )
                .unwrap(),
                time_stretch: U256::from_dec_str(wasm_state.config.time_stretch.as_str()).unwrap(),
                position_duration: U256::from_dec_str(wasm_state.config.position_duration.as_str())
                    .unwrap(),
                checkpoint_duration: U256::from_dec_str(
                    wasm_state.config.checkpoint_duration.as_str(),
                )
                .unwrap(),
                oracle_size: U256::from_dec_str(wasm_state.config.oracle_size.as_str()).unwrap(),
                update_gap: U256::from_dec_str(wasm_state.config.update_gap.as_str()).unwrap(),
            },
            info: PoolInfo {
                share_reserves: U256::from_dec_str(wasm_state.info.share_reserves.as_str())
                    .unwrap(),
                bond_reserves: U256::from_dec_str(wasm_state.info.bond_reserves.as_str()).unwrap(),
                long_exposure: U256::from_dec_str(wasm_state.info.long_exposure.as_str()).unwrap(),
                share_price: U256::from_dec_str(wasm_state.info.share_price.as_str()).unwrap(),
                longs_outstanding: U256::from_dec_str(wasm_state.info.longs_outstanding.as_str())
                    .unwrap(),
                shorts_outstanding: U256::from_dec_str(wasm_state.info.shorts_outstanding.as_str())
                    .unwrap(),
                long_average_maturity_time: U256::from_dec_str(
                    wasm_state.info.long_average_maturity_time.as_str(),
                )
                .unwrap(),
                short_average_maturity_time: U256::from_dec_str(
                    wasm_state.info.short_average_maturity_time.as_str(),
                )
                .unwrap(),
                lp_total_supply: U256::from_dec_str(wasm_state.info.lp_total_supply.as_str())
                    .unwrap(),
                lp_share_price: U256::from_dec_str(wasm_state.info.lp_share_price.as_str())
                    .unwrap(),
                withdrawal_shares_proceeds: U256::from_dec_str(
                    wasm_state.info.withdrawal_shares_proceeds.as_str(),
                )
                .unwrap(),
                withdrawal_shares_ready_to_withdraw: U256::from_dec_str(
                    wasm_state.info.withdrawal_shares_ready_to_withdraw.as_str(),
                )
                .unwrap(),
            },
        }
    }
}
