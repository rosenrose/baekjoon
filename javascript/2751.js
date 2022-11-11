const input = require("fs").readFileSync("/dev/stdin").toString().trim().split("\n");

let arr = input.slice(1).map((i) => parseInt(i));

// arr = quickSort(arr);
arr = mergeSort(arr);

console.log(arr.join("\n"));

function quickSort(arr) {
  const len = arr.length;

  if (len <= 1) {
    return arr;
  }

  let [i, j] = [0, len - 1];
  const pivot = arr[Math.floor(Math.random() * len)];

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

  const [left, right] = [arr.slice(0, j + 1), arr.slice(i)];

  arr.splice(0, left.length, ...quickSort(left));
  arr.splice(i, right.length, ...quickSort(right));

  return arr;
}

function mergeSort(arr) {
  const len = arr.length;

  if (len <= 1) {
    return arr;
  }

  const pivot = parseInt(len / 2);
  const [left, right] = [arr.slice(0, pivot), arr.slice(pivot)];

  arr.splice(0, left.length, ...mergeSort(left));
  arr.splice(pivot, right.length, ...mergeSort(right));

  let [a, b] = [0, pivot];
  const temp = [];

  for (let i = 0; i < len; i++) {
    if (a < pivot && b < len) {
      if (arr[a] < arr[b]) {
        temp.push(arr[a]);
        a++;
      } else {
        temp.push(arr[b]);
        b++;
      }
    } else {
      if (a == pivot) {
        temp.push(arr[b]);
        b++;
      } else {
        temp.push(arr[a]);
        a++;
      }
    }
  }

  return temp;
}
