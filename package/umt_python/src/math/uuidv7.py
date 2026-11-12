import os
import time


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
    unix_ts_ms = time.time_ns() // 1_000_000
    random_bytes = bytearray(os.urandom(10))
    random_bytes[0] = 0x70 | (random_bytes[0] & 0x0F)
    random_bytes[2] = 0x80 | (random_bytes[2] & 0x3F)

    digits = ((unix_ts_ms & 0xFFFFFFFFFFFF).to_bytes(6, "big") + random_bytes).hex()
    return f"{digits[:8]}-{digits[8:12]}-{digits[12:16]}-{digits[16:20]}-{digits[20:]}"
