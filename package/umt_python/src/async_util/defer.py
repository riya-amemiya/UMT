import asyncio
from dataclasses import dataclass
from typing import Any, Generic, TypeVar

T = TypeVar("T")


@dataclass
class Deferred(Generic[T]):
    """A deferred future whose resolution can be controlled externally.

    Mirrors the TypeScript ``Deferred`` interface. ``future`` is the awaitable
    that callers wait on, while ``resolve`` and ``reject`` settle it from the
    outside.

    Attributes:
        future: The :class:`asyncio.Future` that resolves or rejects later.
        resolve: Callback that fulfills ``future`` with a value.
        reject: Callback that rejects ``future`` with an exception.
    """

    future: "asyncio.Future[T]"

    def resolve(self, value: T) -> None:
        """Fulfill the deferred future with *value* if not already settled."""
        if not self.future.done():
            self.future.set_result(value)

    def reject(self, reason: Any = None) -> None:  # noqa: ANN401
        """Reject the deferred future with *reason* if not already settled."""
        if self.future.done():
            return
        if isinstance(reason, BaseException):
            self.future.set_exception(reason)
        else:
            self.future.set_exception(Exception(reason))


def defer() -> Deferred[T]:
    """Create a deferred future whose resolve and reject can be called
    externally.

    Mirrors the TypeScript ``defer`` helper, returning an object exposing the
    awaitable along with ``resolve`` and ``reject`` callbacks.

    Returns:
        A :class:`Deferred` with ``future``, ``resolve``, and ``reject``.

    Example:
        >>> import asyncio
        >>> async def main() -> int:
        ...     d: Deferred[int] = defer()
        ...     d.resolve(42)
        ...     return await d.future
        >>> asyncio.run(main())
        42
    """
    loop = asyncio.get_event_loop()
    future: asyncio.Future[T] = loop.create_future()
    return Deferred(future)
