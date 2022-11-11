const input = require("fs").readFileSync("/dev/stdin").toString().trim().split("\n");

const regex = /^[A-F]?A+F+C+[A-F]?$/;

input.slice(1).forEach((line) => {
  console.log(regex.test(line) ? "Infected!" : "Good");
});
