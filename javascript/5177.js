const input = require("fs").readFileSync("/dev/stdin").toString().split("\n");

const sanatize = (str) => {
  str = str.trim().toLowerCase();
  str = str.replaceAll(/\s{2,}/g, " ");
  str = str.replaceAll(/[{\[]/g, "(");
  str = str.replaceAll(/[}\]]/g, ")");
  str = str.replaceAll(";", ",");
  str = str.replaceAll(/\s*([().,:])\s*/g, "$1");

  return str;
};

const N = parseInt(input.shift());

for (let i = 1; i <= N; i++) {
  console.log(
    `Data Set ${i}: ${sanatize(input.shift()) === sanatize(input.shift()) ? "equal" : "not equal"}`
  );

  if (i < N) {
    console.log("");
  }
}
