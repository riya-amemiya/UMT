from src.random import seeded_random
from src.random.seeded_random import _to_uint32


def _take(seed, count=5):
    rand = seeded_random(seed)
    return [rand() for _ in range(count)]


class TestToUint32:
    def test_keeps_small_non_negative_integers(self):
        assert _to_uint32(0) == 0
        assert _to_uint32(42) == 42

    def test_wraps_negative_integers(self):
        assert _to_uint32(-1) == 0xFFFFFFFF

    def test_wraps_integers_above_32_bits(self):
        assert _to_uint32(2**32 + 5) == 5

    def test_truncates_floats_toward_zero(self):
        assert _to_uint32(1.9) == 1
        assert _to_uint32(-1.9) == 0xFFFFFFFF

    def test_maps_non_finite_floats_to_zero(self):
        assert _to_uint32(float("nan")) == 0
        assert _to_uint32(float("inf")) == 0
        assert _to_uint32(float("-inf")) == 0


class TestSeededRandomNumericSeed:
    def test_float_seed_matches_its_truncated_integer_seed(self):
        assert _take(1.5) == _take(1)
        assert _take(42.0) == _take(42)

    def test_negative_seed_matches_its_unsigned_seed(self):
        assert _take(-1) == _take(0xFFFFFFFF)

    def test_nan_seed_matches_zero_seed(self):
        assert _take(float("nan")) == _take(0)

    def test_string_seed_still_hashes(self):
        assert _take("1") != _take(1)
        assert _take("hello") == _take("hello")
