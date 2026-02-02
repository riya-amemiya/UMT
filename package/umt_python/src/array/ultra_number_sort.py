from typing import overload


@overload
def ultra_number_sort(array: list[int], ascending: bool = True) -> list[int]: ...
@overload
def ultra_number_sort(array: list[float], ascending: bool = True) -> list[float]: ...
def ultra_number_sort(
    array: list[int] | list[float], ascending: bool = True
) -> list[int] | list[float]:
    """
    Ultra-fast sorting specifically optimized for number arrays.

    Args:
        array: Array of numbers to sort.
        ascending: Sort in ascending order if True, descending if False.

    Returns:
        Sorted array.

    Example:
        >>> ultra_number_sort([3, 1, 4, 1, 5, 9, 2, 6])
        [1, 1, 2, 3, 4, 5, 6, 9]
        >>> ultra_number_sort([3, 1, 4, 1, 5], False)
        [5, 4, 3, 1, 1]
    """
    length = len(array)

    if length <= 1:
        return list(array)

    # Check for NaNs and separate them if present
    # We use a fast scan to check for NaNs.

    # We create new lists for valid and nans, so we don't modify the input array.
    nans = []
    valid = []

    # Use explicit loop for single pass split
    for x in array:
        if x != x:  # noqa: PLR0124
            nans.append(x)
        else:
            valid.append(x)

    # Sort the valid numbers
    valid.sort(reverse=not ascending)

    # If we had NaNs, append them.
    if nans:
        valid.extend(nans)

    return valid
