"""Python wrapper for the rust hyperdrive_math::utils module."""

from __future__ import annotations

# pylint: disable=no-name-in-module
from . import hyperdrivepy as rust_module  # type: ignore


def get_time_stretch(rate: str, position_duration: str) -> str:
    """Calculate the time stretch parameter given a pool's spot rate.

    ..math::
        tau = 5.24592 / (0.4665 * r * 100)

    Arguments
    ---------
    rate: str (FixedPoint)
        The pool's spot rate (aka apr, or fixed rate).
    position_duration: str(FixedPoint)
        The amount of time before a trade matures.

    Returns
    -------
    time_stretch: str (FixedPoint)
        The time stretch parameter (tau).
    """
    return rust_module.get_time_stretch(rate, position_duration)


def get_effective_share_reserves(
    share_reserves: str,
    share_adjustment: str,
) -> str:
    r"""Calculate the effective share reserves given the share reserves and share adjustment.

    ..math::
        z_effective = z - \zeta

    Arguments
    ---------
    share_reserves: str (FixedPoint)
        The pool's share reserves.
    share_adjustment: str (I256)
        The zeta factor for adjusting share reserves.

    Returns
    -------
    effective_share_reserves: str (FixedPoint)
        The adjusted share reserves, accounting for the zeta factor.
    """
    return rust_module.get_effective_share_reserves(share_reserves, share_adjustment)


def calculate_initial_bond_reserves(
    effective_share_reserves: str,
    initial_vault_share_price: str,
    apr: str,
    position_duration: str,
    time_stretch: str,
) -> str:
    """Calculates the bond reserves assuming that the pool has a given
    share reserves and fixed rate APR.

    ..math::
        r = ((1/p)-1)/t = (1-p)/(pt)
        p = ((u * z) / y) ** t
        y = mu * (z - zeta) * (1 + apr * t) ** (1/tau)

    Arguments
    ---------
    effective_share_reserves: str (FixedPoint)
        The pool's effective share reserves. The effective share
        reserves are a modified version of the share reserves
        used when pricing trades.
    initial_vault_share_price: str (FixedPoint)
        The pool's initial share price.
    apr: str (FixedPoint)
        The pool's APR.
    position_duration: str (FixedPoint)
        The amount of time until maturity in seconds.
    time_stretch: str (FixedPoint)
        The time stretch parameter (tau).

    Returns
    -------
    bond_reserves: str (FixedPoint)
        The bond reserves (without adjustment) that make
        the pool have a specified APR.
    """
    return rust_module.calculate_initial_bond_reserves(
        effective_share_reserves, initial_vault_share_price, apr, position_duration, time_stretch
    )
