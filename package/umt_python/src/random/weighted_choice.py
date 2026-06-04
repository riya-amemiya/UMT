import random as random_module
from collections.abc import Sequence
from typing import Any, TypedDict


class WeightedItem(TypedDict):
    value: Any
    weight: float


def weighted_choice(items: Sequence[WeightedItem]) -> Any:  # noqa: ANN401
    """
    Returns a random element using cumulative weights and binary search.

    Items with non-positive weights are skipped from the pool.

    Args:
        items: Items with non-negative weights, each a mapping with
            ``value`` and ``weight`` keys.

    Returns:
        A randomly selected value.

    Example:
        >>> weighted_choice([{"value": "skip", "weight": 0}, {"value": "win", "weight": 5}])
        'win'
        >>> weighted_choice([{"value": "a", "weight": 1}, {"value": "b", "weight": 1}]) in ("a", "b")
        True
    """
    cumulative: list[float] = []
    total = 0.0
    for item in items:
        if item["weight"] > 0:
            total += item["weight"]
        cumulative.append(total)
    target = random_module.random() * total
    lo = 0
    hi = len(cumulative) - 1
    while lo < hi:
        mid = (lo + hi) >> 1
        if cumulative[mid] <= target:
            lo = mid + 1
        else:
            hi = mid
    return items[lo]["value"]
