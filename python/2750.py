def main():
    N = int(input())
    arr = [int(input()) for _ in range(N)]

    # bubble_sort(arr)
    # selection_sort(arr)
    # insertion_sort(arr)
    quick_sort(arr, 0, N - 1)

    print(*arr, sep="\n")


def bubble_sort(arr):
    length = len(arr)

    for i in range(length - 1):
        for j in range(length - 1 - i):
            if arr[j] <= arr[j + 1]:
                continue

            arr[j], arr[j + 1] = arr[j + 1], arr[j]


def selection_sort(arr):
    for i in range(len(arr) - 1):
        right_part = arr[i:]
        min_index = right_part.index(min(right_part)) + i

        arr[i], arr[min_index] = arr[min_index], arr[i]


def insertion_sort(arr):
    for i in range(1, len(arr)):
        for j in range(i - 1, -1, -1):
            if arr[j] <= arr[j + 1]:
                break

            arr[j], arr[j + 1] = arr[j + 1], arr[j]


def quick_sort(arr, left, right):
    if left >= right:
        return

    i, j = left, right
    pivot = arr[(i + j) // 2]

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

    quick_sort(arr, left, j)
    quick_sort(arr, i, right)


if __name__ == "__main__":
    main()
