def has_more_trues(booleans: list) -> bool:
    """Return whether booleans contains more True values than False values.

    >>> has_more_trues([True, False, True])
    True
    >>> has_more_trues([True, False, False])
    False
    """
    return sum(booleans) > (len(booleans)/2)    # Function body omitted


if __name__ == '__main__':
    import pytest
    pytest.main(['test_trues.py'])
