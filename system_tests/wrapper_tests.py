"""Tests for hyperdrive_math.rs wrappers"""

import hyperdrivepy
import pytest
from hyperdrivepy.pypechain_types import Fees, PoolConfig, PoolInfo

POOL_CONFIG = PoolConfig(
    baseToken="0x1234567890abcdef1234567890abcdef12345678",
    vaultSharesToken="0x1234567890abcdef1234567890abcdef12345678",
    linkerFactory="0x1234567890abcdef1234567890abcdef12345678",
    linkerCodeHash=bytes(32),
    initialVaultSharePrice=1 * 10**18,  # 1e18
    minimumShareReserves=1 * 10**17,  # 0.1e18
    minimumTransactionAmount=1 * 10**16,  # 0.001e18
    positionDuration=60 * 60 * 24 * 365,  # 1 year
    checkpointDuration=86_400,
    timeStretch=1 * 10**17,  # 0.1e18
    governance="0xabcdef1234567890abcdef1234567890abcdef12",
    feeCollector="0xfedcba0987654321fedcba0987654321fedcba09",
    sweepCollector="0xfedcba0987654321fedcba0987654321fedcba09",
    fees=Fees(curve=0, flat=0, governanceLP=0, governanceZombie=0),
)


POOL_INFO = PoolInfo(
    shareReserves=1_000_000 * 10**18,
    shareAdjustment=0,
    zombieBaseProceeds=0,
    zombieShareReserves=0,
    bondReserves=2_000_000 * 10**18,
    lpTotalSupply=3_000_000 * 10**18,
    vaultSharePrice=1 * 10**18,
    longsOutstanding=0,
    longAverageMaturityTime=0,
    shortsOutstanding=0,
    shortAverageMaturityTime=0,
    withdrawalSharesReadyToWithdraw=0,
    withdrawalSharesProceeds=0,
    lpSharePrice=1 * 10**18,
    longExposure=0,
)


def test_calculate_max_spot_price():
    """test get_max_spot_rate."""
    max_spot_price = hyperdrivepy.calculate_max_spot_price(POOL_CONFIG, POOL_INFO)
    assert max_spot_price is not None, "Failed to get max spot price."
    assert isinstance(max_spot_price, str), "Expected spot rate to be a string."
    assert int(max_spot_price) > 0, "Expected max_spot_price to > 0."


def test_calculate_spot_price_after_long():
    """test calculate_spot_price_after_long."""
    spot_price = hyperdrivepy.calculate_spot_price_after_long(
        POOL_CONFIG, POOL_INFO, base_amount=str(1_000 * 10**18), bond_amount=None
    )
    assert spot_price is not None, "Failed to calculate spot price after long."
    assert isinstance(spot_price, str), "Expected spot rate to be a string."
    assert int(spot_price) > 0, "Expected max_spot_price to > 0."


def test_calculate_spot_price_after_short():
    """test calculate_spot_price_after_short."""
    spot_price = hyperdrivepy.calculate_spot_price_after_short(
        POOL_CONFIG, POOL_INFO, bond_amount=str(100 * 10**18), base_amount=None
    )
    assert spot_price is not None, "Failed to calculate spot price after short."
    assert isinstance(spot_price, str), "Expected spot rate to be a string."
    assert int(spot_price) > 0, "Expected max_spot_price to > 0."


def test_calculate_solvency():
    """test calculate_max_spot_rate."""
    solvency = hyperdrivepy.calculate_solvency(POOL_CONFIG, POOL_INFO)
    assert solvency is not None, "Failed to calculate spot price after long."
    assert isinstance(solvency, str), "Expected spot rate to be a string."
    assert int(solvency) > 0, "Expected max_spot_price to > 0."


def test_calculate_spot_rate_after_long():
    """test calculate_spot_rate_after_long."""
    spot_rate = hyperdrivepy.calculate_spot_rate_after_long(POOL_CONFIG, POOL_INFO, base_amount=str(int(1_000e18)))
    assert spot_rate is not None, "Failed to calculate spot rate after long."
    assert isinstance(spot_rate, str), "Expected spot rate to be a string."
    assert int(spot_rate) > 0, "Expected spot rate to > 0."


def test_calculate_spot_rate():
    """test calculate_spot_rate."""
    spot_rate = hyperdrivepy.calculate_spot_rate(POOL_CONFIG, POOL_INFO)
    assert spot_rate is not None, "Failed to calculate spot rate."
    assert isinstance(spot_rate, str), "Expected spot rate to be a string."
    assert int(spot_rate) > 0, "Expected spot rate to > 0."


def test_to_checkpoint():
    """test to_checkpoint."""
    checkpoint_time = hyperdrivepy.to_checkpoint(POOL_CONFIG, POOL_INFO, time=str(100))
    assert checkpoint_time is not None, "Failed to get checkpoint time."
    assert isinstance(checkpoint_time, str), "Expected checkpoint time to be a string."


def test_calculate_spot_price():
    """test calculate_spot_price."""
    spot_price = hyperdrivepy.calculate_spot_price(POOL_CONFIG, POOL_INFO)
    assert spot_price is not None, "Failed to calculate spot price."
    assert isinstance(spot_price, str), "Expected spot price to be a string."
    assert int(spot_price) > 0, "Expected spot price to > 0."


def test_calculate_time_stretch():
    """test calculate_time_stretch."""
    time_stretch = hyperdrivepy.calculate_time_stretch(
        hyperdrivepy.calculate_spot_rate(POOL_CONFIG, POOL_INFO),
        str(60 * 60 * 24 * 365),  # 1 year
    )
    assert time_stretch is not None, "Failed to calculate time_stretch."
    assert isinstance(time_stretch, str), "Expected time_stretch to be a string."
    assert float(time_stretch) > 0, "Expected time_stretch to be > 0."

    time_stretch = hyperdrivepy.calculate_time_stretch(
        hyperdrivepy.calculate_spot_rate(POOL_CONFIG, POOL_INFO),
        str(60 * 60 * 24 * 30),  # ~1 month
    )
    assert time_stretch is not None, "Failed to calculate time_stretch."
    assert isinstance(time_stretch, str), "Expected time_stretch to be a string."
    assert float(time_stretch) > 0, "Expected time_stretch to be > 0."


def test_calculate_effective_share_reserves():
    """Test calculate_effective_share_reserves."""
    effective_share_reserves = hyperdrivepy.calculate_effective_share_reserves(
        str(POOL_INFO.shareReserves),
        str(POOL_INFO.shareAdjustment),
    )
    assert effective_share_reserves is not None, "Failed to calculate effective_share_reserves."
    assert isinstance(effective_share_reserves, str), "Expected effective_share_reserves to be a string."
    assert int(effective_share_reserves) > 0, "Expected effective_share_reserves to be > 0."


def test_calculate_initial_bond_reserves():
    """Test calculate_initial_bond_reserves."""
    effective_share_reserves = hyperdrivepy.calculate_effective_share_reserves(
        str(POOL_INFO.shareReserves),
        str(POOL_INFO.shareAdjustment),
    )
    bonds = hyperdrivepy.calculate_initial_bond_reserves(
        effective_share_reserves,
        str(POOL_CONFIG.initialVaultSharePrice),
        hyperdrivepy.calculate_spot_rate(POOL_CONFIG, POOL_INFO),
        str(POOL_CONFIG.positionDuration),
        str(POOL_CONFIG.timeStretch),
    )
    assert bonds is not None, "Failed to calculate bonds."
    assert isinstance(bonds, str), "Expected bonds to be a string."
    assert int(bonds) > 0, "Expected bonds to be > 0."


def test_calculate_bonds_out_given_shares_in_down():
    """Test calculate_bonds_out_given_shares_in_down."""
    amount_in = str(1_000 * 10**18)
    out = hyperdrivepy.calculate_bonds_out_given_shares_in_down(POOL_CONFIG, POOL_INFO, amount_in)
    assert int(out) > 0


def test_calculate_shares_in_given_bonds_out_up():
    """Test calculate_shares_in_given_bonds_out_up."""
    amount_in = str(1_000 * 10**18)
    out = hyperdrivepy.calculate_shares_in_given_bonds_out_up(POOL_CONFIG, POOL_INFO, amount_in)
    assert int(out) > 0


def test_calculate_shares_in_given_bonds_out_down():
    """Test calculate_shares_in_given_bonds_out_down."""
    amount_in = str(1_000 * 10**18)
    out = hyperdrivepy.calculate_shares_in_given_bonds_out_down(POOL_CONFIG, POOL_INFO, amount_in)
    assert int(out) > 0


def test_calculate_shares_out_given_bonds_in_down():
    """Test calculate_shares_out_given_bonds_in_down."""
    amount_in = str(1_000 * 10**18)
    out = hyperdrivepy.calculate_shares_out_given_bonds_in_down(POOL_CONFIG, POOL_INFO, amount_in)
    assert int(out) > 0


def test_calculate_open_long():
    """Test for calculate_open_long."""
    base_amount = str(500 * 10**18)
    long_amount = hyperdrivepy.calculate_open_long(POOL_CONFIG, POOL_INFO, base_amount)
    assert int(long_amount) > 0


def test_calculate_close_long():
    """Test for calculate_close_long."""
    bond_amount = str(500 * 10**18)
    maturity_time = str(9 * 10**17 + 10)
    current_time = str(9 * 10**17)
    shares_returned = hyperdrivepy.calculate_close_long(
        POOL_CONFIG, POOL_INFO, bond_amount, maturity_time, current_time
    )
    assert int(shares_returned) > 0


def test_calculate_open_short():
    """Test for calculate_open_short."""
    short_amount = str(50 * 10**18)
    open_vault_share_price = str(9 * 10**17)
    base_required = hyperdrivepy.calculate_open_short(POOL_CONFIG, POOL_INFO, short_amount, open_vault_share_price)
    assert int(base_required) > 0
    base_required_default_vault_share_price = hyperdrivepy.calculate_open_short(
        POOL_CONFIG, POOL_INFO, short_amount, None
    )
    assert int(base_required_default_vault_share_price) > 0
    assert base_required_default_vault_share_price == hyperdrivepy.calculate_open_short(
        POOL_CONFIG, POOL_INFO, short_amount, "0"
    )


def test_calculate_close_short():
    """Test for calculate_close_short."""
    short_amount = str(50 * 10**18)
    open_vault_share_price = str(8 * 10**17)
    close_vault_share_price = str(9 * 10**17)
    maturity_time = str(9 * 10**17 + 10)
    current_time = str(9 * 10**17)
    shares_received = hyperdrivepy.calculate_close_short(
        POOL_CONFIG,
        POOL_INFO,
        short_amount,
        open_vault_share_price,
        close_vault_share_price,
        maturity_time,
        current_time,
    )
    assert int(shares_received) > 0


def test_targeted_long():
    """Test calculate_targeted_long_with_budget."""
    budget = "1000000000000000000"  # 1 base
    checkpoint_exposure = "10000"
    max_iterations = 20
    allowable_error = "0000000010000000000"
    target = "001000000000000000"
    targeted_long = hyperdrivepy.calculate_targeted_long(
        POOL_CONFIG,
        POOL_INFO,
        budget,
        target,
        checkpoint_exposure,
        max_iterations,
        allowable_error,
    )
    assert int(targeted_long) > 0


def test_max_long():
    """Test calculate_max_long."""
    budget = "1000000000000000000"  # 1 base
    checkpoint_exposure = "10000"
    max_iterations = 20
    max_long = hyperdrivepy.calculate_max_long(POOL_CONFIG, POOL_INFO, budget, checkpoint_exposure, max_iterations)
    assert int(max_long) > 0  # should == "1000000000000000000", or 1 base


def test_max_long_fail_conversion():
    """Test calculate_max_long."""
    max_iterations = 20
    # bad string inputs
    budget = "asdf"
    checkpoint_exposure = "100"
    with pytest.raises(ValueError, match="Failed to convert budget string to U256"):
        hyperdrivepy.calculate_max_long(POOL_CONFIG, POOL_INFO, budget, checkpoint_exposure, max_iterations)
    budget = "1.23"
    checkpoint_exposure = "100"
    with pytest.raises(ValueError, match="Failed to convert budget string to U256"):
        hyperdrivepy.calculate_max_long(POOL_CONFIG, POOL_INFO, budget, checkpoint_exposure, max_iterations)
    budget = "1000000000000000000"  # 1 base
    checkpoint_exposure = "asdf"
    with pytest.raises(ValueError, match="Failed to convert checkpoint_exposure string to I256"):
        hyperdrivepy.calculate_max_long(POOL_CONFIG, POOL_INFO, budget, checkpoint_exposure, max_iterations)


def test_max_short():
    """Test get_max_short."""
    # test using the state directly
    budget = str(int(10 * 10**18))  # 10k base
    open_vault_share_price = str(int(1 * 10**18))  # 1 base
    checkpoint_exposure = str(0)
    conservative_price = None
    max_iterations = 20
    max_short = hyperdrivepy.calculate_max_short(
        POOL_CONFIG,
        POOL_INFO,
        budget,
        open_vault_share_price,
        checkpoint_exposure,
        conservative_price,
        max_iterations,
    )
    assert int(max_short) > 0  # should == "2583754033693357393077", or apprx 2583 base


def test_max_short_fail_conversion():
    """Test calculate_max_short."""
    open_vault_share_price = str(int(1 * 10**18))  # 1 base
    checkpoint_exposure = str(0)
    conservative_price = None
    max_iterations = 20
    # bad string inputs
    budget = "asdf"
    with pytest.raises(ValueError, match="Failed to convert budget string to U256"):
        hyperdrivepy.calculate_max_short(
            POOL_CONFIG,
            POOL_INFO,
            budget,
            open_vault_share_price,
            checkpoint_exposure,
            conservative_price,
            max_iterations,
        )
    budget = "1.23"
    with pytest.raises(ValueError, match="Failed to convert budget string to U256"):
        hyperdrivepy.calculate_max_short(
            POOL_CONFIG,
            POOL_INFO,
            budget,
            open_vault_share_price,
            checkpoint_exposure,
            conservative_price,
            max_iterations,
        )
    budget = "10000000000000000000000"  # 10k base
    open_vault_share_price = "asdf"
    with pytest.raises(ValueError, match="Failed to convert open_vault_share_price string to U256"):
        hyperdrivepy.calculate_max_short(
            POOL_CONFIG,
            POOL_INFO,
            budget,
            open_vault_share_price,
            checkpoint_exposure,
            conservative_price,
            max_iterations,
        )


def test_calculate_present_value():
    """Test calculate_present_value."""
    current_block_timestamp = str(1000)
    present_value = hyperdrivepy.calculate_present_value(POOL_CONFIG, POOL_INFO, current_block_timestamp)
    assert int(present_value) > 0


def test_calculate_idle_share_reserves_in_base():
    """Test calculate_idle_share_reserves_in_base."""
    idle_share_reserves = hyperdrivepy.calculate_idle_share_reserves_in_base(POOL_CONFIG, POOL_INFO)
    assert int(idle_share_reserves) > 0
