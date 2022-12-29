from itertools import permutations

_, m = map(int, input().split())
arr = sorted(map(int, input().split()))

print(*[" ".join(map(str, i)) for i in sorted(set(permutations(arr, m)))], sep="\n")
