const input = require("fs").readFileSync("/dev/stdin").toString().trim().split("\n");
input.shift();

const slump = String.raw`((D|E)F+)+G`;
const slimp = String.raw`(?<ab>(AB)*)A(H|${slump}C)(?<c>C*)`;
const slurpy = new RegExp(String.raw`^${slimp}${slump}$`);

console.log("SLURPYS OUTPUT");

input.forEach((line) => {
  const match = slurpy.exec(line);

  if (!match) {
    console.log("NO");
    return;
  }

  const { ab, c } = match.groups;

  console.log([...ab.matchAll(/AB/g)].length === [...c.matchAll(/C/g)].length ? "YES" : "NO");
});

console.log("END OF OUTPUT");
