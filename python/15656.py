from itertools import product

m = int(input().split()[1])

print(
    *[" ".join(map(str, i)) for i in product(sorted(map(int, input().split())), repeat=m)], sep="\n"
)
