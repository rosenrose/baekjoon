import sys

for line in sys.stdin.read().splitlines():
    while "BUG" in (line := line.replace("BUG", "")):
        ...

    print(line)
