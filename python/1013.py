import re

T = int(input())
regex = re.compile(r"^(100+1+|01)+$")

for _ in range(T):
    print("YES" if regex.match(input().strip()) else "NO")
