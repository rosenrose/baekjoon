using System;

class Program
{
    static void Main()
    {
        string line;

        while ((line = Console.ReadLine()) != null)
        {
            while (line.Contains("BUG"))
            {
                line = line.Replace("BUG", "");
            }

            Console.WriteLine(line);
        }
    }
}
