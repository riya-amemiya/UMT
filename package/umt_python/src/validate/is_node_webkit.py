from .is_browser import is_browser
from .is_node import is_node


def is_node_webkit() -> bool:
    """
    Determines if the current environment is Node-Webkit.

    Node-Webkit (NW.js) exposes both the browser globals and the Node.js
    globals in the same runtime, so it is detected when both environments are
    present at once. This mirrors the TypeScript ``isNodeWebkit`` helper, which
    returns ``isBrowser() && isNode()``.

    Returns:
        True if both the browser and Node.js environments are detected,
        False otherwise.

    Example:
        >>> is_node_webkit()
        False
    """
    return is_browser() and is_node()
