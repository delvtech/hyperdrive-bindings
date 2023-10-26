"""Python wrapper for the rust hyperdrive_math::State module."""
from __future__ import annotations

# pylint: disable=no-name-in-module
from . import pyperdrive as rust_module  # type: ignore
from . import types
from .utils import _serialize_pool_config, _serialize_pool_info

# The module itself doesn't have type hints, so we will ignore warnings in this file.
# pylint: disable=c-extension-no-member
# we don't control the number of arguments, wrapping rust functions
# pylint: disable=too-many-arguments


## Hyperdrive state functions.


def get_solvency(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
) -> str:
    """Get the pool's solvency.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.
        Includes things like reserve levels and share prices.

    Returns
    -------
    str (FixedPoint)
        solvency = share_reserves - long_exposure / share_price - minimum_share_reserves
    """
    pool_config_serialized = _serialize_pool_config(pool_config)
    pool_info_serialized = _serialize_pool_info(pool_info)
    rust_interface: rust_module.HyperdriveState = rust_module.HyperdriveState(
        pool_config_serialized, pool_info_serialized
    )
    return rust_interface.get_solvency()


def get_spot_rate(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
) -> str:
    """Get the spot rate (fixed rate) for the market.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.
        Includes things like reserve levels and share prices.

    Returns
    -------
    str (FixedPoint)
        The pool's spot rate.
    """
    pool_config_serialized = _serialize_pool_config(pool_config)
    pool_info_serialized = _serialize_pool_info(pool_info)
    rust_interface: rust_module.HyperdriveState = rust_module.HyperdriveState(
        pool_config_serialized, pool_info_serialized
    )
    return rust_interface.get_spot_rate()


def get_spot_price(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
) -> str:
    """Get the spot price of the bond.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.
        Includes things like reserve levels and share prices.

    Returns
    -------
    str (FixedPoint)
        The pool's spot price.
    """
    pool_config_serialized = _serialize_pool_config(pool_config)
    pool_info_serialized = _serialize_pool_info(pool_info)
    rust_interface: rust_module.HyperdriveState = rust_module.HyperdriveState(
        pool_config_serialized, pool_info_serialized
    )
    return rust_interface.get_spot_price()


def get_long_amount(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    base_amount: str,
) -> str:
    """Gets the long amount that will be opened for a given base amount.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.
        Includes things like reserve levels and share prices.
    base_amount : str (FixedPoint)
        The amount to spend, in base.

    Returns
    -------
    long_amount : str (FixedPoint)
        The amount of bonds purchased.
    """
    pool_config_serialized = _serialize_pool_config(pool_config)
    pool_info_serialized = _serialize_pool_info(pool_info)
    rust_interface: rust_module.HyperdriveState = rust_module.HyperdriveState(
        pool_config_serialized, pool_info_serialized
    )
    return rust_interface.get_long_amount(base_amount)


def get_short_deposit(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    short_amount: str,
    spot_price: str,
    open_share_price: str | None = None,
) -> str:
    """Gets the amount of base the trader will need to deposit for a short of a given size.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.
        Includes things like reserve levels and share prices.
    short_amount : str (FixedPoint)
        The amount to of bonds to short.
    spot_price : str (FixedPoint)
        The pool's current price for bonds.
    open_share_price : str (FixedPoint), optional
        Optionally provide the open share price for the short.
        If this is not provided or is None, then we will use the pool's current share price.

    Returns
    -------
    short_amount : str (FixedPoint)
        The amount of base required to short the bonds (aka the "max loss").
    """
    if open_share_price is None:
        # the underlying rust code uses current market share price if this is 0
        # zero value is used because the smart contract will return 0 if the checkpoint hasn't been minted
        open_share_price = "0"
    pool_config_serialized = _serialize_pool_config(pool_config)
    pool_info_serialized = _serialize_pool_info(pool_info)
    rust_interface: rust_module.HyperdriveState = rust_module.HyperdriveState(
        pool_config_serialized, pool_info_serialized
    )
    return rust_interface.get_short_deposit(short_amount, spot_price, open_share_price)


def to_checkpoint(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    time: str,
) -> str:
    """Converts a timestamp to the checkpoint timestamp that it corresponds to.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.
        Includes things like reserve levels and share prices.
    time : str (U256)
        A string representation of any timestamp (in seconds) before or at the present.

    Returns
    -------
    str (U256)
        The checkpoint timestamp.
    """
    pool_config_serialized = _serialize_pool_config(pool_config)
    pool_info_serialized = _serialize_pool_info(pool_info)
    rust_interface: rust_module.HyperdriveState = rust_module.HyperdriveState(
        pool_config_serialized, pool_info_serialized
    )
    return rust_interface.to_checkpoint(time)


def get_max_long(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    budget: str,
    checkpoint_exposure: str,
    maybe_max_iterations: int | None,
) -> str:
    """Get the max amount of bonds that can be purchased for the given budget.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.
        Includes things like reserve levels and share prices.
    budget : str (FixedPont)
        The account budget in base for making a long.
    checkpoint_exposure : str (I256)
        The net exposure for the given checkpoint.
    maybe_max_iterations : int, optional
        The number of iterations to use for the Newtonian method.

    Returns
    -------
    str (FixedPoint)
        The maximum long the pool and user's wallet can support.
    """
    pool_config_serialized = _serialize_pool_config(pool_config)
    pool_info_serialized = _serialize_pool_info(pool_info)
    rust_interface: rust_module.HyperdriveState = rust_module.HyperdriveState(
        pool_config_serialized, pool_info_serialized
    )
    return rust_interface.get_max_long(budget, checkpoint_exposure, maybe_max_iterations)


def get_max_short(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    budget: str,
    open_share_price: str,
    checkpoint_exposure: str,
    maybe_conservative_price: str | None,
    maybe_max_iterations: int | None,
) -> str:
    """Get the max amount of bonds that can be shorted for the given budget.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.
        Includes things like reserve levels and share prices.
    budget : str (FixedPoint)
        The account budget in base for making a short.
    open_share_price : str (FixedPoint)
        The share price of underlying vault.
    checkpoint_exposure : str (FixedPoint)
        The net exposure for the given checkpoint.
    maybe_conservative_price : str (FixedPoint), optional
        A lower bound on the realized price that the short will pay.
    maybe_max_iterations : int, optional
        The number of iterations to use for the Newtonian method.

    Returns
    -------
    str (FixedPoint)
        The maximum short the pool and user's wallet can handle.
    """
    pool_config_serialized = _serialize_pool_config(pool_config)
    pool_info_serialized = _serialize_pool_info(pool_info)
    rust_interface: rust_module.HyperdriveState = rust_module.HyperdriveState(
        pool_config_serialized, pool_info_serialized
    )
    return rust_interface.get_max_short(
        budget,
        open_share_price,
        checkpoint_exposure,
        maybe_conservative_price,
        maybe_max_iterations,
    )


def get_out_for_in(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    amount_in: str,
    shares_in: bool,
) -> str:
    """Gets the amount of an asset for a given amount in of the other.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.
        Includes things like reserve levels and share prices.
    amount_in : str (FixedPoint)
        The amount of asset going into the pool.
    shares_in : bool
        True if the asset in is shares, False if it is bonds.
        The amount out will be the opposite type.

    Returns
    -------
    str (FixedPoint)
        The amount out.
    """
    pool_config_serialized = _serialize_pool_config(pool_config)
    pool_info_serialized = _serialize_pool_info(pool_info)
    rust_interface: rust_module.HyperdriveState = rust_module.HyperdriveState(
        pool_config_serialized, pool_info_serialized
    )
    return rust_interface.get_out_for_in(amount_in, shares_in)


def get_out_for_in_safe(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    amount_in: str,
    shares_in: bool,
) -> str:
    """Gets the amount of an asset for a given amount in of the other.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.
        Includes things like reserve levels and share prices.
    amount_in : str (FixedPoint)
        The amount of asset going into the pool.
    shares_in : bool
        True if the asset in is shares, False if it is bonds.
        The amount out will be the opposite type.

    Returns
    -------
    str (FixedPoint)
        The amount out.
    """
    pool_config_serialized = _serialize_pool_config(pool_config)
    pool_info_serialized = _serialize_pool_info(pool_info)
    rust_interface: rust_module.HyperdriveState = rust_module.HyperdriveState(
        pool_config_serialized, pool_info_serialized
    )
    return rust_interface.get_out_for_in_safe(amount_in, shares_in)


def get_in_for_out(
    pool_config: types.PoolConfigType,
    pool_info: types.PoolInfoType,
    amount_out: str,
    shares_out: bool,
) -> str:
    """Gets the amount of an asset for a given amount out of the other.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.
        Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.
        Includes things like reserve levels and share prices.
    amount_out : str (FixedPoint)
        The amount of asset the user expects to receive from the pool.
    shares_out : bool
        True if the asset out is shares, False if it is bonds.
        The amount in will be the opposite type.

    Returns
    -------
    str (FixedPoint)
        The amount in as a string representation of a Solidity uint256 value.
    """
    pool_config_serialized = _serialize_pool_config(pool_config)
    pool_info_serialized = _serialize_pool_info(pool_info)
    rust_interface: rust_module.HyperdriveState = rust_module.HyperdriveState(
        pool_config_serialized, pool_info_serialized
    )
    return rust_interface.get_in_for_out(amount_out, shares_out)
