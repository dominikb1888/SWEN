with open("chat.txt", "rb") as file:
    for line in file:
        print(str(line.strip()))
