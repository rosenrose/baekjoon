from itertools import combinations

m = int(input().split()[1])

print(
    *[" ".join(map(str, i)) for i in combinations(sorted(map(int, input().split())), m)], sep="\n"
)
