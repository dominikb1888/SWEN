# In file test_trues.py
import trues


def test_mixture_one_more_true() -> None:
    """Test has_more_trues on a list with a mixture of True and False,
    with one more True than False.
    """
    assert trues.has_more_trues([True, False, True])


def test_mixture_one_more_false() -> None:
    """Test has_more_trues on a list with a mixture of True and False,
    with one more False than True.
    """
    assert not trues.has_more_trues([True, False, False])

