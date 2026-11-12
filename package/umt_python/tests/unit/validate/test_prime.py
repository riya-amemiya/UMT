import unittest

from src.validate import number, prime


class TestPrime(unittest.TestCase):
    def test_returns_true_for_prime_numbers(self):
        rule = prime()
        for value in (2, 3, 5, 7, 11, 97, 7.0):
            self.assertTrue(rule.validate(value))

    def test_returns_false_for_non_prime_numbers(self):
        rule = prime()
        for value in (0, 1, 4, 6, 9, 100, -7, 2.5):
            self.assertFalse(rule.validate(value))

    def test_carries_the_custom_message(self):
        self.assertEqual(prime("must be prime").message, "must be prime")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(prime().message, "")

    def test_composes_with_number(self):
        validator = number([prime("must be prime")])
        self.assertTrue(validator(7).validate)
        self.assertEqual(validator(8).message, "must be prime")


if __name__ == "__main__":
    unittest.main()
