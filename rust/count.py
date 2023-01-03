import sys
import re
from pathlib import Path

rs = (Path(__file__) / "../src").rglob("*.rs")

for r in rs:
    text = r.read_text(encoding="utf-8")

    if (count := text.count("input.next().unwrap()")) >= int(sys.argv[1]):
        print(r.name)

    if re.match(r"input\.next\(\)\.unwrap\(\),\n+", text):
        print("re", r.name, count)
