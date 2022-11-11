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

for (let i = 0; i < N; i++) {
  const [s1, s2] = input.slice(i * 2, (i + 1) * 2);

  console.log(`Data Set ${i + 1}: ${sanatize(s1) === sanatize(s2) ? "equal" : "not equal"}`);

  if (i < N - 1) {
    console.log("");
  }
}
