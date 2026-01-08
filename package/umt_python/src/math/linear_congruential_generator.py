def linear_congruential_generator(
    seed: int,
    max_value: int = 4_294_967_296,
    multiplier: int = 1_664_525,
    increment: int = 1_013_904_223,
) -> int:
    """
    Linear Congruential Generator for random number generation.

    Args:
        seed: Initial seed value.
        max_value: Maximum value (default: 4294967296).
        multiplier: Multiplier parameter (default: 1664525).
        increment: Increment parameter (default: 1013904223).

    Returns:
        Generated random number.

    Example:
        >>> linear_congruential_generator(12345)
        1406932606
    """
    return (multiplier * seed + increment) % max_value
