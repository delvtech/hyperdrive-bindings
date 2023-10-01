"""Tests for hyperdrive_math.rs wrappers"""
import pytest

import pyperdrive
from pyperdrive.types import Fees, PoolConfig, PoolInfo

# pylint: disable=no-member

sample_pool_config = PoolConfig(
    base_token="0x1234567890abcdef1234567890abcdef12345678",
    initial_share_price=str(1 * 10**18),  # 1e18
    minimum_share_reserves=str(1 * 10**17),  # 0.1e18
    minimum_transaction_amount=str(1 * 10**16),  # 0.001e18
    position_duration=str(604_800),
    checkpoint_duration=str(86_400),
    time_stretch=str(1 * 10**17),  # 0.1e18
    governance="0xabcdef1234567890abcdef1234567890abcdef12",
    fee_collector="0xfedcba0987654321fedcba0987654321fedcba09",
    fees=Fees(curve=str(0), flat=str(0), governance=str(0)),
    oracle_size=str(10),
    update_gap=str(3_600),
)


sample_pool_info = PoolInfo(
    share_reserves=str(int(1_000_000 * 10**18)),
    bond_reserves=str(int(2_000_000 * 10**18)),
    share_adjustment=str(0),
    lp_total_supply=str(int(3_000_000 * 10**18)),
    share_price=str(int(1 * 10**18)),
    longs_outstanding=str(0),
    long_average_maturity_time=str(0),
    shorts_outstanding=str(0),
    short_average_maturity_time=str(0),
    short_base_volume=str(0),
    withdrawal_shares_ready_to_withdraw=str(0),
    withdrawal_shares_proceeds=str(0),
    lp_share_price=str(int(1 * 10**18)),
    long_exposure=str(0),
)


def test_initialization():
    """test initialization."""
    state = pyperdrive.HyperdriveState(sample_pool_config, sample_pool_info)
    assert state is not None, "State initialization failed."


def test_get_spot_rate():
    """test get_spot_rate."""
    state = pyperdrive.HyperdriveState(sample_pool_config, sample_pool_info)
    spot_rate = state.get_spot_rate()
    assert spot_rate is not None, "Failed to get spot rate."
    assert isinstance(spot_rate, str), "Expected spot rate to be a string."
    assert int(spot_rate) > 0, "Expected spot rate to > 0."


def test_to_checkpoint():
    """test to_checkpoint."""
    state = pyperdrive.HyperdriveState(sample_pool_config, sample_pool_info)
    checkpoint_time = state.to_checkpoint(time=str(100))
    assert checkpoint_time is not None, "Failed to get checkpoint time."
    assert isinstance(checkpoint_time, str), "Expected checkpoint time to be a string."


def test_get_spot_price():
    """test get_spot_price."""
    state = pyperdrive.HyperdriveState(sample_pool_config, sample_pool_info)
    spot_price = state.get_spot_price()
    assert spot_price is not None, "Failed to get spot price."
    assert isinstance(spot_price, str), "Expected spot price to be a string."
    assert int(spot_price) > 0, "Expected spot price to > 0."
    # test the helper function
    assert pyperdrive.get_spot_price(sample_pool_config, sample_pool_info) == spot_price


def test_max_long():
    """test get_max_long."""
    state = pyperdrive.HyperdriveState(sample_pool_config, sample_pool_info)
    budget = "1000000000000000000"  # 1 base
    checkpoint_exposure = "10000"
    max_iterations = 20
    max_long = state.get_max_long(budget, checkpoint_exposure, max_iterations)
    assert int(max_long) > 0  # should == "1000000000000000000", or 1 base
    # test the helper function
    max_long_direct = pyperdrive.get_max_long(
        sample_pool_config,
        sample_pool_info,
        budget,
        checkpoint_exposure,
        max_iterations,
    )
    assert max_long_direct == max_long


def test_max_long_fail_conversion():
    """test get_max_long."""
    state = pyperdrive.HyperdriveState(sample_pool_config, sample_pool_info)
    max_iterations = 20
    # bad string inputs
    budget = "asdf"
    checkpoint_exposure = "100"
    with pytest.raises(ValueError, match="Failed to convert budget string to U256"):
        state.get_max_long(budget, checkpoint_exposure, max_iterations)
    budget = "1.23"
    checkpoint_exposure = "100"
    with pytest.raises(ValueError, match="Failed to convert budget string to U256"):
        state.get_max_long(budget, checkpoint_exposure, max_iterations)
    budget = "1000000000000000000"  # 1 base
    checkpoint_exposure = "asdf"
    with pytest.raises(ValueError, match="Failed to convert checkpoint_exposure string to I256"):
        state.get_max_long(budget, checkpoint_exposure, max_iterations)


def test_max_short():
    """test get_max_short."""
    # test using the state directly
    state = pyperdrive.HyperdriveState(sample_pool_config, sample_pool_info)
    budget = str(int(10 * 10**18))  # 10k base
    open_share_price = str(int(1 * 10**18))  # 1 base
    checkpoint_exposure = str(0)
    conservative_price = None
    max_iterations = 20
    max_short = state.get_max_short(budget, open_share_price, checkpoint_exposure, conservative_price, max_iterations)
    assert int(max_short) > 0  # should == "2583754033693357393077", or apprx 2583 base
    # test the helper function
    max_short_direct = pyperdrive.get_max_short(
        sample_pool_config,
        sample_pool_info,
        budget,
        open_share_price,
        checkpoint_exposure,
        conservative_price,
        max_iterations,
    )
    assert max_short_direct == max_short


def test_max_short_fail_conversion():
    """test get_max_short."""
    state = pyperdrive.HyperdriveState(sample_pool_config, sample_pool_info)
    open_share_price = str(int(1 * 10**18))  # 1 base
    checkpoint_exposure = str(0)
    conservative_price = None
    max_iterations = 20
    # bad string inputs
    budget = "asdf"
    with pytest.raises(ValueError, match="Failed to convert budget string to U256"):
        state.get_max_short(budget, open_share_price, checkpoint_exposure, conservative_price, max_iterations)
    budget = "1.23"
    with pytest.raises(ValueError, match="Failed to convert budget string to U256"):
        state.get_max_short(budget, open_share_price, checkpoint_exposure, conservative_price, max_iterations)
    budget = "10000000000000000000000"  # 10k base
    open_share_price = "asdf"
    with pytest.raises(ValueError, match="Failed to convert open_share_price string to U256"):
        state.get_max_short(budget, open_share_price, checkpoint_exposure, conservative_price, max_iterations)
