import asyncio


async def sleep(seconds: float) -> None:
    """Return after the specified number of seconds have elapsed.

    Mirrors the TypeScript ``sleep`` helper. The TS version takes
    milliseconds; this Python port uses seconds to match the rest of the
    package's async utilities.

    Args:
        seconds: The number of seconds to sleep.

    Returns:
        ``None`` once the delay has elapsed.

    Example:
        >>> import asyncio
        >>> asyncio.run(sleep(0))
    """
    await asyncio.sleep(seconds)
