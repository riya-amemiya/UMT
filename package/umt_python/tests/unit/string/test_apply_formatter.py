import unittest

from src.string import apply_formatter, js_string


def _pad(value, length, char, *_):
    from src.string import pad_start

    return pad_start(js_string(value), int(length), char)


def _currency(value, locale, currency, *_):
    from src.number import format_number

    return format_number(
        float(value), locale=locale, style="currency", currency=currency
    )


def _multiply(value, factor, *_):
    product = float(value) * float(factor)
    return str(int(product)) if product == int(product) else str(product)


MOCK_FORMATTERS = {
    "upper": lambda value, *_: js_string(value).upper(),
    "lower": lambda value, *_: js_string(value).lower(),
    "pad": _pad,
    "currency": _currency,
    "multiply": _multiply,
}


class TestApplyFormatterSimple(unittest.TestCase):
    def test_uppercase(self):
        self.assertEqual(apply_formatter("hello", "upper", MOCK_FORMATTERS), "HELLO")
        self.assertEqual(apply_formatter("WORLD", "upper", MOCK_FORMATTERS), "WORLD")
        self.assertEqual(apply_formatter(123, "upper", MOCK_FORMATTERS), "123")

    def test_lowercase(self):
        self.assertEqual(apply_formatter("HELLO", "lower", MOCK_FORMATTERS), "hello")
        self.assertEqual(apply_formatter("WoRlD", "lower", MOCK_FORMATTERS), "world")
        self.assertEqual(apply_formatter(456, "lower", MOCK_FORMATTERS), "456")


class TestApplyFormatterArguments(unittest.TestCase):
    def test_single_argument(self):
        self.assertEqual(apply_formatter(42, "multiply(2)", MOCK_FORMATTERS), "84")
        self.assertEqual(apply_formatter(10, "multiply(0.5)", MOCK_FORMATTERS), "5")

    def test_multiple_arguments(self):
        self.assertEqual(apply_formatter(42, "pad(4,0)", MOCK_FORMATTERS), "0042")
        self.assertEqual(apply_formatter(7, "pad(3, )", MOCK_FORMATTERS), "  7")
        self.assertEqual(apply_formatter("test", "pad(6,*)", MOCK_FORMATTERS), "**test")

    def test_quoted_arguments(self):
        self.assertEqual(apply_formatter(5, 'pad(3,"x")', MOCK_FORMATTERS), "xx5")
        self.assertEqual(apply_formatter(5, "pad(3,'y')", MOCK_FORMATTERS), "yy5")

    def test_currency_formatter(self):
        result = apply_formatter(1234.56, "currency(en-US,USD)", MOCK_FORMATTERS)
        self.assertEqual(result, "$1,234.56")

    def test_trims_whitespace_in_arguments(self):
        self.assertEqual(apply_formatter(42, "pad( 4 , 0 )", MOCK_FORMATTERS), "0042")
        self.assertEqual(apply_formatter(5, "multiply( 3 )", MOCK_FORMATTERS), "15")


class TestApplyFormatterInvalid(unittest.TestCase):
    def test_invalid_syntax_returns_string_value(self):
        self.assertEqual(apply_formatter("test", "invalid!@#", MOCK_FORMATTERS), "test")
        self.assertEqual(apply_formatter(123, "bad-format", MOCK_FORMATTERS), "123")
        self.assertEqual(apply_formatter("hello", "upper(", MOCK_FORMATTERS), "hello")
        self.assertEqual(apply_formatter("world", "lower)", MOCK_FORMATTERS), "world")

    def test_non_existent_formatter(self):
        self.assertEqual(
            apply_formatter("test", "nonexistent", MOCK_FORMATTERS), "test"
        )
        self.assertEqual(apply_formatter(456, "missing(arg)", MOCK_FORMATTERS), "456")


class TestApplyFormatterEdgeCases(unittest.TestCase):
    def test_empty_arguments(self):
        self.assertEqual(apply_formatter("test", "upper()", MOCK_FORMATTERS), "TEST")

    def test_different_value_types(self):
        self.assertEqual(apply_formatter(None, "upper", MOCK_FORMATTERS), "NULL")
        self.assertEqual(apply_formatter(True, "upper", MOCK_FORMATTERS), "TRUE")
        self.assertEqual(apply_formatter(False, "lower", MOCK_FORMATTERS), "false")
        self.assertEqual(
            apply_formatter({}, "upper", MOCK_FORMATTERS), "[OBJECT OBJECT]"
        )
        self.assertEqual(apply_formatter([1, 2, 3], "upper", MOCK_FORMATTERS), "1,2,3")

    def test_empty_formatters(self):
        self.assertEqual(apply_formatter("test", "upper", {}), "test")
        self.assertEqual(apply_formatter(123, "nonexistent", {}), "123")

    def test_complex_argument_parsing(self):
        self.assertEqual(apply_formatter("x", "pad(5,abc)", MOCK_FORMATTERS), "abcax")
        self.assertEqual(apply_formatter("y", 'pad(4,"z,w")', MOCK_FORMATTERS), "z,wy")

    def test_empty_arguments_in_list(self):
        self.assertEqual(apply_formatter("x", "pad(3,)", MOCK_FORMATTERS), "  x")
        self.assertEqual(apply_formatter("y", "pad(4,,)", MOCK_FORMATTERS), "   y")


class TestApplyFormatterNameValidation(unittest.TestCase):
    def test_only_word_characters_in_names(self):
        invalid_formatters = {
            "with-dash": lambda v, *_: str(v),
            "with.dot": lambda v, *_: str(v),
            "with space": lambda v, *_: str(v),
        }
        self.assertEqual(
            apply_formatter("test", "with-dash", invalid_formatters), "test"
        )
        self.assertEqual(
            apply_formatter("test", "with.dot", invalid_formatters), "test"
        )
        self.assertEqual(
            apply_formatter("test", "with space", invalid_formatters), "test"
        )


if __name__ == "__main__":
    unittest.main()
