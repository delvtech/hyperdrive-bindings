"""Python wrapper for the rust hyperdrive_math::State module."""

from __future__ import annotations

from . import types
from .utils import _get_interface

# We don't control the number of arguments when wrapping rust functions.
# pylint: disable=too-many-arguments


def calculate_max_spot_price(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
) -> str:
    """Get the pool's max spot price.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.

    Returns
    -------
    str (FixedPoint)
        max_spot_price = 1/1 + curve_fee * (1 / (spot_price - 1))
    """
    return _get_interface(pool_config, pool_info).calculate_max_spot_price()


def calculate_spot_price_after_long(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    base_amount: str,
    bond_amount: str | None = None,
) -> str:
    """Get the spot price after opening a long on Hyperdrive, including fees.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.
    base_amount: str (FixedPoint)
        The amount base provided.
    bond_amount: str (FixedPoint) | None, optional
        The number of bonds purchased.
        Defaults to the output of `calculate_open_long(base_amount)`.

    Returns
    -------
    str (FixedPoint)
        The spot price after opening the long.
    """
    return _get_interface(pool_config, pool_info).calculate_spot_price_after_long(base_amount, bond_amount)


def calculate_spot_price_after_short(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    bond_amount: str,
    base_amount: str | None = None,
) -> str:
    """Get the spot price after opening a short on Hyperdrive, including fees.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.
    bond_amount: str (FixedPoint)
        The amount bonds shorted.
    base_amount: str (FixedPoint) | None, optional
        The amount of base supplied.
        Defaults to the output of `calculate_open_short(bond_amount)`.

    Returns
    -------
    str (FixedPoint)
        The spot price after opening the long.
    """
    return _get_interface(pool_config, pool_info).calculate_spot_price_after_short(bond_amount, base_amount)


def calculate_solvency(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
) -> str:
    """Get the pool's solvency.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.

    Returns
    -------
    str (FixedPoint)
        solvency = share_reserves - long_exposure / vault_share_price - minimum_share_reserves
    """
    return _get_interface(pool_config, pool_info).calculate_solvency()


def calculate_spot_rate_after_long(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    base_amount: str,
    bond_amount: str | None = None,
) -> str:
    """Get the spot rate after opening a long on Hyperdrive, including fees.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.
    base_amount: str (FixedPoint)
        The amount base provided.
    bond_amount: str (FixedPoint) | None, optional
        The number of bonds purchased.
        Defaults to the output of `calculate_open_long(base_amount)`.

    Returns
    -------
    str (FixedPoint)
        The spot rate after opening the long.
    """
    return _get_interface(pool_config, pool_info).calculate_spot_rate_after_long(base_amount, bond_amount)


def calculate_spot_rate(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
) -> str:
    """Get the spot rate (fixed rate) for the market.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.

    Returns
    -------
    str (FixedPoint)
        The pool's spot rate.
    """
    return _get_interface(pool_config, pool_info).calculate_spot_rate()


def calculate_spot_price(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
) -> str:
    """Get the spot price of the bond.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.

    Returns
    -------
    str (FixedPoint)
        The pool's spot price.
    """
    return _get_interface(pool_config, pool_info).calculate_spot_price()


def calculate_open_long(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    base_amount: str,
) -> str:
    """Gets the long amount that will be opened for a given base amount.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.
    base_amount: str (FixedPoint)
        The amount to spend, in base.

    Returns
    -------
    str (FixedPoint)
        The amount of bonds purchased.
    """
    return _get_interface(pool_config, pool_info).calculate_open_long(base_amount)


def calculate_close_long(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    bond_amount: str,
    maturity_time: str,
    current_time: str,
) -> str:
    """Calculates the amount of shares that will be returned after fees for closing a long.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.
    bond_amount: str (FixedPoint)
        The amount of bonds to sell.
    maturity_time: str (FixedPoint)
        The maturity time of the long.
    current_time: str (FixedPoint)
        The current block time.

    Returns
    -------
    str (FixedPoint)
        The amount of shares returned.
    """
    return _get_interface(pool_config, pool_info).calculate_close_long(bond_amount, maturity_time, current_time)


def calculate_open_short(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    short_amount: str,
    open_vault_share_price: str | None = None,
) -> str:
    """Gets the amount of base the trader will need to deposit for a short of a given size.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.
    short_amount: str (FixedPoint)
        The amount to of bonds to short.
    open_vault_share_price: str (FixedPoint) | None, optional
        Optionally provide the open share price for the short.
        If this is not provided or is None, then we will use the pool's current share price.

    Returns
    -------
    str (FixedPoint)
        The amount of base required to short the bonds (aka the "max loss").
    """
    if open_vault_share_price is None:
        # the underlying rust code uses current market share price if this is 0
        # zero value is used because the smart contract will return 0 if the checkpoint hasn't been minted
        open_vault_share_price = "0"
    return _get_interface(pool_config, pool_info).calculate_open_short(short_amount, open_vault_share_price)


def calculate_close_short(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    bond_amount: str,
    open_vault_share_price: str,
    close_vault_share_price: str,
    maturity_time: str,
    current_time: str,
) -> str:
    """Gets the amount of shares the trader will receive from closing a short.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.
    bond_amount: str (FixedPoint)
        The amount to of bonds provided.
    open_vault_share_price: str (FixedPoint)
        The share price when the short was opened.
    close_vault_share_price: str (FixedPoint)
        The share price when the short was closed.
    maturity_time: str (FixedPoint)
        The maturity time of the long.
    current_time: str (FixedPoint)
        The current block time.

    Returns
    -------
    str (FixedPoint)
        The amount of shares the trader will receive for closing the short.
    """
    return _get_interface(pool_config, pool_info).calculate_close_short(
        bond_amount, open_vault_share_price, close_vault_share_price, maturity_time, current_time
    )


def to_checkpoint(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    time: str,
) -> str:
    """Converts a timestamp to the checkpoint timestamp that it corresponds to.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.
    time: str (U256)
        A string representation of any timestamp (in seconds) before or at the present.

    Returns
    -------
    str (U256)
        The checkpoint timestamp.
    """
    return _get_interface(pool_config, pool_info).to_checkpoint(time)


def calculate_targeted_long(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    budget: str,
    target_rate: str,
    checkpoint_exposure: str,
    maybe_max_iterations: int | None,
    maybe_allowable_error: str | None,
) -> str:
    """Calculate the amount of bonds that can be purchased for the given budget.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.
    budget: str (FixedPont)
        The account budget in base for making a long.
    target: str (FixedPoint)
        The target fixed rate.
    checkpoint_exposure: str (I256)
        The net exposure for the given checkpoint.
    maybe_max_iterations: int, optional
        The number of iterations to use for the Newtonian method.
        Defaults to 7.
    maybe_allowable_error: str (FixedPoint) | None, Optional
        The amount of error supported for reaching the target rate.
        Defaults to 1e-4.


    Returns
    -------
    str (FixedPoint)
        The long to hit the target rate.
    """
    return _get_interface(pool_config, pool_info).calculate_targeted_long_with_budget(
        budget,
        target_rate,
        checkpoint_exposure,
        maybe_max_iterations,
        maybe_allowable_error,
    )


def calculate_max_long(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    budget: str,
    checkpoint_exposure: str,
    maybe_max_iterations: int | None,
) -> str:
    """Get the max amount of bonds that can be purchased for the given budget.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.
    budget: str (FixedPont)
        The account budget in base for making a long.
    checkpoint_exposure: str (I256)
        The net exposure for the given checkpoint.
    maybe_max_iterations: int, optional
        The number of iterations to use for the Newtonian method.

    Returns
    -------
    str (FixedPoint)
        The maximum long the pool and user's wallet can support.
    """
    return _get_interface(pool_config, pool_info).calculate_max_long(budget, checkpoint_exposure, maybe_max_iterations)


def calculate_max_short(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    budget: str,
    open_vault_share_price: str,
    checkpoint_exposure: str,
    maybe_conservative_price: str | None,
    maybe_max_iterations: int | None,
) -> str:
    """Get the max amount of bonds that can be shorted for the given budget.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.
    budget: str (FixedPoint)
        The account budget in base for making a short.
    open_vault_share_price: str (FixedPoint)
        The share price of underlying vault.
    checkpoint_exposure: str (FixedPoint)
        The net exposure for the given checkpoint.
    maybe_conservative_price: str (FixedPoint), optional
        A lower bound on the realized price that the short will pay.
    maybe_max_iterations: int, optional
        The number of iterations to use for the Newtonian method.

    Returns
    -------
    str (FixedPoint)
        The maximum short the pool and user's wallet can handle.
    """
    return _get_interface(pool_config, pool_info).calculate_max_short(
        budget,
        open_vault_share_price,
        checkpoint_exposure,
        maybe_conservative_price,
        maybe_max_iterations,
    )


def calculate_bonds_out_given_shares_in_down(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    amount_in: str,
) -> str:
    """Calculates the amount of bonds a user will receive from the pool by
    providing a specified amount of shares. We underestimate the amount of
    bonds. This uses Yieldspace math, and thus ignores Hyperdrive fees.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.
    amount_in: str (FixedPoint)
        The amount of shares going into the pool.

    Returns
    -------
    str (FixedPoint)
        The amount of bonds out.
    """
    return _get_interface(pool_config, pool_info).calculate_bonds_out_given_shares_in_down(amount_in)


def calculate_shares_in_given_bonds_out_up(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    amount_in: str,
) -> str:
    """Calculates the amount of shares a user must provide the pool to receive
    a specified amount of bonds. We overestimate the amount of shares in.
    This uses Yieldspace math, and thus ignores Hyperdrive fees.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.
    amount_in: str (FixedPoint)
        The amount of bonds to target.

    Returns
    -------
    str (FixedPoint)
        The amount of shares in to reach the target.
    """
    return _get_interface(pool_config, pool_info).calculate_shares_in_given_bonds_out_up(amount_in)


def calculate_shares_in_given_bonds_out_down(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    amount_in: str,
) -> str:
    """Calculates the amount of shares a user must provide the pool to receive
    a specified amount of bonds. We underestimate the amount of shares in.
    This uses Yieldspace math, and thus ignores Hyperdrive fees.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.
    amount_in: str (FixedPoint)
        The amount of bonds to target.

    Returns
    -------
    str (FixedPoint)
        The amount of shares in to reach the target.
    """
    return _get_interface(pool_config, pool_info).calculate_shares_in_given_bonds_out_down(amount_in)


def calculate_shares_out_given_bonds_in_down(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    amount_in: str,
) -> str:
    """Calculates the amount of shares a user will receive from the pool by
    providing a specified amount of bonds. We underestimate the amount of
    shares out. This uses Yieldspace math, and thus ignores Hyperdrive fees.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.
    amount_in: str (FixedPoint)
        The amount of bonds in.

    Returns
    -------
    str (FixedPoint)
        The amount of shares out.
    """
    return _get_interface(pool_config, pool_info).calculate_shares_out_given_bonds_in_down(amount_in)


def calculate_present_value(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    current_block_timestamp: str,
) -> str:
    """Calculates the present value of LPs capital in the pool.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.
    current_block_timestamp: str (U256)
        The current block timestamp, as an epoch time integer.

    Returns
    -------
    str (FixedPoint)
        The present value of all LP capital in the pool.
    """
    return _get_interface(pool_config, pool_info).calculate_present_value(current_block_timestamp)


def calculate_idle_share_reserves_in_base(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
) -> str:
    """Calculates the idle share reserves in base of the pool.

    Arguments
    ---------
    pool_config: PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info: PoolInfo
        Current state information of the hyperdrive contract.
        Includes attributes like reserve levels and share prices.

    Returns
    -------
    str (FixedPoint)
        The idle share reserves in base of the pool.
    """
    return _get_interface(pool_config, pool_info).calculate_idle_share_reserves_in_base()
