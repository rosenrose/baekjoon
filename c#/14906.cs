using System;
using System.Text.RegularExpressions;

class Program
{
    static void Main()
    {
        string slump = @"((D|E)F+)+G";
        string slimp = $@"(?<ab>(AB)*)A(H|{slump}C)(?<c>C*)";
        Regex slurpy = new($@"^{slimp}{slump}$");

        Console.WriteLine("SLURPYS OUTPUT");

        int N = int.Parse(Console.ReadLine());

        foreach (var line in Enumerable.Range(0, N).Select(_ => Console.ReadLine()))
        {
            Match match = slurpy.Match(line);

            if (!match.Success)
            {
                Console.WriteLine("NO");
                continue;
            }

            string ab = match.Groups["ab"].Value;
            string c = match.Groups["c"].Value;

            Console.WriteLine(new Regex("AB").Matches(ab).Count == new Regex("C").Matches(c).Count ? "YES" : "NO");
        }

        Console.WriteLine("END OF OUTPUT");
    }
}
