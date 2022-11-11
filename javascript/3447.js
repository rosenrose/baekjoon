const input = require("fs").readFileSync("/dev/stdin").toString().trim().split("\n");

input.forEach((line) => {
  while (line.includes("BUG")) {
    line = line.replaceAll("BUG", "");
  }

  console.log(line);
});
