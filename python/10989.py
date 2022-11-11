import sys

N = int(sys.stdin.readline())

count = []

for i in range(N):
    num = int(sys.stdin.readline())

    while len(count) <= num:
        count.append(0)

    count[num] += 1

for num, c in enumerate(count):
    if c == 0:
        continue

    for _ in range(c):
        print(num)
