const input = require("fs").readFileSync("/dev/stdin").toString();

const words = [...input.matchAll(/[a-zA-Z-]+/g)].map((match) => match[0]);
const longest = Math.max(...words.map((w) => w.length));

console.log(words.find((w) => w.length === longest).toLowerCase());
