import sys
from random import randint


def main():
    N = int(sys.stdin.readline())
    arr = [int(sys.stdin.readline()) for _ in range(N)]

    # arr = quick_sort(arr)
    arr = merge_sort(arr)

    print(*arr, sep="\n")


def quick_sort(arr):
    if (length := len(arr)) <= 1:
        return arr

    pivot = arr[randint(0, length - 1)]
    i = 0
    j = length - 1

    while i <= j:
        while arr[i] < pivot:
            i += 1

        while arr[j] > pivot:
            j -= 1

        if i > j:
            break

        arr[i], arr[j] = arr[j], arr[i]
        i += 1
        j -= 1

    arr[: j + 1] = quick_sort(arr[: j + 1])
    arr[i:] = quick_sort(arr[i:])

    return arr


def merge_sort(arr):
    if (length := len(arr)) <= 1:
        return arr

    pivot = length // 2

    arr[:pivot] = merge_sort(arr[:pivot])
    arr[pivot:] = merge_sort(arr[pivot:])

    a, b = 0, pivot
    temp = []

    for _ in range(length):
        if a < pivot and b < length:
            if arr[a] < arr[b]:
                temp.append(arr[a])
                a += 1
            else:
                temp.append(arr[b])
                b += 1
        else:
            if a == pivot:
                temp.append(arr[b])
                b += 1
            else:
                temp.append(arr[a])
                a += 1

    return temp


if __name__ == "__main__":
    main()
