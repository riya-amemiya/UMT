from datetime import datetime, timezone


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
        local_tz = datetime.now(timezone.utc).astimezone().tzinfo
        local_now = datetime.now(local_tz)
        utc_now = datetime.now(timezone.utc)
        offset_seconds = (
            local_now.replace(tzinfo=None) - utc_now.replace(tzinfo=None)
        ).total_seconds()
    else:
        offset = instance.utcoffset()
        offset_seconds = 0 if offset is None else offset.total_seconds()

    neg_minutes = int(offset_seconds / 60)
    minutes = abs(neg_minutes)
    hour_offset = minutes // 60
    minute_offset = minutes % 60

    sign = "+" if neg_minutes >= 0 else "-"
    return f"{sign}{hour_offset:02d}:{minute_offset:02d}"
