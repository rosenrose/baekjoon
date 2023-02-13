import sys
import re

for line in sys.stdin.read().splitlines():
    print("She called me!!!" if re.match(r"^da+dd?(i|y)$", line) else "Cooing")
