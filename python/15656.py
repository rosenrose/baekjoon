from itertools import product

_, m = map(int, input().split())
arr = sorted(map(int, input().split()))

print(*[" ".join(map(str, i)) for i in product(arr, repeat=m)], sep="\n")
