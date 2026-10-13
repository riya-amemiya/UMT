import re

_LEADING_SLASHES_RE = re.compile(r"^/+")
_TRAILING_SLASHES_RE = re.compile(r"/+$")


def join_path(*segments: str) -> str:
    """Joins multiple path segments into a single path,
    normalizing slashes between segments.

    Leading slash of the first segment is preserved.
    Trailing slash of the last segment is preserved.
    All intermediate slashes are normalized to a single slash.

    :param segments: The path segments to join
    :return: The joined and normalized path
    """
    if not segments:
        return ""

    normalized = []
    for index, segment in enumerate(segments):
        s = segment
        if index > 0:
            s = _LEADING_SLASHES_RE.sub("", s)
        if index < len(segments) - 1:
            s = _TRAILING_SLASHES_RE.sub("", s)

        if s:
            normalized.append(s)

    return "/".join(normalized)
