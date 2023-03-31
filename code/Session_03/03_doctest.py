def is_even(n: int) -> bool:
    """ Return whether n is even

    >>> is_even(1)
    False

    >>> is_even(2)
    True
    """
    return n % 2 == 0



if __name__ == '__main__':
    import doctest     # import the doctest library
    doctest.testmod(verbose=True)  # run the tests
