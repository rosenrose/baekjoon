import subprocess
from pathlib import Path

base = Path(__file__) / "../../../rust/target/Debug"
# with open(base / "input.txt", encoding="utf-8") as f:
save = set()

for i in range(1, 1000):
    for j in range(i, 1000):
        input_str = [str(a) for a in range(i, j + 1)]
        input_str = "".join(input_str if len(input_str) > 1 else input_str[0])

        if input_str in save or len(input_str) > 2889:
            continue
        save.add(input_str)

        try:
            proc = subprocess.Popen(
                [base / "baekjoon.exe"], stdin=subprocess.PIPE, stdout=subprocess.PIPE
            )
            proc.stdin.write(input_str.encode("utf-8"))
            proc.stdin.close()
            proc.wait(timeout=2)

            output = b""
            while bytes := proc.stdout.read():
                output += bytes

            output = [int(a) for a in output.decode("utf-8").split(" ")]
            output = (output[0], output[1])
        except:
            print("\a", input_str, (i, j))
            raise RuntimeError

        proc.terminate()

        try:
            assert (i, j) == output
        except:
            print(
                "\a",
                f"input: {input_str}, expected: {(i, j)}, output: {output}",
                end=" ",
                flush=True,
            )
            raise AssertionError

print("\a")
# print((i, j), end=" ", flush=True)
