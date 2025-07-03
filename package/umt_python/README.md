# UMT Python Package

UMT Python Package is a collection of useful utility functions for various tasks in Python, focusing on string manipulation and formatting.

## Install

```bash
pip install umt-python

# or using uv
uv add umt-python

# or using poetry
poetry add umt-python
```

## Quick Start

```python
from umt_python import random_string, format_string, to_base64

# Generate a random string
random_str = random_string(10)
print(random_str)  # e.g., "aBcD3fGh1j"

# Format a string with placeholders
formatted = format_string("Hello, {0}! Today is {1}.", "World", "Monday")
print(formatted)  # "Hello, World! Today is Monday."

# Encode string to Base64
encoded = to_base64("Hello World")
print(encoded)  # "SGVsbG8gV29ybGQ="
```

## Function List

### String Manipulation Functions

| Function | Type | Description | Example |
|----------|------|-------------|---------|
| `delete_spaces` | `(string_: str) -> str` | Remove all whitespace characters from a string | `delete_spaces("Hello World")  # "HelloWorld"` |
| `format_string` | `(template: str, *values: object) -> str` | Replace placeholders {0}, {1}, etc. in a template string | `format_string("Sum of {0} and {1} is {2}", 1, 2, 3)  # "Sum of 1 and 2 is 3"` |
| `reverse_string` | `(char: str) -> str` | Reverse a string | `reverse_string("Hello")  # "olleH"` |
| `to_half_width` | `(string_: str) -> str` | Convert full-width characters to half-width | `to_half_width("Ｈｅｌｌｏ １２３")  # "Hello 123"` |

### Base64 Encoding Functions

| Function | Type | Description | Example |
|----------|------|-------------|---------|
| `to_base64` | `(char: str) -> str` | Encode a string to Base64 | `to_base64("Hello")  # "SGVsbG8="` |
| `from_base64` | `(base64_string: str) -> str` | Decode a Base64 string | `from_base64("SGVsbG8=")  # "Hello"` |

### String Padding Functions

| Function | Type | Description | Example |
|----------|------|-------------|---------|
| `pad_start` | `(string_: str, target_length: int, pad_string: str) -> str` | Pad string from the start to reach target length | `pad_start("123", 5, "0")  # "00123"` |
| `pad_end` | `(string_: str, target_length: int, pad_string: str) -> str` | Pad string from the end to reach target length | `pad_end("abc", 5, "0")  # "abc00"` |

### String Trimming Functions

| Function | Type | Description | Example |
|----------|------|-------------|---------|
| `trim_characters` | `(string_: str, chars: str) -> str` | Remove specified characters from both ends | `trim_characters("-.-hello-.-", "-.")  # "hello"` |
| `trim_start_characters` | `(string_: str, chars: str) -> str` | Remove specified characters from the start | `trim_start_characters("!!!hello", "!")  # "hello"` |
| `trim_end_characters` | `(string_: str, chars: str) -> str` | Remove specified characters from the end | `trim_end_characters("hello!!!", "!")  # "hello"` |

### Random String Generation

| Function | Type | Description | Example |
|----------|------|-------------|---------|
| `random_string` | `(size: int = 8, char_pool: str = DEFAULT_RANDOM_STRING_CHARS) -> str` | Generate a random string of specified length | `random_string(10)  # "aBcD3fGh1j"` |
| `random_string_initialization` | `(char_pool: str = DEFAULT_RANDOM_STRING_CHARS) -> Callable[[int], str]` | Create a custom random string generator with specific character pool | `custom_random = random_string_initialization("xyz")` → `custom_random(3)  # "xyx"` |

### String Validation Functions

| Function | Type | Description | Example |
|----------|------|-------------|---------|
| `has_no_letters` | `(text: str) -> bool` | Check if string contains no letters (only numbers, emojis, special chars) | `has_no_letters("123!@#")  # True` / `has_no_letters("abc123")  # False` |

## Constants

- `DEFAULT_RANDOM_STRING_CHARS`: Default character pool for random string generation (ASCII letters + digits)

## Requirements

- Python 3.9 or higher

## Development

This project uses `uv` for dependency management.

```bash
# Install dependencies
uv sync

# Run tests
uv run pytest

# Format code
uv run ruff format

# Lint code
uv run ruff check
```

## License

MIT License
