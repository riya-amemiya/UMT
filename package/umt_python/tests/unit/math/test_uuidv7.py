import re
import unittest

from src.math import uuidv7


class TestUuidv7(unittest.TestCase):
    def test_basic_format(self):
        result = uuidv7()
        pattern = (
            r"^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$"
        )
        self.assertTrue(re.match(pattern, result))

    def test_version_7_indicator(self):
        result = uuidv7()

        parts = result.split("-")
        self.assertTrue(parts[2].startswith("7"))

    def test_variant_bits(self):
        result = uuidv7()
        parts = result.split("-")

        self.assertIn(parts[3][0], ["8", "9", "a", "b"])

    def test_unique_generation(self):
        uuid1 = uuidv7()
        uuid2 = uuidv7()
        self.assertNotEqual(uuid1, uuid2)

    def test_docstring_example(self):
        pattern = (
            r"^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$"
        )
        self.assertTrue(bool(re.match(pattern, uuidv7())))


if __name__ == "__main__":
    unittest.main()
