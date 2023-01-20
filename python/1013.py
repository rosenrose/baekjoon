import re

regex = re.compile(r"^(100+1+|01)+$")

for _ in range(int(input())):
    print("YES" if regex.match(input().strip()) else "NO")
