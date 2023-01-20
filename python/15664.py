from itertools import combinations

m = int(input().split()[1])
arr = sorted(map(int, input().split()))

print(*[" ".join(map(str, i)) for i in sorted(set(combinations(arr, m)))], sep="\n")
