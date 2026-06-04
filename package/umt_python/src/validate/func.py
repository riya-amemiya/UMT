from collections.abc import Callable, Sequence
from dataclasses import dataclass


@dataclass(frozen=True)
class FunctionReturnType:
    """
    Result produced by a function validator.

    Carries the validated callable through the type field so downstream
    composition helpers can recover it.

    Attributes:
        validate: Whether validation succeeded.
        message: The validation message (empty on success).
        type: The validated value.
    """

    validate: bool
    message: str
    type: object


@dataclass(frozen=True)
class FunctionSchema:
    """
    Function schema definition.

    Both fields are optional; when omitted the validator only enforces that
    the value is callable.

    Attributes:
        input: Validators describing positional parameters.
        output: Validator describing the return value.
    """

    input: Sequence[Callable[[object], FunctionReturnType]] | None = None
    output: Callable[[object], FunctionReturnType] | None = None


class FunctionValidator:
    """
    Validator that checks a value is callable.

    Calling the validator only checks that the value is callable, since
    argument and return-value contracts can only be enforced when the
    function is actually invoked. Use ``implement`` to obtain a
    runtime-checked wrapper that asserts the schema on every call.
    """

    def __init__(
        self,
        schema: FunctionSchema | None = None,
        message: str | None = None,
    ) -> None:
        self._schema = schema
        self._message = message

    def __call__(self, value: object) -> FunctionReturnType:
        if not callable(value):
            return FunctionReturnType(
                validate=False,
                message=self._message or "",
                type=value,
            )
        return FunctionReturnType(validate=True, message="", type=value)

    def implement(self, function_: Callable[..., object]) -> Callable[..., object]:
        """
        Wraps a concrete function with runtime validation.

        The returned wrapper validates positional arguments against the
        schema's input validators before the call and validates the return
        value against the output validator after the call.

        Args:
            function_: The concrete function to wrap.

        Returns:
            A wrapper that asserts the schema on every call.

        Raises:
            TypeError: If function_ is not callable, or if an argument or
                the return value fails validation.
        """
        if not callable(function_):
            raise TypeError(self._message or "value is not a function")

        inputs = self._schema.input if self._schema else None
        output = self._schema.output if self._schema else None

        def wrapped(*arguments_: object) -> object:
            if inputs is not None:
                for index, input_validator in enumerate(inputs):
                    result = input_validator(arguments_[index])
                    if not result.validate:
                        raise TypeError(
                            result.message
                            or f"function input at index {index} failed validation"
                        )
            return_value = function_(*arguments_)
            if output is not None:
                result = output(return_value)
                if not result.validate:
                    raise TypeError(
                        result.message or "function output failed validation"
                    )
            return return_value

        return wrapped


def func(
    schema: FunctionSchema | None = None,
    message: str | None = None,
) -> FunctionValidator:
    """
    Creates a function validator.

    When invoked the validator only checks that the value is callable; use
    ``implement`` on the returned validator to obtain a runtime-checked
    wrapper that asserts the schema for each call.

    Args:
        schema: Function schema definition describing input and output.
        message: Custom error message for type validation.

    Returns:
        A validator augmented with an implement method.

    Example:
        >>> validator = func()
        >>> validator(lambda: 0).validate
        True
        >>> validator("nope").validate
        False
        >>> def to_number(value):
        ...     valid = isinstance(value, (int, float)) and not isinstance(
        ...         value, bool
        ...     )
        ...     return FunctionReturnType(validate=valid, message="", type=value)
        >>> schema = FunctionSchema(input=[to_number], output=to_number)
        >>> wrapped = func(schema).implement(lambda n: n * 2)
        >>> wrapped(3)
        6
    """
    return FunctionValidator(schema, message)
