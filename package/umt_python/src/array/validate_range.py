from dataclasses import dataclass
from typing import TypeVar

T = TypeVar("T")


@dataclass
class ValidatedSortRange:
    """Result of validating a sort range."""

    start_index: int
    end_index: int
    should_sort: bool


def validate_range(
    array: list[T],
    start_index: int,
    end_index: int,
) -> ValidatedSortRange:
    """
    Validates and adjusts the start and end indices for an operation on an array.

    Ensures indices are within the bounds of the array.

    Args:
        array: The array.
        start_index: The desired starting index.
        end_index: The desired ending index.

    Returns:
        ValidatedSortRange containing the validated start index, end index,
        and a boolean indicating if the operation should proceed on the range.

    Example:
        >>> validate_range([1, 2, 3, 4, 5], 0, 4)
        ValidatedSortRange(start_index=0, end_index=4, should_sort=True)
        >>> validate_range([1, 2, 3, 4, 5], -1, 10)
        ValidatedSortRange(start_index=0, end_index=4, should_sort=True)
        >>> validate_range([], 0, 0)
        ValidatedSortRange(start_index=0, end_index=-1, should_sort=False)
        >>> validate_range([1, 2, 3], 5, 10)
        ValidatedSortRange(start_index=2, end_index=2, should_sort=True)
    """
    length = len(array)

    if length == 0:
        return ValidatedSortRange(start_index=0, end_index=-1, should_sort=False)

    validated_start_index = max(0, min(start_index, length - 1))
    validated_end_index = max(validated_start_index, min(end_index, length - 1))

    return ValidatedSortRange(
        start_index=validated_start_index,
        end_index=validated_end_index,
        should_sort=validated_end_index >= validated_start_index,
    )
