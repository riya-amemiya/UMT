import re

def format_string(template: str, *values: object) -> str:
    """
    Replaces placeholders in a template string with specified values.
    Placeholders are in the format {0}, {1}, {2}... and corresponding
    values are passed as arguments.

    Args:
        template: Template string containing placeholders.
        *values: Values to replace the placeholders in the template.

    Returns:
        String with placeholders replaced with values.

    Example:
        >>> format_string("Hello, {0}!", "World")
        'Hello, World!'
        >>> format_string("The sum of {0} and {1} is {2}.", 1, 2, 3)
        'The sum of 1 and 2 is 3.'
        >>> format_string("Not enough values: {0} {1}", "val1") # Mimics TS behavior
        'Not enough values: val1 {1}'
    """
    def replace(match):
        index = int(match.group(1))
        if index < len(values):
            return str(values[index])
        return match.group(0) # Return the original placeholder if value not found

    return re.sub(r'{(\d+)}', replace, template)
