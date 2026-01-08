from .bitwise import bitwise


def xoshiro256(
    state: list[int],
    min_value: float = 0,
    max_value: float = 1,
) -> float:
    """
    Generates random numbers using the Xoshiro256** algorithm.

    Args:
        state: Array of four 32-bit state values (will be modified in place).
        min_value: Minimum value of the generated random number (inclusive).
        max_value: Maximum value of the generated random number (exclusive).

    Returns:
        Generated random number between min and max.

    Example:
        >>> state = [1, 2, 3, 4]
        >>> result = xoshiro256(state)
        >>> 0 <= result < 1
        True
    """
    # Ensure unsigned 32-bit operations
    s0 = state[0] & 0xFFFFFFFF
    s3 = state[3] & 0xFFFFFFFF
    total = (s0 + s3) & 0xFFFFFFFF
    result = (bitwise(total, 23) + s0) & 0xFFFFFFFF

    t = (state[1] << 17) & 0xFFFFFFFF

    state[2] ^= state[0]
    state[3] ^= state[1]
    state[1] ^= state[2]
    state[0] ^= state[3]

    state[2] ^= t
    state[3] = bitwise(state[3], 45)

    return min_value + (result / (2**32)) * (max_value - min_value)
