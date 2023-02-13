import re
import sys

for line in sys.stdin.read().splitlines()[:-1]:
    print(
        f"Quine({result[1]})" if (result := re.match(r'^"([A-Z ]+)" \1$', line)) else "not a quine"
    )
