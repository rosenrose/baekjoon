from itertools import combinations

_, m = map(int, input().split())
arr = sorted(map(int, input().split()))

print(*[" ".join(map(str, i)) for i in combinations(arr, m)], sep="\n")
