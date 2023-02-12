const input = require("fs").readFileSync("/dev/stdin").toString().trim().split("\n");
const mass = { C: 12.01, H: 1.008, O: 16.0, N: 14.01 };

input.slice(1).forEach((line) => {
  const sum = [...line.matchAll(/([CHON])(\d*)/g)].reduce(
    (acc, [_, atom, num]) => mass[atom] * parseInt(num || 1) + acc,
    0
  );

  console.log(sum.toFixed(3));
});
