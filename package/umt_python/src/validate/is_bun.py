import sys


def is_bun() -> bool:
    """
    Determines if the current environment is the Bun runtime.

    This mirrors the TypeScript ``isBun`` helper, which detects Bun by
    probing for the runtime-specific ``Bun`` global. The Python equivalent
    checks the running interpreter's implementation name, mirroring the way
    the ``isNode`` helper probes the runtime itself rather than inherited
    environment variables. A standard CPython interpreter reports ``cpython``
    and never the Bun runtime.

    Returns:
        True if the current environment is the Bun runtime, False otherwise.

    Example:
        >>> is_bun()
        False
    """
    try:
        return sys.implementation.name == "bun"
    except Exception:
        return False
