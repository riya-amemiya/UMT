from typing import Literal


def bitwise(
    x: int,
    k: int,
    direction: Literal["left", "right"] = "left",
) -> int:
    """
    Performs bit rotation on a number.

    Args:
        x: 32-bit integer to rotate.
        k: Number of bits to rotate.
        direction: Direction of rotation ('left' or 'right').

    Returns:
        The result after rotating k bits in the specified direction.

    Example:
        >>> hex(bitwise(0x12345678, 8))
        '0x34567812'
        >>> hex(bitwise(0x12345678, 8, 'right'))
        '0x78123456'
    """
    # Ensure we're working with 32-bit unsigned integers
    x = x & 0xFFFFFFFF
    rotation = ((k % 32) + 32) % 32

    if direction == "left":
        return ((x << rotation) | (x >> (32 - rotation))) & 0xFFFFFFFF
    elif direction == "right":
        return ((x >> rotation) | (x << (32 - rotation))) & 0xFFFFFFFF
    else:
        raise ValueError(f"Invalid direction {direction}")
