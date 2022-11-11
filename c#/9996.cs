using System;
using System.Text.RegularExpressions;

class Program
{
    static void Main()
    {
        int N = int.Parse(Console.ReadLine());
        string[] Pattern = Console.ReadLine().Split('*');

        Regex regex = new(@$"^{Pattern[0]}.*{Pattern[^1]}+$", RegexOptions.Compiled);

        foreach (var line in Enumerable.Range(0, N).Select(_ => Console.ReadLine()))
        {
            Console.WriteLine(regex.IsMatch(line) ? "DA" : "NE");
        }
    }
}
