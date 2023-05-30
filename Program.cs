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
            if (OperatingSystem.IsWindows()) // Attempt native resize on Windows
            {
                Console.SetWindowSize(1, 1); // done to be able to set the buffer to the correct size
                Console.SetBufferSize(70, 50);
                Console.SetWindowSize(70, 50);
            }
            else // Attempt ANSI escape resize otherwise
            {
                Console.Write("\u001b[8;50;70t");
            }

            Console.Clear();

            var pf = new Playfield(args);

            while (true)
            {
                Console.CursorVisible = false;
                var tick = pf.Tick();
                if (!pf.quiet && (tick % 10) == 0)
                {
                    pf.Render();
                    Console.Write(pf.builder);
                    Thread.Sleep(10);
                }
                if (pf.Finished()) break;
                Console.SetCursorPosition(0, 0);
            } 
            
            Console.WriteLine("done.");
        }
    }
}
