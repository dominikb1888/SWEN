def print_pairs(n: int) -> None:
    """Print all combinations of pairs of the first n natural numbers."""
    for i in range(0, n):
        for j in range(0, n):
            print(i, j)

if __name__ == "__main__":
    print_pairs(1000)
