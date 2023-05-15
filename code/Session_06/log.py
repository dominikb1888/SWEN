import math


def print_powers_of_two(n: int) -> None:
    """Print the powers of two that are less than n.

    Preconditions:
        - n > 0
    """
    for i in range(0, math.ceil(math.log2(n))):
        print(2 ** i)

if __name__ == "__main__":
    print_powers_of_two(1000)
