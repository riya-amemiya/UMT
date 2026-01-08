from typing import TypeVar, Callable, Generic, Union, overload

T = TypeVar("T")
U = TypeVar("U")


class Pipeline(Generic[T]):
    """
    A pipeline class for chaining transformations.

    When called without arguments, returns the stored value.
    When called with a function argument, applies the function and generates a new Pipeline instance.
    """

    def __init__(self, value: T):
        self._value = value

    @overload
    def __call__(self) -> T: ...

    @overload
    def __call__(self, transformer: Callable[[T], U]) -> "Pipeline[U]": ...

    def __call__(
        self, transformer: Callable[[T], U] | None = None
    ) -> Union[T, "Pipeline[U]"]:
        if transformer is None:
            return self._value
        return Pipeline(transformer(self._value))


def create_pipeline(initial_value: T) -> Pipeline[T]:
    """
    Creates a Pipeline instance.

    Args:
        initial_value: Initial value to start the pipeline

    Returns:
        Pipeline instance

    Example:
        >>> pipeline = create_pipeline(1)
        >>> pipeline()
        1
        >>> pipeline(lambda x: x + 1)()
        2
    """
    return Pipeline(initial_value)
