def print_pairs(n: int, start: int = None, end: int = None) -> None:
    """Print all combinations of pairs of the first n natural numbers."""
    rlist = list(range(0,n))
    listc = [(i, rlist) for i in range(0, n)]
    print(listc[start:end])

if __name__ == "__main__":
    print_pairs(10000, 9900, 10000)
