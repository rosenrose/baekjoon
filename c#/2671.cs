using System;
using System.Text.RegularExpressions;

class Program
{
    static void Main()
    {
        Console.WriteLine(new Regex(@"^(100+1+|01)+$").IsMatch(Console.ReadLine()) ? "SUBMARINE" : "NOISE");
    }
}
