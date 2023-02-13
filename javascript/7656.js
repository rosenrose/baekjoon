const input = require("fs").readFileSync("/dev/stdin").toString();

for (const [_, question] of input.matchAll(/What is([^?.]*)\?/g)) {
  console.log(`Forty-two is${question}.`);
}
