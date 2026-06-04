from collections.abc import Callable

from src.math.xoshiro256 import xoshiro256


def _hash_string_to_seed(value: str) -> int:
    """Computes an FNV-1a 32-bit hash of a string's UTF-8 bytes."""
    hash_value = 2_166_136_261
    for byte in value.encode("utf-8"):
        hash_value ^= byte
        hash_value = (hash_value * 16_777_619) & 0xFFFFFFFF
    return hash_value & 0xFFFFFFFF


def seeded_random(seed: int | str) -> Callable[[], float]:
    """
    Returns a deterministic PRNG seeded from a number or string.

    Each call to the returned function produces a float in [0, 1) from the
    same xoshiro256** stream.

    Args:
        seed: Seed value (number or string).

    Returns:
        A function that returns sequential pseudo-random floats.

    Example:
        >>> rand = seeded_random("hello")
        >>> rand2 = seeded_random("hello")
        >>> rand() == rand2()
        True
        >>> 0 <= seeded_random(42)() < 1
        True
    """
    initial = (
        (seed & 0xFFFFFFFF) if isinstance(seed, int) else _hash_string_to_seed(seed)
    )
    state = [
        initial or 1,
        ((initial * 0x9E_37_79_B9) & 0xFFFFFFFF) or 2,
        ((initial * 0x85_EB_CA_6B) & 0xFFFFFFFF) or 3,
        ((initial * 0xC2_B2_AE_35) & 0xFFFFFFFF) or 4,
    ]
    return lambda: xoshiro256(state)
