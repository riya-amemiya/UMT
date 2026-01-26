from typing import TypedDict

from ..array import quick_sort
from .levenshtein_distance import levenshtein_distance


class FuzzySearchResult(TypedDict):
    """Result item from fuzzy search."""

    item: str
    score: float


def fuzzy_search(
    query: str,
    items: list[str],
    threshold: float = 0.6,
) -> list[FuzzySearchResult]:
    """
    Perform fuzzy string matching on an array of strings.

    Args:
        query: The search query.
        items: Array of strings to search in.
        threshold: Similarity threshold (0-1) for matching (default 0.6).

    Returns:
        Array of matching items with their similarity scores, sorted by best match.

    Example:
        >>> fuzzy_search("hello", ["hello", "world", "helo", "help"])
        [{'item': 'hello', 'score': 1.0}, {'item': 'helo', 'score': 0.8}, {'item': 'help', 'score': 0.6}]
    """
    if len(query) == 0:
        return []

    results: list[FuzzySearchResult] = []

    for item in items:
        distance = levenshtein_distance(query.lower(), item.lower())
        max_length = max(len(query), len(item))
        score = 1 - distance / max_length

        if score >= threshold:
            results.append({"item": item, "score": score})

    return quick_sort(results, lambda a, b: b["score"] - a["score"])
