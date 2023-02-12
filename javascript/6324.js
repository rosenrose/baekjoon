const input = require("fs").readFileSync("/dev/stdin").toString().trim().split("\n");

input.slice(1).forEach((line, i) => {
  const [protocol, url] = [line.split("://")[0], line.slice(line.indexOf("://") + "://".length)];
  const [host, port] = url.split("/")[0].split(":");
  const path = url.indexOf("/") > 0 ? url.slice(url.indexOf("/") + 1) : "";

  console.log(`URL #${i + 1}`);
  console.log(`Protocol = ${protocol}`);
  console.log(`Host     = ${host}`);
  console.log(`Port     = ${port || "<default>"}`);
  console.log(`Path     = ${path || "<default>"}`);
  console.log("");
});
