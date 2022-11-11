import re

print("SUBMARINE" if re.match(r"^(100+1+|01)+$", input().strip()) else "NOISE")
