import sys

count = [0] * 10001

for _ in range(int(sys.stdin.readline())):
    count[int(sys.stdin.readline())] += 1

for num, c in enumerate(count):
    if c == 0:
        continue

    for _ in range(c):
        print(num)
