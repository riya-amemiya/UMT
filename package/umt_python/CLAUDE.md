# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

**Don't use pip, use uv.**

## Project Overview

UMT Python is a port of the TypeScript package in `package/main`. Behavior and API should match that source of truth where applicable. Each function lives in its own module file; public names are re-exported from `src/__init__.py`.

## Development Commands

```bash
make install      # uv sync
make test         # uv run pytest
make lint         # ruff format --check && ruff check
make format       # uv run ruff format
make typecheck    # uv run pyright
make all          # format, lint, typecheck, test

# Single test file
uv run pytest tests/unit/date/test_is_between.py
```

## Architecture and Structure

### Module Organization

- One function per file under `src/<module>/` (for example `src/date/is_between.py`)
- Barrel exports in `src/<module>/__init__.py` and `src/__init__.py`
- No runtime dependencies

### Testing Approach

- Unit tests: `tests/unit/<module>/test_<function>.py`
- Benchmarks: `tests/benchmark/` (`timeit`; `__file__` is not defined inside timeit strings)
- Import from `src`, not the installed package name:

```python
from src.date import is_between
from src.validate import is_number
```

### Code Style Guidelines

- `int | float` pipe unions for numeric arguments (Python 3.10+)
- Avoid `typing.Any`; use `object` if needed
- Reject `isinstance(n, bool)` in numeric validators (`bool` is an `int`)
- Use `decimal.Decimal` from strings for exact math parity with JavaScript
- Import `Callable` / `Iterable` from `collections.abc`
- Do not put `try`/`except` inside loops (`PERF203`)

### Adding New Functions

1. Implement in `src/<module>/<name>.py` with types and a docstring example
2. Re-export from `src/<module>/__init__.py` and `src/__init__.py` (`__all__`)
3. Add `tests/unit/<module>/test_<name>.py`
4. Update `README.md` when the public surface or constraints change
5. Run `make format lint typecheck test`

Date, array, and math ports must follow `package/main` tests for Sunday-start weeks, NaN partitioning in sorts, and exclusivity defaults on `is_between`.
