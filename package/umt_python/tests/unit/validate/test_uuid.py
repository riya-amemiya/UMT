import unittest

from src.validate import string, uuid


class TestUuid(unittest.TestCase):
    def test_validates_version_4_uuids_by_default(self):
        rule = uuid()
        self.assertTrue(rule.validate("123e4567-e89b-42d3-a456-426614174000"))
        self.assertTrue(rule.validate("3A86B1AB-237E-473D-A38F-06AA6A2A4783"))
        self.assertTrue(rule.validate("A95E3F622D0E425795D3B239AF08115D"))
        self.assertTrue(rule.validate("123e4567-E89b-42D3-a456-426614174000"))

    def test_rejects_invalid_uuids(self):
        rule = uuid()
        self.assertFalse(rule.validate("123e4567-e89b-42d3-a456-42661417400Z"))
        self.assertFalse(rule.validate("123e4567-e89b-92d3-a456-426614174000"))
        self.assertFalse(rule.validate("123e4567-e89b-42d3-c456-426614174000"))
        self.assertFalse(rule.validate("completely-invalid-format"))
        self.assertFalse(rule.validate(""))

    def test_rejects_wrong_lengths(self):
        rule = uuid()
        self.assertFalse(rule.validate("123e4567-e89b-42d3-a456-42661417400"))
        self.assertFalse(rule.validate("123e4567-e89b-42d3-a456-4266141740001234"))

    def test_rejects_a_trailing_newline(self):
        self.assertFalse(uuid().validate("123e4567-e89b-42d3-a456-426614174000\n"))

    def test_rejects_non_ascii_digits(self):
        value = chr(0x0661) * 8 + "-e89b-42d3-a456-426614174000"
        self.assertFalse(uuid().validate(value))

    def test_validates_the_requested_versions(self):
        rule = uuid([1, 2, 3, 4, 5])
        for version in (1, 2, 3, 4, 5):
            self.assertTrue(
                rule.validate(f"123e4567-e89b-{version}2d3-a456-426614174000")
            )
        self.assertFalse(rule.validate("123e4567-e89b-62d3-a456-426614174000"))

    def test_rejects_everything_when_all_versions_are_out_of_range(self):
        rule = uuid([0, 8, 99])
        self.assertFalse(rule.validate("123e4567-e89b-42d3-a456-426614174000"))
        self.assertFalse(rule.validate("123e4567-e89b-82d3-a456-426614174000"))

    def test_rejects_everything_for_an_empty_version_list(self):
        self.assertFalse(uuid([]).validate("123e4567-e89b-42d3-a456-426614174000"))

    def test_filters_out_invalid_versions_and_keeps_valid_ones(self):
        rule = uuid([4, 99])
        self.assertTrue(rule.validate("123e4567-e89b-42d3-a456-426614174000"))
        self.assertFalse(rule.validate("123e4567-e89b-92d3-a456-426614174000"))

    def test_ignores_boolean_versions(self):
        rule = uuid([True])
        self.assertFalse(rule.validate("123e4567-e89b-12d3-a456-426614174000"))

    def test_carries_the_custom_message(self):
        self.assertEqual(uuid([4], "bad uuid").message, "bad uuid")
        self.assertEqual(uuid([0], "bad uuid").message, "bad uuid")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(uuid().message, "")

    def test_composes_with_string(self):
        validator = string([uuid([4], "bad uuid")])
        valid = validator("123e4567-e89b-42d3-a456-426614174000")
        self.assertTrue(valid.validate)
        self.assertEqual(valid.message, "")
        invalid = validator("123e4567-e89b-12d3-a456-42661417400Z")
        self.assertFalse(invalid.validate)
        self.assertEqual(invalid.message, "bad uuid")


if __name__ == "__main__":
    unittest.main()
