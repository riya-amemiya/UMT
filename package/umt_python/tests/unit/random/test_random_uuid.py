import re

from src.random import random_uuid

UUID_V4_PATTERN = re.compile(
    r"^[\da-f]{8}-[\da-f]{4}-4[\da-f]{3}-[89ab][\da-f]{3}-[\da-f]{12}$",
    re.IGNORECASE,
)


class TestRandomUuid:
    def test_returns_a_v4_formatted_uuid(self):
        assert UUID_V4_PATTERN.match(random_uuid())

    def test_returns_unique_values_across_calls(self):
        values = {random_uuid() for _ in range(100)}
        assert len(values) == 100
