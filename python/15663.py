from itertools import permutations

m = int(input().split()[1])
arr = sorted(map(int, input().split()))

print(*[" ".join(map(str, i)) for i in sorted(set(permutations(arr, m)))], sep="\n")
