const input = require("fs").readFileSync("/dev/stdin").toString().trim().split("\n");

const regex = /^(100+1+|01)+$/;

input.slice(1).forEach((line) => {
  console.log(regex.test(line) ? "YES" : "NO");
});
