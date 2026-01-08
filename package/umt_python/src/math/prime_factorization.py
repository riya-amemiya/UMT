from typing import TypedDict


class PrimeFactor(TypedDict):
    number: int
    count: int


def prime_factorization(x: int) -> list[PrimeFactor]:
    """
    Performs prime factorization of a number.

    Args:
        x: Number to factorize.

    Returns:
        Array of prime factors and their counts.

    Example:
        >>> prime_factorization(12)
        [{'number': 2, 'count': 2}, {'number': 3, 'count': 1}]
    """
    n = 0
    copy_x = x
    out: list[PrimeFactor] = []
    index = 2
    while index * index <= copy_x:
        if copy_x % index == 0:
            n = 0
            while copy_x % index == 0:
                n += 1
                copy_x //= index
            out.append({"number": index, "count": n})
        index += 1
    # If remaining value is greater than 1, it's a prime factor
    if copy_x > 1:
        out.append({"number": copy_x, "count": 1})
    return out
