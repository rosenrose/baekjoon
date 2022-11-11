const input = require("fs").readFileSync("/dev/stdin").toString().trim().split("\n");
input.shift();

const pattern = input.shift().split("*");
const regex = new RegExp(String.raw`^${pattern[0]}.*${pattern.at(-1)}$`);

input.forEach((line) => {
  console.log(regex.test(line) ? "DA" : "NE");
});
