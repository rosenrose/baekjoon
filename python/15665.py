from itertools import product

m = int(input().split()[1])
arr = sorted(map(int, input().split()))

print(*[" ".join(map(str, i)) for i in sorted(set(product(arr, repeat=m)))], sep="\n")
