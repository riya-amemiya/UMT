import importlib.util


def is_browser() -> bool:
    """
    Determines if the current environment is a browser.

    This mirrors the TypeScript ``isBrowser`` helper, which detects a browser
    by probing for environment-specific globals (``window`` and ``document``).
    The Python equivalent checks for the ``js`` module that browser-style
    runtimes such as Pyodide or Brython expose to bridge the JavaScript
    globals, while a standard server-side interpreter has no such module.

    Returns:
        True if the current environment is a browser, False otherwise.

    Example:
        >>> is_browser()
        False
    """
    try:
        return importlib.util.find_spec("js") is not None
    except Exception:
        return False
