import importlib.util


def is_node() -> bool:
    """
    Determines if the current environment is a standard CPython runtime.

    This mirrors the TypeScript ``isNode`` helper, which detects Node.js by
    probing for environment-specific globals (``process`` and ``require``).
    The Python equivalent checks for the runtime-specific modules (``sys`` and
    ``os``) that are available in a standard server-side interpreter but absent
    from browser-style runtimes such as Pyodide or Brython.

    Returns:
        True if the current environment is a standard CPython runtime, False otherwise.

    Example:
        >>> is_node()
        True
    """
    try:
        return (
            importlib.util.find_spec("sys") is not None
            and importlib.util.find_spec("os") is not None
        )
    except Exception:
        return False
