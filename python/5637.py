import sys
import re

string = sys.stdin.read()
words = re.findall(r"[a-zA-Z-]+", string)
longest = max([len(w) for w in words])

print(next((w.lower() for w in words if len(w) == longest)))
