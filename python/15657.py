from itertools import combinations_with_replacement

_, m = map(int, input().split())
arr = sorted(map(int, input().split()))

print(*[" ".join(map(str, i)) for i in combinations_with_replacement(arr, m)], sep="\n")
