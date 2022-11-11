using System;
using System.Text.RegularExpressions;

class Program
{
    static string sanitize(string str)
    {
        str = str.Trim().ToLower();
        str = new Regex(@"\s{2,}").Replace(str, " ");
        str = new Regex(@"[{\[]").Replace(str, "(");
        str = new Regex(@"[}\]]").Replace(str, ")");
        str = str.Replace(';', ',');
        str = new Regex(@"\s*([().,:])\s*").Replace(str, "$1");

        return str;
    }

    static void Main()
    {
        int K = int.Parse(Console.ReadLine());

        foreach (var i in Enumerable.Range(1, K))
        {
            Console.WriteLine($"Data Set {i}: {(sanitize(Console.ReadLine()) == sanitize(Console.ReadLine()) ? "equal" : "not equal")}");
            
            if (i < K)
            {
                Console.WriteLine("");
            }
        }
    }
}
