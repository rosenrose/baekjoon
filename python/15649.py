from itertools import permutations

n, m = map(int, input().split())

print(*[" ".join(map(str, i)) for i in permutations(range(1, n + 1), m)], sep="\n")
