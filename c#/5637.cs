using System;
using System.Text;
using System.Text.RegularExpressions;

class Program
{
    static void Main()
    {
        StringBuilder builder = new();
        string line;

        while ((line = Console.ReadLine()) != null)
        {
            builder.Append(line + "\n");
        }

        Regex regex = new(@"[a-zA-Z-]+", RegexOptions.Compiled);
        var words = regex.Matches(builder.ToString()).Select(m => m.Value);
        int longest = words.Select(w => w.Length).Max();

        Console.WriteLine(words.First(w => w.Length == longest).ToLower());
    }
}
