"""Template literal validation core module.

Provides a validator that checks whether a string matches a template literal
pattern composed of string fragments and primitive validators (string /
number / boolean / bigint). The runtime check is performed by an
auto-generated regular expression assembled from the parts.
"""

import re
from collections.abc import Callable, Sequence
from dataclasses import dataclass

TemplateLiteralValidator = Callable[..., object]

# A part of a template literal definition is either a string literal that must
# appear verbatim or a primitive validator whose accepted shape is converted to
# a regex fragment at construction time.
TemplateLiteralPart = str | TemplateLiteralValidator


@dataclass
class TemplateLiteralReturnType:
    """
    Result produced by a ``template_literal`` validator.

    Attributes:
        validate: Whether the value matched the template literal pattern.
        message: Error message reported when the value does not match.
        type: The value that was validated.
    """

    validate: bool
    message: str
    type: object


def _escape_regex(input_value: str) -> str:
    return re.sub(r"([$()*+.?\[\\\]^{|}])", r"\\\1", input_value)


def _tag_to_pattern(tag: object) -> str:
    if tag == "string":
        return ".*?"
    if tag == "number":
        return r"-?(?:\d+\.\d+|\d+(?:\.\d*)?|\.\d+)"
    if tag == "boolean":
        return "(?:true|false)"
    if tag == "bigint":
        return r"-?\d+"
    return ".+?"


def _detect_validator_tag(validator: TemplateLiteralValidator) -> object:
    result = validator(None)
    return getattr(result, "type", None)


def template_literal(
    parts: Sequence[TemplateLiteralPart],
    message: str = "",
) -> Callable[[object], TemplateLiteralReturnType]:
    """
    Creates a validator that checks whether a value matches a template literal
    pattern.

    Each part is either a string literal that must appear verbatim or a
    primitive validator (string / number / boolean / bigint) that contributes
    a regex fragment.

    Args:
        parts: Sequence of literal strings and primitive validators.
        message: Custom error message reported when the value does not match
            (default: empty string).

    Returns:
        A validator function that checks whether a value is a string matching
        the assembled template literal pattern.

    Example:
        >>> from src.validate import number, string
        >>> validator = template_literal(["hello, ", string(), "!"])
        >>> validator("hello, world!").validate
        True
        >>> validator("hi, world!").validate
        False
        >>> item = template_literal(["item-", number()])
        >>> item("item-1.5").validate
        True
        >>> item("item-abc").validate
        False
    """
    pattern = "^"
    for part in parts:
        if isinstance(part, str):
            pattern += _escape_regex(part)
        else:
            pattern += f"(?:{_tag_to_pattern(_detect_validator_tag(part))})"
    pattern += "$"
    regex = re.compile(pattern)

    def validator(value: object) -> TemplateLiteralReturnType:
        if not isinstance(value, str) or regex.fullmatch(value) is None:
            return TemplateLiteralReturnType(
                validate=False, message=message, type=value
            )
        return TemplateLiteralReturnType(validate=True, message="", type=value)

    return validator
