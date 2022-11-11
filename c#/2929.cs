using System;
using System.Text.RegularExpressions;

class Program
{
    static void Main()
    {
        var ops = new Regex(@"(?<=[A-Z])[a-z]*").Matches(Console.ReadLine()).Select(m => m.Value).ToArray();

        Console.WriteLine(ops[..^1].Select(op => 3 - (op.Length % 4)).Sum());
    }
}
