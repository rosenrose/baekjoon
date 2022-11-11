using System;

class Program
{
    static void Main(string[] args)
    {
        int N = int.Parse(Console.ReadLine());

        int[] Arr = Enumerable.Range(0, N)
            .Select(_ => int.Parse(Console.ReadLine()))
            .ToArray();

        // QuickSort(Arr, 0, N - 1);
        MergeSort(Arr, 0, N - 1);

        Console.WriteLine(string.Join("\n", Arr));
    }

    static Random rand = new();

    static void QuickSort(int[] Arr, int Left, int Right)
    {
        if (Left >= Right)
        {
            return;
        }

        var (i, j) = (Left, Right);
        int pivot = Arr[rand.Next(i, j + 1)]; // O(n^2)을 피할 수 있을지?

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

    static void MergeSort(int[] Arr, int Left, int Right)
    {
        int Len = Right - Left + 1;

        if (Len <= 1)
        {
            return;
        }

        int Pivot = (Left + Right) / 2;

        MergeSort(Arr, Left, Pivot);
        MergeSort(Arr, Pivot + 1, Right);

        var (a, b) = (Left, Pivot + 1);
        int[] temp = new int[Len];

        for (int i = 0; i < Len; i++)
        {
            if (a < Pivot + 1 && b < Right + 1)
            {
                if (Arr[a] < Arr[b])
                {
                    temp[i] = Arr[a];
                    a++;
                }
                else
                {
                    temp[i] = Arr[b];
                    b++;
                }
            }
            else
            {
                if (a == Pivot + 1)
                {
                    temp[i] = Arr[b];
                    b++;
                }
                else
                {
                    temp[i] = Arr[a];
                    a++;
                }
            }
        }

        Array.Copy(temp, Arr, Len);
    }
}
