from collections.abc import Callable
from typing import Generic, TypeVar

from ..error.safe_execute import Result, safe_execute

T = TypeVar("T")
U = TypeVar("U")


class Pipe(Generic[T]):
    """
    A class to handle pipeline processing.
    Allows chaining transformations in a fluent interface.
    """

    def __init__(self, value: T) -> None:
        self._value = value

    def map(self, fn: Callable[[T], U]) -> "Pipe[U]":
        """
        Applies a transformation function and returns a new Pipe instance.

        Args:
            fn: Transformation function to apply

        Returns:
            New Pipe instance with transformed value
        """
        return Pipe(fn(self._value))

    def when(
        self, predicate: Callable[[T], bool], fn: Callable[[T], U]
    ) -> "Pipe[T | U]":
        """
        Applies a transformation function only if the condition is met.

        Args:
            predicate: Condition function
            fn: Transformation function to apply if condition is met

        Returns:
            New Pipe instance with conditionally transformed value
        """
        if predicate(self._value):
            return Pipe(fn(self._value))
        return Pipe(self._value)  # type: ignore

    def tap(self, fn: Callable[[T], None]) -> "Pipe[T]":
        """
        Executes a side effect without changing the value.

        Args:
            fn: Function to execute as a side effect

        Returns:
            Same Pipe instance
        """
        fn(self._value)
        return self

    def filter_strict(self, predicate: Callable[[T], bool]) -> "Pipe[T]":
        """
        Strictly filters the value based on a predicate function.
        Raises an error if the predicate returns false.

        Args:
            predicate: Condition function that determines if value should be filtered

        Returns:
            New Pipe instance with filtered value

        Raises:
            ValueError: If the predicate returns false
        """
        if predicate(self._value):
            return Pipe(self._value)
        raise ValueError("Value did not match filter predicate")

    def filter_with_default(
        self, predicate: Callable[[T], bool], default_value: T
    ) -> "Pipe[T]":
        """
        Filters the value based on a predicate function.
        Returns a default value if the predicate returns false.

        Args:
            predicate: Condition function that determines if value should be filtered
            default_value: Default value to use if predicate returns false

        Returns:
            New Pipe instance with filtered value or default value
        """
        if predicate(self._value):
            return Pipe(self._value)
        return Pipe(default_value)

    def filter_result(
        self, predicate: Callable[[T], bool]
    ) -> "Pipe[Result[T, Exception]]":
        """
        Filters the value based on a predicate function.
        Returns a Result type containing either the filtered value or an error.

        Args:
            predicate: Condition function that determines if value should be filtered

        Returns:
            New Pipe instance with Result containing filtered value or error
        """

        def check() -> T:
            if predicate(self._value):
                return self._value
            raise ValueError("Value did not match filter predicate")

        return Pipe(safe_execute(check))

    def end(self) -> T:
        """
        Terminates the pipeline and returns the final value.

        Returns:
            Final result of the pipeline processing
        """
        return self._value


def pipe(initial_value: T) -> Pipe[T]:
    """
    Creates a new Pipe instance with an initial value.

    Args:
        initial_value: Initial value for the pipeline

    Returns:
        New Pipe instance

    Example:
        >>> pipe(1).map(lambda x: x + 1).map(lambda x: x * 2).end()
        4
    """
    return Pipe(initial_value)
