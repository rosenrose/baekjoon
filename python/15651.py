from itertools import product

n, m = map(int, input().split())

print(*[" ".join(map(str, i)) for i in product(range(1, n + 1), repeat=m)], sep="\n")
