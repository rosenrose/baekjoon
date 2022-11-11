using System;
using System.Text.RegularExpressions;

class Program
{
    static void Main()
    {
        int T = int.Parse(Console.ReadLine());
        Regex regex = new(@"^[A-F]?A+F+C+[A-F]?$", RegexOptions.Compiled);

        foreach (var line in Enumerable.Range(0, T).Select(_ => Console.ReadLine()))
        {
            Console.WriteLine(regex.IsMatch(line) ? "Infected!" : "Good");
        }
    }
}
