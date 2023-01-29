import re


def sanitize(string):
    string = string.strip().lower()
    string = re.sub(r"\s{2,}", " ", string)
    string = re.sub(r"[{\[]", "(", string)
    string = re.sub(r"[}\]]", ")", string)
    string = string.replace(";", ",")
    string = re.sub(r"\s*([().,:])\s*", r"\1", string)

    return string


for i in range(1, K := int(input()) + 1):
    print(f"Data Set {K}: {'equal' if sanitize(input()) == sanitize(input()) else 'not equal'}")

    if i < K:
        print("")
