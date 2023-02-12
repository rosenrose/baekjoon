import re

for title, p in re.findall(r"<div title=\"([^\"]+)\">(.+?)</div>", input()):
    print(f"title : {title}")

    for inside_p in re.findall(r"<p>(.+?)</p>", p):
        paragraph = re.sub(r"<[^>]+>", "", inside_p)
        paragraph = re.sub(r"\s+", " ", paragraph).strip()

        print(paragraph)
