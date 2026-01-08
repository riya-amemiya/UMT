import math


def _inline_sort_3(array: list[float], ascending: bool) -> None:
    """Inline sort for 3 elements."""
    a, b, c = array[0], array[1], array[2]

    if ascending:
        if a > b:
            a, b = b, a
        if b > c:
            b, c = c, b
            if a > b:
                a, b = b, a
    else:
        if a < b:
            a, b = b, a
        if b < c:
            b, c = c, b
            if a < b:
                a, b = b, a

    array[0], array[1], array[2] = a, b, c


def _handle_nan_sort(array: list[float], ascending: bool) -> list[float]:
    """Handle arrays with NaN values."""
    valid: list[float] = []
    nan_count = 0

    for element in array:
        if element == element:
            valid.append(element)
        else:
            nan_count += 1

    _numeric_quick_sort(valid, 0, len(valid) - 1, ascending)

    for _ in range(nan_count):
        valid.append(float("nan"))

    for i in range(len(array)):
        array[i] = valid[i]

    return array


def _counting_sort(
    array: list[float], min_val: int, max_val: int, ascending: bool
) -> list[float]:
    """Counting sort for small integer ranges."""
    range_size = max_val - min_val + 1
    count = [0] * range_size

    for element in array:
        count[int(element) - min_val] += 1

    index = 0
    if ascending:
        for i in range(range_size):
            cnt = count[i]
            value = i + min_val
            for _ in range(cnt):
                array[index] = value
                index += 1
    else:
        for i in range(range_size - 1, -1, -1):
            cnt = count[i]
            value = i + min_val
            for _ in range(cnt):
                array[index] = value
                index += 1

    return array


def _radix_sort_positive(array: list[int]) -> None:
    """Radix sort for positive integers."""
    length = len(array)
    if length <= 1:
        return

    max_val = max(array)
    output = [0] * length
    count = [0] * 256

    shift = 0
    while max_val >> shift > 0:
        for i in range(256):
            count[i] = 0

        for i in range(length):
            digit = (array[i] >> shift) & 0xFF
            count[digit] += 1

        for i in range(1, 256):
            count[i] += count[i - 1]

        for i in range(length - 1, -1, -1):
            digit = (array[i] >> shift) & 0xFF
            count[digit] -= 1
            output[count[digit]] = array[i]

        for i in range(length):
            array[i] = output[i]

        shift += 8


def _radix_sort(array: list[float], ascending: bool) -> list[float]:
    """Radix sort for integers."""
    length = len(array)

    positive: list[int] = []
    negative: list[int] = []
    zero_count = 0

    for i in range(length):
        val = int(array[i])
        if val > 0:
            positive.append(val)
        elif val < 0:
            negative.append(-val)
        else:
            zero_count += 1

    if positive:
        _radix_sort_positive(positive)

    if negative:
        _radix_sort_positive(negative)

    index = 0
    if ascending:
        for i in range(len(negative) - 1, -1, -1):
            array[index] = -negative[i]
            index += 1
        for _ in range(zero_count):
            array[index] = 0
            index += 1
        for element in positive:
            array[index] = element
            index += 1
    else:
        for i in range(len(positive) - 1, -1, -1):
            array[index] = positive[i]
            index += 1
        for _ in range(zero_count):
            array[index] = 0
            index += 1
        for element in negative:
            array[index] = -element
            index += 1

    return array


def _numeric_insertion_sort(
    array: list[float], low: int, high: int, ascending: bool
) -> None:
    """Numeric insertion sort."""
    if ascending:
        for i in range(low + 1, high + 1):
            key = array[i]
            j = i - 1
            while j >= low and array[j] > key:
                array[j + 1] = array[j]
                j -= 1
            array[j + 1] = key
    else:
        for i in range(low + 1, high + 1):
            key = array[i]
            j = i - 1
            while j >= low and array[j] < key:
                array[j + 1] = array[j]
                j -= 1
            array[j + 1] = key


def _numeric_partition(array: list[float], low: int, high: int, ascending: bool) -> int:
    """Numeric partition with median-of-three pivot."""
    mid = low + ((high - low) >> 1)

    if ascending:
        if array[mid] < array[low]:
            array[low], array[mid] = array[mid], array[low]
        if array[high] < array[low]:
            array[low], array[high] = array[high], array[low]
        if array[high] < array[mid]:
            array[mid], array[high] = array[high], array[mid]
    else:
        if array[mid] > array[low]:
            array[low], array[mid] = array[mid], array[low]
        if array[high] > array[low]:
            array[low], array[high] = array[high], array[low]
        if array[high] > array[mid]:
            array[mid], array[high] = array[high], array[mid]

    array[mid], array[high - 1] = array[high - 1], array[mid]
    pivot = array[high - 1]

    i = low
    j = high - 1

    if ascending:
        while True:
            i += 1
            while array[i] < pivot:
                i += 1
            j -= 1
            while array[j] > pivot:
                j -= 1
            if i >= j:
                break
            array[i], array[j] = array[j], array[i]
    else:
        while True:
            i += 1
            while array[i] > pivot:
                i += 1
            j -= 1
            while array[j] < pivot:
                j -= 1
            if i >= j:
                break
            array[i], array[j] = array[j], array[i]

    array[i], array[high - 1] = array[high - 1], array[i]
    return i


def _numeric_quick_sort(
    array: list[float], low: int, high: int, ascending: bool
) -> list[float]:
    """Optimized quicksort for numbers."""
    stack: list[int] = [low, high]

    while stack:
        high_idx = stack.pop()
        low_idx = stack.pop()

        if high_idx <= low_idx:
            continue

        if high_idx - low_idx < 16:
            _numeric_insertion_sort(array, low_idx, high_idx, ascending)
            continue

        pivot = _numeric_partition(array, low_idx, high_idx, ascending)

        if pivot - low_idx > high_idx - pivot:
            stack.extend([low_idx, pivot - 1, pivot + 1, high_idx])
        else:
            stack.extend([pivot + 1, high_idx, low_idx, pivot - 1])

    return array


def ultra_number_sort(array: list[float], ascending: bool = True) -> list[float]:
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
    result = list(array)
    length = len(result)

    if length <= 1:
        return result

    if length == 2:
        if (result[0] > result[1]) == ascending:
            result[0], result[1] = result[1], result[0]
        return result

    if length == 3:
        _inline_sort_3(result, ascending)
        return result

    all_integers = True
    min_val = result[0]
    max_val = result[0]
    has_nan = False

    for i in range(length):
        value = result[i]
        if value != value:
            has_nan = True
            break
        if value < min_val:
            min_val = value
        if value > max_val:
            max_val = value
        if all_integers and value != math.floor(value):
            all_integers = False

    if has_nan:
        return _handle_nan_sort(result, ascending)

    if (
        all_integers
        and max_val - min_val < length * 2
        and max_val - min_val < 1_000_000
    ):
        return _counting_sort(result, int(min_val), int(max_val), ascending)

    if all_integers and length > 100:
        return _radix_sort(result, ascending)

    return _numeric_quick_sort(result, 0, length - 1, ascending)
