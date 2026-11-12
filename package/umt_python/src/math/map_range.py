from .inverse_lerp import inverse_lerp
from .lerp import lerp


def map_range(
    value: int | float,
    in_min: int | float,
    in_max: int | float,
    out_min: int | float,
    out_max: int | float,
) -> float:
    """
    Maps a value from one range to another without clamping.

    Args:
        value: Value in the input range.
        in_min: Start of the input range.
        in_max: End of the input range.
        out_min: Start of the output range.
        out_max: End of the output range.

    Returns:
        The mapped value, or out_min when in_min equals in_max.

    Example:
        >>> map_range(5, 0, 10, 0, 100)
        50.0
        >>> map_range(0.5, 0, 1, 10, 20)
        15.0
        >>> map_range(15, 0, 10, 0, 100)
        150.0
    """
    return lerp(out_min, out_max, inverse_lerp(in_min, in_max, value))
