using System;
using System.Text.RegularExpressions;

class Program
{
    static void Main()
    {
        int T = int.Parse(Console.ReadLine());
        Regex regex = new(@"^(100+1+|01)+$", RegexOptions.Compiled);

        foreach (var line in Enumerable.Range(0, T).Select(_ => Console.ReadLine()))
        {
            Console.WriteLine(regex.IsMatch(line) ? "YES" : "NO");
        }
    }
}
