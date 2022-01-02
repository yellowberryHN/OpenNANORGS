using System;
using System.Threading;

namespace OpenNANORGS
{
    class Program
    {
        protected static void WriteAt(string s, int x, int y)
        {
            try
            {
                //if (x > 69 || y > 39) { throw new ArgumentOutOfRangeException(); }
                Console.SetCursorPosition(x, y);
                Console.Write(s);
            }
            catch (ArgumentOutOfRangeException e)
            {
                Console.Clear();
                Console.WriteLine(e.Message);
            }
        }

        

        public static void Main(string[] args)
        {
            //int seed = (int)DateTimeOffset.Now.ToUnixTimeSeconds();
            int seed = 100;
            uint iterations = 1000000; // 1 million ticks is the default

            // Only resize console on Windows, the only supported platform.
            if (OperatingSystem.IsWindows())
            {
                Console.SetWindowSize(1, 1);
                Console.SetBufferSize(80, 50);
                Console.SetWindowSize(80, 50);
            }

            Console.CursorVisible = false;
            Random rnd = new Random();

            string RandomChar()
            {
                char r = (char)rnd.Next(32, 127);
                return r.ToString();
            }
            // Clear the screen, then save the top and left coordinates.
            Console.Clear();

            var pf = new Playfield(seed, iterations, 'A');

            /*
            while (true)
            {
                Console.Write(pf.Render());
                char t = Console.ReadKey().KeyChar;
                if (t == 'f')
                {
                    //pf.test_SetCollect();
                }
                else if (t == ' ')
                {
                    pf.Randomize();
                }
                
            }
            */

            while (true)
            {
                Console.CursorVisible = false;
                Console.Write(pf.Tick());
                Thread.Sleep(10);
                //Console.ReadKey();
                Console.SetCursorPosition(0, 0);
            }
        }
    }
}
