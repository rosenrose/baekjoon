import subprocess
import sys
from pathlib import Path
from random import randint

sys.set_int_max_str_digits(10001)
base = Path(__file__) / "../../rust"
subprocess.run(["cargo", "build"], cwd=base)

for _ in range(randint(1000, 1000)):
    a = int(
        ("-" if randint(0, 1) == 1 else "")
        + str(randint(1, 9))
        + "".join([str(randint(0, 9)) for _ in range(randint(10, 18))])
    )
    b = int(str(randint(1, 9)) + "".join([str(randint(0, 9)) for _ in range(randint(10, 18))]))

    q, r = divmod(a, b)

    if a != 0 and b < 0:
        q, r = q + 1, r - b

    (in_txt := Path(__file__).with_name("16428_in.txt")).write_text(f"{a} {b}")
    (ans_txt := Path(__file__).with_name("16428_ans.txt")).write_text(f"{q}\n{r}\n")
    out_txt = Path(__file__).with_name("16428_out.txt")

    subprocess.run(
        ["pwsh", "-c", f"cat {in_txt} | {base / 'target/Debug/baekjoon.exe'} > {out_txt}"]
    )

    if ans_txt.read_text() != out_txt.read_text():
        input("false")
