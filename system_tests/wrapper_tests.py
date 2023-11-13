"""Tests for hyperdrive_math.rs wrappers"""
import pytest

import pyperdrive
from pyperdrive.pypechain_types.IHyperdriveTypes import Fees, PoolConfig, PoolInfo

POOL_CONFIG = PoolConfig(
    baseToken="0x1234567890abcdef1234567890abcdef12345678",
    linkerFactory="0x1234567890abcdef1234567890abcdef12345678",
    linkerCodeHash=bytes(32),
    initialSharePrice=1 * 10**18,  # 1e18
    minimumShareReserves=1 * 10**17,  # 0.1e18
    minimumTransactionAmount=1 * 10**16,  # 0.001e18
    precisionThreshold=1 * 10**14,
    positionDuration=604_800,
    checkpointDuration=86_400,
    timeStretch=1 * 10**17,  # 0.1e18
    governance="0xabcdef1234567890abcdef1234567890abcdef12",
    feeCollector="0xfedcba0987654321fedcba0987654321fedcba09",
    fees=Fees(curve=0, flat=0, governance=0),
)


POOL_INFO = PoolInfo(
    shareReserves=1_000_000 * 10**18,
    shareAdjustment=0,
    bondReserves=2_000_000 * 10**18,
    lpTotalSupply=3_000_000 * 10**18,
    sharePrice=1 * 10**18,
    longsOutstanding=0,
    longAverageMaturityTime=0,
    shortsOutstanding=0,
    shortAverageMaturityTime=0,
    withdrawalSharesReadyToWithdraw=0,
    withdrawalSharesProceeds=0,
    lpSharePrice=1 * 10**18,
    longExposure=0,
)


def test_get_max_spot_price():
    """test get_max_spot_rate."""
    max_spot_price = pyperdrive.get_max_spot_price(POOL_CONFIG, POOL_INFO)
    assert max_spot_price is not None, "Failed to get max spot price."
    assert isinstance(max_spot_price, str), "Expected spot rate to be a string."
    assert int(max_spot_price) > 0, "Expected max_spot_price to > 0."


def test_get_spot_price_after_long():
    """test get_spot_price_after_long."""
    spot_price = pyperdrive.get_spot_price_after_long(POOL_CONFIG, POOL_INFO, long_amount=str(1_000 * 10**18))
    assert spot_price is not None, "Failed to get spot price after long."
    assert isinstance(spot_price, str), "Expected spot rate to be a string."
    assert int(spot_price) > 0, "Expected max_spot_price to > 0."


def test_get_solvency():
    """test get_max_spot_rate."""
    solvency = pyperdrive.get_solvency(POOL_CONFIG, POOL_INFO)
    assert solvency is not None, "Failed to get spot price after long."
    assert isinstance(solvency, str), "Expected spot rate to be a string."
    assert int(solvency) > 0, "Expected max_spot_price to > 0."


def test_get_spot_rate():
    """test get_spot_rate."""
    spot_rate = pyperdrive.get_spot_rate(POOL_CONFIG, POOL_INFO)
    assert spot_rate is not None, "Failed to get spot rate."
    assert isinstance(spot_rate, str), "Expected spot rate to be a string."
    assert int(spot_rate) > 0, "Expected spot rate to > 0."


def test_to_checkpoint():
    """test to_checkpoint."""
    checkpoint_time = pyperdrive.to_checkpoint(POOL_CONFIG, POOL_INFO, time=str(100))
    assert checkpoint_time is not None, "Failed to get checkpoint time."
    assert isinstance(checkpoint_time, str), "Expected checkpoint time to be a string."


def test_get_spot_price():
    """test get_spot_price."""
    spot_price = pyperdrive.get_spot_price(POOL_CONFIG, POOL_INFO)
    assert spot_price is not None, "Failed to get spot price."
    assert isinstance(spot_price, str), "Expected spot price to be a string."
    assert int(spot_price) > 0, "Expected spot price to > 0."


def test_get_time_stretch():
    """test get_time_stretch."""
    time_stretch = pyperdrive.get_time_stretch(
        pyperdrive.get_spot_rate(POOL_CONFIG, POOL_INFO),
    )
    assert time_stretch is not None, "Failed to get time_stretch."
    assert isinstance(time_stretch, str), "Expected time_stretch to be a string."
    assert float(time_stretch) > 0, "Expected time_stretch to be > 0."


def test_get_effective_share_reserves():
    """Test get_effective_share_reserves."""
    effective_share_reserves = pyperdrive.get_effective_share_reserves(
        str(POOL_INFO.shareReserves),
        str(POOL_INFO.shareAdjustment),
    )
    assert effective_share_reserves is not None, "Failed to get effective_share_reserves."
    assert isinstance(effective_share_reserves, str), "Expected effective_share_reserves to be a string."
    assert int(effective_share_reserves) > 0, "Expected effective_share_reserves to be > 0."


def test_calculate_bonds_given_shares_and_rate():
    """Test calculate_bonds_given_shares_and_rate."""
    effective_share_reserves = pyperdrive.get_effective_share_reserves(
        str(POOL_INFO.shareReserves),
        str(POOL_INFO.shareAdjustment),
    )
    bonds = pyperdrive.calculate_bonds_given_shares_and_rate(
        effective_share_reserves,
        str(POOL_CONFIG.initialSharePrice),
        pyperdrive.get_spot_rate(POOL_CONFIG, POOL_INFO),
        str(POOL_CONFIG.positionDuration),
        str(POOL_CONFIG.timeStretch),
    )
    assert bonds is not None, "Failed to get bonds."
    assert isinstance(bonds, str), "Expected bonds to be a string."
    assert int(bonds) > 0, "Expected bonds to be > 0."


def test_calculate_bonds_out_given_shares_in_down():
    """Test calculate_bonds_out_given_shares_in_down."""
    amount_in = str(1_000 * 10**18)
    out = pyperdrive.calculate_bonds_out_given_shares_in_down(POOL_CONFIG, POOL_INFO, amount_in)
    assert int(out) > 0


def test_calculate_shares_in_given_bonds_out_up():
    """Test calculate_shares_in_given_bonds_out_up."""
    amount_in = str(1_000 * 10**18)
    out = pyperdrive.calculate_shares_in_given_bonds_out_up(POOL_CONFIG, POOL_INFO, amount_in)
    assert int(out) > 0


def test_calculate_shares_in_given_bonds_out_down():
    """Test calculate_shares_in_given_bonds_out_down."""
    amount_in = str(1_000 * 10**18)
    out = pyperdrive.calculate_shares_in_given_bonds_out_down(POOL_CONFIG, POOL_INFO, amount_in)
    assert int(out) > 0


def test_calculate_shares_out_given_bonds_in_down():
    """Test calculate_shares_out_given_bonds_in_down."""
    amount_in = str(1_000 * 10**18)
    out = pyperdrive.calculate_shares_out_given_bonds_in_down(POOL_CONFIG, POOL_INFO, amount_in)
    assert int(out) > 0


def test_calculate_max_buy():
    """Test calculate_max_buy."""
    out = pyperdrive.calculate_max_buy(POOL_CONFIG, POOL_INFO)
    assert int(out) > 0


def test_calculate_max_sell():
    """Test calculate_max_buy."""
    minimum_share_reserves = str(10 * 10**18)
    out = pyperdrive.calculate_max_sell(POOL_CONFIG, POOL_INFO, minimum_share_reserves)
    assert int(out) > 0


def test_get_long_amount():
    """Test for get_long_amount."""
    base_amount = str(500 * 10**18)
    long_amount = pyperdrive.get_long_amount(POOL_CONFIG, POOL_INFO, base_amount)
    assert int(long_amount) > 0


def test_get_short_deposit():
    """Test for get_long_amount."""
    short_amount = str(50 * 10**18)
    spot_price = pyperdrive.get_spot_price(POOL_CONFIG, POOL_INFO)
    open_share_price = str(9 * 10**17)
    base_required = pyperdrive.get_short_deposit(POOL_CONFIG, POOL_INFO, short_amount, spot_price, open_share_price)
    assert int(base_required) > 0
    base_required_default_share_price = pyperdrive.get_short_deposit(
        POOL_CONFIG, POOL_INFO, short_amount, spot_price, None
    )
    assert int(base_required_default_share_price) > 0
    assert base_required_default_share_price == pyperdrive.get_short_deposit(
        POOL_CONFIG, POOL_INFO, short_amount, spot_price, "0"
    )


def test_max_long():
    """Test get_max_long."""
    budget = "1000000000000000000"  # 1 base
    checkpoint_exposure = "10000"
    max_iterations = 20
    max_long = pyperdrive.get_max_long(POOL_CONFIG, POOL_INFO, budget, checkpoint_exposure, max_iterations)
    assert int(max_long) > 0  # should == "1000000000000000000", or 1 base


def test_max_long_fail_conversion():
    """Test get_max_long."""
    max_iterations = 20
    # bad string inputs
    budget = "asdf"
    checkpoint_exposure = "100"
    with pytest.raises(ValueError, match="Failed to convert budget string to U256"):
        pyperdrive.get_max_long(POOL_CONFIG, POOL_INFO, budget, checkpoint_exposure, max_iterations)
    budget = "1.23"
    checkpoint_exposure = "100"
    with pytest.raises(ValueError, match="Failed to convert budget string to U256"):
        pyperdrive.get_max_long(POOL_CONFIG, POOL_INFO, budget, checkpoint_exposure, max_iterations)
    budget = "1000000000000000000"  # 1 base
    checkpoint_exposure = "asdf"
    with pytest.raises(ValueError, match="Failed to convert checkpoint_exposure string to I256"):
        pyperdrive.get_max_long(POOL_CONFIG, POOL_INFO, budget, checkpoint_exposure, max_iterations)


def test_max_short():
    """Test get_max_short."""
    # test using the state directly
    budget = str(int(10 * 10**18))  # 10k base
    open_share_price = str(int(1 * 10**18))  # 1 base
    checkpoint_exposure = str(0)
    conservative_price = None
    max_iterations = 20
    max_short = pyperdrive.get_max_short(
        POOL_CONFIG,
        POOL_INFO,
        budget,
        open_share_price,
        checkpoint_exposure,
        conservative_price,
        max_iterations,
    )
    assert int(max_short) > 0  # should == "2583754033693357393077", or apprx 2583 base


def test_max_short_fail_conversion():
    """Test get_max_short."""
    open_share_price = str(int(1 * 10**18))  # 1 base
    checkpoint_exposure = str(0)
    conservative_price = None
    max_iterations = 20
    # bad string inputs
    budget = "asdf"
    with pytest.raises(ValueError, match="Failed to convert budget string to U256"):
        pyperdrive.get_max_short(
            POOL_CONFIG,
            POOL_INFO,
            budget,
            open_share_price,
            checkpoint_exposure,
            conservative_price,
            max_iterations,
        )
    budget = "1.23"
    with pytest.raises(ValueError, match="Failed to convert budget string to U256"):
        pyperdrive.get_max_short(
            POOL_CONFIG,
            POOL_INFO,
            budget,
            open_share_price,
            checkpoint_exposure,
            conservative_price,
            max_iterations,
        )
    budget = "10000000000000000000000"  # 10k base
    open_share_price = "asdf"
    with pytest.raises(ValueError, match="Failed to convert open_share_price string to U256"):
        pyperdrive.get_max_short(
            POOL_CONFIG,
            POOL_INFO,
            budget,
            open_share_price,
            checkpoint_exposure,
            conservative_price,
            max_iterations,
        )
