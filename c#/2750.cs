using System;

class Program
{
    static void Main(string[] args)
    {
        int N = int.Parse(Console.ReadLine());

        int[] Arr = Enumerable.Range(0, N)
            .Select(_ => int.Parse(Console.ReadLine()))
            .ToArray();

        QuickSort(Arr, 0, N - 1);

        Console.WriteLine(string.Join("\n", Arr));
    }

    static void QuickSort(int[] Arr, int Left, int Right)
    {
        if (Left >= Right)
        {
            return;
        }

        int i = Left;
        int j = Right;
        int pivot = Arr[(i + j) / 2];

        while (i <= j)
        {
            while (Arr[i] < pivot)
            {
                i++;
            }
            while (Arr[j] > pivot)
            {
                j--;
            }

            if (i > j)
            {
                break;
            }

            (Arr[i], Arr[j]) = (Arr[j], Arr[i]);
            i++;
            j--;
        }

        QuickSort(Arr, Left, j);
        QuickSort(Arr, i, Right);
    }
}
