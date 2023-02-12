const input = require("fs").readFileSync("/dev/stdin").toString().trim();

for (const [_, title, p] of input.matchAll(/<div title="([^"]+)">(.+?)<\/div>/g)) {
  console.log(`title : ${title}`);

  for (const [_, inside_p] of p.matchAll(/<p>(.+?)<\/p>/g)) {
    const paragraph = inside_p
      .replaceAll(/<[^>]+>/g, "")
      .replaceAll(/\s+/g, " ")
      .trim();

    console.log(paragraph);
  }
}
