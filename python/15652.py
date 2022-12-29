from itertools import combinations_with_replacement

n, m = map(int, input().split())

print(*[" ".join(map(str, i)) for i in combinations_with_replacement(range(1, n + 1), m)], sep="\n")
