const input = require("fs").readFileSync("/dev/stdin").toString().trim().split("\n");

const N = parseInt(input.shift());
const arr = input.map((i) => parseInt(i));

quickSort(arr, 0, N - 1);

console.log(arr.join("\n"));

function quickSort(arr, left, right) {
  if (left >= right) {
    return arr;
  }

  let [i, j] = [left, right];
  const pivot = arr.at((i + j) / 2);

  while (i <= j) {
    while (arr[i] < pivot) {
      i++;
    }
    while (arr[j] > pivot) {
      j--;
    }

    if (i > j) {
      break;
    }

    [arr[i], arr[j]] = [arr[j], arr[i]];
    i++;
    j--;
  }

  quickSort(arr, left, j);
  quickSort(arr, i, right);
}
