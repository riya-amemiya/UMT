from datetime import datetime


def get_timezone_offset_string(instance: datetime) -> str:
    """
    Get timezone offset string in format "+HH:mm" or "-HH:mm".

    Args:
        instance: The datetime object to get timezone offset from

    Returns:
        The timezone offset string (e.g. "+09:00" for JST)

    Example:
        >>> from datetime import datetime, timezone, timedelta
        >>> jst = timezone(timedelta(hours=9))
        >>> get_timezone_offset_string(datetime(2021, 1, 1, tzinfo=jst))
        '+09:00'
    """
    if instance.tzinfo is None:
        local_now = datetime.now()
        utc_now = datetime.now(datetime.now().astimezone().tzinfo).replace(tzinfo=None)
        offset_seconds = (local_now - utc_now).total_seconds()
    else:
        offset = instance.utcoffset()
        if offset is None:
            offset_seconds = 0
        else:
            offset_seconds = offset.total_seconds()

    neg_minutes = int(offset_seconds / 60)
    minutes = abs(neg_minutes)
    hour_offset = minutes // 60
    minute_offset = minutes % 60

    sign = "+" if neg_minutes >= 0 else "-"
    return f"{sign}{hour_offset:02d}:{minute_offset:02d}"
