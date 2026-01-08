import asyncio
from typing import Callable, TypeVar, Awaitable, Optional

T = TypeVar("T")


async def retry(
    fn: Callable[[], Awaitable[T]],
    retries: int = 3,
    delay: float = 1.0,
    should_retry: Optional[Callable[[Exception], bool]] = None,
) -> T:
    """
    Retries a given async function with configurable retry logic.

    Args:
        fn: The async function to retry
        retries: Maximum number of retry attempts (default: 3)
        delay: Delay between retries in seconds (default: 1.0)
        should_retry: Function to determine if an error should trigger a retry (default: always retry)

    Returns:
        The result of the function

    Raises:
        The final exception if all retries fail

    Example:
        >>> async def risky_operation():
        ...     return await fetch_data()
        >>> result = await retry(risky_operation, retries=5, delay=2.0)
    """

    def _always_retry(_: Exception) -> bool:
        return True

    if should_retry is None:
        should_retry = _always_retry

    async def attempt(remaining_attempts: int) -> T:
        try:
            return await fn()
        except Exception as error:
            if remaining_attempts <= 0 or not should_retry(error):
                raise
            await asyncio.sleep(delay)
            return await attempt(remaining_attempts - 1)

    return await attempt(retries)
