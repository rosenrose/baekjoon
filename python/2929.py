import re

ops = re.findall(r"(?<=[A-Z])[a-z]*", input())

print(sum([3 - (len(op) % 4) for op in ops[:-1]]))
