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
            

            // Only resize console on Windows, the only supported platform.
            if (OperatingSystem.IsWindows())
            {
                Console.SetWindowSize(1, 1);
                Console.SetBufferSize(80, 50);
                Console.SetWindowSize(80, 50);
            }

            Console.Clear();

            var pf = new Playfield(args);

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
