import re
from datetime import datetime

with open("chat.txt", "rb") as file:
    output = []
    for line in file:
        # check output is not a bytstring
        line = line.decode("utf-8")

        # check split was safe, test data with more than one dash, colon
        datetime_obj, text = line.split("-", 1)
        datetime_obj = datetime.strptime(
            datetime_obj.strip(), "%m/%d/%y, %I:%M %p")
        name, message = text.split(":", 1)
        regex = re.compile(r"[\u263a-\U0001f929]")
        emojis = regex.findall(message)
        emoji_list = [emoji.encode("ascii", "namereplace").decode()
                      for emoji in emojis]

        # check if all text was successfully stripped
        output.append([datetime, name.strip(), message.strip(), emoji_list])

print(output)
