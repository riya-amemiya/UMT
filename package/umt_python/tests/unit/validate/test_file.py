import io
import unittest

from src.validate import file


class TestFile(unittest.TestCase):
    def test_accepts_file_like_objects(self):
        validator = file()
        self.assertTrue(validator(io.BytesIO(b"hello")).validate)
        self.assertTrue(validator(io.StringIO("hello")).validate)

    def test_accepts_byte_buffers(self):
        validator = file()
        self.assertTrue(validator(b"hello").validate)
        self.assertTrue(validator(bytearray(b"hello")).validate)

    def test_rejects_non_file_values(self):
        validator = file()
        self.assertFalse(validator("text").validate)
        self.assertFalse(validator(42).validate)
        self.assertFalse(validator({}).validate)

    def test_returns_configured_message_on_failure(self):
        validator = file("must be a file")
        self.assertEqual(validator("text").message, "must be a file")

    def test_message_is_empty_on_success(self):
        validator = file("must be a file")
        self.assertEqual(validator(b"hello").message, "")

    def test_exposes_value_through_type_field(self):
        validator = file()
        buffer = io.BytesIO(b"hello")
        self.assertIs(validator(buffer).type, buffer)

    def test_docstring_example(self):
        validator = file()
        self.assertTrue(validator(io.BytesIO(b"hello")).validate)
        self.assertTrue(validator(b"hello").validate)
        self.assertFalse(validator("hello").validate)


if __name__ == "__main__":
    unittest.main()
