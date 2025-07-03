# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

UMT Python Package is a collection of utility functions for string manipulation and formatting. The project follows a modular architecture where each function is implemented in its own module file.

## Development Commands

```bash
# Install dependencies
uv sync

# Run all tests
uv run pytest

# Run a specific test class
uv run pytest tests/test_string_utils.py::TestDeleteSpaces

# Run a specific test method
uv run pytest tests/test_string_utils.py::TestDeleteSpaces::test_basic_cases

# Format code
uv run ruff format

# Lint code
uv run ruff check

# Fix linting issues automatically
uv run ruff check --fix

# Type check with Pyright
uv run pyright
```

## Architecture and Structure

### Module Organization

Each utility function is implemented in its own Python file under `src/`:

- One function per file pattern (e.g., `delete_spaces.py` contains only the `delete_spaces` function)
- All functions are imported and re-exported through `src/__init__.py`
- Functions use pure Python with no external dependencies

### Testing Approach

- All tests are in `tests/test_string_utils.py`
- Each function has its own test class (e.g., `TestDeleteSpaces` for `delete_spaces`)
- Test classes include:
  - Basic functionality tests
  - Edge case tests
  - Docstring example verification

### Import Pattern

When testing or using functions from this package:

```python
from package.umt_python.src import function_name
```

### Code Style Guidelines

- Use type hints for all function parameters and return values
- Include comprehensive docstrings with examples
- Follow existing naming conventions (snake_case for functions and variables)
- Keep functions pure (no side effects)
- Handle edge cases gracefully

### Adding New Functions

1. Create a new file in `src/` with the function name
2. Implement the function with type hints and docstring
3. Add the import to `src/__init__.py`
4. Add the function name to `__all__` in `src/__init__.py`
5. Create a test class in `tests/test_string_utils.py`
6. Update README.md with the new function in the appropriate table

### Common Development Tasks

When making changes:

1. Always run `uv run ruff format` before committing
2. Ensure `uv run ruff check` passes
3. Ensure `uv run pyright` passes for type checking
4. Run tests with `uv run pytest` to verify functionality
5. Update README.md if adding new functions or changing signatures
