import time

from .random_int import random_int


def uuidv7() -> str:
    """
    Generates a UUID v7 (Universally Unique Identifier version 7).

    Returns:
        A UUID v7 string in the format xxxxxxxx-xxxx-7xxx-8xxx-xxxxxxxxxxxx.

    Example:
        >>> import re
        >>> pattern = r'^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$'
        >>> bool(re.match(pattern, uuidv7()))
        True
    """
    digits = "0123456789abcdef"
    unix_ts_ms = int(time.time() * 1000)
    rand_a = random_int(0xFFF)
    rand_b_hi = random_int(0x3FFFFFFF)
    rand_b_lo = random_int(0xFFFFFFFF)

    bytes_arr = [0] * 16

    for index in range(6):
        bytes_arr[index] = (unix_ts_ms >> ((5 - index) * 8)) & 0xFF

    bytes_arr[6] = 0x70 | (rand_a >> 8)
    bytes_arr[7] = rand_a & 0xFF
    bytes_arr[8] = 0x80 | (rand_b_hi >> 24)
    bytes_arr[9] = (rand_b_hi >> 16) & 0xFF
    bytes_arr[10] = (rand_b_hi >> 8) & 0xFF
    bytes_arr[11] = rand_b_hi & 0xFF
    bytes_arr[12] = (rand_b_lo >> 24) & 0xFF
    bytes_arr[13] = (rand_b_lo >> 16) & 0xFF
    bytes_arr[14] = (rand_b_lo >> 8) & 0xFF
    bytes_arr[15] = rand_b_lo & 0xFF

    uuid = ""
    for index, byte in enumerate(bytes_arr):
        uuid += digits[byte >> 4] + digits[byte & 0xF]
        if index in (3, 5, 7, 9):
            uuid += "-"

    return uuid
