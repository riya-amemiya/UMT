from collections.abc import Callable
from typing import Any


class OnceFunction:
    """
    A function wrapper that ensures the underlying function is called only once.
    Subsequent calls return the cached result of the first invocation.

    Attributes:
        called: Whether the function has been invoked yet.
        result: The cached result of the first invocation, or None if not yet called.
    """

    def __init__(self, fn: Callable[..., Any]) -> None:
        self._fn = fn
        self.called: bool = False
        self.result: Any = None

    def __call__(self, *args: Any, **kwargs: Any) -> Any:  # noqa: ANN401
        if not self.called:
            self.called = True
            self.result = self._fn(*args, **kwargs)
        return self.result


def once(fn: Callable[..., Any]) -> OnceFunction:
    """
    Creates a function that is restricted to be called only once.
    Subsequent calls return the result of the first invocation,
    ignoring any new arguments.

    Args:
        fn: The function to restrict.

    Returns:
        A OnceFunction that invokes the original only on its first call.

    Example:
        >>> initialized = once(lambda: 42)
        >>> initialized()
        42
        >>> initialized()
        42
        >>> initialized.called
        True
    """
    return OnceFunction(fn)
