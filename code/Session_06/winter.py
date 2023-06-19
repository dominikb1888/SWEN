def emphasize_v2(words: list[str]) -> None:
    """Add emphasis to the end of a list of words."""
    new_words = ['believe', 'me!']
    words = words + new_words
    return words

# In the Python console
sentence = ['summer', 'is', 'coming']
new_sentence = emphasize_v2(sentence)
print(sentence)
print(new_sentence)
