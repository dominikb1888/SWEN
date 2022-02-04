## Encoding Strings

### 1. From Bytestrings to Emojis

Read the following text line by line and split it into an adequate data structure making the following items accessible:
1. Date and Time (Type Date)
2. User (Type String)
3. Text (Type String)
4. Emoji Used (Type List of Strings - Display the CLDR Name(s) instead of the Unicode Sequence)

Print the output in tabular format. There is no need to import modules like tabulate or pandas to accomplish this task.

Documentation:
- https://docs.python.org/3/howto/unicode.html
- https://unicode.org/emoji/charts/full-emoji-list.html
- https://docs.python.org/3/tutorial/inputoutput.html

Hints:
https://www.delftstack.com/howto/python/data-in-table-format-python/
https://stackoverflow.com/questions/61524478/how-to-convert-unicode-of-an-emoji-into-cldr-short-name
https://medium.com/analytics-vidhya/how-to-print-emojis-using-python-2e4f93443f7e
https://stackoverflow.com/questions/63497088/how-to-convert-bytes-utf-8-embeded-emoji-in-a-string
