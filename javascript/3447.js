const input = require("fs").readFileSync("/dev/stdin").toString().trim().split("\n");

input.forEach((line) => {
  while ((line = line.replaceAll("BUG", "")).includes("BUG")) {}

  console.log(line);
});
