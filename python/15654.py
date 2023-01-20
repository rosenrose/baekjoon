from itertools import permutations

m = int(input().split()[1])

print(
    *[" ".join(map(str, i)) for i in permutations(sorted(map(int, input().split())), m)], sep="\n"
)
