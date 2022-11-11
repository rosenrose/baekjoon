const input = require("fs").readFileSync("/dev/stdin").toString().trim();

console.log(/^(100+1+|01)+$/.test(input) ? "SUBMARINE" : "NOISE");
