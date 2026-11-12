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
        The result after rotating k bits in the specified direction,
        as a signed 32-bit integer.

    Example:
        >>> hex(bitwise(0x12345678, 8))
        '0x34567812'
        >>> hex(bitwise(0x12345678, 8, 'right'))
        '0x78123456'
    """
    unsigned = x & 0xFFFFFFFF
    rotation = k % 32

    if direction == "left":
        rotated = ((unsigned << rotation) | (unsigned >> (32 - rotation))) & 0xFFFFFFFF
    else:
        rotated = ((unsigned >> rotation) | (unsigned << (32 - rotation))) & 0xFFFFFFFF
    return rotated - 0x100000000 if rotated >= 0x80000000 else rotated
