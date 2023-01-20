const input = require("fs").readFileSync("/dev/stdin").toString().split("\n");

const sanatize = (str) =>
  str
    .trim()
    .toLowerCase()
    .replaceAll(/\s{2,}/g, " ")
    .replaceAll(/[{\[]/g, "(")
    .replaceAll(/[}\]]/g, ")")
    .replaceAll(";", ",")
    .replaceAll(/\s*([().,:])\s*/g, "$1");

const N = parseInt(input.shift());

for (let i = 1; i <= N; i++) {
  console.log(
    `Data Set ${i}: ${sanatize(input.shift()) === sanatize(input.shift()) ? "equal" : "not equal"}`
  );

  if (i < N) {
    console.log("");
  }
}
