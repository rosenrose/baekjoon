const input = require("fs").readFileSync("/dev/stdin").toString().trim();

const ops = [...input.matchAll(/(?<=[A-Z])[a-z]*/g)].map((match) => match[0]);

console.log(ops.slice(0, -1).reduce((sum, op) => sum + 3 - (op.length % 4), 0));
