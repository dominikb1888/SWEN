from math import floor, sqrt
from timeit import timeit


def is_prime(p: int) -> bool:
    """Return whether p is prime."""
    possible_divisors = range(2, floor(sqrt(p)) + 1)
    return (
        p > 1 and
        all(not p % d == 0 for d in possible_divisors)
    )


def test_is_prime_performance() -> None:
    """Test the efficiency of is_prime."""
    numbers_to_test = range(2, 10000)
    for number in numbers_to_test:
        time = timeit(f'is_prime({number})', number=100, globals=globals())
        print(f"is_prime({number}) took {time} to evaluate")
        assert time < 0.001, 'Failed performance constraint of 0.001s.'
