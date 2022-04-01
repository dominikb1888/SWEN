import os
from collections import Counter
from operator import itemgetter


stream = os.popen(
    "gh repo list DB-Teaching -L 10000 --json name | jq -r '.[] | .name'")
repos = stream.read().split()

ignorelist = []
with open("ignore", "r") as f:
    lines = f.read().splitlines()

unique_repos = list(set(repos) - set(lines))

names = []
for line in unique_repos:
    name = line.split("-")[-1]
    names.append(name)

namecount = dict(Counter(names))

leaderboard = sorted(namecount.items(), key=itemgetter(1), reverse=True)

for line in leaderboard:
    name, ex = line
    if ex > 1:
        print(f"{name} | {ex}")

# send email to Software Development - HI-2 _SS22: d4d8bb55.th-deg.de@emea.teams.ms
