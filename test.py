import subprocess
from pathlib import Path

base = Path(r"C:\Users\crazy\Documents\git\baekjoon\rust\target\debug")
# with open(base / "input.txt", encoding="utf-8") as f:

for i in range(1, 1000):
    for j in range(1, 1000):
        proc = subprocess.Popen(
            [base / "baekjoon.exe"], stdin=subprocess.PIPE, stdout=subprocess.PIPE
        )

        input_str = "".join([str(a) for a in range(i, j + 1)])
        proc.stdin.write(input_str.encode("utf-8"))
        proc.stdin.close()
        proc.wait(timeout=2)

        output = b""
        while bytes := proc.stdout.read():
            output += bytes
        output = output.decode("utf-8")
        output = [int(a) for a in output.split(" ")]
        output = (output[0], output[1])

        print(i, j, input_str)
        assert (i, j) == output
        proc.terminate()
