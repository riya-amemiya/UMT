import unittest

from src.color import hexa_to_rgba


class TestHexaToRgba(unittest.TestCase):
    def test_basic_cases(self):
        result = hexa_to_rgba("#00000000")
        self.assertEqual(result["r"], 0)
        self.assertEqual(result["g"], 0)
        self.assertEqual(result["b"], 0)
        self.assertEqual(result["a"], 0.0)

    def test_shorthand(self):
        result = hexa_to_rgba("#fff")
        self.assertEqual(result["r"], 255)
        self.assertEqual(result["g"], 255)
        self.assertEqual(result["b"], 255)
        self.assertEqual(result["a"], 1.0)

    def test_six_digit(self):
        result = hexa_to_rgba("#ff0000")
        self.assertEqual(result["r"], 255)
        self.assertEqual(result["g"], 0)
        self.assertEqual(result["b"], 0)
        self.assertEqual(result["a"], 1.0)

    def test_invalid_input(self):
        with self.assertRaises(ValueError):
            hexa_to_rgba("invalid")
        with self.assertRaises(ValueError):
            hexa_to_rgba("#gg0000")

    def test_docstring_example(self):
        result = hexa_to_rgba("#00000000")
        self.assertEqual(result, {"r": 0, "g": 0, "b": 0, "a": 0.0})


if __name__ == "__main__":
    unittest.main()
