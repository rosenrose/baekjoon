import re

for question in re.findall(r"What is([^?.]*)\?", input()):
    print(f"Forty-two is{question}.")
