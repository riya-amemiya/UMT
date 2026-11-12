import asyncio
import random as random_module
from collections.abc import Awaitable, Callable
from typing import TYPE_CHECKING, Literal, TypeVar

from .safe_execute import Success
from .safe_execute_async import safe_execute_async

if TYPE_CHECKING:
    from ..async_util.wait_for import AbortSignal

T = TypeVar("T")

Backoff = Literal["fixed", "exponential", "linear"]


def _compute_delay(
    base_delay: float,
    attempt_number: int,
    backoff: Backoff,
    jitter: bool,
) -> float:
    wait = base_delay
    if backoff == "linear":
        wait = base_delay * attempt_number
    elif backoff == "exponential":
        wait = base_delay * 2 ** (attempt_number - 1)
    if jitter:
        wait *= random_module.random()
    return wait


async def retry(
    fn: Callable[[], Awaitable[T]],
    retries: int = 3,
    delay: float = 1.0,
    should_retry: Callable[[Exception], bool] | None = None,
    *,
    backoff: Backoff = "fixed",
    jitter: bool = False,
    max_delay: float | None = None,
    on_retry: Callable[[Exception, int], None] | None = None,
    signal: "AbortSignal | None" = None,
) -> T:
    """
    Retries a given async function with configurable retry logic.

    Supports fixed, linear, or exponential backoff, optional jitter, an
    optional ``max_delay`` upper bound applied to each computed delay after
    jitter, and abort signal cancellation.

    Args:
        fn: The async function to retry
        retries: Maximum number of retry attempts (default: 3)
        delay: Base delay between retries in seconds (default: 1.0)
        should_retry: Function to determine if an error should trigger a retry (default: always retry)
        backoff: Delay growth strategy, one of "fixed", "linear", or "exponential" (default: "fixed")
        jitter: Whether to multiply each delay by a random factor in [0, 1) (default: False)
        max_delay: Upper bound in seconds for each delay, applied after jitter (default: no bound)
        on_retry: Callback invoked before each retry with the error and the zero-based attempt number
        signal: Optional abort signal checked before every attempt

    Returns:
        The result of the function

    Raises:
        The final exception if all retries fail, or the abort reason if the signal is aborted

    Example:
        >>> async def risky_operation():
        ...     return await fetch_data()
        >>> result = await retry(risky_operation, retries=5, delay=2.0, backoff="exponential")
        >>> result = await retry(risky_operation, delay=0.5, backoff="exponential", max_delay=5.0)
    """
    attempt_number = 0
    while True:
        if signal is not None and signal.aborted:
            if signal.reason is None:
                raise Exception("Aborted")
            if isinstance(signal.reason, BaseException):
                raise signal.reason
            raise Exception(signal.reason)

        result = await safe_execute_async(fn)
        if isinstance(result, Success):
            return result.value

        error = result.error
        if retries - attempt_number <= 0 or (
            should_retry is not None and not should_retry(error)
        ):
            raise error
        if on_retry is not None:
            on_retry(error, attempt_number)
        wait = _compute_delay(delay, attempt_number + 1, backoff, jitter)
        await asyncio.sleep(wait if max_delay is None else min(wait, max_delay))
        attempt_number += 1
