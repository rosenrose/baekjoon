const input = require("fs").readFileSync("/dev/stdin").toString().trim().split("\n");

input.slice(0, -1).forEach((line) => {
  const match = /^"([A-Z ]+)" \1$/.exec(line);
  console.log(match ? `Quine(${match[1]})` : "not a quine");
});
