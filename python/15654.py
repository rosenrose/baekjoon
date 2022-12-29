from itertools import permutations

_, m = map(int, input().split())
arr = sorted(map(int, input().split()))

print(*[" ".join(map(str, i)) for i in permutations(arr, m)], sep="\n")
