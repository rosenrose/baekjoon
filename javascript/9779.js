const input = require("fs").readFileSync("/dev/stdin").toString().trim().split("\n");

input.forEach((line) => {
  console.log(/^da+dd?(i|y)$/.test(line) ? "She called me!!!" : "Cooing");
});
