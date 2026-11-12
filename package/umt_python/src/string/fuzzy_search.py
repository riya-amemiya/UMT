from typing import TypedDict

from ..array import quick_sort


class FuzzySearchResult(TypedDict):
    """Result item from fuzzy search."""

    item: str
    score: float


def _bounded_distance(
    string1: str,
    string2: str,
    max_allowed_distance: float,
    row: list[int],
) -> int | None:
    if len(string1) > len(string2):
        string1, string2 = string2, string1
    length1 = len(string1)

    row[: length1 + 1] = range(length1 + 1)

    for row_index, char2 in enumerate(string2, 1):
        previous_diagonal = row[0]
        row[0] = row_index
        min_distance_in_row = row_index

        for column_index, char1 in enumerate(string1, 1):
            above = row[column_index]
            value = previous_diagonal if char1 == char2 else previous_diagonal + 1
            if above < value:
                value = above + 1
            left = row[column_index - 1]
            if left < value:
                value = left + 1
            row[column_index] = value
            previous_diagonal = above
            min_distance_in_row = min(min_distance_in_row, value)

        if min_distance_in_row > max_allowed_distance:
            return None

    distance = row[length1]
    return None if distance > max_allowed_distance else distance


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
    query_length = len(query)
    if query_length == 0:
        return []

    results: list[FuzzySearchResult] = []
    lower_query = query.lower()[:query_length]
    row: list[int] = []

    for item in items:
        item_length = len(item)
        max_length = max(item_length, query_length)
        max_allowed_distance = max_length * (1 - threshold)

        if abs(query_length - item_length) > max_allowed_distance:
            continue

        distance = _bounded_distance(
            lower_query,
            item.lower()[:item_length],
            max_allowed_distance,
            row,
        )
        if distance is None:
            continue

        results.append({"item": item, "score": 1 - distance / max_length})

    return quick_sort(results, lambda a, b: b["score"] - a["score"])
