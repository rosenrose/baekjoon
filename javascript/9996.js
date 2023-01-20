const input = require("fs").readFileSync("/dev/stdin").toString().trim().split("\n");

const [_, pattern] = [input.shift(), input.shift().split("*")];
const regex = new RegExp(String.raw`^${pattern[0]}.*${pattern.at(-1)}$`);

input.forEach((line) => {
  console.log(regex.test(line) ? "DA" : "NE");
});
