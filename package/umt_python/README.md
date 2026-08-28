# UMT Python Package

UMT Python Package is a Python port of the TypeScript [`umt`](https://github.com/riya-amemiya/UMT/tree/main/package/main) utility library. `package/main` is the source of truth; this package mirrors those modules (`array`, `date`, `math`, `string`, `validate`, …) with `snake_case` names.

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
from umt_python import (
    add_business_days,
    format_string,
    is_between,
    random_string,
    to_base64,
)

random_string(10)  # e.g. "aBcD3fGh1j"
format_string("Hello, {0}! Today is {1}.", "World", "Monday")
to_base64("Hello World")  # "SGVsbG8gV29ybGQ="

from datetime import datetime

is_between(datetime(2025, 4, 15), datetime(2025, 4, 10), datetime(2025, 4, 20))
add_business_days(datetime(2025, 4, 18, 9, 30), 1)  # 2025-04-21 09:30
```

Import the published package as `umt_python`. Unit tests in this repo import from `src` (for example `from src.date import is_between`).

## Modules

Public names are re-exported from `src/__init__.py`. Grouped the same way as TypeScript:

| Module | Examples |
| --- | --- |
| `advance` | `range_advance` |
| `array` | `chunk`, `unique`, `ultra_number_sort`, `zip_arrays` |
| `async_util` | `sleep`, `parallel`, `timeout`, `debounce_async` |
| `color` | `hexa_to_rgba`, `rgba_to_hsla` |
| `consts` | `ONE_DAY_MS`, `HttpStatus` |
| `crypto` | `encode_base32`, `decode_base58` |
| `data_structure` | `LRUCache`, `TTLCache`, `PriorityQueue` |
| `date` | `is_between`, `add_business_days`, `from_unix` (see below) |
| `error` | `safe_execute`, `match_result` |
| `function` | `debounce`, `memoize`, `throttle` |
| `ip` | `ip_to_long`, `cidr_to_long`, `is_in_range`, `is_private_ip` (see below) |
| `iterator` | `lazy_map`, `lazy_filter`, `lazy_take` |
| `map` | `group_by_to_map`, `zip_to_map` |
| `math` | `gcd`, `n_cr`, `sum_precise` |
| `number` | `format_number`, `to_ordinal`, `to_percentage` |
| `object` | `object_get`, `deep_clone`, `flatten_object` |
| `predicate` | `every`, `some`, `is_nullish` |
| `random` | `random_choice`, `seeded_random` |
| `simple` | `birthday_simple`, `now_simple` |
| `string` | `format_string`, `slugify`, `levenshtein_distance` |
| `time` | `convert_time` |
| `tool` | `pipe`, `unwrap`, `parse_json` |
| `ua` | `parse_user_agent` |
| `unit` | `to_celsius`, `to_kelvin` |
| `url` | `build_url`, `parse_query_string` |
| `validate` | `is_number`, `array_of`, `parse_email` |

Not every TypeScript helper is ported yet (for example there is no `is_same`; use `is_same_day` or compare truncated values with `start_of`).

## Date helpers

Local-time calendar helpers aligned with TypeScript `package/main/src/Date`. Week boundaries are Sunday-start (JavaScript `Date#getDay()` 0, not Python `weekday()` Monday=0). There are no UTC variants.

| Function | Type | Description | Example |
| --- | --- | --- | --- |
| `is_between` | `(date, start, end, unit=None, inclusivity="()") -> bool` | Range check. Default inclusivity is exclusive on both ends. Ranges are **not** swapped. Omit `unit` for exact timestamps; with a unit, all three dates are truncated with `start_of` first. | `is_between(datetime(2025, 4, 15), datetime(2025, 4, 10), datetime(2025, 4, 20))  # True` / `is_between(datetime(2025, 4, 10), datetime(2025, 4, 10), datetime(2025, 4, 20))  # False` / `is_between(datetime(2025, 4, 10), datetime(2025, 4, 10), datetime(2025, 4, 20), None, "[]")  # True` |
| `add_business_days` | `(date, amount, holidays=None) -> datetime` | Walk calendar days until `amount` weekdays (minus optional holidays compared with `is_same_day`) have been counted. Start date is not counted: Friday + 1 is Monday. `0` returns a clone and does not snap to a business day. | `add_business_days(datetime(2025, 4, 18, 9, 30), 1)  # datetime(2025, 4, 21, 9, 30)` |
| `sub_business_days` | `(date, amount, holidays=None) -> datetime` | `add_business_days(date, -amount, holidays)` | `sub_business_days(datetime(2025, 4, 21, 9, 30), 1)  # datetime(2025, 4, 18, 9, 30)` |
| `is_business_day` | `(date, holidays=None) -> bool` | Weekday and not in `holidays` | `is_business_day(datetime(2025, 4, 21))  # True` |
| `get_quarter` | `(date) -> int` | Local month → 1–4 (Jan–Mar = 1) | `get_quarter(datetime(2025, 4, 15))  # 2` |
| `week_of_year` | `(date) -> int` | Sunday-start week index. Week 1 contains January 1 of that year. Not ISO-8601. | `week_of_year(datetime(2025, 1, 1))  # 1` / `week_of_year(datetime(2025, 1, 5))  # 2` |
| `from_unix` | `(value, unit="s") -> datetime` | Epoch to naive local datetime whose timestamp matches JavaScript `Date.getTime()` | `from_unix(0).timestamp()  # 0.0` |
| `to_unix` | `(date, unit="s") -> float` | Default seconds, floored | `to_unix(datetime(1970, 1, 1, tzinfo=timezone.utc))  # 0.0` |
| `start_of` / `end_of` | `(date, unit) -> datetime` | Boundary truncation. `unit` is `second` \| `minute` \| `hour` \| `day` \| `week` \| `month` \| `quarter` \| `year` | `start_of(datetime(2025, 4, 15, 10, 30), "day")  # datetime(2025, 4, 15, 0, 0)` |

`DateInclusivity` is `"()"` \| `"[]"` \| `"[)"` \| `"(]"`. `UnixTimeUnit` is `"s"` \| `"ms"`.

## IP helpers

IPv4 dotted-decimal only, aligned with TypeScript `package/main/src/IP`. Unlike TypeScript, these functions validate input and raise `ValueError` (or `TypeError` from `get_network_address`) on malformed values.

| Function | Type | Description | Example |
| --- | --- | --- | --- |
| `ip_to_long` / `long_to_ip` | `(ip: str) -> int` / `(long: int \| float) -> str` | Pack or unpack four octets. Rejects leading zeros (`"192.168.01.1"`). | `ip_to_long("192.168.1.1")  # 3232235777` / `long_to_ip(3232235777)  # "192.168.1.1"` |
| `cidr_to_long` / `cidr_to_subnet_mask` | `(cidr: int) -> int` / `(cidr: int) -> str` | Prefix `0`–`32`. Outside that range raises `ValueError`. | `cidr_to_long(24)  # 4294967040` / `cidr_to_subnet_mask(24)  # "255.255.255.0"` |
| `subnet_mask_to_cidr` | `(subnet_mask: str) -> int` | Requires contiguous `1` bits then `0` bits. `"255.0.255.0"` raises. TypeScript `subnetMaskToCidr` only counts bits. | `subnet_mask_to_cidr("255.255.255.0")  # 24` |
| `is_in_range` | `(remote_ip, network_ip, cidr) -> bool` | `(ip & mask) == (network & mask)`. CIDR `0` matches every IPv4 address. | `is_in_range("192.168.1.2", "192.168.1.0", 24)  # True` |
| `is_private_ip` | `(ip: str) -> bool` | RFC 1918 only: `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`. Loopback and link-local are not private. | `is_private_ip("10.0.0.1")  # True` / `is_private_ip("127.0.0.1")  # False` |
| `get_ip_class` | `(ip: str) -> str` | Classful first-octet lookup (`A`–`E`). `0.0.0.0` and malformed input return `""`. | `get_ip_class("10.0.0.1")  # "A"` |
| `get_network_address` | `(ip: str, subnet_mask: str) -> int` | Returns an unsigned 32-bit **int**, not a dotted string. | `get_network_address("192.168.1.1", "255.255.255.0")  # 3232235776` |
| `ip_to_binary_string` | `(ip: str) -> str` | 32-character `0`/`1` string, eight bits per octet. | `ip_to_binary_string("192.168.0.1")  # "11000000101010000000000000000001"` |

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

### Number Functions

| Function | Type | Description | Example |
|----------|------|-------------|---------|
| `format_number` | `(value: float \| int, *, locale: str \| None = None, minimum_fraction_digits: int \| None = None, maximum_fraction_digits: int \| None = None, style: Literal["decimal", "currency", "percent"] = "decimal", currency: str \| None = None) -> str` | Format a number with locale-aware separators, fraction-digit controls, and decimal/percent/currency styles. Mirrors `Intl.NumberFormat` from the TypeScript source. | `format_number(1234567.89)  # "1,234,567.89"` / `format_number(1234567.89, locale="de-DE")  # "1.234.567,89"` / `format_number(0.75, style="percent")  # "75%"` / `format_number(1234.5, style="currency", currency="USD")  # "$1,234.50"` |
| `to_ordinal` | `(value: int \| float) -> str` | Convert a number to its English ordinal string (handles 11th/12th/13th specially) | `to_ordinal(1)  # "1st"` / `to_ordinal(11)  # "11th"` / `to_ordinal(21)  # "21st"` |
| `to_percentage` | `(value: float \| int, total: float \| int, decimals: int = 2) -> float` | Calculate the percentage of a value relative to a total (returns 0 when total is 0) | `to_percentage(1, 3)  # 33.33` / `to_percentage(25, 100)  # 25.0` |

## Constants

- `DEFAULT_RANDOM_STRING_CHARS`: Default character pool for random string generation (ASCII letters + digits)

## Constraints

- **Python 3.10+** (`requires-python = ">=3.10"`). CI currently runs 3.10–3.15.
- `bool` is a subclass of `int` in Python. Numeric validators must reject `True` / `False` explicitly so they do not treat them as `1` / `0`.
- Exact arithmetic in math helpers uses `decimal.Decimal` constructed from strings, to match JavaScript number-string behavior.
- Date week math uses Sunday-start weeks. Do not assume ISO weeks (`datetime.isocalendar()`).
- IP helpers are IPv4 only. They validate input (TypeScript does not). `subnet_mask_to_cidr` requires a contiguous mask; TypeScript `subnetMaskToCidr` only counts set bits. `is_private_ip` is RFC 1918 only (not loopback or link-local).

## Development

This project uses `uv`. Prefer the Makefile targets (same commands as CI):

```bash
make install      # uv sync
make test         # uv run pytest
make lint         # ruff format --check && ruff check
make format       # uv run ruff format
make typecheck    # uv run pyright
make all          # format, lint, typecheck, test
```

Tests live under `tests/unit/` (mirroring `src/`) and import from `src`:

```python
from src.date import is_between
```

## License

MIT License
