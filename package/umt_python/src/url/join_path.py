import re


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
            s = re.sub(r"^/+", "", s)
        if index < len(segments) - 1:
            s = re.sub(r"/+$", "", s)

        if s:
            normalized.append(s)

    return "/".join(normalized)
